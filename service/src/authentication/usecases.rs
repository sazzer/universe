use super::ProviderID;

/// Use case for getting the list of authentication providers that can be used.
pub trait ListProvidersUseCase {
    /// Get the list of authentication providers.
    fn list_providers(&self) -> Vec<ProviderID>;
}
