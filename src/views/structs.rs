use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_msg: String,
}


#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}


#[derive(Serialize)]
pub struct Page<T>
    where T: Serialize
{
    pub items: Vec<T>,
    pub page: u32,
    pub pages: u32,
}
