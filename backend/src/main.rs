mod db;
mod router;
mod tracks;

use crate::{
    db::State,
    tracks::{
        get_data_from_mongodb,
        insert_data_into_mongodb,
    },
};
use axum::{
    http,
    http::{
        HeaderValue,
        Method,
    },
    routing::{
        get,
        post,
    },
    Extension,
    Router,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = State::new().await.unwrap();

    let app = Router::new()
        .route("/track/:key", get(get_data_from_mongodb))
        .route("/track", post(insert_data_into_mongodb))
        .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "http://localhost:8081".parse::<HeaderValue>().unwrap(),
                )
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::DELETE]),
        );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
