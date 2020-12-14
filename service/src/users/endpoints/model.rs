use actix_http::http::header::{CacheDirective, EntityTag};
use serde::Serialize;

use crate::{
    http::HalResponse,
    users::{Email, UserModel, Username},
};

/// Representation of the data that makes up a user in the HTTP response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// The display name of the user
    display_name: String,
    /// The email address of the user
    email: Option<Email>,
    /// The username of the user
    username: Option<Username>,
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
            }),
            ..Self::default()
        }
        .with_link("self", user.identity.id.into())
    }
}
