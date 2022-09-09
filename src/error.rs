use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid credentials")]
    Unauthorized,

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Self::Other(ref root_cause) => {
                tracing::error!("{:?}", root_cause);
                match root_cause.downcast_ref::<sqlx::Error>() {
                    Some(sqlx::Error::RowNotFound) => {
                        (StatusCode::NOT_FOUND, root_cause.to_string())
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_owned()),
                }
            }
        };

        (status, Json(json!({ "error": msg }))).into_response()
    }
}
