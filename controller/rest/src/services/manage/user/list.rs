use poem_openapi::{ApiResponse, payload::Json};

use crate::{api::ApiRepositories, models::user::User, util::error::ApiError};

#[derive(ApiResponse)]
pub enum ListUsersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<User>>),
    #[oai(status = 500)]
    Failed(Json<ApiError>),
    #[oai(status = 401)]
    Unauthorized,
}

pub async fn list_users(repositories: ApiRepositories, enabled: Option<bool>) -> ListUsersResponse {
    match repositories.user.list(enabled).await {
        Ok(users) => ListUsersResponse::Ok(Json(users.into_iter().map(User::from).collect())),
        Err(e) => ListUsersResponse::Failed(Json(ApiError::from(e))),
    }
}
