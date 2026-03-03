use std::sync::Arc;

use thiserror::Error;
#[cfg(feature = "tonic")]
use tonic::Status;

pub mod prelude {
    #[cfg(test)]
    pub use super::EXPECTED_ERROR;
    pub use super::{ClientError, DbError, ValConfigError};
}

#[cfg(test)]
pub const EXPECTED_ERROR: &str = "Test expected error but returned:";

#[derive(Error, Debug)]
/// Handles issues with validation builder errors
pub enum ValConfigError {
    #[error("Argument validation configuration invalid: Field '{0}' missing")]
    Arguments(&'static str),
}

#[derive(Error, Debug)]
/// Handles a number of issues with client side validations
pub enum ClientError {
    #[error("Arguments must be passed for this service: {0}")]
    EmptyArgs(Arc<str>),
    #[error("Cannot edit or delete non existant entries: Value(s): '{0}'")]
    EntryNotFound(Arc<str>, Arc<str>),
    #[error("Arugments to server must all be unique: Value(s) '{0}'")]
    RepeatArgs(String),
    #[error("Arguments to server are already in the database: Value(s): '{0}'")]
    UniqueConstraint(Arc<str>),
    #[error("Cannot parse as uuid: Value(s) '{0}'")]
    Uuid(String),
}

#[cfg(feature = "tonic")]
impl From<ClientError> for Status {
    fn from(value: ClientError) -> Self {
        Status::invalid_argument(value.to_string())
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Couldn't establish a connection to {0}.")]
    Connection(Arc<str>),
    #[error("Database took too long to perform {0} task.")]
    Context(&'static str),
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
}

#[cfg(feature = "tonic")]
impl From<DbError> for Status {
    fn from(value: DbError) -> Self {
        match value {
            DbError::Context(_) => Status::deadline_exceeded(value.to_string()),
            DbError::Connection(_) => Status::unavailable(value.to_string()),
            _ => Status::internal(value.to_string()),
        }
    }
}
