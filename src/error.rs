use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid credentials")]
    Unauthorized,

    #[error("{0}")]
    Conflict(String),

    #[error("{0:?}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Conflict(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::Other(ref root_cause) => {
                match root_cause.downcast_ref::<sqlx::Error>() {
                    Some(sqlx::Error::RowNotFound) => {
                        (StatusCode::NOT_FOUND, "not found".to_owned())
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_owned()),
                }
            }
        };
        
        tracing::error!("{}", self);
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
