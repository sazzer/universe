use crate::{authentication::endpoints, server::Configurer};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Authentication Component.
pub struct AuthenticationComponent {}

impl Configurer for AuthenticationComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        endpoints::configure(config);
    }
}

/// Build the Authentication Component.
///
/// # Returns
/// The authentication component to wire in to other components.
pub fn build() -> Arc<AuthenticationComponent> {
    Arc::new(AuthenticationComponent {})
}
