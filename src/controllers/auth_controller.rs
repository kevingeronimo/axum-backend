use crate::{
    dto::{AuthBodyDto, LoginDto, RegisterDto},
    error,
    extractor::Claims,
    services::auth_service::AuthService,
    utils::jwt,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(login_dto): Json<LoginDto>,
) -> Result<impl IntoResponse, error::ReportError> {
    let user = AuthService::sign_in(login_dto, &pool).await?;
    let token = jwt::sign(user.id)?;

    Ok(Json(AuthBodyDto::new(token)))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(register_dto): Json<RegisterDto>,
) -> Result<impl IntoResponse, error::ReportError> {
    let user = AuthService::sign_up(register_dto, &pool).await?;
    let token = jwt::sign(user.id)?;

    Ok((StatusCode::CREATED, Json(AuthBodyDto::new(token))))
}

pub async fn protected(claims: Claims) -> Result<String, error::Error> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{:?}",
        claims
    ))
}
