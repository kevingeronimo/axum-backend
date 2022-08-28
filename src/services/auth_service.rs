use crate::dto::RegisterDto;
use crate::error::{Error, Result};
use crate::{dto::LoginDto, models::user::User, utils::encryption};
use error_stack::{IntoReport, Report, ResultExt};
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(dto: LoginDto, pool: &PgPool) -> Result<User> {
        let user = User::get_by_username(&dto.username, pool)
            .await
            .report()
            .change_context(Error::WrongCredentials)?;
        if encryption::verify_password(dto.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Report::new(Error::WrongCredentials).attach_printable("Password is incorrect"))
        }
    }

    pub async fn sign_up(dto: RegisterDto, pool: &PgPool) -> Result<User> {
        // password is dropped after hashing.
        let password = encryption::hash_password(dto.password).await?;
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
