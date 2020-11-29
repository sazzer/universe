use std::sync::Arc;

use crate::server::{Configurer, Server};

/// Builder for building the server.
#[derive(Default)]
pub struct ServerBuilder {
    config: Vec<Arc<dyn Configurer>>,
}

/// Create a new server builder.
pub fn builder() -> ServerBuilder {
    ServerBuilder::default()
}

impl ServerBuilder {
    /// Add a new component to the server.
    pub fn with_component(self, component: Arc<dyn Configurer>) -> Self {
        let mut config = self.config;
        config.push(component);

        Self { config }
    }
    /// Build the HTTP Server component.
    pub fn build(self) -> Server {
        tracing::debug!("Building HTTP Server");

        Server::new(self.config)
    }
}
