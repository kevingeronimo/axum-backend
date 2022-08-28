use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::{error::Error as StdError, fmt};

pub type Result<T> = error_stack::Result<T, self::Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    BadRequest,
    UserNotFound,
    InternalServerError,
    BcryptError,
    TokioRecvError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error: something went wrong")
    }
}

impl StdError for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
