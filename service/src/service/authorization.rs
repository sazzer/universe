use crate::{
    authorization::service::AuthorizationService, authorization::service::SigningKey,
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use chrono::Duration;
use std::sync::Arc;

/// The Authorization Component.
pub struct AuthorizationComponent {
    /// The authorization service.
    service: Arc<AuthorizationService>,
}

impl Configurer for AuthorizationComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
    }
}

/// Build the Authorization Component.
///
/// # Returns
/// The build authorization component.
pub fn build() -> Arc<AuthorizationComponent> {
    let signing_key = SigningKey::new("todo-replace-me");
    let duration = Duration::days(365);

    let service = Arc::new(AuthorizationService::new(signing_key, duration));
    Arc::new(AuthorizationComponent { service })
}
