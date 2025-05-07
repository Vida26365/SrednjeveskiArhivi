use dioxus::logger::tracing::info;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbConn, EntityTrait, Schema};
use tokio::sync::OnceCell;

use crate::directories::DIRECTORIES;
use crate::entities::{Document, Location, Organization, Person};

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::const_new();

async fn create_table<E: EntityTrait>(database: &DbConn, entity: E) {
    let backend = database.get_database_backend();
    let schema = Schema::new(backend);

    database
        .execute(backend.build(schema.create_table_from_entity(entity).if_not_exists()))
        .await
        .expect("Failed to create table");

    for mut stmt in schema.create_index_from_entity(entity) {
        database
            .execute(backend.build(stmt.if_not_exists()))
            .await
            .expect("Failed to create index");
    }
}

async fn init_database() -> DatabaseConnection {
    info!("Connecting to database...");

    let path = DIRECTORIES.userdata.join("storage.db");
    let url = format!("sqlite:{}?mode=rwc", path.display());
    let database = Database::connect(url).await.expect("Failed to connect to database");

    info!("Creating tables...");
    create_table(&database, Document).await;
    create_table(&database, Location).await;
    create_table(&database, Organization).await;
    create_table(&database, Person).await;

    database
}

pub async fn get_database() -> &'static DatabaseConnection {
    DATABASE.get_or_init(|| async { init_database().await }).await
}
