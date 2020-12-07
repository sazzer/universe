use super::{AccessToken, Principal, SecurityContext};

/// Use Case for generating an security context for a principal.
pub trait GenerateSecurityContextUseCase {
    /// Generate the security context for the given principal.
    /// This returns the security context both as a raw and serialized form.
    fn generate_security_context(&self, principal: Principal) -> (SecurityContext, AccessToken);
}
