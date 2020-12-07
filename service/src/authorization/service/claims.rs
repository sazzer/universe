use biscuit::{ClaimsSet, RegisteredClaims, SingleOrMultiple};

use crate::authorization::{Principal, SecurityContext};

impl From<&SecurityContext> for ClaimsSet<()> {
    fn from(security_context: &SecurityContext) -> Self {
        Self {
            registered: RegisteredClaims {
                issuer: Some("tag:universe,2020:authorization".to_owned()),
                audience: Some(SingleOrMultiple::Single(
                    "tag:universe,2020:authorization".to_owned(),
                )),
                subject: match &security_context.principal {
                    Principal::User(user_id) => Some(user_id.clone()),
                },
                not_before: Some(security_context.issued.into()),
                issued_at: Some(security_context.issued.into()),
                expiry: Some(security_context.expires.into()),
                ..RegisteredClaims::default()
            },
            private: (),
        }
    }
}
