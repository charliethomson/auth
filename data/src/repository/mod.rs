use sea_orm::{Database, DatabaseConnection};

use crate::repository::error::RepositoryError;

pub mod application;
pub mod error;
pub mod grant;
pub mod user;

pub async fn connect(connection_string: &str) -> Result<DatabaseConnection, RepositoryError> {
    Database::connect(connection_string)
        .await
        .map_err(RepositoryError::from)
}
