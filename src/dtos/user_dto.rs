use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserDto {
    pub username: String,
    pub password: String,
}
