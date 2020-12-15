use super::{Email, UserID, Username};
use crate::model::Model;

/// Data that makes up a User.
#[derive(Debug, PartialEq)]
pub struct UserData {
    /// The email address of the user, if known.
    pub email: Email,
    /// The username of the user, if known.
    pub username: Username,
    /// The display name of the user.
    pub display_name: String,
}

/// Model representation for a persisted user.
pub type UserModel = Model<UserID, UserData>;
