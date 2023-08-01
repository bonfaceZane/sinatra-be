use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    name: String,
}

impl Image {
    pub fn new(name: String) -> Image {
        Image { name }
    }
}
