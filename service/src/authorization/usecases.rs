use super::{AccessToken, Principal, SecurityContext};

/// Use Case for generating an security context for a principal.
pub trait GenerateSecurityContextUseCase {
    /// Generate the security context for the given principal.
    /// This returns the security context both as a raw and serialized form.
    fn generate_security_context(&self, principal: Principal) -> (SecurityContext, AccessToken);
}

/// Use Case for verifying and parsing an Access Token
pub trait VerifyAccessTokenUseCase {
    /// Verify the `AccessToken` is valid, and parse it back into a `SecurityContext`.
    fn verify_access_token(&self, access_token: AccessToken) -> Result<SecurityContext, ()>;
}
