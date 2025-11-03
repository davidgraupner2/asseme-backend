use crate::model::tenant::Tenant;
use surrealdb::{engine::remote::ws::Client, Result, Surreal};

pub struct TenantRepository {
    table: String,
    db: Surreal<Client>,
}

impl TenantRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            table: String::from("tenant"),
            db,
        }
    }

    pub async fn create_tenant(&self, tenant: Tenant) -> Result<Tenant> {
        let record: Option<Tenant> = self.db.create(&self.table).content(tenant).await?;
        Ok(record.unwrap())
    }
}
