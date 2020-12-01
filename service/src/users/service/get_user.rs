use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserModel};

impl GetUserUseCase for UsersService {
    fn get_user(&self, user_id: UserID) -> Option<UserModel> {
        todo!()
    }
}
