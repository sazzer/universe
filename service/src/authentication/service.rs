use crate::{authorization::GenerateSecurityContextUseCase, users::GetUserUseCase};
use std::sync::Arc;

mod authenticate;

/// Service implementation for authenticating users.
pub struct AuthenticationService {
    users_service: Arc<dyn GetUserUseCase>,
    authorization_service: Arc<dyn GenerateSecurityContextUseCase>,
}

impl AuthenticationService {
    /// Create a new instance of the authentication service.
    pub fn new(
        users_service: Arc<dyn GetUserUseCase>,
        authorization_service: Arc<dyn GenerateSecurityContextUseCase>,
    ) -> Self {
        Self {
            users_service,
            authorization_service,
        }
    }
}
