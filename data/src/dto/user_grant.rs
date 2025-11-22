use sea_orm::{prelude::DateTime, sqlx::types::chrono::Utc};
use serde::{Deserialize, Serialize};
use valuable::Valuable;

use crate::{
    dto::{error::DtoError, grant::GrantDetailDto},
    impl_try_from_with,
};

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct UserGrantDto {
    pub user_id: i32,
    pub grant_id: String,
    pub enabled: bool,
    #[valuable(skip)]
    pub enabled_at: Option<sea_orm::sqlx::types::chrono::DateTime<Utc>>,
    #[valuable(skip)]
    pub disabled_at: Option<sea_orm::sqlx::types::chrono::DateTime<Utc>>,
    pub created_by: String,
    pub updated_by: String,
    #[valuable(skip)]
    pub created_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
    #[valuable(skip)]
    pub updated_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
}

impl UserGrantDto {
    pub fn from_ordered(
        user_id: i32,
        grant_id: String,
        enabled: i8,
        enabled_at: Option<DateTime>,
        disabled_at: Option<DateTime>,
        created_by: String,
        updated_by: String,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Result<Self, DtoError> {
        Ok(Self {
            user_id,
            grant_id,
            enabled: enabled != 0,
            enabled_at: enabled_at.map(|dt| dt.and_utc()),
            disabled_at: disabled_at.map(|dt| dt.and_utc()),
            created_by,
            updated_by,
            created_at: created_at.and_utc(),
            updated_at: updated_at.and_utc(),
        })
    }
}

impl_try_from_with!(
    UserGrantDto,
    user_grant,
    from_ordered,
    DtoError,
    [
        user_id,
        grant_id,
        enabled,
        enabled_at,
        disabled_at,
        created_by,
        updated_by,
        created_at,
        updated_at,
    ]
);

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct UserGrantDetailDto {
    pub user_grant: UserGrantDto,
    pub grant: GrantDetailDto,
}
