pub mod structs;


use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{repositories::task_execute_request::TaskExecuteRequestRepository, services::task_execute_request::approve::change_approve_status};

use self::structs::{ConfirmQuery, TaskExecuteRequest};

use super::{structs::{Page, PaginationParams}, CurrentUser, Database};


pub async fn get_execute_requests_handler(
    Query(pagination): Query<PaginationParams>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let task_execute_request_repo = TaskExecuteRequestRepository::new(db.clone());

    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let requests_page = task_execute_request_repo
        .list(current_user.id, page, page_size)
        .await;

    (
        StatusCode::OK,
        Json(Page::<TaskExecuteRequest> {
            items: requests_page.items.into_iter().map(TaskExecuteRequest::from).collect(),
            page: requests_page.page,
            pages: requests_page.pages,
        })
    ).into_response()
}

pub async fn confirm_task_execute_request_handler(
    Path(id): Path<i32>,
    Query(ConfirmQuery{ approved }): Query<ConfirmQuery>,
    db: Database,
    current_user: CurrentUser
) -> impl IntoResponse {
    let updated_request = match change_approve_status(id, db.clone(), current_user, approved).await {
        Ok(v) => v,
        Err(err_response) => return (
            err_response.0,
            Json(err_response.1)
        ).into_response(),
    };

    (
        StatusCode::OK,
        Json(TaskExecuteRequest::from(updated_request))
    ).into_response()
}


pub async fn get_execute_request_router() -> Router {
    Router::new()
        .route("/", get(get_execute_requests_handler))
        .route("/:id/confirm/", post(confirm_task_execute_request_handler))
}
