use serde::Serialize;

/// A username..
#[derive(Debug, Serialize)]
pub struct Username(String);
