pub mod structs;
pub mod utils;
pub mod middleware;

use axum::{http::StatusCode, routing::post, Json, Router, response::IntoResponse};

use crate::prisma::user;

use self::{structs::{AuthData, LoginResponse}, utils::{get_token, hash_password}};

use super::{structs::ErrorResponse, Database};


pub async fn register(
    db: Database,
    Json(data): Json<AuthData>,
) -> impl IntoResponse {
    let user = db
        .user()
        .find_unique(user::email::equals(data.email.clone()))
        .exec()
        .await
        .unwrap();

    if user.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "User already exists".to_owned() })
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


pub async fn login(
    db: Database,
    Json(data): Json<AuthData>,
) -> impl IntoResponse {
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
            Json(ErrorResponse { error_msg: "User or password incorrect".to_owned() })
        ).into_response()
    };

    let password_hash = hash_password(data.password.as_str());

    if password_hash != user.password {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error_msg: "User or password incorrect".to_owned() })
        ).into_response()
    }

    let token = get_token(user.id);

    (StatusCode::OK, Json(LoginResponse { token })).into_response()
}


pub async fn get_auth_router() -> Router {
    Router::new()
        .route("/register/", post(register))
        .route("/login/", post(login))
}
