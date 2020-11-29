use actix_web::web::ServiceConfig;

use crate::{
    health::endpoints,
    health::HealthCheckUseCase,
    health::{service::HealthService, Healthcheck},
    server::Configurer,
};
use std::{collections::HashMap, sync::Arc};

/// Representation of the Health component in the service
pub struct HealthComponent {
    healthservice: Arc<HealthService>,
}

impl Configurer for HealthComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.healthservice.clone() as Arc<dyn HealthCheckUseCase>);
        endpoints::configure(config);
    }
}

/// Builder for creating the Health Component
#[derive(Default)]
pub struct HealthComponentBuilder {
    components: HashMap<String, Arc<dyn Healthcheck>>,
}

/// Create the Health Component builder
pub fn builder() -> HealthComponentBuilder {
    HealthComponentBuilder::default()
}

impl HealthComponentBuilder {
    /// Add a new component into the healthchecks.
    pub fn with_component<S>(self, name: S, component: Arc<dyn Healthcheck>) -> Self
    where
        S: Into<String>,
    {
        let mut components = self.components;
        components.insert(name.into(), component);
        Self { components }
    }

    /// Build the healthchecks component itself.
    pub fn build(self) -> Arc<HealthComponent> {
        Arc::new(HealthComponent {
            healthservice: Arc::new(HealthService::new(self.components)),
        })
    }
}
