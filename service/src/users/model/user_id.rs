use std::str::FromStr;

use uuid::Uuid;

use crate::http::Link;

/// The unique ID of a user.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Clone, Copy, thiserror::Error)]
pub enum ParseUserIDError {
    #[error("The User ID was malformed")]
    Malformed,
}

impl FromStr for UserID {
    type Err = ParseUserIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s.trim()).map(Self).map_err(|e| {
            tracing::warn!(e = ?e, input = s, "Malformed User ID");
            ParseUserIDError::Malformed
        })
    }
}

impl From<UserID> for Link {
    fn from(user_id: UserID) -> Self {
        Self::new(format!("/users/{}", user_id.0), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{check, let_assert};
    use test_case::test_case;

    #[test_case("", ParseUserIDError::Malformed ; "Blank")]
    #[test_case("  ", ParseUserIDError::Malformed ; "Whitespace")]
    #[test_case("Invalid", ParseUserIDError::Malformed ; "Not a UUID")]
    #[test_case("5046e050-47d3-4a11-b2c6-0517ed30d80", ParseUserIDError::Malformed ; "Truncated")]
    #[test_case("5046e050-47d3-4a11-b2c6-0517ed30d8055", ParseUserIDError::Malformed ; "Extended")]
    #[test_case("5046e050-47d3-4a11-b2c6-0517ed30d80g", ParseUserIDError::Malformed ; "Invalid character")]
    fn failed_parse(input: &str, expected: ParseUserIDError) {
        let parsed: Result<UserID, ParseUserIDError> = input.parse();
        let_assert!(Err(err) = parsed);
        check!(err == expected);
    }

    #[test_case("5046e050-47d3-4a11-b2c6-0517ed30d805" ; "Simple")]
    #[test_case("  5046e050-47d3-4a11-b2c6-0517ed30d805" ; "Left padded")]
    #[test_case("5046e050-47d3-4a11-b2c6-0517ed30d805  " ; "Right padded")]
    #[test_case("  5046e050-47d3-4a11-b2c6-0517ed30d805  " ; "Both padded")]
    #[test_case("5046e05047d34a11b2c60517ed30d805" ; "Missing hyphens")]
    fn successful_parse(input: &str) {
        let parsed: Result<UserID, ParseUserIDError> = input.parse();
        let_assert!(Ok(_) = parsed);
    }
}
