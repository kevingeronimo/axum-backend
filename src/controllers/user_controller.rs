use crate::{error, services::user_service};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use error_stack::{IntoReport, ResultExt};
use sqlx::PgPool;
use tracing::{event, instrument, Level};

#[instrument]
pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, error::Error> {
    let us = user_service::UserService::new(&pool);
    let users = us
        .get_all_users()
        .await
        .report()
        .attach_printable("Could not retrieve all users")
        .change_context(error::Error::InternalServerError);

    match users {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(e) => {
            event!(Level::ERROR, "{e:?}");
            Err(*e.current_context())
        }
    }
}
