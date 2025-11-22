use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::grant::Grant, util::error::ApiError};

#[derive(ApiResponse)]
pub enum GetGrantByApplicationIdResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Grant>>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn get_grants_by_application_id(
    repositories: ApiRepositories,
    application_id: &str,
) -> GetGrantByApplicationIdResponse {
    match repositories.grant.by_application(application_id).await {
        Ok(grants) => {
            GetGrantByApplicationIdResponse::Ok(Json(grants.into_iter().map(Grant::from).collect()))
        }
        Err(e) => GetGrantByApplicationIdResponse::Failed(Json(ApiError::from(e))),
    }
}
