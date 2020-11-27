use std::{boxed::Box, collections::HashMap, error::Error};

/// Trait that all components able to report on their health should implement.
pub trait Healthcheck {
    /// Check the health of the component.
    fn check_health(&self) -> Result<(), Box<dyn Error>>;
}

/// ComponentHealth represents the health of a single component
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    Healthy,
    Unhealthy(String),
}

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
