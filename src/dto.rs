use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBodyDto {
    access_token: String,
    token_type: String,
}

impl AuthBodyDto {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}