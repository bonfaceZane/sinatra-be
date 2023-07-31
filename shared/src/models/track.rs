use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    title: String,
    url: String,
    description: String,
    tags: Vec<String>,
}
