use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}


#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32
}
