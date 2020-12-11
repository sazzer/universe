use crate::http::Link;

/// The Id of an authentication provider.
#[derive(Debug, PartialEq)]
pub struct ProviderID(String);

impl<S> From<S> for ProviderID
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl From<ProviderID> for Link {
    fn from(provider_id: ProviderID) -> Self {
        Self::new(format!("/authentication/{}/start", provider_id.0)).with_name(provider_id.0)
    }
}
