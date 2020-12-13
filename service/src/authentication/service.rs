use super::ProviderID;
use std::{collections::HashMap, sync::Arc};

pub mod google;
mod list;

/// Trait that authentication providers implement.
pub trait AuthenticationProvider: Send + Sync {}

/// Service for dealing with Authentication.
pub struct AuthenticationService {
    providers: HashMap<ProviderID, Arc<dyn AuthenticationProvider>>,
}

impl AuthenticationService {
    /// Create a new instance of the Authentication service.
    pub fn new(providers: HashMap<ProviderID, Arc<dyn AuthenticationProvider>>) -> Self {
        Self { providers }
    }
}
