use super::home::Contributor;
use crate::{
    authentication::{endpoints, service::AuthenticationService, AuthenticateUserUseCase},
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Authentication Component.
pub struct Component {
    /// The authentication service.
    service: Arc<AuthenticationService>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn AuthenticateUserUseCase>);

        endpoints::configure(config);
    }
}

impl Contributor for Component {
    fn links(&self) -> std::collections::HashMap<String, crate::http::hal::Link> {
        endpoints::home_document_links()
    }
}

/// Build the Authentication Component.
///
/// # Returns
/// The authentication component to wire in to other components.
pub fn build(
    users_service: Arc<super::users::Component>,
    authorization_service: Arc<super::authorization::Component>,
) -> Arc<Component> {
    let service = Arc::new(AuthenticationService::new(
        users_service.service.clone(),
        authorization_service.service.clone(),
    ));
    Arc::new(Component { service })
}
