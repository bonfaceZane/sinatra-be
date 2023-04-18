use crate::models::tracks::Track;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Playlist {
    pub tracks: Vec<Track>,
    pub author: String,
    pub title: String,
}

impl Playlist {
    pub fn new(tracks: Vec<Track>, author: String, title: String) -> Self {
        Self {
            tracks,
            author,
            title,
        }
    }
}
