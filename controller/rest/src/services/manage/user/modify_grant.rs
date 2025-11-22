use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{api::ApiRepositories, util::error::ApiError};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct ModifyGrantPayload {
    pub user_id: i32,
    pub grant_id: String,
    pub enabled: bool,
}

#[derive(ApiResponse)]
pub enum ModifyGrantResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn modify_grant(
    repositories: ApiRepositories,
    payload: ModifyGrantPayload,
    agent: &str,
) -> ModifyGrantResponse {
    match repositories
        .user
        .update_grant(agent, payload.user_id, &payload.grant_id, payload.enabled)
        .await
    {
        Ok(_) => ModifyGrantResponse::Ok,
        Err(e) => ModifyGrantResponse::Failed(Json(ApiError::from(e))),
    }
}
