use crate::{prisma::user, views::Database};

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: u32) -> user::Data {
        self.db
            .user()
            .find_unique(user::UniqueWhereParam::IdEquals(id.try_into().unwrap()))
            .exec()
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn update_balance(&self, id: u32, balance: u32) -> user::Data {
        self.db
            .user()
            .update(
                user::UniqueWhereParam::IdEquals(id.try_into().unwrap()),
                vec![
                    user::SetParam::SetBalance(balance.try_into().unwrap()),
                ],
            )
            .exec()
            .await
            .unwrap()
    }

    pub async fn update_partner(&self, id: u32, partner_id: Option<u32>) -> user::Data {
        self.db
            .user()
            .update(
                user::UniqueWhereParam::IdEquals(id.try_into().unwrap()),
                vec![
                    match partner_id {
                        Some(v) => user::SetParam::SetPartnerId(Some(v.try_into().unwrap())),
                        None => user::SetParam::SetPartnerId(None),
                    }
                ],
            )
            .exec()
            .await
            .unwrap()
    }
}
