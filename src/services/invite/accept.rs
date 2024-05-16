use axum::http::StatusCode;

use crate::{repositories::{invite::InviteRepository, users::UserRepository}, views::{structs::ErrorResponse, CurrentUser, Database}};


pub async fn accept_invite(
    id: i32,
    db: Database,
    current_user: CurrentUser,
    token: String,
) -> Result<(), (StatusCode, ErrorResponse)> {
    let user_repo = UserRepository::new(db.clone());
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return Err((
            StatusCode::NOT_FOUND,
            ErrorResponse { error_msg: "Invite not found".to_owned() }
        )),
    };

    if invite.token != token {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse { error_msg: "Invalid token".to_owned() }
        ));
    };

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    if user.partner_id.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse { error_msg: "You already have a partner".to_owned() }
        ));
    };

    user_repo
        .update_partner(current_user.id, Some(invite.from_id))
        .await;

    user_repo
        .update_partner(invite.from_id, Some(current_user.id))
        .await;

    invite_repo
        .delete(id)
        .await;

    Ok(())
}
