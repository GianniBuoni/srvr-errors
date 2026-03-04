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

#[cfg(feature = "tonic")]
impl From<ValConfigError> for Status {
    fn from(value: ValConfigError) -> Self {
        Status::internal(value.to_string())
    }
}

#[derive(Error, Debug)]
/// Handles a number of issues with client side validations
pub enum ClientError {
    /// For when client sends an empty request.
    /// Error returns the failed task.
    /// ```
    /// use srvr_errors::prelude::*;
    ///
    /// let task = "user_create";
    ///
    /// let err = ClientError::EmptyArgs(task.into());
    /// ```
    #[error("Arguments must be passed for this service: {0}")]
    EmptyArgs(Arc<str>),
    /// For when client sends arguments that need to already exist
    /// in the database, but some do not exist.
    /// Error returns the table name and the offending values.
    /// ```
    /// use srvr_errors::prelude::*;
    ///
    /// let table = "users";
    /// let not_found = "john, paul, ringo, george";
    ///
    /// let err = ClientError::EntryNotFound(table.into(), not_found.into());
    /// ```
    #[error("Cannot edit or delete non existant entries: Value(s): '{0}'")]
    EntryNotFound(Arc<str>, Arc<str>),
    /// For when client sends arguments that have arguments that repeat.
    /// Sending non-unique values to the db could create issues with
    /// unique constraints.
    /// Error returns the offending values.
    /// ```
    /// use srvr_errors::prelude::*;
    ///
    /// let args = "john, john, john";
    /// let repeated_values = "john";
    ///
    /// let err = ClientError::RepeatArgs(repeated_values.into());
    /// ```
    #[error("Arugments to server must all be unique: Value(s) '{0}'")]
    RepeatArgs(String),
    /// For when client sends arguments that would violate a table's
    /// unique constraint.
    /// Error returns the table name and the offending values.
    /// ```
    /// use srvr_errors::prelude::*;
    ///
    /// let table = "users";
    /// let non_unique_value = "john";
    ///
    /// let err = ClientError::UniqueConstraint(
    ///     table.into(), non_unique_value.into(),
    /// );
    /// ```
    #[error("Arguments to server are already in Table {0}, Value(s): '{1}'")]
    UniqueConstraint(Arc<str>, Arc<str>),
    /// For when client sends arguments that aren't valid uuids.
    /// Error returns the offending values.
    /// ```
    /// use srvr_errors::prelude::*;
    ///
    /// let bad_uuid_value = "john";
    ///
    /// let err = ClientError::Uuid(bad_uuid_value.into());
    /// ```
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
