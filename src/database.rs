use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbConn, EntityTrait, Schema};
use tokio::sync::OnceCell;

use crate::directories::DIRECTORIES;
use crate::entities::Document;

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn create_table<E: EntityTrait>(database: &DbConn, entity: E) {
    let backend = database.get_database_backend();
    let schema = Schema::new(backend);
    let statement = backend.build(schema.create_table_from_entity(entity).if_not_exists());

    if let Err(error) = database.execute(statement).await {
        panic!("Failed to create table {}: {}", entity.table_name(), error);
    }
}

async fn init_database() -> DatabaseConnection {
    let path = DIRECTORIES.userdata.join("storage.db");
    let url = format!("sqlite:{}?mode=rwc", path.display());
    let database = Database::connect(url).await.expect("Failed to connect to database");

    create_table(&database, Document).await;

    database
}

pub async fn get_database() -> &'static DatabaseConnection {
    DATABASE.get_or_init(|| async { init_database().await }).await
}
