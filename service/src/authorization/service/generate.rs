use super::AuthorizationService;
use crate::authorization::{
    AccessToken, GenerateSecurityContextUseCase, Principal, SecurityContext,
};

impl GenerateSecurityContextUseCase for AuthorizationService {
    fn generate_security_context(&self, principal: Principal) -> (SecurityContext, AccessToken) {
        todo!()
    }
}
