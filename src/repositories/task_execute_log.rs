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
        task_id: u32,
        price: u32,
        executor_id: u32,
    ) -> crate::prisma::task_execute_log::Data {
        self.db
            .task_execute_log()
            .create(
                task::UniqueWhereParam::IdEquals(task_id.try_into().unwrap()),
                price.try_into().unwrap(),
                user::UniqueWhereParam::IdEquals(executor_id.try_into().unwrap()),
                vec![]
            )
            .exec()
            .await
            .unwrap()
    }
}
