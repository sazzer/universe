use super::AuthorizationService;
use crate::authorization::{
    AccessToken, GenerateSecurityContextUseCase, Principal, SecurityContext, SecurityContextId,
};
use biscuit::{jwa::SignatureAlgorithm, jws::Compact, jws::RegisteredHeader, ClaimsSet};
use chrono::{Timelike, Utc};

impl GenerateSecurityContextUseCase for AuthorizationService {
    fn generate_security_context(&self, principal: Principal) -> (SecurityContext, AccessToken) {
        let issued = Utc::now().with_nanosecond(0).unwrap();
        let expires = issued + self.duration;

        let security_context = SecurityContext {
            id: SecurityContextId::default(),
            principal,
            issued,
            expires,
        };

        let decoded_jwt = Compact::new_decoded(
            RegisteredHeader {
                algorithm: SignatureAlgorithm::HS512,
                ..RegisteredHeader::default()
            }
            .into(),
            ClaimsSet::<()>::from(&security_context),
        );

        let access_token = decoded_jwt
            .into_encoded(self.signing_key.as_ref())
            .unwrap()
            .encoded()
            .unwrap()
            .to_string();

        tracing::debug!(access_token = ?access_token, security_context = ?security_context, "Generated security context");

        (security_context, AccessToken(access_token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authorization::service::SigningKey;
    use assert2::check;
    use chrono::Duration;

    #[test]
    fn generate() {
        let duration = Duration::days(5);

        let sut = AuthorizationService::new(SigningKey::new("signingkey"), duration);

        let principal = Principal::User("user_id".to_string());

        let (security_context, _a) = sut.generate_security_context(principal.clone());

        check!(security_context.principal == principal);
        check!(security_context.issued + duration == security_context.expires);
    }
}
