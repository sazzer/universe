use crate::{
    authentication::AuthenticatedUser,
    authorization::AccessToken,
    http::hal::{HalPayload, HalResponse},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserResponse {
    pub access_token: AccessToken,
    pub expires: DateTime<Utc>,
}

impl From<AuthenticatedUser> for HalResponse<AuthenticatedUserResponse> {
    fn from(input: AuthenticatedUser) -> Self {
        let payload = HalPayload::new(AuthenticatedUserResponse {
            access_token: input.access_token,
            expires: input.security_context.expires,
        })
        .with_link("related", input.user.identity.id);

        Self {
            body: Some(payload),
            ..Self::default()
        }
    }
}
