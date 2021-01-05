use super::AuthenticationService;
use crate::authentication::AuthenticateUserUseCase;

impl AuthenticateUserUseCase for AuthenticationService {
    fn authenticate_user(
        &self,
        username: crate::users::Username,
        password: String,
    ) -> Result<crate::authentication::AuthenticatedUser, crate::authentication::AuthenticationError>
    {
        todo!()
    }
}
