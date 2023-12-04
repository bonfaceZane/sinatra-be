mod db;

use crate::db::AppState;
use axum::{
    http::{
        header,
        HeaderValue,
        Method,
    },
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
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
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

    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "http://localhost:8081".parse::<HeaderValue>().unwrap(),
                )
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::DELETE]),
        )
        .layer(TraceLayer::new_for_http());

    let port_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    info!("Listening on {port_address}");
    dbg!(option_env!("DATABASE"));

    axum::Server::bind(&port_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
