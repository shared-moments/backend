pub mod structs;

use axum::{extract::{self, Path}, http::StatusCode, response::IntoResponse, routing::{delete, post}, Json, Router};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{repositories::{invite::InviteRepository, users::UserRepository}, services::invite::accept::accept_invite};

use self::structs::{InviteIdResponse, InviteResponse, InviteTokenData};

use super::{structs::ErrorResponse, CurrentUser, Database};


pub async fn create_invite_handler(
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let user_repo = UserRepository::new(db.clone());
    let invite_repo = InviteRepository::new(db.clone());

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    if user.partner_id.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "You already have a partner".to_owned() })
        ).into_response();
    };

    let invites_count = invite_repo
        .get_count(current_user.id)
        .await;

    if invites_count != 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "You already have an invite".to_owned() })
        ).into_response();
    };

    let token = thread_rng().sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)  // From link above, this is needed in later versions
        .collect();

    let invite = invite_repo
        .create(current_user.id, token)
        .await;

    Json(InviteResponse {
        id: invite.id,
        token: invite.token,
    }).into_response()
}


pub async fn delete_invite_handler(
    Path(id): Path<i32>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoResponse {
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Invite not found".to_owned() })
        ).into_response(),
    };

    if invite.from_id != current_user.id {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Invite not found".to_owned() })
        ).into_response();
    };

    invite_repo
        .delete(id)
        .await;

    Json(InviteIdResponse {
        id: invite.id
    }).into_response()
}


pub async fn accept_invite_handler(
    Path(id): Path<i32>,
    db: Database,
    current_user: CurrentUser,
    extract::Json(InviteTokenData { token }): extract::Json<InviteTokenData>,
) -> impl IntoResponse {
    match accept_invite(id, db, current_user, token).await {
        Ok(_) => Json(InviteIdResponse { id }).into_response(),
        Err(err_response) => (
            err_response.0,
            Json(err_response.1)
        ).into_response(),
    }
}


pub async fn decline_invite_handler(
    Path(id): Path<i32>,
    db: Database,
    extract::Json(InviteTokenData { token }): extract::Json<InviteTokenData>,
) -> impl IntoResponse {
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error_msg: "Invite not found".to_owned() })
        ).into_response(),
    };

    if invite.token != token {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "Invalid token".to_owned() })
        ).into_response();
    };

    invite_repo
        .delete(id)
        .await;

    Json(InviteIdResponse {
        id: invite.id
    }).into_response()
}


pub async fn get_invite_router() -> Router {
    Router::new()
        .route("/", post(create_invite_handler))
        .route("/{id}/", delete(delete_invite_handler))
        .route("/{id}/accept/", post(accept_invite_handler))
        .route("/{id}/decline/", post(decline_invite_handler))
}