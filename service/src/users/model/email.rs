use serde::Serialize;

/// An email address.
#[derive(Debug, Serialize)]
pub struct Email(String);
