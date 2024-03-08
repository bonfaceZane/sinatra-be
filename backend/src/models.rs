use bson::oid::ObjectId;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    description: String,
    url: String,
    tags: Vec<String>,
    duration: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    start_date: String,
    end_date: String,
}
