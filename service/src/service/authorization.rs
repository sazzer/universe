use crate::{
    authorization::service::AuthorizationService, authorization::service::SigningKey,
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use chrono::Duration;
use std::sync::Arc;

/// The Authorization Component.
pub struct Component {
    /// The authorization service.
    service: Arc<AuthorizationService>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
    }
}

/// Build the Authorization Component.
///
/// # Returns
/// The build authorization component.
pub fn build() -> Arc<Component> {
    let signing_key = SigningKey::new("todo-replace-me");
    let duration = Duration::days(365);

    let service = Arc::new(AuthorizationService::new(signing_key, duration));
    Arc::new(Component { service })
}
