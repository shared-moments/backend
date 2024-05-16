use axum::http::StatusCode;

use crate::{errors::AppError, repositories::{invite::InviteRepository, users::UserRepository}, views::{CurrentUser, Database}};


pub async fn accept_invite(
    id: u32,
    db: Database,
    current_user: CurrentUser,
    token: String,
) -> Result<(), (StatusCode, AppError)> {
    let user_repo = UserRepository::new(db.clone());
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return Err((
            StatusCode::NOT_FOUND,
            AppError::new("Invite not found")
        )),
    };

    if invite.token != token {
        return Err((
            StatusCode::BAD_REQUEST,
            AppError::new("Invalid token")
        ));
    };

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    if user.partner_id.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            AppError::new("You already have a partner")
        ));
    };

    user_repo
        .update_partner(current_user.id, Some(invite.from_id.try_into().unwrap()))
        .await;

    user_repo
        .update_partner(invite.from_id.try_into().unwrap(), Some(current_user.id))
        .await;

    invite_repo
        .delete(id)
        .await;

    Ok(())
}
