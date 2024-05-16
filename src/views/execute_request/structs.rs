use serde::{Deserialize, Serialize};

use crate::prisma::{task, task_execute_request, user};


#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: u32
}


impl From<Box<task::Data>> for Task {
    fn from(data: Box<task::Data>) -> Self {
        Self {
            id: data.id,
            title: data.title.clone(),
            description: data.description.clone(),
            price: data.price.try_into().unwrap()
        }
    }
}


#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String
}


impl From<Box<user::Data>> for User {
    fn from(data: Box<user::Data>) -> Self {
        Self {
            id: data.id,
            name: data.name.clone()
        }
    }
}


#[derive(Serialize)]
pub struct TaskExecuteRequest {
    pub id: i32,
    pub task: Task,
    pub executor: User,
    pub approver: User,
    pub approved: Option<bool>,
}


impl From<task_execute_request::Data> for TaskExecuteRequest {
    fn from(data: task_execute_request::Data) -> Self {
        Self {
            id: data.id,
            task: data.task.unwrap().into(),
            executor: data.executor.unwrap().into(),
            approver: data.approver.unwrap().into(),
            approved: data.approved
        }
    }
}

#[derive(Deserialize)]
pub struct ConfirmQuery {
    pub approved: bool
}
