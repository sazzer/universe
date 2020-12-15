use std::borrow::ToOwned;
use tokio_postgres::error::{DbError, SqlState};

/// Possible errors that can occur when saving a user record in the database - either as a create or update.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SaveUserError {
    /// The username is already registered
    #[error("The username is already registered")]
    DuplicateUsername,

    /// The email address is already registered
    #[error("The email address is already registered")]
    DuplicateEmail,

    /// An unexpected error occured
    #[error("An unexpected error occurred")]
    UnexpectedError,
}

impl From<tokio_postgres::Error> for SaveUserError {
    /// Convert a database error into a `SaveUserError`.
    ///
    /// # Parameters
    /// - `e` - The error to convert
    ///
    /// # Returns
    /// The new error code
    fn from(e: tokio_postgres::Error) -> Self {
        if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
            e.into_source()
                .and_then(|e| e.downcast_ref::<DbError>().cloned())
                .and_then(|e| e.constraint().map(ToOwned::to_owned))
                .map(|constraint| match constraint.as_str() {
                    "users_email_key" => Self::DuplicateEmail,
                    "users_username_key" => Self::DuplicateUsername,
                    _ => {
                        tracing::warn!("Unexpected constraint violation error: {:?}", constraint);
                        Self::UnexpectedError
                    }
                })
                .unwrap_or(Self::UnexpectedError)
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
            Self::UnexpectedError
        }
    }
}
