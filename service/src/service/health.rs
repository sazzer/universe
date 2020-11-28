use crate::health::service::HealthService;
use std::sync::Arc;

/// Representation of the Health component in the service
pub struct HealthComponent {
    healthservice: Arc<HealthService>,
}

impl HealthComponent {
    pub fn new() -> Self {
        let components = std::collections::HashMap::new();

        Self {
            healthservice: Arc::new(HealthService::new(components)),
        }
    }
}
