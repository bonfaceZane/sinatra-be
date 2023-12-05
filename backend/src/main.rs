mod db;

use crate::db::AppState;
use axum::{
    routing::get,
    Extension,
    Json,
    Router,
};
use dotenv::dotenv;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    env,
    net::SocketAddr,
};
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    dotenv::from_filename(".env").ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = AppState::new("sinatra").await.unwrap();

    let app = Router::new().route("/", get(root)).layer(Extension(state));
    // .layer(TraceLayer::new_for_http());

    let port_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    info!("Listening on {port_address}");
    dbg!(env::var("DATABASE_URL").ok());

    let database_url =
        option_env!("DATABASE_URL").unwrap_or("default_database_url");

    let key: Option<&'static str> = option_env!("DATABASE_URL");
    println!("the secret key might be: {key:?}");

    dbg!(database_url);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    height: f32,
}

async fn root() -> Json<User> {
    info!("Running home page");

    Json(User {
        name: "Bonface Zane".to_string(),
        height: 1.2,
    })
}
