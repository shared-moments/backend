use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, JsonSchema)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}


#[derive(Serialize, JsonSchema)]
pub struct Page<T>
    where T: Serialize
{
    pub items: Vec<T>,
    pub page: u32,
    pub pages: u32,
}
