pub mod structs;

use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};

use crate::{repositories::tasks::TaskRepository, services::tasks::execute::execute_task};

use self::structs::{CreateTask, DetailedTask, UpdateTask};

use super::{structs::{ErrorResponse, Page, PaginationParams}, CurrentUser, Database};


pub async fn create_task_handler(
    db: Database,
    current_user: CurrentUser,
    Json(data): Json<CreateTask>,
) -> impl IntoResponse {
    let task = TaskRepository::new(db.clone())
        .create(data, current_user.id)
        .await;

    (StatusCode::CREATED, Json(DetailedTask::from(task))).into_response()
}


pub async fn update_task_handler(
    Path(id): Path<i32>,
    db: Database,
    current_user: CurrentUser,
    Json(data): Json<UpdateTask>,
) -> impl IntoResponse {
    let task_repo = TaskRepository::new(db.clone());

    let task = task_repo
        .get_by_id(id)
        .await;

    if task.author_id != current_user.id {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Task not found".to_owned() })
        ).into_response();
    }

    let task = task_repo
        .update(id, data)
        .await;

    (StatusCode::OK, Json(DetailedTask::from(task))).into_response()
}


pub async fn delete_task_handler(
    Path(id): Path<i32>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let task_repo = TaskRepository::new(db.clone());

    let task = task_repo
        .get_by_id(id)
        .await;

    if task.author_id != current_user.id {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Task not found".to_owned() })
        ).into_response();
    }

    task_repo
        .delete(id)
        .await;

    StatusCode::NO_CONTENT.into_response()
}


pub async fn execute_task_handler(
    Path(id): Path<i32>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let result = execute_task(
        id,
        db.clone(),
        current_user,
    ).await;

    match result {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err((status, response)) => (status, response).into_response(),
    }
}


pub async fn get_tasks_handler(
    Query(pagination): Query<PaginationParams>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let task_repo = TaskRepository::new(db.clone());

    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let tasks_page = task_repo
        .list(current_user.id, page, page_size)
        .await;

    (
        StatusCode::OK,
        Json(Page::<DetailedTask> {
            items: tasks_page.items.into_iter().map(DetailedTask::from).collect(),
            page: tasks_page.page,
            pages: tasks_page.pages,
        })
    ).into_response()
}


pub async fn get_task_router() -> Router {
    Router::new()
        .route("/", get(get_tasks_handler))
        .route("/", post(create_task_handler))
        .route("/{id}/", put(update_task_handler))
        .route("/{id}/", delete(delete_task_handler))
        .route("/{id}/execute/", post(execute_task_handler))
}
