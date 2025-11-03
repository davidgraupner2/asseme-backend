use crate::model::user_role::UserRole;
use surrealdb::{engine::remote::ws::Client, Result, Surreal};

pub struct UserRoleRepository {
    table: String,
    db: Surreal<Client>,
}

impl UserRoleRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            table: String::from("user_role"),
            db,
        }
    }

    pub async fn create_user_role(&self, user_role: UserRole) -> Result<UserRole> {
        let record: Option<UserRole> = self.db.create(&self.table).content(user_role).await?;
        Ok(record.unwrap())
    }
}
