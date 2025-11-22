use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{api::ApiRepositories, models::application::Application, util::error::ApiError};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateApplicationPayload {
    #[oai(validator(min_length = 3))]
    application_id: String,
    display_name: Option<String>,
    description: String,
}

#[derive(ApiResponse)]
pub enum CreateApplicationResponse {
    #[oai(status = 200)]
    Ok(Json<Application>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn create_application(
    repositories: ApiRepositories,
    payload: CreateApplicationPayload,
    agent: &str,
) -> CreateApplicationResponse {
    match repositories
        .application
        .create(
            agent,
            &payload.application_id,
            &payload
                .display_name
                .unwrap_or(payload.application_id.clone()),
            &payload.description,
        )
        .await
    {
        Ok(app) => CreateApplicationResponse::Ok(Json(Application::from(app))),
        Err(e) => CreateApplicationResponse::Failed(Json(ApiError::from(e))),
    }
}
