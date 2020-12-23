use super::home::Contributor;
use crate::{authentication::endpoints, server::Configurer};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Authentication Component.
pub struct Component {}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        endpoints::configure(config);
    }
}

impl Contributor for Component {
    fn entities(&self) -> Vec<crate::http::siren::Entity> {
        endpoints::home_document_entities()
    }
}

/// Build the Authentication Component.
///
/// # Returns
/// The authentication component to wire in to other components.
pub fn build() -> Arc<Component> {
    Arc::new(Component {})
}
