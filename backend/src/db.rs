use mongodb::{
    options::ClientOptions,
    Client,
};

type Database = mongodb::Database;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub(crate) async fn new() -> Result<AppState, mongodb::error::Error> {
        let client_options =
            ClientOptions::parse("mongodb://localhost:27017").await?;

        let client = Client::with_options(client_options)?;
        let db = client.database("sinatra");

        Ok(AppState { db })
    }
}
