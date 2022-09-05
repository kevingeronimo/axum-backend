use crate::dto::RegisterDto;
use crate::error::{Error, Result};
use crate::{dto::LoginDto, models::user::User, utils::bcrypt_hash};
use error_stack::{report, IntoReport, ResultExt};
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(dto: LoginDto, pool: &PgPool) -> Result<User> {
        let user = User::get_by_username(&dto.username, pool)
            .await
            .report()
            .change_context(Error::UserNotFound)
            .attach_printable(format!("No user with username = \"{}\"", dto.username))?;
        if bcrypt_hash::verify_password(dto.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(report!(Error::WrongCredentials))
        }
    }

    pub async fn sign_up(dto: RegisterDto, pool: &PgPool) -> Result<User> {
        if User::get_by_username(&dto.username, pool).await.is_ok() {
            return Err(report!(Error::DuplicateUserName));
        }

        // password is dropped after hashing.
        let password = bcrypt_hash::hash_password(dto.password).await?;
        let dto = RegisterDto {
            username: dto.username,
            password,
        };

        User::create(dto, pool)
            .await
            .report()
            .attach_printable("Fail to create new user")
            .change_context(Error::SqlxError)
    }
}
