use super::AuthenticationService;
use crate::authentication::{ListProvidersUseCase, ProviderID};

impl ListProvidersUseCase for AuthenticationService {
    fn list_providers(&self) -> Vec<ProviderID> {
        self.providers
            .iter()
            .map(|provider| provider.0)
            .cloned()
            .collect()
    }
}
