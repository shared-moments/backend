use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, JsonSchema)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}


#[derive(Serialize, JsonSchema)]
pub struct LoginResponse {
    pub token: String,
}


#[derive(Clone)]
pub struct CurrentUser {
    pub id: u32
}
