use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The details of the principal that is being authenticated.
#[derive(Debug)]
pub enum Principal {
    User(String),
}

/// The ID of an Access Token.
#[derive(Debug)]
pub struct AccessTokenId(Uuid);

/// The actual details of an Access Token.
#[derive(Debug)]
pub struct AccessToken {
    pub id: AccessTokenId,
    pub principal: Principal,
    pub issued: DateTime<Utc>,
    pub expires: DateTime<Utc>,
}

/// The details of an access token that has been serialized for transmission.
pub struct SerializedAccessToken(String);
