use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{api::ApiRepositories, models::application::Application, util::error::ApiError};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UpdateApplicationPayload {
    application_id: String,
    display_name: Option<String>,
    description: Option<String>,
}

#[derive(ApiResponse)]
pub enum UpdateApplicationResponse {
    #[oai(status = 200)]
    Ok(Json<Application>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn update_application(
    repositories: ApiRepositories,
    payload: UpdateApplicationPayload,
    agent: &str,
) -> UpdateApplicationResponse {
    match repositories
        .application
        .update(
            agent,
            &payload.application_id,
            payload.display_name.as_deref(),
            payload.description.as_deref(),
        )
        .await
    {
        Ok(app) => UpdateApplicationResponse::Ok(Json(Application::from(app))),
        Err(e) => UpdateApplicationResponse::Failed(Json(ApiError::from(e))),
    }
}
