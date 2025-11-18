use async_trait::async_trait;
use libsql::{params, Builder, Connection, Database, Result};
use serde_json::Value;
use tokio::sync::Mutex;

pub mod models;

#[async_trait]
pub trait Db: Send + Sync {
    async fn create_collection(&self, name: &str) -> Result<i64>;
    async fn create_record(
        &self,
        collection_id: i64,
        data: &Value,
    ) -> std::result::Result<i64, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl Db for Database {
    async fn create_collection(&self, name: &str) -> Result<i64> {
        let conn = self.connect()?;
        conn.execute("INSERT INTO collections (name) VALUES (?1)", params![name])
            .await?;
        Ok(conn.last_insert_rowid())
    }

    async fn create_record(
        &self,
        collection_id: i64,
        data: &Value,
    ) -> std::result::Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.connect()?;
        let data_str = serde_json::to_string(data)?;
        conn.execute(
            "INSERT INTO records (collection_id, data) VALUES (?1, ?2)",
            params![collection_id, data_str],
        )
        .await?;
        Ok(conn.last_insert_rowid())
    }
}

#[async_trait]
impl Db for Mutex<Connection> {
    async fn create_collection(&self, name: &str) -> Result<i64> {
        let conn = self.lock().await;
        conn.execute("INSERT INTO collections (name) VALUES (?1)", params![name])
            .await?;
        Ok(conn.last_insert_rowid())
    }

    async fn create_record(
        &self,
        collection_id: i64,
        data: &Value,
    ) -> std::result::Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.lock().await;
        let data_str = serde_json::to_string(data)?;
        conn.execute(
            "INSERT INTO records (collection_id, data) VALUES (?1, ?2)",
            params![collection_id, data_str],
        )
        .await?;
        Ok(conn.last_insert_rowid())
    }
}

pub async fn a_new_database_connection() -> Result<Database> {
    let db = Builder::new_local("local.db").build().await?;
    setup_database(&db).await?;
    Ok(db)
}

async fn setup_database(db: &Database) -> Result<()> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS collections (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, schema JSON)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS records (id INTEGER PRIMARY KEY AUTOINCREMENT, collection_id INTEGER NOT NULL, data TEXT NOT NULL)",
        (),
    )
    .await?;
    Ok(())
}
