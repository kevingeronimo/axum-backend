use crate::{
    error::{Error, ReportError},
    utils::jwt,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, TypedHeader};
use chrono::{Duration, Utc};
use error_stack::{IntoReport, ResultExt};
use headers::{authorization::Bearer, Authorization};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(1);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ReportError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .report()
                .change_context(Error::InvalidToken)?;

        // Decode the user data
        let claims = jwt::verify(bearer.token())?;

        Ok(claims)
    }
}