use crate::{
    dto::{LoginDto, RegisterDto},
    error,
    services::auth_service::AuthService,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_in(login_dto, &pool).await?;

    Ok(Json(user))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(register_dto): Json<RegisterDto>,
) -> Result<impl IntoResponse, error::Error> {
    let user = AuthService::sign_up(register_dto, &pool).await?;

    Ok((StatusCode::CREATED, Json(user)))
}
