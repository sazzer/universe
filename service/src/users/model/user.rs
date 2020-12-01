use super::{Authentication, Email, UserID, Username};
use crate::model::Model;

/// Data that makes up a User.
#[derive(Debug)]
pub struct UserData {
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub display_name: String,
    pub authentications: Vec<Authentication>,
}

/// Model representation for a persisted user.
pub type UserModel = Model<UserID, UserData>;
