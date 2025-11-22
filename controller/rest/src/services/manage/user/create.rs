use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    api::ApiRepositories, models::user::User, services::ApiServices, util::error::ApiError,
};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateUserPayload {
    #[oai(validator(min_length = 3))]
    username: String,
    #[oai(validator(min_length = 8))]
    password: String,
    display_name: Option<String>,
    email: Option<String>,
    image_url: Option<String>,
}

#[derive(ApiResponse)]
pub enum CreateUserResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn create_user(
    repositories: ApiRepositories,
    services: ApiServices,
    payload: CreateUserPayload,
    agent: &str,
) -> CreateUserResponse {
    let hash = match services.hasher.hash(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return CreateUserResponse::Failed(Json(ApiError::from(e))),
    };

    match repositories
        .user
        .create(
            agent,
            &payload.username,
            &hash,
            payload.display_name.as_deref(),
            payload.email.as_deref(),
            payload.image_url.as_deref(),
        )
        .await
    {
        Ok(user) => CreateUserResponse::Ok(Json(User::from(user))),
        Err(e) => CreateUserResponse::Failed(Json(ApiError::from(e))),
    }
}
