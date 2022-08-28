use crate::{models::user::User, dtos::user_dto::UserDto};
use sqlx::{Error, PgPool};

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
        User::get_all(pool).await
    }

    pub async fn get_user_by_id(id: i32, pool: &PgPool) -> Result<User, Error> {
        User::get_by_id(id, pool).await
    }

    pub async fn create_user(user: UserDto, pool: &PgPool) -> Result<User, Error> {
        User::create(user, pool).await
    }
}
