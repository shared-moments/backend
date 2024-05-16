pub mod structs;
pub mod utils;
pub mod middleware;

use aide::{axum::{routing::post_with, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::{http::StatusCode, response::IntoResponse};

use crate::{errors::AppError, extractors::Json, prisma::user};

use self::{structs::{AuthData, LoginResponse}, utils::{get_token, hash_password}};

use super::Database;


pub async fn register(
    db: Database,
    Json(data): Json<AuthData>,
) -> impl IntoApiResponse {
    let user = db
        .user()
        .find_unique(user::email::equals(data.email.clone()))
        .exec()
        .await
        .unwrap();

    if user.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            AppError::new("User already exists")
        ).into_response()
    }

    let password_hash = hash_password(data.password.as_str());

    db
        .user()
        .create(data.email.clone(), password_hash, data.email, vec![])
        .exec()
        .await
        .unwrap();

    (StatusCode::CREATED, "User created").into_response()
}


fn register_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<201, &str>()
        .response::<400, Json<AppError>>()
}


pub async fn login(
    db: Database,
    Json(data): Json<AuthData>,
) -> impl IntoApiResponse {
    let user = db
        .user()
        .find_unique(user::email::equals(data.email.clone()))
        .exec()
        .await
        .unwrap();

    let user = match user {
        Some(user) => user,
        None => return (
            StatusCode::BAD_REQUEST,
            AppError::new("User or password incorrect")
        ).into_response()
    };

    let password_hash = hash_password(data.password.as_str());

    if password_hash != user.password {
        return (
            StatusCode::BAD_REQUEST,
            AppError::new("User or password incorrect")
        ).into_response()
    }

    let token = get_token(user.id);

    (StatusCode::OK, Json(LoginResponse { token })).into_response()
}


fn login_op(op: TransformOperation) -> TransformOperation {
    op
        .response::<201, Json<LoginResponse>>()
        .response::<400, Json<AppError>>()
}


pub async fn get_auth_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/register/", post_with(register, register_op))
        .api_route("/login/", post_with(login, login_op))
}
