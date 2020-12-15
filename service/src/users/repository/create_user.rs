use super::UsersRepository;
use crate::users::{CreateUserError, UserData, UserModel};

impl UsersRepository {
    /// Create a new user with the given data.
    ///
    /// # Parameters
    /// - `data` - The data for the new user
    ///
    /// # Returns
    /// The newly created user.
    /// If the user creation fails then rerturn an indication as to why.
    pub async fn create_user(&self, data: UserData) -> Result<UserModel, CreateUserError> {
        todo!()
    }
}
