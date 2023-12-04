use mongodb::{
    options::ClientOptions,
    Client,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: mongodb::Database,
}

impl AppState {
    pub(crate) async fn new(
        db_name: &str
    ) -> Result<AppState, mongodb::error::Error> {
        let client_options =
            ClientOptions::parse("mongodb://localhost:27017").await?;

        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);

        Ok(AppState { db })
    }
}
