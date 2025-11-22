use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::grant::Grant, util::error::ApiError};

#[derive(ApiResponse)]
pub enum GetGrantByIdResponse {
    #[oai(status = 200)]
    Ok(Json<Grant>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
    #[oai(status = 404)]
    NotFound,
}

pub async fn get_grant_by_id(
    repositories: ApiRepositories,
    grant_id: &str,
) -> GetGrantByIdResponse {
    match repositories.grant.by_id(grant_id).await {
        Ok(Some(grant)) => GetGrantByIdResponse::Ok(Json(Grant::from(grant))),
        Ok(None) => GetGrantByIdResponse::NotFound,
        Err(e) => GetGrantByIdResponse::Failed(Json(ApiError::from(e))),
    }
}
