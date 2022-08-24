use crate::services::user_service;
use axum::{response::IntoResponse, Extension, http::StatusCode, Json};
use sqlx::PgPool;

pub async fn get_all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let us = user_service::UserService::new(&pool);
    let users = us.get_all_users().await.unwrap();

    (StatusCode::OK, Json(users))
}
