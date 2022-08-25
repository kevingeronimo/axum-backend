use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    id: i32,
    username: String,
    password: String
}
