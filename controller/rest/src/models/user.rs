use std::collections::HashMap;

use chrono::Utc;
use data::dto::user::{UserDetailDto, UserDto};
use poem_openapi::Object;

use crate::models::user_grant::UserGrant;

#[derive(Object, Debug)]
pub struct User {
    pub user_id: i32,
    pub display_name: String,
    pub username: String,
    pub enabled: bool,
    pub email: Option<String>,
    pub image_url: Option<String>,
    pub last_login: Option<chrono::DateTime<Utc>>,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,

    pub grants: HashMap<String, UserGrant>,
}
impl From<UserDetailDto> for User {
    fn from(user: UserDetailDto) -> Self {
        let mut this = Self::from(user.user);
        this.grants = user
            .grants
            .into_iter()
            .map(|ug| {
                (
                    ug.grant.grant.grant_id.clone(),
                    UserGrant {
                        grant_id: ug.grant.grant.grant_id,
                        application_id: ug.grant.application.application_id,
                        display_name: ug.grant.grant.display_name,
                        description: ug.grant.grant.description,
                        enabled: ug.user_grant.enabled,
                        enabled_at: ug.user_grant.enabled_at,
                        disabled_at: ug.user_grant.disabled_at,
                        created_by: ug.user_grant.created_by,
                        updated_by: ug.user_grant.updated_by,
                        created_at: ug.user_grant.created_at,
                        updated_at: ug.user_grant.updated_at,
                    },
                )
            })
            .collect();

        this
    }
}

impl From<UserDto> for User {
    fn from(user: UserDto) -> Self {
        Self {
            user_id: user.user_id,
            display_name: user.display_name,
            username: user.username,
            enabled: user.enabled,
            email: user.email,
            image_url: user.image_url,
            last_login: user.last_login,
            created_by: user.created_by,
            updated_by: user.updated_by,
            created_at: user.created_at,
            updated_at: user.updated_at,
            grants: HashMap::new(),
        }
    }
}
