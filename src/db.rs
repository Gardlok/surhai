use std::sync::Arc;
use std::sync::OnceLock;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Value;
use surrealdb::Surreal;

pub static DB_CONNECTION: OnceLock<Arc<Surreal<Db>>> = OnceLock::new();

pub async fn init_db() -> Result<Arc<Surreal<Db>>, Box<dyn std::error::Error>> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db("test").await?;
    Ok(Arc::new(db))
}

pub async fn get_db_connection() -> Arc<Surreal<Db>> {
    DB_CONNECTION
        .get()
        .expect("DB connection is not initialized")
        .clone()
}

// To execute a custom query
pub async fn query_surrealdb(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let db = get_db_connection().await;

    let mut response = db.query(query).await?;

    // Process the response to extract the result as a string
    let result: String = response
        .take(0)
        .into_iter()
        .map(|res: Value| res.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Ok(result)
}
