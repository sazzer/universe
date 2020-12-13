use crate::{
    authentication::service::google::Provider,
    authentication::ListProvidersUseCase,
    authentication::{endpoints, service::AuthenticationService},
    authentication::{service::AuthenticationProvider, ProviderID},
    server::Configurer,
};
use actix_web::web::ServiceConfig;
use std::{collections::HashMap, sync::Arc};

/// Configuration for the Google authenticator
#[derive(Debug)]
pub struct GoogleConfig {
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_auth_uri: Option<String>,
    pub google_token_uri: Option<String>,
}

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
    /// Configure the Google Authenticator
    pub fn with_google(mut self, config: GoogleConfig) -> Self {
        if let (Some(client_id), Some(client_secret)) =
            (config.google_client_id, config.google_client_secret)
        {
            let google_provider = Provider::new(
                client_id.into(),
                client_secret.into(),
                config.google_auth_uri.unwrap_or_else(|| "".to_string()),
                config.google_token_uri.unwrap_or_else(|| "".to_string()),
            );

            self.providers
                .insert("google".into(), Arc::new(google_provider));
        }

        self
    }

    /// Build the Authentication Component.
    pub fn build(self) -> Arc<AuthenticationComponent> {
        let service = Arc::new(AuthenticationService::new(self.providers));
        Arc::new(AuthenticationComponent { service })
    }
}
