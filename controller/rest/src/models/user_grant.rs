use chrono::Utc;
use poem_openapi::Object;

#[derive(Object, Debug)]
pub struct UserGrant {
    pub grant_id: String,
    pub application_id: String,
    pub display_name: String,
    pub description: String,

    pub enabled: bool,
    pub enabled_at: Option<chrono::DateTime<Utc>>,
    pub disabled_at: Option<chrono::DateTime<Utc>>,
    // NOTE: these fields refer to the user <-> grant relation, NOT the grant itself
    pub created_by: String,
    pub updated_by: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
