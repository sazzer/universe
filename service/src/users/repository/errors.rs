use std::borrow::ToOwned;
use tokio_postgres::error::{DbError, SqlState};

use crate::users::SaveUserError;

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
                        Self::UnknownError
                    }
                })
                .unwrap_or(Self::UnknownError)
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
            Self::UnknownError
        }
    }
}
