use mongodb::{
    options::ClientOptions,
    Client,
};

type Database = mongodb::Database;

#[derive(Debug, Clone)]
pub struct State {
    pub db: Database,
}

impl State {
    pub(crate) async fn new() -> Result<State, mongodb::error::Error> {
        let client_options =
            ClientOptions::parse("mongodb://localhost:27017").await?;

        let client = Client::with_options(client_options)?;
        let db = client.database("sinatra");

        Ok(State { db })
    }
}
