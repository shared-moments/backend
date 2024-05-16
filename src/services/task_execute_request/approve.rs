use axum::http::StatusCode;

use crate::{prisma::task_execute_request, repositories::task_execute_request::TaskExecuteRequestRepository, views::{structs::ErrorResponse, CurrentUser, Database}};


pub async fn change_approve_status(
    id: i32,
    db: Database,
    current_user: CurrentUser,
    new_status: bool,
) -> Result<task_execute_request::Data, (StatusCode, ErrorResponse)> {
    let task_execute_request_repo = TaskExecuteRequestRepository::new(db.clone());

    let request = task_execute_request_repo
        .get_by_id(id)
        .await;

    let request = match request {
        Some(v) => v,
        None => return Err((
            StatusCode::NOT_FOUND,
            ErrorResponse {
                error_msg: "Task execute request not found".to_string()
            }
        )),
    };

    if request.approver_id != current_user.id {
        return Err((
            StatusCode::FORBIDDEN,
            ErrorResponse {
                error_msg: "You are not allowed to approve this request".to_string()
            }
        ));
    }

    if request.approved.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse {
                error_msg: "Request already approved".to_string()
            }
        ));
    }

    let updated_request = task_execute_request_repo
        .update_approved(id, new_status)
        .await;

    Ok(updated_request)
}