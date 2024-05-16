pub mod structs;


use aide::{axum::{routing::{get_with, post_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{errors::AppError, extractors::Json, repositories::task_execute_request::TaskExecuteRequestRepository, services::task_execute_request::approve::change_approve_status};

use self::structs::{ConfirmQuery, TaskExecuteIdPath, TaskExecuteRequest};

use super::{structs::{Page, PaginationParams}, CurrentUser, Database};


pub async fn get_execute_requests_handler(
    Query(pagination): Query<PaginationParams>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
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


fn get_execute_requests_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<Page<TaskExecuteRequest>>>()
}


pub async fn confirm_task_execute_request_handler(
    Path(TaskExecuteIdPath { id }): Path<TaskExecuteIdPath>,
    Query(ConfirmQuery{ approve }): Query<ConfirmQuery>,
    db: Database,
    current_user: CurrentUser
) -> impl IntoApiResponse {
    let updated_request = match change_approve_status(id, db.clone(), current_user, approve).await {
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

fn confirm_task_execute_request_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<TaskExecuteRequest>>()
        .response::<400, Json<AppError>>()
        .response::<403, Json<AppError>>()
        .response::<404, Json<AppError>>()
}


pub async fn get_execute_request_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(get_execute_requests_handler, get_execute_requests_op))
        .api_route("/:id/confirm/", post_with(confirm_task_execute_request_handler, confirm_task_execute_request_op))
}
