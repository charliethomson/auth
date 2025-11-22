use chrono::{DateTime, Utc};
use data::dto::application::{ApplicationDetailDto, ApplicationDto};
use poem_openapi::Object;

use crate::models::application_grant::ApplicationGrant;

#[derive(Object, Debug)]
pub struct Application {
    pub application_id: String,
    pub display_name: String,
    pub description: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub grants: Vec<ApplicationGrant>,
}
impl From<ApplicationDetailDto> for Application {
    fn from(application: ApplicationDetailDto) -> Self {
        let mut this = Self::from(application.application);
        this.grants = application
            .grants
            .into_iter()
            .map(|grant| ApplicationGrant {
                grant_id: grant.grant_id,
                display_name: grant.display_name,
                description: grant.description,
                created_by: grant.created_by,
                updated_by: grant.updated_by,
                created_at: grant.created_at,
                updated_at: grant.updated_at,
            })
            .collect();

        this
    }
}
impl From<ApplicationDto> for Application {
    fn from(application: ApplicationDto) -> Self {
        Self {
            application_id: application.application_id,
            display_name: application.display_name,
            description: application.description,
            created_by: application.created_by,
            updated_by: application.updated_by,
            created_at: application.created_at,
            updated_at: application.updated_at,
            grants: vec![],
        }
    }
}
