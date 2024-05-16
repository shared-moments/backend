pub mod structs;

use aide::{axum::{routing::{delete_with, get_with, post_with, put_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse};

use crate::{errors::AppError, extractors::Json, repositories::tasks::TaskRepository, services::tasks::execute::execute_task};

use self::structs::{CreateTask, DetailedTask, TaskIdPath, UpdateTask};

use super::{structs::{Page, PaginationParams}, CurrentUser, Database};


pub async fn create_task_handler(
    db: Database,
    current_user: CurrentUser,
    Json(data): Json<CreateTask>,
) -> impl IntoApiResponse {
    let task = TaskRepository::new(db.clone())
        .create(data, current_user.id)
        .await;

    (StatusCode::CREATED, Json(DetailedTask::from(task))).into_response()
}

fn create_task_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<201, Json<DetailedTask>>()
        .security_requirement("jwt-key")
        .tag("tasks")
}


pub async fn update_task_handler(
    Path(TaskIdPath { id }): Path<TaskIdPath>,
    db: Database,
    current_user: CurrentUser,
    Json(data): Json<UpdateTask>,
) -> impl IntoApiResponse {
    let task_repo = TaskRepository::new(db.clone());

    let task = task_repo
        .get_by_id(id)
        .await;

    {
        let author_id: u32 = task.author_id.try_into().unwrap();
        if author_id != current_user.id {
            return (
                StatusCode::NOT_FOUND,
                AppError::new("Task not found")
            ).into_response();
        }
    }

    let task = task_repo
        .update(id, data)
        .await;

    (StatusCode::OK, Json(DetailedTask::from(task))).into_response()
}

fn update_task_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<DetailedTask>>()
        .response::<404, Json<AppError>>()
        .security_requirement("jwt-key")
        .tag("tasks")
}


pub async fn delete_task_handler(
    Path(TaskIdPath { id }): Path<TaskIdPath>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
    let task_repo = TaskRepository::new(db.clone());

    let task = task_repo
        .get_by_id(id)
        .await;

    {
        let author_id: u32 = task.author_id.try_into().unwrap();
        if author_id != current_user.id {
            return (
                StatusCode::NOT_FOUND,
                AppError::new("Task not found")
            ).into_response();
        }
    }

    task_repo
        .delete(id)
        .await;

    StatusCode::NO_CONTENT.into_response()
}

fn delete_task_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<204, ()>()
        .response::<404, Json<AppError>>()
        .security_requirement("jwt-key")
        .tag("tasks")
}


pub async fn execute_task_handler(
    Path(TaskIdPath { id }): Path<TaskIdPath>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
    let result = execute_task(
        id,
        db.clone(),
        current_user,
    ).await;

    match result {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(err) => err.into_response(),
    }
}

fn execute_task_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<201, Json<DetailedTask>>()
        .response::<400, Json<AppError>>()
        .response::<404, Json<AppError>>()
        .security_requirement("jwt-key")
        .tag("tasks")
}


pub async fn get_tasks_handler(
    Query(pagination): Query<PaginationParams>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
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

fn get_task_router_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<Page<DetailedTask>>>()
        .security_requirement("jwt-key")
        .tag("tasks")
}


pub async fn get_task_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(get_tasks_handler, get_task_router_op))
        .api_route("/", post_with(create_task_handler, create_task_op))
        .api_route("/{id}/", put_with(update_task_handler, update_task_op))
        .api_route("/{id}/", delete_with(delete_task_handler, delete_task_op))
        .api_route("/{id}/execute/", post_with(execute_task_handler, execute_task_op))
}
