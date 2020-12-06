use super::{Email, ProviderID, ProviderUserID, UserID, UserModel, Username};
use async_trait::async_trait;

/// Use Case to get a single user by their unique ID.
#[async_trait]
pub trait GetUserUseCase {
    /// Get the user that has the given User Id.
    async fn get_user(&self, user_id: &UserID) -> Option<UserModel>;
}

/// Input to authenticate a user.
pub struct UserAuthentication {
    pub provider: ProviderID,
    pub user_id: ProviderUserID,
    pub authentication_display_name: String,
    pub user_display_name: String,
    pub email: Option<Email>,
    pub username: Option<Username>,
}

/// Use Case for authenticating a user.
#[async_trait]
pub trait AuthenticateUserUseCase {
    /// Authenticate the user with the given details.
    /// This will either register a new user or retrieve the existing user that matches the details.
    async fn authenticate(&self, authentication: UserAuthentication) -> UserModel;
}
