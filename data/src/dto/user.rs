use sea_orm::{prelude::DateTime, sqlx::types::chrono::Utc};
use serde::{Deserialize, Serialize};
use valuable::Valuable;

use crate::{
    dto::{error::DtoError, user_grant::UserGrantDetailDto},
    impl_try_from_with,
};

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct UserDto {
    pub user_id: i32,
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub enabled: bool,
    pub email: Option<String>,
    pub image_url: Option<String>,
    #[valuable(skip)]
    pub last_login: Option<sea_orm::sqlx::types::chrono::DateTime<Utc>>,
    pub created_by: String,
    pub updated_by: String,
    #[valuable(skip)]
    pub created_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
    #[valuable(skip)]
    pub updated_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
}

impl UserDto {
    pub fn from_ordered(
        user_id: i32,
        display_name: String,
        username: String,
        password: String,
        enabled: i8,
        email: Option<String>,
        image_url: Option<String>,
        last_login: Option<DateTime>,
        created_by: String,
        updated_by: String,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Result<Self, DtoError> {
        Ok(Self {
            user_id,
            display_name,
            username,
            password,
            enabled: enabled != 0,
            email,
            image_url,
            last_login: last_login.map(|dt| dt.and_utc()),
            created_by,
            updated_by,
            created_at: created_at.and_utc(),
            updated_at: updated_at.and_utc(),
        })
    }
}

impl_try_from_with!(
    UserDto,
    user,
    from_ordered,
    DtoError,
    [
        user_id,
        display_name,
        username,
        password,
        enabled,
        email,
        image_url,
        last_login,
        created_by,
        updated_by,
        created_at,
        updated_at,
    ]
);

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct UserDetailDto {
    pub user: UserDto,
    pub grants: Vec<UserGrantDetailDto>,
}
