use crate::{prisma::user, views::Database};

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i32) -> user::Data {
        self.db
            .user()
            .find_unique(user::UniqueWhereParam::IdEquals(id))
            .exec()
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn update_balance(&self, id: i32, balance: i32) -> user::Data {
        self.db
            .user()
            .update(
                user::UniqueWhereParam::IdEquals(id),
                vec![
                    user::SetParam::SetBalance(balance),
                ],
            )
            .exec()
            .await
            .unwrap()
    }

    pub async fn update_partner(&self, id: i32, partner_id: Option<i32>) -> user::Data {
        self.db
            .user()
            .update(
                user::UniqueWhereParam::IdEquals(id),
                vec![
                    user::SetParam::SetPartnerId(partner_id),
                ],
            )
            .exec()
            .await
            .unwrap()
    }
}
