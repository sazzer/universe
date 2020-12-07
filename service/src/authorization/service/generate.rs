use super::AuthorizationService;
use crate::authorization::{
    AccessToken, GenerateAccessTokenUseCase, Principal, SerializedAccessToken,
};

impl GenerateAccessTokenUseCase for AuthorizationService {
    fn generate_access_token(&self, principal: Principal) -> (AccessToken, SerializedAccessToken) {
        todo!()
    }
}
