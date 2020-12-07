use biscuit::jws::Secret;

/// Representation of a signing key needed to sign access tokens
pub struct SigningKey {
    secret: Secret,
}

impl SigningKey {
    /// Construct a new signing key.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        let secret = value.into();
        Self {
            secret: Secret::bytes_from_str(&secret),
        }
    }
}

impl AsRef<Secret> for SigningKey {
    fn as_ref(&self) -> &Secret {
        &self.secret
    }
}
