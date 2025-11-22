use sea_orm::{prelude::DateTime, sqlx::types::chrono::Utc};
use serde::{Deserialize, Serialize};
use valuable::Valuable;

use crate::{
    dto::{error::DtoError, grant::GrantDto},
    impl_try_from_with,
};

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct ApplicationDto {
    #[valuable(skip)]
    pub application_id: String,
    pub display_name: String,
    pub description: String,
    pub created_by: String,
    pub updated_by: String,
    #[valuable(skip)]
    pub created_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
    #[valuable(skip)]
    pub updated_at: sea_orm::sqlx::types::chrono::DateTime<Utc>,
}

impl ApplicationDto {
    pub fn from_ordered(
        application_id: String,
        display_name: String,
        description: String,
        created_by: String,
        updated_by: String,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Result<Self, DtoError> {
        Ok(Self {
            application_id: application_id,
            display_name: display_name,
            description: description,
            created_by: created_by,
            updated_by: updated_by,
            created_at: created_at.and_utc(),
            updated_at: updated_at.and_utc(),
        })
    }
}

impl_try_from_with!(
    ApplicationDto,
    application,
    from_ordered,
    DtoError,
    [
        application_id,
        display_name,
        description,
        created_by,
        updated_by,
        created_at,
        updated_at,
    ]
);

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct ApplicationDetailDto {
    pub application: ApplicationDto,
    pub grants: Vec<GrantDto>,
}
