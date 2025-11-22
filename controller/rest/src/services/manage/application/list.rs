use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::application::Application, util::error::ApiError};

#[derive(ApiResponse)]
pub enum ListApplicationsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Application>>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn list_applications(repositories: ApiRepositories) -> ListApplicationsResponse {
    match repositories.application.list().await {
        Ok(apps) => {
            ListApplicationsResponse::Ok(Json(apps.into_iter().map(Application::from).collect()))
        }
        Err(e) => ListApplicationsResponse::Failed(Json(ApiError::from(e))),
    }
}
