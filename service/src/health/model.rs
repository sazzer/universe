use async_trait::async_trait;
use std::{boxed::Box, collections::HashMap, error::Error};

/// The health of a single component
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    Healthy,
    Unhealthy(String),
}

/// The health of the whole system
#[derive(Debug)]
pub struct SystemHealth {
    pub components: HashMap<String, ComponentHealth>,
}

impl ComponentHealth {
    /// Check if the component is healthy.
    pub fn is_healthy(&self) -> bool {
        self == &Self::Healthy
    }
}

impl SystemHealth {
    /// Check if th system as a whole is healthy.
    pub fn is_healthy(&self) -> bool {
        self.components.iter().all(|c| c.1.is_healthy())
    }
}

/// Trait that all components able to report on their health should implement.
#[async_trait]
pub trait Healthcheck: Send + Sync {
    /// Check the health of the component.
    async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
#[cfg(test)]
impl Healthcheck for ComponentHealth {
    async fn check_health(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Healthy => Ok(()),
            Self::Unhealthy(e) => Err(string_error::new_err(e)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn healthy_component() {
        let sut = ComponentHealth::Healthy;
        check!(sut.is_healthy() == true);
    }

    #[test]
    fn unhealthy_component() {
        let sut = ComponentHealth::Unhealthy("Oops".to_string());
        check!(sut.is_healthy() == false);
    }

    #[test]
    fn empty_system() {
        let components = HashMap::new();
        let sut = SystemHealth { components };
        check!(sut.is_healthy() == true);
    }

    #[test]
    fn healthy_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_string(), ComponentHealth::Healthy);
        let sut = SystemHealth { components };
        check!(sut.is_healthy() == true);
    }

    #[test]
    fn unhealthy_system() {
        let mut components = HashMap::new();
        components.insert(
            "unhealthy".to_string(),
            ComponentHealth::Unhealthy("Oops".to_string()),
        );
        let sut = SystemHealth { components };
        check!(sut.is_healthy() == false);
    }

    #[test]
    fn mixed_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_string(), ComponentHealth::Healthy);
        components.insert(
            "unhealthy".to_string(),
            ComponentHealth::Unhealthy("Oops".to_string()),
        );
        let sut = SystemHealth { components };
        check!(sut.is_healthy() == false);
    }
}
