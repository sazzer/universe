use serde::{Deserialize, Serialize};

/// ID of an authentication provider.
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ProviderID(String);

impl<S> From<S> for ProviderID
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
impl PartialEq<&str> for ProviderID {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

/// ID of a user at an authentication provider.
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ProviderUserID(String);

impl<S> From<S> for ProviderUserID
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
impl PartialEq<&str> for ProviderUserID {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

/// Details of a users authentication details at an authentication provider.
#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    pub provider: ProviderID,
    pub user_id: ProviderUserID,
    pub display_name: String,
}
