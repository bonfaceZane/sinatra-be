use crate::models::albums::Album;
use crate::models::artists::Artist;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
}
