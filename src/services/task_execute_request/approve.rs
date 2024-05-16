use axum::http::StatusCode;

use crate::{errors::AppError, prisma::task_execute_request, repositories::task_execute_request::TaskExecuteRequestRepository, views::{CurrentUser, Database}};


pub async fn change_approve_status(
    id: u32,
    db: Database,
    current_user: CurrentUser,
    new_status: bool,
) -> Result<task_execute_request::Data, (StatusCode, AppError)> {
    let task_execute_request_repo = TaskExecuteRequestRepository::new(db.clone());

    let request = task_execute_request_repo
        .get_by_id(id)
        .await;

    let request = match request {
        Some(v) => v,
        None => return Err((
            StatusCode::NOT_FOUND,
            AppError::new("Task execute request not found")
        )),
    };

    {
        let approver_id: u32 = request.approver_id.try_into().unwrap();
        if approver_id != current_user.id {
            return Err((
                StatusCode::FORBIDDEN,
                AppError::new("You are not allowed to approve this request")
            ));
        }
    }

    if request.approved.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            AppError::new("Request already approved")
        ));
    }

    let updated_request = task_execute_request_repo
        .update_approved(id, new_status)
        .await;

    Ok(updated_request)
}