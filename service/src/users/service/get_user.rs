use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserModel};
use async_trait::async_trait;

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user(&self, user_id: UserID) -> Option<UserModel> {
        self.repository.get_user(user_id).await
    }
}
