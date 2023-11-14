use crate::State;
use axum::{
    extract,
    http::StatusCode,
};
use mongodb::{
    bson::doc,
    Collection,
};

pub async fn get_data_from_mongodb(
    key: String,
    state: extract::Extension<State>,
) -> Result<String, StatusCode> {
    let collection = state.db.collection::<Collection<Vec<String>>>("tracks");

    let filter = doc! { "_id": key };

    if let Some(doc) = collection.find_one(filter, None).await.unwrap() {
        if let Some(data) = doc.get_str("track") {
            return Ok(data.to_string());
        }
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn insert_data_into_mongodb(
    (key, value): (String, String),
    state: extract::Extension<State>,
) -> Result<(), StatusCode> {
    let collection = state.db.collection::<Collection<Vec<String>>>("tracks");

    let filter = doc! { "_id": key };
    let update = doc! { "$set": { "track": value } };

    match collection.update_one(filter, update, None).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
