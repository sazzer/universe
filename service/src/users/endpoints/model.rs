use actix_http::http::header::{CacheDirective, EntityTag};
use serde::Serialize;

use crate::{
    http::HalResponse,
    users::{Email, UserModel, Username},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    display_name: String,
    email: Option<Email>,
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
