use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

use super::{ComponentHealth, HealthCheckUseCase, Healthcheck, SystemHealth};

/// The actual health service.
pub struct HealthService {
    /// The components to check the health of.
    components: HashMap<String, Arc<dyn Healthcheck>>,
}

impl HealthService {
    /// Create a new instance of the health service.
    pub fn new(components: HashMap<String, Arc<dyn Healthcheck>>) -> Self {
        Self { components }
    }
}

#[async_trait]
impl HealthCheckUseCase for HealthService {
    async fn check_health(&self) -> SystemHealth {
        let mut components = HashMap::new();

        for (k, v) in &self.components {
            let result = match v.check_health().await {
                Ok(()) => ComponentHealth::Healthy,
                Err(err) => ComponentHealth::Unhealthy(err.to_string()),
            };

            tracing::debug!(component = ?k, health = ?result, "Component health");

            components.insert(k.clone(), result);
        }

        SystemHealth { components }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[actix_rt::test]
    async fn empty_system() {
        let components = HashMap::new();
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.is_healthy() == true);
        check!(result.components.len() == 0);
    }

    #[actix_rt::test]
    async fn healthy_system() {
        let mut components: HashMap<String, Arc<dyn Healthcheck>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(ComponentHealth::Healthy));
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.is_healthy() == true);
        check!(result.components.len() == 1);
        let_assert!(Some(healthy) = result.components.get(&"healthy".to_string()));
        check!(healthy == &ComponentHealth::Healthy);
    }

    #[actix_rt::test]
    async fn unhealthy_system() {
        let mut components: HashMap<String, Arc<dyn Healthcheck>> = HashMap::new();
        components.insert(
            "unhealthy".to_string(),
            Arc::new(ComponentHealth::Unhealthy("Oops".to_string())),
        );
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.is_healthy() == false);
        check!(result.components.len() == 1);
        let_assert!(Some(unhealthy) = result.components.get(&"unhealthy".to_string()));
        check!(unhealthy == &ComponentHealth::Unhealthy("Error: Oops".to_string()));
    }

    #[actix_rt::test]
    async fn mixed_system() {
        let mut components: HashMap<String, Arc<dyn Healthcheck>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(ComponentHealth::Healthy));
        components.insert(
            "unhealthy".to_string(),
            Arc::new(ComponentHealth::Unhealthy("Oops".to_string())),
        );
        let sut = HealthService::new(components);

        let result = sut.check_health().await;

        check!(result.is_healthy() == false);
        check!(result.components.len() == 2);
        let_assert!(Some(healthy) = result.components.get(&"healthy".to_string()));
        check!(healthy == &ComponentHealth::Healthy);
        let_assert!(Some(unhealthy) = result.components.get(&"unhealthy".to_string()));
        check!(unhealthy == &ComponentHealth::Unhealthy("Error: Oops".to_string()));
    }
}
