use crate::{home::endpoints, http::hal::Link, server::Configurer};
use actix_web::web::ServiceConfig;
use std::{collections::HashMap, sync::Arc};

/// Trait for components that can contribute to the home document to implement.
pub trait Contributor {
    /// Generate a collection of the links to contribute to the home document.
    fn links(&self) -> HashMap<String, Link> {
        HashMap::new()
    }
}

/// The Component for managing the home document.
pub struct Component {
    /// The links to represent on the home document.
    links: Vec<HashMap<String, Link>>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        endpoints::configure(config, &self.links);
    }
}

/// Builder for building the home document.
#[derive(Default)]
pub struct Builder {
    /// The components that can contribute to the home document.
    components: Vec<Arc<dyn Contributor>>,
}

impl Builder {
    /// Add a component to the home document.
    ///
    /// # Parameters
    /// - `component` - The component to add to the home document
    pub fn with_component(mut self, component: Arc<dyn Contributor>) -> Self {
        self.components.push(component);

        self
    }

    /// Build the Home Component.
    ///
    /// # Returns
    /// The home component to wire in to other components.
    pub fn build(self) -> Arc<Component> {
        let mut links: Vec<HashMap<String, Link>> = vec![];

        for component in self.components {
            links.push(component.links());
        }

        Arc::new(Component { links })
    }
}
