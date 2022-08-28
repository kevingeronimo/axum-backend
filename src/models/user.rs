use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::dto::UserDto;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

impl User {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_username(username: String, pool: &PgPool) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=$1")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    pub async fn create(user: UserDto, pool: &PgPool) -> Result<User, Error> {
        let sql = "
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING *
            ";

        sqlx::query_as::<_, User>(sql)
            .bind(user.username)
            .bind(user.password)
            .fetch_one(pool)
            .await
    }
}
