use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }
}
