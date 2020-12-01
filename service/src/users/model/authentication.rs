/// ID of an authentication provider.
#[derive(Debug)]
pub struct ProviderID(String);

/// ID of a user at an authentication provider.
#[derive(Debug)]
pub struct ProviderUserID(String);

/// Details of a users authentication details at an authentication provider.
#[derive(Debug)]
pub struct Authentication {
    pub provider: ProviderID,
    pub user_id: ProviderUserID,
    pub display_name: String,
}
