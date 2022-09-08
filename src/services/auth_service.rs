use crate::dto::RegisterDto;
use crate::error::{Error, Result};
use crate::{dto::LoginDto, models::user::User, utils::bcrypt_hash};
use anyhow::{Context, anyhow};
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(dto: LoginDto, pool: &PgPool) -> Result<User> {
        let user = User::get_by_username(&dto.username, pool)
            .await
            .with_context(|| format!("no user with username=\"{}\"", dto.username))?;

        if bcrypt_hash::verify_password(dto.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(anyhow!("wrong password").into())
        }
    }                    

    pub async fn sign_up(dto: RegisterDto, pool: &PgPool) -> Result<User> {
        if User::get_by_username(&dto.username, pool).await.is_ok() {
            return Err(Error::Conflict("username already taken".to_string()));
        }

        // password is dropped after hashing.
        let password = bcrypt_hash::hash_password(dto.password).await?;
        let dto = RegisterDto {
            username: dto.username,
            password,
        };

        User::create(dto, pool).await.map_err(|e| e.into())
    }
}
