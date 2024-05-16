pub mod auth;
pub mod invite;
pub mod structs;
pub mod task;
pub mod execute_log;
pub mod execute_request;

use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{db::get_prisma_client, prisma::PrismaClient};

use self::{auth::get_auth_router, execute_log::get_execute_log_router, invite::get_invite_router, task::get_task_router};


pub type Database = Extension<Arc<PrismaClient>>;
pub type CurrentUser = Extension<auth::structs::CurrentUser>;


pub async fn get_router() -> Router {
    let client = Arc::new(get_prisma_client().await);

    let public_router = Router::new()
        .nest("/api/auth", get_auth_router().await)
        .layer(Extension(client.clone()));

    let app_router = Router::new()
        .nest("/api/invites/", get_invite_router().await)
        .nest("/api/tasks/", get_task_router().await)
        .nest("/api/execute-logs/", get_execute_log_router().await)
        .nest("/api/execute-requests/", execute_request::get_execute_request_router().await)
        .layer(middleware::from_fn(auth::middleware::auth))
        .layer(Extension(client));

    Router::new()
        .nest("/", public_router)
        .nest("/", app_router)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
