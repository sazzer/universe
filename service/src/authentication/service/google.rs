use super::AuthenticationProvider;

/// The Client ID for working with Google
#[derive(Debug)]
pub struct ClientID(String);

impl<S> From<S> for ClientID
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

/// The Client Secret for working with Google
#[derive(Debug)]
pub struct ClientSecret(String);

impl<S> From<S> for ClientSecret
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

/// The actual Google authentication provider
pub struct Provider {
    client_id: ClientID,
    client_secret: ClientSecret,
    auth_uri: String,
    token_uri: String,
}

impl Provider {
    pub const fn new(
        client_id: ClientID,
        client_secret: ClientSecret,
        auth_uri: String,
        token_uri: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            auth_uri,
            token_uri,
        }
    }
}

impl AuthenticationProvider for Provider {}
