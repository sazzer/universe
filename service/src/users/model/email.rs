use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// An email address.
#[derive(Debug, PartialEq, Serialize, FromSql)]
pub struct Email(String);

/// Potential errors from parsing an email address from a string.
#[derive(Debug, PartialEq, Clone, Copy, thiserror::Error)]
pub enum ParseEmailError {
    /// The Email was blank
    #[error("The Email was blank")]
    Blank,
}

impl FromStr for Email {
    type Err = ParseEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed == "" {
            Err(ParseEmailError::Blank)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl ToSql for Email {
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
impl PartialEq<&str> for Email {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("", ParseEmailError::Blank ; "Blank")]
    #[test_case("  ", ParseEmailError::Blank ; "Whitespace")]
    fn failed_parse(input: &str, expected: ParseEmailError) {
        let parsed: Result<Email, ParseEmailError> = input.parse();
        let_assert!(Err(err) = parsed);
        check!(err == expected);
    }

    #[test_case("testuser@example.com", "testuser@example.com" ; "Simple")]
    #[test_case("  testuser@example.com", "testuser@example.com" ; "Left padded")]
    #[test_case("testuser@example.com  ", "testuser@example.com" ; "Right padded")]
    #[test_case("  testuser@example.com  ", "testuser@example.com" ; "Both padded")]
    fn successful_parse(input: &str, expected: &str) {
        let parsed: Result<Email, ParseEmailError> = input.parse();
        let_assert!(Ok(user_id) = parsed);

        check!(user_id == expected);
    }
}
