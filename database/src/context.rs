use surrealdb::{
    engine::remote::ws::{Client, Ws, Wss},
    opt::auth::Root,
    Result, Surreal,
};
use surrealdb_migrations::MigrationRunner;
use tracing::{event, Level};

pub async fn get_database(
    connection_type: String,
    url: String,
    user_name: String,
    password: String,
    namespace: String,
    database: String,
) -> Result<Surreal<Client>> {
    event!(
        Level::DEBUG,
        "Attempting Connection to database on: {}://{}",
        &connection_type,
        &url
    );

    let db = match connection_type.as_str() {
        "wss" => Surreal::new::<Wss>(&url).await,
        _ => Surreal::new::<Ws>(&url).await,
    };

    let db_client =
        db.expect("DATABASE_CONNECTION_ERROR: API Server Failed to connect to database");

    if let Err(e) = db_client
        .signin(Root {
            username: &user_name,
            password: &password,
        })
        .await
    {
        panic!("DATABASE_AUTH_ERROR: API Server Failed to authenticate with database. Check username/password in config: {}", e);
    } else {
        db_client.use_ns(&namespace).await.expect(&format!(
            "DATABASE_CONNECTION_ERROR: Namespace {} is not valid",
            namespace
        ));
        db_client.use_db(&database).await.expect(&format!(
            "DATABASE_CONNECTION_ERROR: Database {} is not valid",
            database
        ));

        // Apply all database migrations
        MigrationRunner::new(&db_client)
            .up()
            .await
            .expect("Failed to apply database migrations");

        Ok(db_client)
    }
}
