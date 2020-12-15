use argonautica::{Hasher, Verifier};
use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;

/// Representation of a password for a user, either hashed or not.
#[derive(Debug, PartialEq, Serialize, FromSql)]
pub struct Password(String);

impl Password {
    /// Construct a new hashed password from the already hashed string.
    ///
    /// # Parameters
    /// - `input` - The hashed password to wrap.
    ///
    /// # Returns
    /// The password
    pub fn from_hash<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        Self(input.into())
    }

    /// Construct a new hashed password from the plaintext version.
    ///
    /// # Parameters
    /// - `input` - The plaintext password to hash.
    ///
    /// # Returns
    /// The password
    pub fn from_plaintext<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        let hash = Hasher::default()
            .opt_out_of_secret_key(true)
            .with_password(input.into())
            .hash()
            .unwrap();
        Self(hash)
    }
}

impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        Verifier::default()
            .with_hash(&self.0)
            .with_password(*other)
            .verify()
            .unwrap()
    }
}

impl ToSql for Password {
    accepts!(VARCHAR, TEXT);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};

    #[test]
    fn new_plaintext() {
        let password = Password::from_plaintext("Pa55word");

        check!(password == "Pa55word");

        let_assert!(Password(plaintext) = password);
        check!(plaintext != "Pa55word");
    }

    #[test]
    fn new_hash() {
        let password = Password::from_hash("Hashed");

        let_assert!(Password(hash) = password);
        check!(hash == "Hashed");
    }
}
