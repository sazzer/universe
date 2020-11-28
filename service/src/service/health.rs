use crate::health::{service::HealthService, Healthcheck};
use std::{collections::HashMap, sync::Arc};

/// Representation of the Health component in the service
pub struct HealthComponent {
    healthservice: Arc<HealthService>,
}

impl HealthComponent {
    pub fn builder() -> HealthComponentBuilder {
        HealthComponentBuilder::default()
    }
}

#[derive(Default)]
pub struct HealthComponentBuilder {
    components: HashMap<String, Arc<dyn Healthcheck>>,
}

impl HealthComponentBuilder {
    pub fn with_component<S>(self, name: S, component: Arc<dyn Healthcheck>) -> Self
    where
        S: Into<String>,
    {
        let mut components = self.components;
        components.insert(name.into(), component);
        Self { components }
    }

    pub fn build(self) -> HealthComponent {
        HealthComponent {
            healthservice: Arc::new(HealthService::new(self.components)),
        }
    }
}
