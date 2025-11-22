use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{api::ApiRepositories, models::grant::Grant, util::error::ApiError};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateGrantPayload {
    #[oai(validator(min_length = 3))]
    grant_id: String,
    #[oai(validator(min_length = 3))]
    application_id: String,
    display_name: Option<String>,
    description: String,
}

#[derive(ApiResponse)]
pub enum CreateGrantResponse {
    #[oai(status = 200)]
    Ok(Json<Grant>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn create_grant(
    repositories: ApiRepositories,
    payload: CreateGrantPayload,
    agent: &str,
) -> CreateGrantResponse {
    match repositories
        .grant
        .create(
            agent,
            &payload.grant_id,
            &payload.application_id,
            &payload.display_name.unwrap_or(payload.grant_id.clone()),
            &payload.description,
        )
        .await
    {
        Ok(grant) => CreateGrantResponse::Ok(Json(Grant::from(grant))),
        Err(e) => CreateGrantResponse::Failed(Json(ApiError::from(e))),
    }
}
