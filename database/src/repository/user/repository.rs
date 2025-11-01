use surrealdb::{
    engine::remote::ws::{Client, Ws, Wss},
    Error, Result, Surreal,
};
use tracing::{error, event, info, warn, Level};

use crate::model::user::User;

pub struct UserRepository {
    table: String,
    db: Surreal<Client>,
}

impl UserRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            table: String::from("user"),
            db,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<User>> {
        let records = self.db.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<User> {
        if let Some(record) = self.db.select((&self.table, id.clone())).await? {
            return Ok(record);
        }
        let error = Error::Db(surrealdb::error::Db::Thrown(format!(
            "User with id {} not found!",
            id
        )));
        Err(error)
    }

    pub async fn get_by_email(&self, email: String) -> Result<User> {
        let mut result = self
            .db
            .query(format!("select * from user where email = '{}'", email))
            .await?;
        let user: Vec<User> = result.take(0)?;
        return Ok(user[0].clone());
    }

    pub async fn create_user(&self, content: User) -> Result<Vec<User>> {
        let record = self.db.create(&self.table).content(content).await?;
        Ok(record.unwrap())
    }

    pub async fn update_user(&self, id: String, content: User) -> Result<User> {
        let record: Option<User> = self.db.update((&self.table, id)).content(content).await?;
        Ok(record.unwrap())
    }

    pub async fn delete_user(&self, id: String) -> Result<User> {
        let result = self.db.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}
