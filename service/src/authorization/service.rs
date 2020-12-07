mod claims;
mod generate;
mod signing_key;

use chrono::Duration;
pub use signing_key::*;

/// Service for dealing with Authorization.
pub struct AuthorizationService {
    signing_key: SigningKey,
    duration: Duration,
}

impl AuthorizationService {
    /// Create a new instance of the Authorization service.
    pub const fn new(signing_key: SigningKey, duration: Duration) -> Self {
        Self {
            signing_key,
            duration,
        }
    }
}
