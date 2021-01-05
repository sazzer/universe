use crate::users::Username;

use super::AuthenticatedUser;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("The username was unknown")]
    UnknownUser,

    #[error("The password was invalid")]
    InvalidPassword,
}

/// Use Case for authenticating a user.
pub trait AuthenticateUserUseCase {
    /// Authenticate the user with the provided details.
    ///
    /// # Parameters
    /// - `username` - The username to authenticate
    /// - `password` - The password to authenticate
    ///
    /// - # Returns
    /// The authenticated user details.
    fn authenticate_user(
        &self,
        username: Username,
        password: String,
    ) -> Result<AuthenticatedUser, AuthenticationError>;
}
