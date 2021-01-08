use super::UsersService;
use crate::users::{
    repository::SaveUserError, CreateUserError, CreateUserUseCase, UserData, UserModel,
};
use async_trait::async_trait;

#[async_trait]
impl CreateUserUseCase for UsersService {
    async fn create_user(&self, data: UserData) -> Result<UserModel, CreateUserError> {
        self.repository
            .create_user(data)
            .await
            .map_err(|e| match e {
                SaveUserError::DuplicateEmail => CreateUserError::DuplicateEmail,
                SaveUserError::DuplicateUsername => CreateUserError::DuplicateUsername,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        database::test::TestDatabase,
        users::{GetUserUseCase, Password},
    };
    use assert2::{check, let_assert};
    use universe_testdatabase::seed::SeedUser;

    #[actix_rt::test]
    async fn create_user() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let result = sut
            .create_user(UserData {
                username: "testuser".parse().unwrap(),
                email: "testuser@example.com".parse().unwrap(),
                display_name: "Test User".to_string(),
                password: Password::from_plaintext("Pa55word"),
            })
            .await;

        let_assert!(Ok(user) = result);

        check!(user.data.display_name == "Test User");
        check!(user.data.username == "testuser");
        check!(user.data.email == "testuser@example.com");
        check!(user.data.password == "Pa55word");
    }

    #[actix_rt::test]
    async fn create_refetch_user() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let result = sut
            .create_user(UserData {
                username: "testuser".parse().unwrap(),
                email: "testuser@example.com".parse().unwrap(),
                display_name: "Test User".to_string(),
                password: Password::from_plaintext("Pa55word"),
            })
            .await;

        let_assert!(Ok(user) = result);

        let fetched = sut.get_user_by_id(&user.identity.id).await;
        check!(fetched.unwrap() == user);
    }

    #[actix_rt::test]
    async fn create_user_duplicate_username() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            username: "testuser".to_owned(),
            ..SeedUser::default()
        };
        test_database.seed(&test_user).await;

        let result = sut
            .create_user(UserData {
                username: "testuser".parse().unwrap(),
                email: "testuser@example.com".parse().unwrap(),
                display_name: "Test User".to_string(),
                password: Password::from_plaintext("Pa55word"),
            })
            .await;

        let_assert!(Err(err) = result);

        check!(err == CreateUserError::DuplicateUsername);
    }

    #[actix_rt::test]
    async fn create_user_duplicate_email() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            email: "testuser@example.com".to_owned(),
            ..SeedUser::default()
        };
        test_database.seed(&test_user).await;

        let result = sut
            .create_user(UserData {
                username: "testuser".parse().unwrap(),
                email: "testuser@example.com".parse().unwrap(),
                display_name: "Test User".to_string(),
                password: Password::from_plaintext("Pa55word"),
            })
            .await;

        let_assert!(Err(err) = result);

        check!(err == CreateUserError::DuplicateEmail);
    }
}
