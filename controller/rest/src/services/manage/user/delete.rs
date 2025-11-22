use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, util::error::ApiError};

#[derive(ApiResponse)]
pub enum DeleteUserResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn delete_user(repositories: ApiRepositories, user_id: i32) -> DeleteUserResponse {
    match repositories.user.delete(user_id).await {
        Ok(_) => DeleteUserResponse::Ok,
        Err(e) => DeleteUserResponse::Failed(Json(ApiError::from(e))),
    }
}
