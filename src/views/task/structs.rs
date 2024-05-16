use serde::{Deserialize, Serialize};

use crate::prisma::task;


#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}


#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: String,
    pub price: u32,
    pub is_need_request: bool,
    pub executor_id: Option<i32>,
}


#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: String,
    pub description: String,
    pub price: u32,
    pub is_need_request: bool,
    pub is_enabled: bool,
    pub executor_id: Option<i32>,
}


#[derive(Serialize)]
pub struct DetailedTask {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: u32,
    pub is_need_request: bool,
    pub executor: Option<User>,
}


impl From<task::Data> for DetailedTask {
    fn from(task: task::Data) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            price: task.price.try_into().unwrap(),
            is_need_request: task.is_need_request,
            executor: task.executor.map(|executor| User {
                id: executor.clone().unwrap().id,
                name: executor.unwrap().name,
            }),
        }
    }
}


#[derive(Serialize)]
pub struct ExecuteResponse {
    pub request_id: Option<i32>,
    pub log_id: Option<i32>
}
