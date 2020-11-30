use actix_http::http::StatusCode;
use serde::Serialize;
use std::collections::HashMap;

use crate::{
    health::{ComponentHealth, SystemHealth},
    http::Response,
};

/// HTTP Model to represent the health of a single component
#[derive(Debug, Serialize)]
pub struct ComponentHealthModel {
    pub healthy: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// HTTP Model to represent the health of the entire system
#[derive(Debug, Serialize)]
pub struct SystemHealthModel {
    pub healthy: bool,
    pub components: HashMap<String, ComponentHealthModel>,
}

impl From<ComponentHealth> for ComponentHealthModel {
    fn from(component_health: ComponentHealth) -> Self {
        match component_health {
            ComponentHealth::Healthy => Self {
                healthy: true,
                message: None,
            },
            ComponentHealth::Unhealthy(msg) => Self {
                healthy: false,
                message: Some(msg),
            },
        }
    }
}

impl From<SystemHealth> for SystemHealthModel {
    fn from(system_health: SystemHealth) -> Self {
        Self {
            healthy: system_health.is_healthy(),
            components: system_health
                .components
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<SystemHealth> for Response<SystemHealthModel> {
    fn from(system_health: SystemHealth) -> Self {
        let status = if system_health.is_healthy() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        Self {
            body: Some(system_health.into()),
            status,
            ..Self::default()
        }
    }
}
