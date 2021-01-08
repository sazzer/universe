use crate::{
    authorization::GenerateSecurityContextUseCase,
    users::{CreateUserUseCase, GetUserUseCase},
};
use std::sync::Arc;

mod authenticate;
mod register;

/// Service implementation for authenticating users.
pub struct AuthenticationService {
    get_users_service: Arc<dyn GetUserUseCase>,
    create_users_service: Arc<dyn CreateUserUseCase>,
    authorization_service: Arc<dyn GenerateSecurityContextUseCase>,
}

impl AuthenticationService {
    /// Create a new instance of the authentication service.
    pub fn new(
        get_users_service: Arc<dyn GetUserUseCase>,
        create_users_service: Arc<dyn CreateUserUseCase>,
        authorization_service: Arc<dyn GenerateSecurityContextUseCase>,
    ) -> Self {
        Self {
            get_users_service,
            create_users_service,
            authorization_service,
        }
    }
}
