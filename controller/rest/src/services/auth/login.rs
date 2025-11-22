use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    api::ApiRepositories,
    models::user::User,
    services::{ApiServices, core::jwt::Claims},
    util::error::ApiError,
};

#[derive(Object, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Object, Debug)]
pub struct LoginResponsePayload {
    pub user: User,
    pub claims: Claims,
    pub token: String,
}

#[derive(ApiResponse)]
pub enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<LoginResponsePayload>),
    #[oai(status = 400)]
    InvalidCredentials,
    #[oai(status = 500)]
    Failed(Json<ApiError>),
}

#[tracing::instrument(level = tracing::Level::INFO, "services.auth.login", skip(repositories, services, payload), fields(username = %payload.username))]
pub async fn login(
    repositories: ApiRepositories,
    services: ApiServices,
    payload: LoginPayload,
) -> LoginResponse {
    tracing::info!("Login attempt for user: {}", payload.username);

    let user = match repositories.user.by_username(&payload.username).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(e) => {
            tracing::error!("Database error during login: {:?}", e);
            return LoginResponse::Failed(Json(ApiError::from(e)));
        }
    };

    // Always verify to prevent timing attacks
    let verification_result = match &user {
        Some(u) => services.hasher.verify(&u.user.password, &payload.password),
        None => {
            // Perform dummy verification with dummy hash to normalize timing
            services.hasher.dummy_verification(&payload.password);
            return LoginResponse::InvalidCredentials;
        }
    };

    match verification_result {
        Ok(_) if user.is_some() => {
            tracing::info!("Successful login for user: {}", payload.username);
        }
        _ => {
            tracing::warn!("Failed login attempt for user: {}", payload.username);
            return LoginResponse::InvalidCredentials;
        }
    };

    let user = User::from(user.unwrap());
    let claims = Claims::r#for(&user);

    let token = match services.jwt.sign(&claims) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("JWT signing error: {:?}", e);
            return LoginResponse::Failed(Json(ApiError::from(e)));
        }
    };

    LoginResponse::Ok(Json(LoginResponsePayload {
        user,
        claims,
        token,
    }))
}
