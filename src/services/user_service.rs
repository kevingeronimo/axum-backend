use crate::{daos::user_dao, models::user};
use sqlx::{Error, PgPool};

pub struct UserService<'a> {
    user_dao: user_dao::UserDao<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        UserService {
            user_dao: user_dao::UserDao::new(conn),
        }
    }

    pub async fn get_all_users(self) -> Result<Vec<user::User>, Error> {
        self.user_dao.get_all_users().await
    }

    pub async fn get_user_by_id(self, id: i32) -> Result<user::User, Error> {
        self.user_dao.get_user_by_id(id).await
    }
}
