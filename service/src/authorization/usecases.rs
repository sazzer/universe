use super::{AccessToken, Principal, SerializedAccessToken};

/// Use Case for generating an access token for a principal.
pub trait GenerateAccessTokenUseCase {
    /// Generate the access token for the given principal.
    /// This returns the access token both as a raw and serialized form.
    fn generate_access_token(&self, principal: Principal) -> (AccessToken, SerializedAccessToken);
}
