use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, JsonSchema)]
pub struct InviteIdResponse {
    pub id: u32,
}


#[derive(Serialize, JsonSchema)]
pub struct InviteResponse {
    pub id: i32,
    pub token: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct InviteTokenData {
    pub token: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct InviteIdPath {
    pub id: u32,
}
