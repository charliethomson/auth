use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::user::User, util::error::ApiError};

#[derive(ApiResponse)]
pub enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 401)]
    Unauthorized,
    #[oai(status = 500)]
    Failed(Json<ApiError>),
}

pub async fn get_user(repositories: ApiRepositories, user_id: i32) -> GetUserResponse {
    match repositories.user.by_id(user_id).await {
        Ok(Some(user)) => GetUserResponse::Ok(Json(User::from(user))),
        Ok(None) => GetUserResponse::NotFound,
        Err(e) => GetUserResponse::Failed(Json(ApiError::from(e))),
    }
}
