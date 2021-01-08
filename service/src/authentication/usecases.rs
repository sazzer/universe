use super::AuthenticatedUser;
use crate::users::{UserData, Username};
use async_trait::async_trait;

/// Errors that can occur from authenticating an existing user.
#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    /// The username was unknown.
    #[error("The username was unknown")]
    UnknownUser,

    /// The password was invalid.
    #[error("The password was invalid")]
    InvalidPassword,
}

/// Use Case for authenticating a user.
#[async_trait]
pub trait AuthenticateUserUseCase {
    /// Authenticate the user with the provided details.
    ///
    /// # Parameters
    /// - `username` - The username to authenticate
    /// - `password` - The password to authenticate
    ///
    /// - # Returns
    /// The authenticated user details.
    async fn authenticate_user(
        &self,
        username: &Username,
        password: &str,
    ) -> Result<AuthenticatedUser, AuthenticationError>;
}

/// Errors that can occur from registering a new user.
#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    /// The username is already registered.
    #[error("The username is already registered")]
    DuplicateUsername,

    /// The email address is already registered.
    #[error("The email address is already registered")]
    DuplicateEmail,
}

#[async_trait]
pub trait RegisterUserUseCase {
    /// Register a new user with the provided details.
    ///
    /// # Parameters
    /// - `user_data` - The data for the user to register.
    ///
    /// - # Returns
    /// The authenticated user details.
    async fn register_user(&self, user_data: UserData) -> Result<AuthenticatedUser, RegisterError>;
}
