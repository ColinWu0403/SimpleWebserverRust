use axum::{Router, routing::{get, post}};
use crate::controllers::user_controller::{create_user, get_users};

mod controllers;
mod models;

#[tokio::main]
async fn main() {
    // Create the Axum router for API endpoints
    let app = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_users));

    // Backend running on port 6969 for API
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
