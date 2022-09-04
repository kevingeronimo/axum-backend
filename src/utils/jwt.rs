use error_stack::{IntoReport, ResultExt};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

use crate::{
    error::{Error, Result},
    extractor::Claims,
};

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

pub fn sign(id: i64) -> Result<String> {
    let token = jsonwebtoken::encode(&Header::default(), &Claims::new(id), &KEYS.encoding)
        .report()
        .change_context(Error::TokenCreation)?;

    Ok(token)
}

pub fn verify(token: &str) -> Result<Claims> {
    let claims = jsonwebtoken::decode(token, &KEYS.decoding, &Validation::default())
        .map(|data| data.claims)
        .report()
        .change_context(Error::InvalidToken)
        .attach_printable("Failed to verify token")?;

    Ok(claims)
}
