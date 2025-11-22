use chrono::{DateTime, Utc};
use poem_openapi::Object;

#[derive(Object, Debug)]
pub struct ApplicationGrant {
    pub grant_id: String,
    pub display_name: String,
    pub description: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
