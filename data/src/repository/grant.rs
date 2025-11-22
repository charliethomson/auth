use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
    TransactionTrait, sqlx::types::chrono::Utc,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::Level;
use valuable::Valuable;

use crate::{
    dto::{
        application::ApplicationDto,
        error::DtoError,
        grant::{GrantDetailDto, GrantDto},
    },
    model,
    repository::error::RepositoryError,
};

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum GrantError {
    #[error(transparent)]
    Database { inner_error: RepositoryError },
    #[error(transparent)]
    Dto {
        #[from]
        inner_error: DtoError,
    },
    #[error("No application was found with id={application_id}")]
    ApplicationNotFound { application_id: String },
    #[error("No grant was found with id={grant_id}")]
    GrantNotFound { grant_id: String },
}
impl<E: Into<RepositoryError>> From<E> for GrantError {
    fn from(value: E) -> Self {
        Self::Database {
            inner_error: value.into(),
        }
    }
}
pub type GrantResult<T> = Result<T, GrantError>;

#[derive(Clone, Debug)]
pub struct GrantRepository {
    conn: DatabaseConnection,
}
impl GrantRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    #[tracing::instrument(level = Level::DEBUG, "data.grant.by_application")]
    pub async fn by_application(&self, application_id: &str) -> GrantResult<Vec<GrantDto>> {
        let them = model::application::Entity::find_by_id(application_id)
            .find_with_related(model::prelude::Grant)
            .all(&self.conn)
            .await?
            .into_iter()
            .next()
            .ok_or(GrantError::ApplicationNotFound {
                application_id: application_id.to_string(),
            });

        let mut grants = vec![];
        for (app, app_grants) in them.into_iter() {
            if app.application_id != application_id {
                unreachable!("rx unexpected application with id={}", app.application_id);
            }

            for grant in app_grants.into_iter() {
                grants.push(GrantDto::try_from(grant)?);
            }
        }

        Ok(grants)
    }

    #[tracing::instrument(level = Level::DEBUG, "data.grant.by_id")]
    pub async fn by_id(&self, grant_id: &str) -> GrantResult<Option<GrantDetailDto>> {
        let it = model::grant::Entity::find_by_id(grant_id)
            .find_also_related(model::prelude::Application)
            .one(&self.conn)
            .await?;

        let Some((grant, application)) = it else {
            tracing::info!("Grant not found");
            return Ok(None);
        };

        let Some(application) = application else {
            // TODO: probably dont panic but this should be genuinely unreachable
            unreachable!("Grant exists, but its application doesn't");
        };

        let grant = GrantDto::try_from(grant)?;
        let application = ApplicationDto::try_from(application)?;

        Ok(Some(GrantDetailDto { grant, application }))
    }

    #[tracing::instrument(level = Level::DEBUG, "data.grant.create")]
    pub async fn create(
        &self,
        agent: &str,
        grant_id: &str,
        application_id: &str,
        display_name: &str,
        description: &str,
    ) -> GrantResult<GrantDetailDto> {
        let txn = self.conn.begin().await?;

        let mut app = model::application::Entity::find_by_id(application_id)
            .one(&txn)
            .await?
            .ok_or(GrantError::ApplicationNotFound {
                application_id: application_id.to_string(),
            })?
            .into_active_model();

        let it = model::grant::Entity::insert(model::grant::ActiveModel {
            grant_id: Set(grant_id.into()),
            application_id: Set(application_id.into()),
            display_name: Set(display_name.into()),
            description: Set(description.into()),
            created_by: Set(agent.into()),
            updated_by: Set(agent.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        })
        .exec(&txn)
        .await?;

        let grant_id = it.last_insert_id;

        app.updated_by = Set(agent.into());
        app.updated_at = Set(Utc::now().naive_utc());

        app.update(&txn).await?;

        txn.commit().await?;

        self.by_id(&grant_id)
            .await?
            .ok_or(GrantError::GrantNotFound { grant_id })
    }
}
