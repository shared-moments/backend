use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct InviteIdResponse {
    pub id: i32,
}


#[derive(Serialize)]
pub struct InviteResponse {
    pub id: i32,
    pub token: String,
}

#[derive(Deserialize)]
pub struct InviteTokenData {
    pub token: String,
}
