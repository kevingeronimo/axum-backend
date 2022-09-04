use chrono::{Duration, Utc};
use error_stack::{IntoReport, ResultExt};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: i64) -> Result<String> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &KEYS.encoding,
    )
    .report()
    .change_context(Error::TokenCreation)?;

    Ok(token)
}

pub fn verify(token: &str) -> Result<Claims> {
    let claims = jsonwebtoken::decode(
        token,
        &KEYS.decoding,
        &Validation::default(),
    )
    .map(|data| data.claims)
    .report()
    .change_context(Error::InvalidToken)?;

    Ok(claims)
}
