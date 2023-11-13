use axum::routing::{get, post};
use axum::{extract, http::StatusCode, Extension, Router};
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};


type Database = mongodb::Database;

#[derive(Debug, Clone)]
struct State {
    db: Database,
}

trait DB {}

impl DB for State {}

impl State {
    async fn new() -> Result<State, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("sinatra");

        Ok(State { db })
    }
}

async fn get_data_from_mongodb(
    key: String,
    state: extract::Extension<State>,
) -> Result<String, StatusCode> {
    let collection = state.db.collection::<Collection<Vec<String>>>("tracks"); // Replace "your_collection_name" with your MongoDB collection name
const TRY_ENV = env!("DB_NAME");

    let filter = doc! { "_id": key };



    if let Some(doc) = collection.find_one(filter, None).await.unwrap() {
        if let Some(data) = doc.get_str("track") {
            return Ok(data.to_string());
        }
    }

    Err(StatusCode::NOT_FOUND)
}

async fn insert_data_into_mongodb(
    (key, value): (String, String),
    state: extract::Extension<State>,
) -> Result<(), StatusCode> {
    let collection = state.db.collection("tracks"); // Replace "your_collection_name" with your MongoDB collection name

    let filter = doc! { "_id": key };
    let update = doc! { "$set": { "track": value } };

    match collection.update_one(filter, update, None).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let state = State::new().await.unwrap();

    let app = Router::new()
        .route("/track/:key", get(get_data_from_mongodb))
        .route("/track", post(insert_data_into_mongodb))
        .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
