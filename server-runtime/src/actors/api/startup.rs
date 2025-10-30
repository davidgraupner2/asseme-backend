use server_config::{api::ApiConfiguration, database::DatabaseConfiguration};

pub struct APIStartupArguments {
    pub api_config: ApiConfiguration,
    pub database_config: DatabaseConfiguration,
}
