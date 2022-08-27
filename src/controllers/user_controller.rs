use crate::{error, services::user_service};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use error_stack::{IntoReport, ResultExt};
use sqlx::PgPool;
use tracing::{event, instrument, Level};

#[instrument(skip_all)]
pub async fn get_all_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, error::Error> {
    let users = user_service::UserService::get_all_users(&pool)
        .await
        .report()
        .attach_printable("Could not retrieve all users")
        .change_context(error::Error::InternalServerError)
        .map_err(|e| {
            event!(Level::ERROR, "{e:?}");
            *e.current_context()
        })?;

    Ok((StatusCode::OK, Json(users)))
}

#[instrument(skip_all, fields(id=id))]
pub async fn get_user_by_id(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, error::Error> {
    let user = user_service::UserService::get_user_by_id(id, &pool)
        .await
        .report()
        .attach_printable(format!("Could not find user with id={id}"))
        .change_context(error::Error::UserNotFound)
        .map_err(|e| {
            event!(Level::ERROR, "{e:?}");
            *e.current_context()
        })?;

    Ok((StatusCode::OK, Json(user)))
}
