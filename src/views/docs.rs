use std::sync::Arc;

use aide::{
    axum::{
        routing::get,
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
    scalar::Scalar,
};
use axum::{response::IntoResponse, Extension};

use crate::extractors::Json;


pub fn get_docs_router() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::gen::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .route(
            "/",
            get(
                Scalar::new("/api/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
            )
        )
        .route(
            "/redoc",
            get(
                Redoc::new("/api/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
            )
        )
        .route("/private/api.json", get(serve_docs));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    router
}


async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
