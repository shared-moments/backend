pub mod auth;
pub mod invite;
pub mod structs;
pub mod task;
pub mod execute_log;
pub mod execute_request;
pub mod docs;

use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{middleware, Extension, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing::error;

use crate::{db::get_prisma_client, prisma::PrismaClient};

use self::{auth::get_auth_router, docs::get_docs_router, execute_log::get_execute_log_router, execute_request::get_execute_request_router, invite::get_invite_router, task::get_task_router};


pub type Database = Extension<Arc<PrismaClient>>;
pub type CurrentUser = Extension<auth::structs::CurrentUser>;


pub async fn get_router() -> Router {
    aide::gen::on_error(|error| {
        error!("{error}");
    });

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    let client = Arc::new(get_prisma_client().await);

    let public_router = ApiRouter::new()
        .nest("/api/docs", get_docs_router())
        .nest("/api/auth", get_auth_router().await)
        .layer(Extension(client.clone()));

    let app_router = ApiRouter::new()
        .nest("/api/invites/", get_invite_router().await)
        .nest("/api/tasks/", get_task_router().await)
        .nest("/api/execute-logs/", get_execute_log_router().await)
        .nest("/api/execute-requests/", get_execute_request_router().await)
        .layer(middleware::from_fn(auth::middleware::auth))
        .layer(Extension(client));

    ApiRouter::new()
        .nest("/", public_router)
        .nest("/", app_router)
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}


fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Shared moments OpenAPI")
        // .summary("An example Todo application")
        // .description(include_str!("README.md"))
        // .tag(Tag {
        //     name: "todo".into(),
        //     description: Some("Todo Management".into()),
        //     ..Default::default()
        // })
        // .security_scheme(
        //     "ApiKey",
        //     aide::openapi::SecurityScheme::ApiKey {
        //         location: aide::openapi::ApiKeyLocation::Header,
        //         name: "X-Auth-Key".into(),
        //         description: Some("A key that is ignored.".into()),
        //         extensions: Default::default(),
        //     },
        // )
        // .default_response_with::<Json<AppError>, _>(|res| {
        //     res.example(AppError {
        //         error: "some error happened".to_string(),
        //         error_details: None,
        //         error_id: Uuid::nil(),
        //         // This is not visible.
        //         status: StatusCode::IM_A_TEAPOT,
        //     })
        // })
}
