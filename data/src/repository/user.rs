use crate::{
    dto::user_grant::{UserGrantDetailDto, UserGrantDto},
    util::IntoActiveValueExt,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet, Set},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    sea_query::OnConflict,
    sqlx::types::chrono::Utc,
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
        user::{UserDetailDto, UserDto},
    },
    model,
    repository::error::RepositoryError,
};

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum UserError {
    #[error(transparent)]
    Database { inner_error: RepositoryError },
    #[error(transparent)]
    Dto {
        #[from]
        inner_error: DtoError,
    },
    #[error("The user was not created, an unknown error occurred")]
    NotCreated,
    #[error("No user was found with user_id={user_id}")]
    UserNotFound { user_id: i32 },
    #[error("Called update with no changes")]
    NoChangeRequested,
}
impl<E: Into<RepositoryError>> From<E> for UserError {
    fn from(value: E) -> Self {
        Self::Database {
            inner_error: value.into(),
        }
    }
}
pub type UserResult<T> = Result<T, UserError>;

#[derive(Clone, Debug)]
pub struct UserRepository {
    conn: DatabaseConnection,
}
impl UserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.populate_user")]
    async fn populate_user(&self, user: UserDto) -> UserResult<UserDetailDto> {
        let them: Vec<_> = crate::model::user_grant::Entity::find()
            .find_also_related(model::grant::Entity)
            .and_also_related(model::application::Entity)
            .filter(model::user_grant::Column::UserId.eq(user.user_id))
            .all(&self.conn)
            .await?;

        let mut grants = Vec::with_capacity(them.len());

        for row in them {
            let user_grant = row.0;
            let Some((grant, application)) = row.1.zip(row.2) else {
                continue;
            };

            let user_grant = UserGrantDto::try_from(user_grant)?;
            let grant = GrantDto::try_from(grant)?;
            let application = ApplicationDto::try_from(application)?;
            grants.push(UserGrantDetailDto {
                user_grant,
                grant: GrantDetailDto { grant, application },
            })
        }

        Ok(UserDetailDto { user, grants })
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.by_id")]
    pub async fn by_id(&self, user_id: i32) -> UserResult<Option<UserDetailDto>> {
        let Some(user) = crate::model::user::Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
        else {
            return Ok(None);
        };

        let user = UserDto::try_from(user)?;

        Ok(Some(self.populate_user(user).await?))
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.by_username")]
    pub async fn by_username(&self, username: &str) -> UserResult<Option<UserDetailDto>> {
        let Some(user) = crate::model::user::Entity::find_by_username(username)
            .one(&self.conn)
            .await?
        else {
            return Ok(None);
        };

        let user = UserDto::try_from(user)?;

        Ok(Some(self.populate_user(user).await?))
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.list")]
    pub async fn list(&self, enabled: Option<bool>) -> UserResult<Vec<UserDto>> {
        // TODO: Do I care about pagination?

        let them = model::user::Entity::load()
            .filter(Condition::all().add_option(
                enabled.map(|enabled| model::user::Column::Enabled.eq(if enabled { 1 } else { 0 })),
            ))
            .all(&self.conn)
            .await?;

        Ok(them
            .into_iter()
            .map(UserDto::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.create")]
    pub async fn create(
        &self,
        agent: &str,
        username: &str,
        password: &str,
        display_name: Option<&str>,
        email: Option<&str>,
        image_url: Option<&str>,
    ) -> UserResult<UserDetailDto> {
        let it = model::user::Entity::insert(model::user::ActiveModel {
            display_name: display_name.into_active_value_ext(),
            email: email.into_active_value_opt_ext(),
            image_url: image_url.into_active_value_opt_ext(),
            username: ActiveValue::Set(username.into()),
            password: ActiveValue::Set(password.into()),
            created_by: ActiveValue::Set(agent.into()),
            updated_by: ActiveValue::Set(agent.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        })
        .exec(&self.conn)
        .await?;

        self.by_id(it.last_insert_id)
            .await?
            .ok_or(UserError::UserNotFound {
                user_id: it.last_insert_id,
            })
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.update")]
    pub async fn update(
        &self,
        agent: &str,
        user_id: i32,
        enabled: Option<bool>,
        display_name: Option<&str>,
        password: Option<&str>,
        email: Option<&str>,
        image_url: Option<&str>,
    ) -> UserResult<UserDetailDto> {
        let has_changes = {
            enabled.is_some()
                || display_name.is_some()
                || password.is_some()
                || email.is_some()
                || image_url.is_some()
        };

        if !has_changes {
            return Err(UserError::NoChangeRequested);
        }

        let mut user: model::user::ActiveModel = model::user::Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
            .ok_or(UserError::UserNotFound { user_id: user_id })?
            .into_active_model();

        user.enabled = enabled.into_active_value_ext();
        user.display_name = display_name.into_active_value_ext();
        user.password = password.into_active_value_ext();
        user.email = email.into_active_value_opt_ext();
        user.image_url = image_url.into_active_value_opt_ext();
        user.updated_by = ActiveValue::Set(agent.into());
        user.updated_at = ActiveValue::Set(Utc::now().naive_utc());

        user.update(&self.conn).await?;

        self.by_id(user_id)
            .await?
            .ok_or(UserError::UserNotFound { user_id: user_id })
    }

    // TODO: This is like 4 queries, kinda stupid
    #[tracing::instrument(level = Level::DEBUG, "data.user.set_last_login")]
    pub async fn set_last_login(&self, user_id: i32) -> UserResult<UserDetailDto> {
        let mut model = model::user::Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
            .ok_or(UserError::UserNotFound { user_id })?
            .into_active_model();

        model.last_login = Set(Some(Utc::now().naive_utc()));
        model.updated_at = Set(Utc::now().naive_utc());
        model.updated_by = Set(format!("user.set_last_login:{user_id}"));

        model.update(&self.conn).await?;

        self.by_id(user_id)
            .await?
            .ok_or(UserError::UserNotFound { user_id })
    }

    #[tracing::instrument(level=Level::DEBUG, "data.user.delete")]
    pub async fn delete(&self, user_id: i32) -> UserResult<()> {
        let user: model::user::ActiveModel = model::user::Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
            .ok_or(UserError::UserNotFound { user_id: user_id })?
            .into_active_model();

        user.delete(&self.conn).await?;

        Ok(())
    }

    pub async fn update_grant(
        &self,
        agent: &str,
        user_id: i32,
        grant_id: &str,
        enabled: bool,
    ) -> UserResult<()> {
        let model = model::user_grant::Entity::find_by_id((user_id, grant_id.into()))
            .one(&self.conn)
            .await?;

        let mut user = model::user::Entity::find_by_id(user_id)
            .one(&self.conn)
            .await?
            .ok_or(UserError::UserNotFound { user_id })?
            .into_active_model();

        let mut model = match model {
            Some(model) => model.into_active_model(),
            None => model::user_grant::ActiveModel {
                user_id: Set(user_id),
                grant_id: Set(grant_id.into()),
                created_by: Set(agent.into()),
                created_at: Set(Utc::now().naive_utc()),
                enabled_at: NotSet,
                disabled_at: NotSet,
                enabled: NotSet,
                updated_by: NotSet,
                updated_at: NotSet,
            },
        };

        model.enabled = Set(enabled.into());
        model.updated_by = Set(agent.into());
        model.updated_at = Set(Utc::now().naive_utc());

        if enabled {
            model.enabled_at = Set(Some(Utc::now().naive_utc()));
            model.disabled_at = Set(None);
        } else {
            model.disabled_at = Set(Some(Utc::now().naive_utc()));
            model.enabled_at = Set(None);
        }

        let on_conflict = OnConflict::columns([
            model::user_grant::Column::UserId,
            model::user_grant::Column::GrantId,
        ])
        .update_columns([
            model::user_grant::Column::Enabled,
            model::user_grant::Column::UpdatedBy,
            model::user_grant::Column::UpdatedAt,
            model::user_grant::Column::EnabledAt,
            model::user_grant::Column::DisabledAt,
        ])
        .to_owned();

        crate::model::user_grant::Entity::insert(model)
            .on_conflict(on_conflict)
            .exec(&self.conn)
            .await?;

        user.updated_by = Set(agent.into());
        user.updated_at = Set(Utc::now().naive_utc());

        user.update(&self.conn).await?;

        Ok(())
    }
}
