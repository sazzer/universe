use super::{UserID, UserModel};

/// Use Case to get a single user by their unique ID.
pub trait GetUserUseCase {
    /// Get the user that has the given User Id.
    fn get_user(&self, user_id: UserID) -> Option<UserModel>;
}
