use crate::{
    authorization::{AccessToken, SecurityContext},
    users::UserModel,
};

/// Details of the user that was authenticated
pub struct AuthenticatedUser {
    /// The user that was authenticated
    pub user: UserModel,
    /// The security context for the user
    pub security_context: SecurityContext,
    /// The access token for the user
    pub access_token: AccessToken,
}
