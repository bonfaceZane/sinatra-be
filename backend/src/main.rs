mod db;
mod router;

use crate::db::State;
use axum::{
    http,
    http::{
        HeaderValue,
        Method,
    },
    routing::get,
    Extension,
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = State::new().await.unwrap();

    let app = Router::new()
        // .route("/track/:key", get(get_data_from_mongodb))
        // .route("/track", post(insert_data_into_mongodb))
        .route("/", get(root))
        .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "http://localhost:8081".parse::<HeaderValue>().unwrap(),
                )
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::DELETE]),
        )
        .layer(TraceLayer::new_for_http());

    let port_address = SocketAddr::from(([127, 0, 0, 1], 8000));

    tracing::info!("listening on {}", port_address);

    axum::Server::bind(&port_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
