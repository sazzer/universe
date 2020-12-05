use actix_http::http::header::{CacheDirective, EntityTag};
use serde::Serialize;

use crate::{
    http::HalResponse,
    users::{Email, ProviderID, ProviderUserID, UserModel, Username},
};

/// HTTP Model for a users authentication details
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub provider: ProviderID,
    pub user_id: ProviderUserID,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    display_name: String,
    email: Option<Email>,
    username: Option<Username>,
    authentications: Vec<AuthenticationResponse>,
}

impl From<UserModel> for HalResponse<UserResponse> {
    fn from(user: UserModel) -> Self {
        Self {
            cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
            etag: Some(EntityTag::strong(user.identity.version.to_string())),
            data: Some(UserResponse {
                display_name: user.data.display_name,
                email: user.data.email,
                username: user.data.username,
                authentications: user
                    .data
                    .authentications
                    .into_iter()
                    .map(|a| AuthenticationResponse {
                        provider: a.provider,
                        user_id: a.user_id,
                        display_name: a.display_name,
                    })
                    .collect(),
            }),
            ..Self::default()
        }
        .with_link("self", user.identity.id.into())
    }
}
