use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The details of the principal that is being authenticated.
#[derive(Debug, PartialEq, Clone)]
pub enum Principal {
    /// A principal representing a user
    User(String),
}

/// The ID of a security context.
#[derive(Debug, PartialEq)]
pub struct SecurityContextId(pub(super) String);

impl Default for SecurityContextId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

/// The actual details of a security context.
#[derive(Debug)]
pub struct SecurityContext {
    /// The unique ID of this security context
    pub id: SecurityContextId,
    /// The principal that the security context is for
    pub principal: Principal,
    /// When the security context was issued
    pub issued: DateTime<Utc>,
    /// When the security context expires
    pub expires: DateTime<Utc>,
}

/// The details of an access token. That is a Security Context that has been serialized into a single string form.
#[derive(Debug)]
pub struct AccessToken(pub String);
