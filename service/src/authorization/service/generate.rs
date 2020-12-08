use super::AuthorizationService;
use crate::authorization::{
    AccessToken, GenerateSecurityContextUseCase, Principal, SecurityContext, SecurityContextId,
    VerifyAccessTokenUseCase,
};
use biscuit::{
    jwa::SignatureAlgorithm, jws::Compact, jws::RegisteredHeader, ClaimsSet, Empty,
    RegisteredClaims, SingleOrMultiple,
};
use chrono::{Timelike, Utc};
use std::{convert::TryFrom, ops::Deref};

type SecurityContextClaims = Compact<ClaimsSet<()>, Empty>;

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

        let decoded_jwt = SecurityContextClaims::from(&security_context);

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

impl VerifyAccessTokenUseCase for AuthorizationService {
    fn verify_access_token(&self, access_token: AccessToken) -> Result<SecurityContext, ()> {
        let claims = Compact::new_encoded(&access_token.0)
            .decode(self.signing_key.as_ref(), SignatureAlgorithm::HS512)
            .map_err(|e| {
                tracing::warn!(e = ?e, access_token = ?access_token, "Failed to decode security context");
            })?;

        let security_context = SecurityContext::try_from(claims).map_err(|e| {
            tracing::warn!(e = ?e, access_token = ?access_token, "Failed to verify security context");
        })?;

        tracing::debug!(security_context = ?security_context, access_token = ?access_token, "Verified security context");

        Ok(security_context)
    }
}

impl From<&SecurityContext> for SecurityContextClaims {
    fn from(security_context: &SecurityContext) -> Self {
        Compact::new_decoded(
            RegisteredHeader {
                algorithm: SignatureAlgorithm::HS512,
                ..RegisteredHeader::default()
            }
            .into(),
            ClaimsSet::<()> {
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
                    id: Some(security_context.id.0.clone()),
                },
                private: (),
            },
        )
    }
}

impl TryFrom<SecurityContextClaims> for SecurityContext {
    type Error = ();

    fn try_from(value: SecurityContextClaims) -> Result<Self, Self::Error> {
        if let Compact::Decoded { header: _, payload } = value {
            let jti = payload.registered.id.map(SecurityContextId);
            let nbf = payload.registered.not_before.map(|ts| *ts.deref());
            let exp = payload.registered.expiry.map(|ts| *ts.deref());
            let sub = payload.registered.subject.map(Principal::User);

            if let (Some(id), Some(issued), Some(expires), Some(principal)) = (jti, nbf, exp, sub) {
                Ok(Self {
                    id,
                    issued,
                    expires,
                    principal,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authorization::service::SigningKey;
    use assert2::{check, let_assert};
    use chrono::Duration;

    #[test]
    fn generate_and_verify() {
        let duration = Duration::days(5);

        let sut = AuthorizationService::new(SigningKey::new("signingkey"), duration);

        let principal = Principal::User("user_id".to_string());

        let (security_context, access_token) = sut.generate_security_context(principal.clone());

        check!(security_context.principal == principal);
        check!(security_context.issued + duration == security_context.expires);

        let verified = sut.verify_access_token(access_token);
        let_assert!(Ok(verified_security_context) = verified);
        check!(security_context.id == verified_security_context.id);
        check!(security_context.issued == verified_security_context.issued);
        check!(security_context.expires == verified_security_context.expires);
        check!(security_context.principal == verified_security_context.principal);
    }
}
