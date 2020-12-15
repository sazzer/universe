use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserModel, Username};
use async_trait::async_trait;

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel> {
        self.repository.get_user_by_id(user_id).await
    }

    async fn get_user_by_username(&self, username: &Username) -> Option<UserModel> {
        self.repository.get_user_by_username(username).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use assert2::{check, let_assert};
    use universe_testdatabase::seed::SeedUser;

    #[actix_rt::test]
    async fn get_unknown_user_by_id() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let user_id = "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap();

        let result = sut.get_user_by_id(&user_id).await;
        check!(result.is_none());
    }

    #[actix_rt::test]
    async fn get_populated_user_by_id() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            user_id: "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap(),
            email: "testuser@example.com".to_owned(),
            username: "testuser".to_owned(),
            ..SeedUser::default()
        };
        test_database.seed(&test_user).await;

        let user_id = "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap();

        let result = sut.get_user_by_id(&user_id).await;

        let_assert!(Some(user) = result);

        check!(user.identity.id == test_user.user_id.to_string().as_ref());
        check!(user.identity.version == test_user.version);
        check!(user.identity.created == test_user.created);
        check!(user.identity.updated == test_user.updated);

        check!(user.data.display_name == test_user.display_name);
        check!(user.data.username == test_user.username.as_ref());
        check!(user.data.email == test_user.email.as_ref());
    }

    #[actix_rt::test]
    async fn get_unknown_user_by_username() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let username = "unknown".parse().unwrap();

        let result = sut.get_user_by_username(&username).await;
        check!(result.is_none());
    }

    #[actix_rt::test]
    async fn get_populated_user_by_username() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            user_id: "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap(),
            email: "testuser@example.com".to_owned(),
            username: "testuser".to_owned(),
            ..SeedUser::default()
        };
        test_database.seed(&test_user).await;

        let username = "testuser".parse().unwrap();

        let result = sut.get_user_by_username(&username).await;

        let_assert!(Some(user) = result);

        check!(user.identity.id == test_user.user_id.to_string().as_ref());
        check!(user.identity.version == test_user.version);
        check!(user.identity.created == test_user.created);
        check!(user.identity.updated == test_user.updated);

        check!(user.data.display_name == test_user.display_name);
        check!(user.data.username == test_user.username.as_ref());
        check!(user.data.email == test_user.email.as_ref());
    }
}
