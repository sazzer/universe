use super::{UserID, UserModel};
use async_trait::async_trait;

/// Use Case to get a single user by their unique ID.
#[async_trait]
pub trait GetUserUseCase {
    /// Get the user that has the given User Id.
    async fn get_user(&self, user_id: UserID) -> Option<UserModel>;
}
