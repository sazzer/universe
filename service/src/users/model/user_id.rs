use uuid::Uuid;

/// The unique ID of a user.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);

impl Default for UserID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
