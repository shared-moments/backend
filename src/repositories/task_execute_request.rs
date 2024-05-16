use crate::{prisma::{read_filters::IntFilter, task, task_execute_request, user}, views::{structs::Page, Database}};

pub struct TaskExecuteRequestRepository {
    db: Database,
}


impl TaskExecuteRequestRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        task_id: i32,
        executor_id: i32,
        approver_id: i32
    ) -> task_execute_request::Data {
        self.db
            .task_execute_request()
            .create(
                task::UniqueWhereParam::IdEquals(task_id),
                user::UniqueWhereParam::IdEquals(executor_id),
                user::UniqueWhereParam::IdEquals(approver_id),
                vec![]
            )
            .exec()
            .await
            .unwrap()
    }

    pub async fn get_by_id(&self, id: i32) -> Option<task_execute_request::Data> {
        self.db
            .task_execute_request()
            .find_unique(task_execute_request::UniqueWhereParam::IdEquals(id))
            .exec()
            .await
            .unwrap()
    }

    pub async fn list(&self, user_id: i32, page: u32, page_size: u32) -> Page<task_execute_request::Data> {
        let user = self.db.user().find_unique(user::UniqueWhereParam::IdEquals(user_id)).exec().await.unwrap().unwrap();

        let filter = match user.partner_id {
            Some(partner_id) => vec![
                task_execute_request::WhereParam::ApproverId(IntFilter::Equals(user_id)),
                task_execute_request::WhereParam::ExecutorId(IntFilter::Equals(partner_id))

            ],
            None => vec![
                task_execute_request::WhereParam::ApproverId(IntFilter::Equals(user_id))
            ],
        };

        let pages = {
            let requests = self.db
                .task_execute_request()
                .count(filter.clone())
                .exec()
                .await
                .unwrap();

            (requests as f64 / page_size as f64).ceil() as u32
        };

        let items = self.db
            .task_execute_request()
            .find_many(filter)
            .with(task_execute_request::task::fetch())
            .with(task_execute_request::approver::fetch())
            .with(task_execute_request::executor::fetch())
            .exec()
            .await
            .unwrap();

        Page { items: items.to_owned(), page, pages }
    }

    pub async fn update_approved(&self, id: i32, approved: bool) -> task_execute_request::Data {
        self.db
            .task_execute_request()
            .update(
                task_execute_request::UniqueWhereParam::IdEquals(id),
                vec![task_execute_request::SetParam::SetApproved(Some(approved))]
            )
            .with(task_execute_request::task::fetch())
            .with(task_execute_request::approver::fetch())
            .with(task_execute_request::executor::fetch())
            .exec()
            .await
            .unwrap()
    }
}
