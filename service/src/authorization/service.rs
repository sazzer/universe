mod generate;
mod signing_key;

use chrono::Duration;
pub use signing_key::*;

/// Service for dealing with Authorization.
pub struct AuthorizationService {
    /// The key to sign the security context with
    signing_key: SigningKey,
    /// The duration of generated security contexts
    duration: Duration,
}

impl AuthorizationService {
    /// Create a new instance of the Authorization service.
    ///
    /// # Parameters
    /// - `signing_key` - The key to sign the security context with
    /// - `duration` - The duration of generated security contexts
    pub const fn new(signing_key: SigningKey, duration: Duration) -> Self {
        Self {
            signing_key,
            duration,
        }
    }
}
