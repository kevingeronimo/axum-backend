use crate::models::user;
use sqlx::{Error, PgPool};

pub struct UserDao<'a> {
    conn: &'a PgPool,
}

impl<'a> UserDao<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        UserDao { conn }
    }

    pub async fn get_all_users(self) -> Result<Vec<user::User>, Error> {
        sqlx::query_as::<_, user::User>("SELECT * FROM users")
            .fetch_all(self.conn)
            .await
    }

    pub async fn get_user_by_id(self, id: i32) -> Result<user::User, Error> {
        sqlx::query_as::<_, user::User>("SELECT * FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(self.conn)
            .await
    }
}
