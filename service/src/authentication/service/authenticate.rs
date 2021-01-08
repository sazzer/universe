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
            .get_users_service
            .get_user_by_username(username)
            .await
            .ok_or(AuthenticationError::UnknownUser)?;

        if user.data.password == password {
            let (security_context, access_token) = self
                .authorization_service
                .generate_security_context(user.identity.id.clone().into());
            Ok(AuthenticatedUser {
                user,
                security_context,
                access_token,
            })
        } else {
            Err(AuthenticationError::InvalidPassword)
        }
    }
}
