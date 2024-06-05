use rhai::{Engine, Scope};
use std::sync::Arc;
use tokio::runtime::Runtime;

mod db;
use db::{init_db, query_surrealdb, DB_CONNECTION};

async fn test_func() -> Result<String, Box<dyn std::error::Error>> {
    println!("I'm here");
    Ok("f".to_string())
}

fn main() {
    // Create a new Rhai engine
    let mut engine = Engine::new();

    // Create a new Tokio runtime and wrap it in an Arc
    let rt = Arc::new(Runtime::new().unwrap());

    // Initialize the global database connection at startup
    let mut rt_clone = Arc::clone(&rt);
    rt_clone.block_on(async {
        let db = init_db().await.expect("Failed to initialize DB");
        DB_CONNECTION
            .set(db)
            .expect("DB_CONNECTION initialization failed");
    });

    // Register a blocking function that calls the async function
    engine.register_fn("query_surrealdb", move |query: &str| {
        rt_clone
            .block_on(query_surrealdb(query))
            .unwrap_or_else(|e| e.to_string())
    });

    // Testing multiple registered functions. Current setup needs to
    // have the rt cloned for each function.
    rt_clone = Arc::clone(&rt);
    engine.register_fn("test_func", move || {
        rt_clone
            .block_on(test_func())
            .unwrap_or_else(|e| e.to_string())
    });

    // Create a new scope
    let mut scope = Scope::new();

    // Example Rhai script
    let script = r#"
        let result = query_surrealdb("create table:test set name='hello world';");
        print(result);
        test_func();
        print(query_surrealdb("select name from table:test;"));
    "#;

    // Evaluate the script
    if let Err(e) = engine.eval_with_scope::<()>(&mut scope, script) {
        println!("Error: {}", e);
    }
}
