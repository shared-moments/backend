use crate::{prisma::{task::{self}, user}, views::Database};

pub struct TaskExecuteLogRepository {
    db: Database,
}


impl TaskExecuteLogRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        task_id: i32,
        price: i32,
        executor_id: i32,
    ) -> crate::prisma::task_execute_log::Data {
        self.db
            .task_execute_log()
            .create(
                task::UniqueWhereParam::IdEquals(task_id),
                price,
                user::UniqueWhereParam::IdEquals(executor_id),
                vec![]
            )
            .exec()
            .await
            .unwrap()
    }
}
