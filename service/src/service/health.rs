use actix_web::web::ServiceConfig;

use crate::{
    health::endpoints,
    health::HealthCheckUseCase,
    health::{service::HealthService, Healthcheck},
    server::Configurer,
};
use std::{collections::HashMap, sync::Arc};

/// Representation of the Health component in the service
pub struct Component {
    /// The health service.
    healthservice: Arc<HealthService>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.healthservice.clone() as Arc<dyn HealthCheckUseCase>);
        endpoints::configure(config);
    }
}

/// Builder for creating the Health Component
#[derive(Default)]
pub struct Builder {
    /// The components to check the health of.
    components: HashMap<String, Arc<dyn Healthcheck>>,
}

impl Builder {
    /// Add a new component into the healthchecks.
    ///
    /// # Parameters
    /// - `name` - The name of the component
    /// - `component` - The component itself
    pub fn with_component<S>(self, name: S, component: Arc<dyn Healthcheck>) -> Self
    where
        S: Into<String>,
    {
        let mut components = self.components;
        components.insert(name.into(), component);
        Self { components }
    }

    /// Build the healthchecks component itself.
    ///
    /// # Returns
    /// The actual healthchecks component.
    pub fn build(self) -> Arc<Component> {
        Arc::new(Component {
            healthservice: Arc::new(HealthService::new(self.components)),
        })
    }
}
