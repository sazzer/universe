use super::UsersRepository;
use crate::users::{UserID, UserModel};

impl UsersRepository {
    pub async fn get_user(&self, user_id: UserID) -> Option<UserModel> {
        None
    }
}
