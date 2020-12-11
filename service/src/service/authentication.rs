use crate::{
    authentication::ListProvidersUseCase,
    authentication::{endpoints, service::AuthenticationService},
    authentication::{service::AuthenticationProvider, ProviderID},
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use std::{collections::HashMap, sync::Arc};

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

/// Builder for building the authentication component
#[derive(Default)]
pub struct AuthenticationComponentBuilder {
    providers: HashMap<ProviderID, Arc<dyn AuthenticationProvider>>,
}

/// Construct the builder
pub fn builder() -> AuthenticationComponentBuilder {
    AuthenticationComponentBuilder::default()
}

impl AuthenticationComponentBuilder {
    /// Build the Authentication Component.
    pub fn build(self) -> Arc<AuthenticationComponent> {
        let service = Arc::new(AuthenticationService::new(self.providers));
        Arc::new(AuthenticationComponent { service })
    }
}
