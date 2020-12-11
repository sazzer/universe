use super::AuthenticationService;
use crate::authentication::{ListProvidersUseCase, ProviderID};

impl ListProvidersUseCase for AuthenticationService {
    fn list_providers(&self) -> Vec<ProviderID> {
        vec![ProviderID::from("google"), ProviderID::from("twitter")]
    }
}
