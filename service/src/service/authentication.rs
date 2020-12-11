use crate::{
    authentication::ListProvidersUseCase,
    authentication::{endpoints, service::AuthenticationService},
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Authentication Component.
pub struct AuthenticationComponent {
    service: Arc<AuthenticationService>,
}

impl Configurer for AuthenticationComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn ListProvidersUseCase>);

        endpoints::configure(config);
    }
}

/// Build the Authentication Component.
pub fn build() -> Arc<AuthenticationComponent> {
    let service = Arc::new(AuthenticationService::new());
    Arc::new(AuthenticationComponent { service })
}
