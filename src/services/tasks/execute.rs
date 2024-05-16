use axum::{http::StatusCode, Json};

use crate::{repositories::{task_execute_log::TaskExecuteLogRepository, task_execute_request::TaskExecuteRequestRepository, tasks::TaskRepository, users::UserRepository}, views::{structs::ErrorResponse, task::structs::ExecuteResponse, CurrentUser, Database}};


pub async fn execute_task(
    task_id: i32,
    db: Database,
    current_user: CurrentUser,
) -> Result<ExecuteResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_repo = UserRepository::new(db.clone());
    let task_repo = TaskRepository::new(db.clone());

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    let allowed_authors = match user.partner_id {
        Some(partner_id) => vec![current_user.id, partner_id],
        None => vec![current_user.id],
    };

    let task = task_repo
        .get_by_id(task_id)
        .await;

    if !allowed_authors.contains(&task.author_id) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Task not found".to_owned() })
        ));
    }

    let is_allowed = match task.executor_id {
        Some(executor_id) => executor_id == current_user.id,
        None => true,
    };

    if !is_allowed {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "You are not allowed to execute this task".to_owned() })
        ));
    }

    match task.is_need_request {
        true => {
            let task_execute_request_repo = TaskExecuteRequestRepository::new(db.clone());

            let request = task_execute_request_repo
                .create(task_id, current_user.id, task.author_id)
                .await;

            Ok(ExecuteResponse { request_id: Some(request.id), log_id: None })
        },
        false => {
            let task_execute_log_repo = TaskExecuteLogRepository::new(db.clone());

            let log = task_execute_log_repo
                .create(task_id, task.price, current_user.id)
                .await;

            user_repo
                .update_balance(current_user.id, user.balance + task.price)
                .await;

            Ok(ExecuteResponse { request_id: None, log_id: Some(log.id) })
        }
    }
}
