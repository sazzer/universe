use super::{UserData, UserID, UserModel, Username};
use async_trait::async_trait;

/// Use Case to get a single user by their unique ID.
#[async_trait]
pub trait GetUserUseCase {
    /// Get the user that has the given User Id.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the User to retrieve.
    ///
    /// # Returns
    /// The user with the given ID. If no user was found then returns `None`.
    async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel>;

    /// Get the user that has the given Username.
    ///
    /// # Parameters
    /// - `username` - The username of the User to retrieve.
    ///
    /// # Returns
    /// The user with the given username. If no user was found then returns `None`.
    async fn get_user_by_username(&self, username: &Username) -> Option<UserModel>;
}

/// Errors that can exist from creating a new user.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CreateUserError {
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

/// Use Case to create new users.
#[async_trait]
pub trait CreateUserUseCase {
    /// Create a new user with the given data.
    ///
    /// # Parameters
    /// - `data` - The data for the new user
    ///
    /// # Returns
    /// The newly created user.
    /// If the user creation fails then rerturn an indication as to why.
    async fn create_user(&self, data: UserData) -> Result<UserModel, CreateUserError>;
}
