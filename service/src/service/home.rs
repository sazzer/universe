use crate::{
    home::endpoints,
    http::siren::{Action, Entity},
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// Trait for components that can contribute to the home document to implement.
pub trait Contributor {
    /// Generate a list of all the entities that should be represented on the home document.
    fn entities(&self) -> Vec<Entity> {
        vec![]
    }

    /// Generate a list of all the actions that should be represented on the home document.
    fn actions(&self) -> Vec<Action> {
        vec![]
    }
}

/// The Component for managing the home document.
pub struct Component {
    /// The entities to represent on the home document.
    entities: Vec<Entity>,

    /// The actions to represent on the home document.
    actions: Vec<Action>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        endpoints::configure(&self.entities, &self.actions, config);
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
        let mut entities: Vec<Entity> = vec![];
        let mut actions: Vec<Action> = vec![];

        for component in self.components {
            entities.extend(component.entities());
            actions.extend(component.actions());
        }

        Arc::new(Component { entities, actions })
    }
}
