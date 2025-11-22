use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::application::Application, util::error::ApiError};

#[derive(ApiResponse)]
pub enum GetApplicationResponse {
    #[oai(status = 200)]
    Ok(Json<Application>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn get_application(
    repositories: ApiRepositories,
    application_id: String,
) -> GetApplicationResponse {
    match repositories.application.by_id(&application_id).await {
        Ok(Some(app)) => GetApplicationResponse::Ok(Json(Application::from(app))),
        Ok(None) => GetApplicationResponse::NotFound,
        Err(e) => GetApplicationResponse::Failed(Json(ApiError::from(e))),
    }
}
