use super::AuthenticationService;
use crate::{
    authentication::{AuthenticatedUser, RegisterError, RegisterUserUseCase},
    users::{CreateUserError, UserData},
};
use async_trait::async_trait;

#[async_trait]
impl RegisterUserUseCase for AuthenticationService {
    async fn register_user(&self, user_data: UserData) -> Result<AuthenticatedUser, RegisterError> {
        let user = self
            .create_users_service
            .create_user(user_data)
            .await
            .map_err(|e| match e {
                CreateUserError::DuplicateEmail => RegisterError::DuplicateEmail,
                CreateUserError::DuplicateUsername => RegisterError::DuplicateUsername,
            })?;

        let (security_context, access_token) = self
            .authorization_service
            .generate_security_context(user.identity.id.clone().into());
        Ok(AuthenticatedUser {
            user,
            security_context,
            access_token,
        })
    }
}
