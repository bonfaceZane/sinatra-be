mod db;
mod models;
mod routes;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(// this argument tells axum to parse the request body
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 13373,
        username: "zane".to_owned(),
    };

    (StatusCode::CREATED, Json(user))
}

// async fn get_user(id: u64) -> (StatusCode, Json<User>) {
//     let user = User {
//         id,
//         username: "jane_doe".to_string(),
//     };
//
//     (StatusCode::CREATED, Json(user))
// }

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", get(create_user));
    // .route("/users/:id", get(get_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
