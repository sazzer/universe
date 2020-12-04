use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// A username..
#[derive(Debug, PartialEq, Serialize, FromSql)]
pub struct Username(String);

#[derive(Debug, PartialEq, Clone, Copy, thiserror::Error)]
pub enum ParseUsernameError {
    #[error("The Username was blank")]
    Blank,
}

impl FromStr for Username {
    type Err = ParseUsernameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed == "" {
            Err(ParseUsernameError::Blank)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl ToSql for Username {
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
impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("", ParseUsernameError::Blank ; "Blank")]
    #[test_case("  ", ParseUsernameError::Blank ; "Whitespace")]
    fn failed_parse(input: &str, expected: ParseUsernameError) {
        let parsed: Result<Username, ParseUsernameError> = input.parse();
        let_assert!(Err(err) = parsed);
        check!(err == expected);
    }

    #[test_case("testuser", "testuser" ; "Simple")]
    #[test_case("  testuser", "testuser" ; "Left padded")]
    #[test_case("testuser  ", "testuser" ; "Right padded")]
    #[test_case("  testuser  ", "testuser" ; "Both padded")]
    fn successful_parse(input: &str, expected: &str) {
        let parsed: Result<Username, ParseUsernameError> = input.parse();
        let_assert!(Ok(user_id) = parsed);

        check!(user_id == expected);
    }
}
