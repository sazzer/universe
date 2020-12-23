use crate::server::{Configurer, Server};
use std::sync::Arc;

/// Builder for building the server.
#[derive(Default)]
pub struct Builder {
    /// The set of configuration objects for wiring into the server.
    config: Vec<Arc<dyn Configurer>>,
}

impl Builder {
    /// Add a new component to the server.
    ///
    /// # Parameters
    /// - `component` - The component to add to the server
    pub fn with_component(self, component: Arc<dyn Configurer>) -> Self {
        let mut config = self.config;
        config.push(component);

        Self { config }
    }
    /// Build the HTTP Server component.
    ///
    /// # Returns
    /// The server to work with.
    pub fn build(self) -> Server {
        tracing::debug!("Building HTTP Server");

        Server::new(self.config)
    }
}
