use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::{error::Error as StdError, fmt};

pub type Result<T> = error_stack::Result<T, self::Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    TokenCreation,
    InvalidToken,
    UserNotFound,
    DuplicateUserName,
    BcryptError,
    TokioRecvError,
    WrongCredentials,
    SqlxError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Error: {self:?}"))
    }
}

impl StdError for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Self::DuplicateUserName => (StatusCode::BAD_REQUEST, "Username already taken"),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
