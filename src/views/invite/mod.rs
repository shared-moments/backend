pub mod structs;

use aide::{axum::{routing::{delete_with, post_with}, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::{extract::{self, Path}, http::StatusCode, response::IntoResponse, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{errors::AppError, repositories::{invite::InviteRepository, users::UserRepository}, services::invite::accept::accept_invite};

use self::structs::{InviteIdPath, InviteIdResponse, InviteResponse, InviteTokenData};

use super::{CurrentUser, Database};


pub async fn create_invite_handler(
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
    let user_repo = UserRepository::new(db.clone());
    let invite_repo = InviteRepository::new(db.clone());

    let user = user_repo
        .get_by_id(current_user.id)
        .await;

    if user.partner_id.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            AppError::new("You already have a partner")
        ).into_response();
    };

    let invites_count = invite_repo
        .get_count(current_user.id)
        .await;

    if invites_count != 0 {
        return (
            StatusCode::BAD_REQUEST,
            AppError::new("You already have an invite")
        ).into_response();
    };

    let token = thread_rng().sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)  // From link above, this is needed in later versions
        .collect();

    let invite = invite_repo
        .create(current_user.id, token)
        .await;

    (
        StatusCode::CREATED,
        Json(InviteResponse {
            id: invite.id,
            token: invite.token,
        })
    ).into_response()
}

fn get_invite_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<InviteResponse>>()
        .response::<400, Json<AppError>>()
}


pub async fn delete_invite_handler(
    Path(InviteIdPath { id }): Path<InviteIdPath>,
    db: Database,
    current_user: CurrentUser,
) -> impl IntoApiResponse {
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return (
            StatusCode::NOT_FOUND,
            AppError::new("Invite not found")
        ).into_response(),
    };

    {
        let from_id: u32 = invite.from_id.try_into().unwrap();
        if from_id != current_user.id {
            return (
                StatusCode::NOT_FOUND,
                AppError::new("Invite not found")
            ).into_response();
        };
    }

    invite_repo
        .delete(id)
        .await;

    Json(InviteIdResponse {
        id
    }).into_response()
}

fn delete_invite_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<InviteIdResponse>>()
        .response::<400, Json<AppError>>()
}


pub async fn accept_invite_handler(
    Path(InviteIdPath { id }): Path<InviteIdPath>,
    db: Database,
    current_user: CurrentUser,
    extract::Json(InviteTokenData { token }): extract::Json<InviteTokenData>,
) -> impl IntoApiResponse {
    match accept_invite(id, db, current_user, token).await {
        Ok(_) => Json(InviteIdResponse { id }).into_response(),
        Err(err_response) => (
            err_response.0,
            Json(err_response.1)
        ).into_response(),
    }
}

fn accept_invite_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<InviteIdResponse>>()
        .response::<400, Json<AppError>>()
        .response::<404, Json<AppError>>()
}


pub async fn decline_invite_handler(
    Path(InviteIdPath { id }): Path<InviteIdPath>,
    db: Database,
    extract::Json(InviteTokenData { token }): extract::Json<InviteTokenData>,
) -> impl IntoApiResponse {
    let invite_repo = InviteRepository::new(db.clone());

    let invite = invite_repo
        .get_by_id(id)
        .await;

    let invite = match invite {
        Some(invite) => invite,
        None => return (
            StatusCode::NOT_FOUND,
            AppError::new("Invite not found")
        ).into_response(),
    };

    if invite.token != token {
        return (
            StatusCode::BAD_REQUEST,
            AppError::new("Invite not found")
        ).into_response();
    };

    invite_repo
        .delete(id)
        .await;

    Json(InviteIdResponse {
        id
    }).into_response()
}

fn decline_invite_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<200, Json<InviteIdResponse>>()
        .response::<400, Json<AppError>>()
        .response::<404, Json<AppError>>()
}


pub async fn get_invite_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post_with(create_invite_handler, get_invite_op))
        .api_route("/{id}/", delete_with(delete_invite_handler, delete_invite_op))
        .api_route("/{id}/accept/", post_with(accept_invite_handler, accept_invite_op))
        .api_route("/{id}/decline/", post_with(decline_invite_handler, decline_invite_op))
}