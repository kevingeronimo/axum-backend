use crate::{
    dto::{LoginDto, RegisterDto},
    error,
    services::auth_service::AuthService,
};
use anyhow::Context;
use async_session::Session;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Extension(mut session): Extension<Session>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_in(login_dto, &pool).await?;

    session
        .insert("username", &user.username)
        .context("hi from anyhow")?;

    Ok((StatusCode::OK, Extension(session), Json(user)))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(register_dto): Json<RegisterDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_up(register_dto, &pool).await?;

    Ok((StatusCode::CREATED, Json(user)))
}
