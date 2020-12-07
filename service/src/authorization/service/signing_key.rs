use jsonwebtoken::EncodingKey;

/// Representation of a signing key needed to sign access tokens
pub struct SigningKey {
    encoding_key: EncodingKey,
}

impl SigningKey {
    /// Construct a new signing key.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        let secret = value.into();
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
        }
    }
}

impl AsRef<EncodingKey> for SigningKey {
    fn as_ref(&self) -> &EncodingKey {
        &self.encoding_key
    }
}
