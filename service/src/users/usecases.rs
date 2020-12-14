use super::{UserID, UserModel};
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
    async fn get_user(&self, user_id: &UserID) -> Option<UserModel>;
}
