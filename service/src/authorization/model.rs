use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The details of the principal that is being authenticated.
#[derive(Debug)]
pub enum Principal {
    User(String),
}

/// The ID of a security context.
#[derive(Debug)]
pub struct SecurityContextId(Uuid);

/// The actual details of a security context.
#[derive(Debug)]
pub struct SecurityContext {
    pub id: SecurityContextId,
    pub principal: Principal,
    pub issued: DateTime<Utc>,
    pub expires: DateTime<Utc>,
}

/// The details of an access token. That is a Security Context that has been serialized into a single string form.
pub struct AccessToken(String);
