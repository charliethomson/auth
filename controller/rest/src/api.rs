use std::io;

use chrono::Utc;
use data::repository::{
    application::ApplicationRepository, connect, error::RepositoryError, grant::GrantRepository,
    user::UserRepository,
};
use libbuildinfo::BuildInfo;
use poem::{Request, http::StatusCode, web::Data};
use poem_openapi::{
    OpenApi, SecurityScheme, Tags,
    auth::Bearer,
    param::{Path, Query},
    payload::{Json, PlainText},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

use crate::{
    Args,
    services::{
        ApiServices,
        auth::login::{LoginPayload, LoginResponse, login},
        core::jwt::Claims,
        manage::{
            application::{
                create::{CreateApplicationPayload, CreateApplicationResponse, create_application},
                get::{GetApplicationResponse, get_application},
                list::{ListApplicationsResponse, list_applications},
                update::{UpdateApplicationPayload, UpdateApplicationResponse, update_application},
            },
            grant::{
                create::{CreateGrantPayload, CreateGrantResponse, create_grant},
                get_by_application::{
                    GetGrantByApplicationIdResponse, get_grants_by_application_id,
                },
                get_by_id::{GetGrantByIdResponse, get_grant_by_id},
            },
            user::{
                create::{CreateUserPayload, CreateUserResponse, create_user},
                delete::{DeleteUserResponse, delete_user},
                get::{GetUserResponse, get_user},
                list::{ListUsersResponse, list_users},
                modify_grant::{ModifyGrantPayload, ModifyGrantResponse, modify_grant},
            },
        },
    },
    util::grants::{Grants, HasGrants},
};

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum ApiRepositoriesError {
    #[error(transparent)]
    Repository {
        #[from]
        inner_error: RepositoryError,
    },
}
#[derive(Clone, Debug)]
pub struct ApiRepositories {
    pub user: UserRepository,
    pub grant: GrantRepository,
    pub application: ApplicationRepository,
}
impl ApiRepositories {
    pub async fn new(args: &Args, _build_info: &BuildInfo) -> Result<Self, ApiRepositoriesError> {
        let conn = connect(&args.database_url).await?;
        Ok(Self {
            user: UserRepository::new(conn.clone()),
            grant: GrantRepository::new(conn.clone()),
            application: ApplicationRepository::new(conn.clone()),
        })
    }
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "BearerJwt::extract")]
pub struct BearerJwt(Claims);
impl BearerJwt {
    async fn extract(req: &&Request, from_request: Bearer) -> poem::Result<Claims> {
        let Some(services) = req.data::<ApiServices>() else {
            return Err(poem::Error::new(
                io::Error::other("Failed to acquire services"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        };

        let claims = services.jwt.verify(&from_request.token).map_err(|e| {
            tracing::error!("JWT verification failed: {e}");
            poem::Error::new(io::Error::other("Unauthorized"), StatusCode::UNAUTHORIZED)
        })?;

        if claims.issuer != crate::PRODUCT_IDENTIFIER {
            tracing::error!(
                "JWT verification failed: Received invalid issuer '{}', expected '{}'",
                claims.issuer,
                crate::PRODUCT_IDENTIFIER
            );
            return Err(poem::Error::new(
                io::Error::other("Unauthorized"),
                StatusCode::UNAUTHORIZED,
            ));
        }

        let now = Utc::now().timestamp() as u64;

        if claims.issued_at > now {
            tracing::error!(
                "JWT verification failed: Received invalid issued_at '{}' > '{}'",
                claims.issued_at,
                now
            );
            return Err(poem::Error::new(
                io::Error::other("Unauthorized"),
                StatusCode::UNAUTHORIZED,
            ));
        }
        if claims.expires < now {
            tracing::error!(
                "JWT verification failed: Received expired token '{}' < '{}'",
                claims.expires,
                now
            );
            return Err(poem::Error::new(
                io::Error::other("Unauthorized"),
                StatusCode::UNAUTHORIZED,
            ));
        }

        Ok(claims)
    }
}

pub trait SwaggerApi {
    fn base_uri() -> String;
    fn name() -> String;
}

#[derive(Clone)]
pub struct Api;
impl SwaggerApi for Api {
    fn base_uri() -> String {
        "/".into()
    }
    fn name() -> String {
        "Auth API".into()
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/me", method = "get")]
    async fn me(&self, repositories: Data<&ApiRepositories>, claims: BearerJwt) -> GetUserResponse {
        let user_id = claims.0.user_id;
        get_user(repositories.0.clone(), user_id).await
    }

    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        repositories: Data<&ApiRepositories>,
        services: Data<&ApiServices>,
        payload: Json<LoginPayload>,
    ) -> LoginResponse {
        login(repositories.0.clone(), services.0.clone(), payload.0).await
    }
}

#[derive(Clone)]
pub struct ManageApi;
impl SwaggerApi for ManageApi {
    fn base_uri() -> String {
        "/manage".into()
    }
    fn name() -> String {
        "Auth Management API".into()
    }
}

#[derive(Tags)]
pub enum ManageTags {
    User,
    Application,
    Grant,
}

#[OpenApi]
impl ManageApi {
    #[oai(path = "/user", method = "post", tag = ManageTags::User)]
    async fn user_create(
        &self,
        repositories: Data<&ApiRepositories>,
        services: Data<&ApiServices>,
        claims: BearerJwt,
        payload: Json<CreateUserPayload>,
    ) -> CreateUserResponse {
        if !claims.0.has_grants(&[Grants::UserCreate]) {
            return CreateUserResponse::Unauthorized;
        }

        let agent = &format!("user.create:{}", claims.0.user_id);

        create_user(repositories.0.clone(), services.0.clone(), payload.0, agent).await
    }

    #[oai(path = "/user/:user_id", method = "delete", tag = ManageTags::User)]
    async fn user_delete(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        user_id: Path<i32>,
    ) -> DeleteUserResponse {
        if !claims.0.has_grants(&[Grants::UserDelete]) {
            return DeleteUserResponse::Unauthorized;
        }

        delete_user(repositories.0.clone(), *user_id).await
    }

    #[oai(path = "/user", method = "get", tag = ManageTags::User)]
    async fn user_list(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        enabled: Query<Option<bool>>,
    ) -> ListUsersResponse {
        if !claims.0.has_grants(&[Grants::UserList]) {
            return ListUsersResponse::Unauthorized;
        }

        list_users(repositories.0.clone(), enabled.0).await
    }

    #[oai(path = "/user/:user_id", method = "get", tag = ManageTags::User)]
    async fn user_get(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        user_id: Path<i32>,
    ) -> GetUserResponse {
        if !claims.0.has_grants(&[Grants::UserGet]) {
            return GetUserResponse::Unauthorized;
        }

        get_user(repositories.0.clone(), user_id.0).await
    }

    #[oai(path = "/user/grants", method = "put", tag = ManageTags::User, tag = ManageTags::Grant)]
    async fn user_update_grant(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        payload: Json<ModifyGrantPayload>,
    ) -> ModifyGrantResponse {
        if !claims.0.has_grants(&[Grants::UserGrantUpdate]) {
            return ModifyGrantResponse::Unauthorized;
        }

        let agent = &format!(
            "user.modify_grant:{}:{}",
            claims.0.user_id, payload.0.grant_id
        );

        modify_grant(repositories.0.clone(), payload.0, agent).await
    }

    #[oai(path = "/application", method = "post", tag = ManageTags::Application)]
    async fn create_application(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        payload: Json<CreateApplicationPayload>,
    ) -> CreateApplicationResponse {
        if !claims.0.has_grants(&[Grants::ApplicationCreate]) {
            return CreateApplicationResponse::Unauthorized;
        }

        let agent = &format!("application.create:{}", claims.0.user_id);

        create_application(repositories.0.clone(), payload.0, &agent).await
    }

    #[oai(path = "/application/:application_id", method = "get", tag = ManageTags::Application)]
    async fn get_application(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        application_id: Path<String>,
    ) -> GetApplicationResponse {
        if !claims.0.has_grants(&[Grants::ApplicationGet]) {
            return GetApplicationResponse::Unauthorized;
        }

        get_application(repositories.0.clone(), application_id.0).await
    }

    #[oai(path = "/application", method = "get", tag = ManageTags::Application)]
    async fn list_applications(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
    ) -> ListApplicationsResponse {
        if !claims.0.has_grants(&[Grants::ApplicationList]) {
            return ListApplicationsResponse::Unauthorized;
        }

        list_applications(repositories.0.clone()).await
    }

    #[oai(path = "/application", method = "put", tag = ManageTags::Application)]
    async fn update_application(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        payload: Json<UpdateApplicationPayload>,
    ) -> UpdateApplicationResponse {
        if !claims.0.has_grants(&[Grants::ApplicationUpdate]) {
            return UpdateApplicationResponse::Unauthorized;
        }
        let agent = &format!("application.update:{}", claims.0.user_id);

        update_application(repositories.0.clone(), payload.0.clone(), &agent).await
    }

    #[oai(path = "/application/:application_id/grants", method = "get", tag = ManageTags::Application, tag = ManageTags::Grant)]
    async fn get_grants_by_application_id(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        application_id: Path<String>,
    ) -> GetGrantByApplicationIdResponse {
        if !claims.0.has_grants(&[Grants::ApplicationGetGrants]) {
            return GetGrantByApplicationIdResponse::Unauthorized;
        }

        get_grants_by_application_id(repositories.0.clone(), &application_id).await
    }

    #[oai(path = "/grant", method = "post", tag = ManageTags::Grant)]
    async fn create_grant(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        payload: Json<CreateGrantPayload>,
    ) -> CreateGrantResponse {
        if !claims.0.has_grants(&[Grants::GrantCreate]) {
            return CreateGrantResponse::Unauthorized;
        }

        let agent = &format!("grant.create:{}", claims.0.user_id);

        create_grant(repositories.0.clone(), payload.0, &agent).await
    }

    #[oai(path = "/grant/:grant_id", method = "get", tag = ManageTags::Grant)]
    async fn get_grant_by_id(
        &self,
        repositories: Data<&ApiRepositories>,
        claims: BearerJwt,
        grant_id: Path<String>,
    ) -> GetGrantByIdResponse {
        if !claims.0.has_grants(&[Grants::GrantGet]) {
            return GetGrantByIdResponse::Unauthorized;
        }

        get_grant_by_id(repositories.0.clone(), &grant_id).await
    }
}

#[derive(Clone)]
pub struct DebugApi;
impl SwaggerApi for DebugApi {
    fn base_uri() -> String {
        "/debug".into()
    }
    fn name() -> String {
        "Debug API".into()
    }
}
#[OpenApi]
impl DebugApi {
    #[oai(path = "/jwt", method = "post")]
    async fn create_jwt(&self, services: Data<&ApiServices>) -> poem::Result<PlainText<String>> {
        services
            .0
            .jwt
            .sign(&Claims {
                user_id: 1,
                issuer: "ME!".into(),
                grants: vec![
                    "dev.thmsn.auth.user.create".to_string(),
                    "dev.thmsn.auth.user.delete".to_string(),
                    "dev.thmsn.auth.user.list".to_string(),
                    "dev.thmsn.auth.user.get".to_string(),
                    "dev.thmsn.auth.user.grant.update".to_string(),
                    "dev.thmsn.auth.application.create".to_string(),
                    "dev.thmsn.auth.application.get".to_string(),
                    "dev.thmsn.auth.application.list".to_string(),
                    "dev.thmsn.auth.application.list".to_string(),
                    "dev.thmsn.auth.application.get_grants".to_string(),
                    "dev.thmsn.auth.grant.create".to_string(),
                    "dev.thmsn.auth.grant.get".to_string(),
                ],
                apps: vec![],
                issued_at: Utc::now().timestamp() as u64,
                expires: (Utc::now() + chrono::Duration::minutes(30)).timestamp() as u64,
            })
            .map_err(|e| poem::Error::new(e, StatusCode::INTERNAL_SERVER_ERROR))
            .map(|jwt| PlainText(jwt))
    }

    #[oai(path = "/jwt", method = "get")]
    async fn claims(&self, jwt: BearerJwt) -> Json<Claims> {
        Json(jwt.0)
    }
}
