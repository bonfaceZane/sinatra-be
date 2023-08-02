use axum::routing::{get, post};
use axum::{
    async_trait, extract,
    handler::{get, post},
    http::StatusCode,
    AddExtensionLayer, Router,
};
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::RwLock;

// Assume SurealDB is some database with custom methods, but we'll use a simple HashMap as a stand-in
type Database = RwLock<HashMap<String, String>>;

#[derive(Debug, Clone)]
struct State {
    db: Database,
}

impl State {
    fn new() -> State {
        State {
            db: RwLock::new(HashMap::new()),
        }
    }
}

// This is a hypothetical "SurealDB" function to fetch data from the database
async fn get_data_from_surealdb(
    key: String,
    state: extract::Extension<State>,
) -> Result<String, StatusCode> {
    let db = state.db.read().await;
    match db.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// This is a hypothetical "SurealDB" function to insert data into the database
async fn insert_data_into_surealdb(
    (key, value): (String, String),
    state: extract::Extension<State>,
) -> Result<(), StatusCode> {
    let mut db = state.db.write().await;
    db.insert(key, value);
    Ok(())
}

#[tokio::main]
async fn main() {
    let state = State::new();

    let app = Router::new()
        .route("/data/:key", get(get_data_from_surealdb))
        .route("/data", post(insert_data_into_surealdb))
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// use axum::{routing::get, Router};
//
// #[tokio::main]
// async fn main() {
//     // build our application with a single route
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//
//     // run it with hyper on localhost:3000
//     axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
