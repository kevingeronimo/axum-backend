use axum::{http::StatusCode, response::IntoResponse, Json};
use error_stack::Report;
use serde_json::json;
use std::{error::Error as StdError, fmt};
use tracing::{event, Level};

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
    SqlxError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self {
            Self::WrongCredentials => "wrong credentials",
            Self::DuplicateUserName => "username already taken",
            Self::InvalidToken => "invalid token",
            Self::UserNotFound => "user not found",
            _ => "internal server error",
        };

        f.write_str(err_msg)
    }
}

impl StdError for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::WrongCredentials => StatusCode::UNAUTHORIZED,
            Self::DuplicateUserName => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}

pub struct ErrorStackReport(Report<Error>);

impl From<Report<Error>> for ErrorStackReport {
    fn from(report: Report<Error>) -> Self {
        ErrorStackReport(report)
    }
}

impl IntoResponse for ErrorStackReport {
    fn into_response(self) -> axum::response::Response {
        event!(Level::ERROR, "{:?}", self.0);
        self.0.current_context().into_response()
    }
}
