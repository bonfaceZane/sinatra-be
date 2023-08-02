use axum::routing::{get, post};
use axum::{
    async_trait, extract,
    handler::{get, post},
    http::StatusCode,
    AddExtensionLayer, Router,
};
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use std::convert::Infallible;
use tokio::sync::RwLock;

type Database = mongodb::Database;

#[derive(Debug, Clone)]
struct State {
    db: Database,
}

impl State {
    async fn new() -> Result<State, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("your_database_name"); // Replace "your_database_name" with your MongoDB database name

        Ok(State { db })
    }
}

async fn get_data_from_mongodb(
    key: String,
    state: extract::Extension<State>,
) -> Result<String, StatusCode> {
    let collection = state.db.collection("your_collection_name"); // Replace "your_collection_name" with your MongoDB collection name

    let filter = doc! { "_id": key };
    if let Some(doc) = collection.find_one(filter, None).await.unwrap() {
        // Assuming you have a field named "data" in your documents
        if let Some(data) = doc.get_str("data") {
            return Ok(data.to_string());
        }
    }

    Err(StatusCode::NOT_FOUND)
}

async fn insert_data_into_mongodb(
    (key, value): (String, String),
    state: extract::Extension<State>,
) -> Result<(), StatusCode> {
    let collection = state.db.collection("your_collection_name"); // Replace "your_collection_name" with your MongoDB collection name

    let filter = doc! { "_id": key };
    let update = doc! { "$set": { "data": value } };

    match collection.update_one(filter, update, None).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let state = State::new().await.unwrap();

    let app = Router::new()
        .route("/data/:key", get(get_data_from_mongodb))
        .route("/data", post(insert_data_into_mongodb))
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
