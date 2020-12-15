use super::{UserID, UserModel, Username};
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
