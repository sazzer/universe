use actix_http::http::header::{CacheDirective, EntityTag};
use serde::Serialize;

use crate::{
    http::hal::{HalPayload, HalResponse},
    users::{Email, UserModel, Username},
};

/// Representation of the data that makes up a user in the HTTP response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// The display name of the user
    display_name: String,
    /// The email address of the user
    email: Email,
    /// The username of the user
    username: Username,
}

impl From<UserModel> for HalResponse<UserResponse> {
    fn from(user: UserModel) -> Self {
        let payload = HalPayload::new(UserResponse {
            display_name: user.data.display_name,
            email: user.data.email,
            username: user.data.username,
        })
        .with_link("self", user.identity.id);

        Self {
            cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
            etag: Some(EntityTag::strong(user.identity.version.to_string())),
            body: Some(payload),
            ..Self::default()
        }
    }
}
