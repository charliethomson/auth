use liberror::AnyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valuable::Valuable;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Valuable)]
pub enum RepositoryError {
    #[error("Failed to acquire a database connection: {inner_error}")]
    ConnectionAcquire { inner_error: AnyError },
    #[error("Failed to convert '{from}' to '{to}': {inner_error}")]
    TryInto {
        from: String,
        to: String,
        inner_error: AnyError,
    },

    #[error("Connection error: {inner_error}")]
    Connection { inner_error: AnyError },
    #[error("Execution error: {inner_error}")]
    Execute { inner_error: AnyError },
    #[error("Query error: {inner_error}")]
    Query { inner_error: AnyError },
    #[error("Type {type} cannot be converted from u64")]
    ConvertFromU64 { r#type: String },
    #[error("Failed to unpack last_insert_id")]
    UnpackInsertId,
    #[error("Failed to get primary key from model")]
    UpdateGetPrimaryKey,
    #[error("Not found: {context}")]
    RecordNotFound { context: String },
    #[error("Attribute {attribute} is not set")]
    AttributeNotSet { attribute: String },
    #[error("{context}")]
    Custom { context: String },
    #[error("Type error: {context}")]
    Type { context: String },
    #[error("JSON error: {context}")]
    Json { context: String },
    #[error("Migration error: {context}")]
    Migration { context: String },
    #[error("None of the records are inserted")]
    RecordNotInserted,
    #[error("None of the records are updated")]
    RecordNotUpdated,
    #[error("Operation not supported by backend {backend}: {context}")]
    BackendNotSupported { backend: String, context: String },
    #[error("Key arity mismatch: expected {expected}, received {received}")]
    KeyArityMismatch { expected: u8, received: u8 },
    #[error("Primay key not set for {context}")]
    PrimaryKeyNotSet { context: String },
    #[error("RBAC error: {context}")]
    Rbac { context: String },
    #[error("Access denied: cannot perform `{permission}` on `{resource}`")]
    AccessDenied {
        permission: String,
        resource: String,
    },
}
impl From<sea_orm::DbErr> for RepositoryError {
    fn from(value: sea_orm::DbErr) -> Self {
        match value {
            sea_orm::DbErr::ConnectionAcquire(e) => Self::ConnectionAcquire {
                inner_error: e.into(),
            },
            sea_orm::DbErr::TryIntoErr { from, into, source } => Self::TryInto {
                from: from.into(),
                to: into.into(),
                inner_error: source.into(),
            },
            sea_orm::DbErr::Conn(e) => Self::Connection {
                inner_error: e.into(),
            },
            sea_orm::DbErr::Exec(e) => Self::Execute {
                inner_error: e.into(),
            },
            sea_orm::DbErr::Query(e) => Self::Query {
                inner_error: e.into(),
            },
            sea_orm::DbErr::ConvertFromU64(r#type) => Self::ConvertFromU64 {
                r#type: r#type.into(),
            },
            sea_orm::DbErr::UnpackInsertId => Self::UnpackInsertId,
            sea_orm::DbErr::UpdateGetPrimaryKey => Self::UpdateGetPrimaryKey,
            sea_orm::DbErr::RecordNotFound(context) => Self::RecordNotFound { context },
            sea_orm::DbErr::AttrNotSet(context) => Self::AttributeNotSet { attribute: context },
            sea_orm::DbErr::Custom(context) => Self::Custom { context },
            sea_orm::DbErr::Type(context) => Self::Type { context },
            sea_orm::DbErr::Json(context) => Self::Json { context },
            sea_orm::DbErr::Migration(context) => Self::Migration { context },
            sea_orm::DbErr::RecordNotInserted => Self::RecordNotInserted,
            sea_orm::DbErr::RecordNotUpdated => Self::RecordNotUpdated,
            sea_orm::DbErr::BackendNotSupported { db, ctx } => Self::BackendNotSupported {
                backend: db.to_string(),
                context: ctx.to_string(),
            },
            sea_orm::DbErr::KeyArityMismatch { expected, received } => {
                Self::KeyArityMismatch { expected, received }
            }
            sea_orm::DbErr::PrimaryKeyNotSet { ctx } => Self::PrimaryKeyNotSet {
                context: ctx.to_string(),
            },
            sea_orm::DbErr::RbacError(context) => Self::Rbac { context },
            sea_orm::DbErr::AccessDenied {
                permission,
                resource,
            } => Self::AccessDenied {
                permission,
                resource,
            },
        }
    }
}
