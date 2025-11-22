use chrono::{DateTime, Utc};
use data::dto::grant::{GrantDetailDto, GrantDto};
use poem_openapi::Object;

use crate::models::grant_application::GrantApplication;

#[derive(Object, Debug)]
pub struct Grant {
    pub grant_id: String,
    pub application_id: String,
    pub display_name: String,
    pub description: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub application: Option<GrantApplication>,
}
impl From<GrantDto> for Grant {
    fn from(value: GrantDto) -> Self {
        Self {
            grant_id: value.grant_id,
            application_id: value.application_id,
            display_name: value.display_name,
            description: value.description,
            created_by: value.created_by,
            updated_by: value.updated_by,
            created_at: value.created_at,
            updated_at: value.updated_at,
            application: None,
        }
    }
}
impl From<GrantDetailDto> for Grant {
    fn from(value: GrantDetailDto) -> Self {
        let mut this = Self::from(value.grant);

        this.application = Some(GrantApplication {
            application_id: value.application.application_id,
            display_name: value.application.display_name,
            description: value.application.description,
            created_by: value.application.created_by,
            updated_by: value.application.updated_by,
            created_at: value.application.created_at,
            updated_at: value.application.updated_at,
        });

        this
    }
}
