use super::{Email, UserID, Username};
use crate::model::Model;

/// Data that makes up a User.
#[derive(Debug)]
pub struct UserData {
    /// The email address of the user, if known.
    pub email: Option<Email>,
    /// The username of the user, if known.
    pub username: Option<Username>,
    /// The display name of the user.
    pub display_name: String,
}

/// Model representation for a persisted user.
pub type UserModel = Model<UserID, UserData>;
