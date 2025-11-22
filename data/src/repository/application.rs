use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
    sqlx::types::chrono::Utc,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

use crate::{
    dto::{
        application::{ApplicationDetailDto, ApplicationDto},
        error::DtoError,
        grant::GrantDto,
    },
    model,
    repository::error::RepositoryError,
    util::IntoActiveValueExt,
};

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum ApplicationError {
    #[error(transparent)]
    Database { inner_error: RepositoryError },
    #[error(transparent)]
    Dto {
        #[from]
        inner_error: DtoError,
    },
    #[error("No application was found with id={application_id}")]
    ApplicationNotFound { application_id: String },
    #[error("Called update with no changes")]
    NoChangeRequested,
}
impl<E: Into<RepositoryError>> From<E> for ApplicationError {
    fn from(value: E) -> Self {
        Self::Database {
            inner_error: value.into(),
        }
    }
}
pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Clone, Debug)]
pub struct ApplicationRepository {
    conn: DatabaseConnection,
}
impl ApplicationRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn by_id(
        &self,
        application_id: &str,
    ) -> ApplicationResult<Option<ApplicationDetailDto>> {
        let Some((application, grants)) = model::application::Entity::find_by_id(application_id)
            .find_with_related(model::grant::Entity)
            .all(&self.conn)
            .await?
            .into_iter()
            .next()
        else {
            return Ok(None);
        };

        let application = ApplicationDto::try_from(application)?;
        let grants = grants
            .into_iter()
            .map(|grant| GrantDto::try_from(grant))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some(ApplicationDetailDto {
            application,
            grants,
        }))
    }

    pub async fn list(&self) -> ApplicationResult<Vec<ApplicationDto>> {
        Ok(model::application::Entity::load()
            .all(&self.conn)
            .await?
            .into_iter()
            .map(ApplicationDto::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub async fn create(
        &self,
        agent: &str,
        id: &str,
        display_name: &str,
        description: &str,
    ) -> ApplicationResult<ApplicationDetailDto> {
        model::application::Entity::insert(model::application::ActiveModel {
            application_id: Set(id.into()),
            display_name: Set(display_name.into()),
            description: Set(description.into()),
            created_by: Set(agent.into()),
            updated_by: Set(agent.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        })
        .exec(&self.conn)
        .await?;

        self.by_id(id)
            .await?
            .ok_or(ApplicationError::ApplicationNotFound {
                application_id: id.into(),
            })
    }

    pub async fn update(
        &self,
        agent: &str,
        application_id: &str,
        display_name: Option<&str>,
        description: Option<&str>,
    ) -> ApplicationResult<ApplicationDetailDto> {
        let has_changes = { description.is_some() || display_name.is_some() };

        if !has_changes {
            return Err(ApplicationError::NoChangeRequested);
        }

        let mut app = model::application::Entity::find_by_id(application_id.to_string())
            .one(&self.conn)
            .await?
            .ok_or(ApplicationError::ApplicationNotFound {
                application_id: application_id.into(),
            })?
            .into_active_model();

        app.display_name = display_name.into_active_value_ext();
        app.description = description.into_active_value_ext();
        app.updated_at = Set(Utc::now().naive_utc());
        app.updated_by = Set(agent.into());

        app.update(&self.conn).await?;

        self.by_id(&application_id)
            .await?
            .ok_or(ApplicationError::ApplicationNotFound {
                application_id: application_id.into(),
            })
    }
}
