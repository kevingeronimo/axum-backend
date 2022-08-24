use crate::{
    models::user,
    daos::user_dao
};
use sqlx::{PgPool, Error};

pub struct UserService<'a> {
    user_dao: user_dao::UserDao<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        UserService { user_dao: user_dao::UserDao::new(conn) }
    }

    pub async fn get_all_users(self) -> Result<Vec<user::User>, Error> {
        self.user_dao.get_all_users().await
    }
}