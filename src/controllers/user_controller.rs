use axum::{Json, http::StatusCode};
use crate::models::user_model::{User, CreateUser};
use std::sync::Arc;

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,  // Hardcoded ID for simplicity
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn get_users() -> Json<Vec<User>> {
    // In a real app, you'd fetch from a database.
    let users = vec![
        User { id: 1, username: "user1".to_string() },
        User { id: 2, username: "user2".to_string() },
    ];

    Json(users)
}
