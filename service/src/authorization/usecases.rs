use super::{AccessToken, Principal, SecurityContext};

/// Use Case for generating an security context for a principal.
pub trait GenerateSecurityContextUseCase: Send + Sync {
    /// Generate the security context for the given principal.
    ///
    /// # Parameters
    /// - `principal` - The principal to generate the security context for
    ///
    /// # Returns
    /// The security context, both in real and serialized form.
    fn generate_security_context(&self, principal: Principal) -> (SecurityContext, AccessToken);
}

/// Errors from verifying an access token
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum VerifyAccessTokenError {
    /// The access token was malformed
    #[error("The access token was malformed")]
    Malformed,
}

/// Use Case for verifying and parsing an Access Token
pub trait VerifyAccessTokenUseCase {
    /// Verify the `AccessToken` is valid, and parse it back into a `SecurityContext`.
    ///
    /// # Parameters
    /// - `access_token` - The serialized access token to verify
    ///
    /// # Returns
    /// The verified security context.
    fn verify_access_token(
        &self,
        access_token: AccessToken,
    ) -> Result<SecurityContext, VerifyAccessTokenError>;
}
