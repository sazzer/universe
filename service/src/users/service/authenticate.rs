use super::UsersService;
use crate::users::{AuthenticateUserUseCase, UserAuthentication, UserModel};
use async_trait::async_trait;

#[async_trait]
impl AuthenticateUserUseCase for UsersService {
    async fn authenticate(&self, authentication: UserAuthentication) -> UserModel {
        todo!()
    }
}
