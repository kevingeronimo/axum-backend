use crate::{error, services::user_service};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use error_stack::{IntoReport, ResultExt};
use sqlx::PgPool;
use tracing::{event, instrument, Level};

#[instrument]
pub async fn get_all_users(
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, error::Error> {
    let us = user_service::UserService::new(&pool);
    let users_result = us
        .get_all_users()
        .await
        .report()
        .attach_printable("Could not retrieve all users")
        .change_context(error::Error::InternalServerError);

    match users_result {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(e) => {
            event!(Level::ERROR, "{e:?}");
            Err(*e.current_context())
        }
    }
}

#[instrument]
pub async fn get_user_by_id(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, error::Error> {
    let us = user_service::UserService::new(&pool);
    let user_result = us
        .get_user_by_id(id)
        .await
        .report()
        .attach_printable(format!("Could not find user with id={id}"))
        .change_context(error::Error::UserNotFound);

    match user_result {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => {
            event!(Level::ERROR, "{e:?}");
            Err(*e.current_context())
        }
    }
}
