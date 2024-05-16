use crate::{prisma::{read_filters::IntFilter, task, user}, views::{structs::Page, task::structs::{CreateTask, UpdateTask}, Database}};

pub struct TaskRepository {
    db: Database,
}

impl TaskRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, data: CreateTask, author_id: i32) -> task::Data {
        self.db
            .task()
            .create(
                data.title.clone(),
                data.description.clone(),
                data.price.try_into().unwrap(),
                user::id::equals(author_id),
                vec![
                    task::SetParam::SetIsNeedRequest(data.is_need_request),
                    task::SetParam::SetExecutorId(data.executor_id)
                ],
            )
            .with(task::executor::fetch())
            .exec()
            .await
            .unwrap()
    }

    pub async fn get_by_id(&self, id: i32) -> task::Data {
        self.db
            .task()
            .find_unique(task::UniqueWhereParam::IdEquals(id))
            .exec()
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn update(&self, id: i32, data: UpdateTask) -> task::Data {
        self.db
            .task()
            .update(
                task::UniqueWhereParam::IdEquals(id),
                vec![
                    task::SetParam::SetTitle(data.title.clone()),
                    task::SetParam::SetDescription(data.description.clone()),
                    task::SetParam::SetPrice(data.price.try_into().unwrap()),
                    task::SetParam::SetIsNeedRequest(data.is_need_request),
                    task::SetParam::SetExecutorId(data.executor_id)
                ],
            )
            .with(task::executor::fetch())
            .exec()
            .await
            .unwrap()
    }

    pub async fn delete(&self, id: i32) -> task::Data {
        self.db
            .task()
            .delete(task::UniqueWhereParam::IdEquals(id))
            .exec()
            .await
            .unwrap()
    }

    pub async fn list(&self, user_id: i32, page: u32, page_size: u32) -> Page<task::Data> {
        let user = self.db.user().find_unique(user::UniqueWhereParam::IdEquals(user_id)).exec().await.unwrap().unwrap();
        let allowed_authors = match user.partner_id {
            Some(partner_id) => vec![user.id, partner_id],
            None => vec![user.id],
        };

        let pages = {
            let tasks = self.db
                .task()
                .count(vec![task::WhereParam::AuthorId(IntFilter::InVec(allowed_authors.clone()))])
                .exec()
                .await
                .unwrap();

            (tasks as f64 / page_size as f64).ceil() as u32
        };

        let items = &self.db
            .task()
            .find_many(vec![task::WhereParam::AuthorId(IntFilter::InVec(allowed_authors))])
            .with(task::executor::fetch())
            .take(page_size.into())
            .skip(((page - 1) * page_size).into())
            .exec()
            .await
            .unwrap();

        Page { items: items.to_owned(), page, pages }
    }
}
