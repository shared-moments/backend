use crate::{prisma::{invite, user}, views::Database};


pub struct InviteRepository {
    db: Database,
}

impl InviteRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_count(&self, user_id: u32) -> i64 {
        self.db
            .invite()
            .count(vec![invite::from_id::equals(user_id.try_into().unwrap())])
            .exec()
            .await
            .unwrap()
    }

    pub async fn create(&self, user_id: u32, token: String) -> invite::Data {
        self.db
            .invite()
            .create(user::UniqueWhereParam::IdEquals(user_id.try_into().unwrap()), token, vec![])
            .exec()
            .await
            .unwrap()
    }

    pub async fn get_by_id(&self, id: u32) -> Option<invite::Data> {
        self.db
            .invite()
            .find_unique(invite::UniqueWhereParam::IdEquals(id.try_into().unwrap()))
            .exec()
            .await
            .unwrap()
    }

    pub async fn delete(&self, id: u32) -> invite::Data {
        self.db
            .invite()
            .delete(invite::UniqueWhereParam::IdEquals(id.try_into().unwrap()))
            .exec()
            .await
            .unwrap()
    }
}
