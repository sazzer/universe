use super::AuthenticationService;
use crate::{
    authentication::{AuthenticateUserUseCase, AuthenticatedUser, AuthenticationError},
    users::Username,
};
use async_trait::async_trait;

#[async_trait]
impl AuthenticateUserUseCase for AuthenticationService {
    async fn authenticate_user(
        &self,
        username: &Username,
        password: &str,
    ) -> Result<AuthenticatedUser, AuthenticationError> {
        let user = self
            .users_service
            .get_user_by_username(username)
            .await
            .ok_or_else(|| AuthenticationError::UnknownUser)?;

        if user.data.password == password.as_ref() {
            todo!()
        } else {
            Err(AuthenticationError::InvalidPassword)
        }
    }
}
