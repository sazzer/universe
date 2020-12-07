use crate::{authorization::service::AuthorizationService, server::Configurer};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Authorization Component.
pub struct AuthorizationComponent {
    service: Arc<AuthorizationService>,
}

impl Configurer for AuthorizationComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
    }
}

/// Build the Authorization Component.
pub fn build() -> Arc<AuthorizationComponent> {
    let service = Arc::new(AuthorizationService::new());
    Arc::new(AuthorizationComponent { service })
}
