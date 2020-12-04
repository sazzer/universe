use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserModel};
use async_trait::async_trait;

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user(&self, user_id: UserID) -> Option<UserModel> {
        self.repository.get_user(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::database::Database;

    use super::*;
    use assert2::check;
    use universe_testdatabase::TestDatabase;

    #[actix_rt::test]
    async fn get_unknown_user() {
        let test_database = TestDatabase::default();
        let database = Arc::new(Database::new(&crate::database::Config {
            url: test_database.url,
        }));
        let sut = UsersService::new(database);

        let user_id = "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap();

        let result = sut.get_user(user_id).await;
        check!(result.is_none());
    }
}
