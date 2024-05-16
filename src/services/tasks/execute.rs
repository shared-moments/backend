use axum::http::StatusCode;

use crate::{errors::AppError, repositories::{task_execute_log::TaskExecuteLogRepository, task_execute_request::TaskExecuteRequestRepository, tasks::TaskRepository, users::UserRepository}, views::{task::structs::ExecuteResponse, CurrentUser, Database}};


pub async fn execute_task(
    task_id: u32,
    db: Database,
    current_user: CurrentUser,
) -> Result<ExecuteResponse, AppError> {
    let user_repo = UserRepository::new(db.clone());
    let task_repo = TaskRepository::new(db.clone());

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    let allowed_authors = match user.partner_id {
        Some(partner_id) => vec![current_user.id, partner_id.try_into().unwrap()],
        None => vec![current_user.id],
    };

    let task = task_repo
        .get_by_id(task_id)
        .await;

    {
        let author_id: u32 = task.author_id.try_into().unwrap();
        if !allowed_authors.contains(&author_id) {
            return Err(
                AppError::new("Task not found")
                    .with_status(StatusCode::NOT_FOUND)
            );
        }
    }

    let is_allowed = match task.executor_id {
        Some(executor_id) => {
            let executor_id: u32 = executor_id.try_into().unwrap();
            executor_id == current_user.id
        },
        None => true,
    };

    if !is_allowed {
        return Err(
            AppError::new("You are not allowed to execute this task")
                .with_status(StatusCode::BAD_REQUEST)
        );
    }

    match task.is_need_request {
        true => {
            let task_execute_request_repo = TaskExecuteRequestRepository::new(db.clone());

            let request = task_execute_request_repo
                .create(task_id, current_user.id, task.author_id.try_into().unwrap())
                .await;

            Ok(ExecuteResponse { request_id: Some(request.id), log_id: None })
        },
        false => {
            let task_execute_log_repo = TaskExecuteLogRepository::new(db.clone());

            let log = task_execute_log_repo
                .create(task_id, task.price.try_into().unwrap(), current_user.id)
                .await;

            user_repo
                .update_balance(current_user.id, (user.balance + task.price).try_into().unwrap())
                .await;

            Ok(ExecuteResponse { request_id: None, log_id: Some(log.id) })
        }
    }
}
