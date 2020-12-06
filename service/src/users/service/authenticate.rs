use super::UsersService;
use crate::users::{
    AuthenticateUserUseCase, Authentication, SaveUserError, UserAuthentication, UserData, UserModel,
};
use async_trait::async_trait;

#[async_trait]
impl AuthenticateUserUseCase for UsersService {
    async fn authenticate(
        &self,
        authentication: UserAuthentication,
    ) -> Result<UserModel, SaveUserError> {
        let user = self
            .repository
            .find_authenticated_user(&authentication.provider, &authentication.user_id)
            .await;

        if let Some(user) = user {
            tracing::info!(user = ?user, "Authenticated as existing user");

            Ok(user)
        } else {
            let user_data = UserData {
                username: authentication.username,
                email: authentication.email,
                display_name: authentication.user_display_name,
                authentications: vec![Authentication {
                    provider: authentication.provider,
                    user_id: authentication.user_id,
                    display_name: authentication.authentication_display_name,
                }],
            };

            let user = self.repository.create(user_data).await;

            tracing::info!(user = ?user, "Authenticated as new user");

            user
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use assert2::{check, let_assert};
    use universe_testdatabase::seed::SeedUser;

    #[actix_rt::test]
    async fn authenticate_known_user() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            user_id: "2caefb5e-712c-4e99-8d18-199c344cc311".parse().unwrap(),
            email: Some("testuser@example.com".to_owned()),
            username: Some("testuser".to_owned()),
            ..SeedUser::default()
        }
        .with_authentication("twitter", "abcdefgh", "@testuser")
        .with_authentication("google", "12345678", "testuser@example.com");

        test_database.seed(&test_user).await;

        let result = sut
            .authenticate(UserAuthentication {
                provider: "google".into(),
                user_id: "12345678".into(),
                authentication_display_name: "testuser@example.com".to_string(),
                user_display_name: "Display Name".to_string(),
                email: None,
                username: None,
            })
            .await;

        let_assert!(Ok(user) = result);

        check!(user.identity.id == "2caefb5e-712c-4e99-8d18-199c344cc311");
        check!(user.identity.version == test_user.version);
        check!(user.identity.created == test_user.created);
        check!(user.identity.updated == test_user.updated);

        check!(user.data.display_name == test_user.display_name);
        check!(user.data.username.unwrap() == "testuser");
        check!(user.data.email.unwrap() == "testuser@example.com");

        check!(user.data.authentications.len() == 2);

        let_assert!(Some(authentication1) = user.data.authentications.get(0));
        check!(authentication1.provider == "google");
        check!(authentication1.user_id == "12345678");
        check!(authentication1.display_name == "testuser@example.com");

        let_assert!(Some(authentication2) = user.data.authentications.get(1));
        check!(authentication2.provider == "twitter");
        check!(authentication2.user_id == "abcdefgh");
        check!(authentication2.display_name == "@testuser");
    }

    #[actix_rt::test]
    async fn authenticate_unknown_bare_user() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let result = sut
            .authenticate(UserAuthentication {
                provider: "google".into(),
                user_id: "12345678".into(),
                authentication_display_name: "testuser@example.com".to_string(),
                user_display_name: "Display Name".to_string(),
                email: None,
                username: None,
            })
            .await;

        let_assert!(Ok(user) = result);

        check!(user.data.display_name == "Display Name");
        check!(user.data.username.is_none());
        check!(user.data.email.is_none());

        check!(user.data.authentications.len() == 1);

        let_assert!(Some(authentication1) = user.data.authentications.get(0));
        check!(authentication1.provider == "google");
        check!(authentication1.user_id == "12345678");
        check!(authentication1.display_name == "testuser@example.com");
    }

    #[actix_rt::test]
    async fn authenticate_unknown_full_user() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let result = sut
            .authenticate(UserAuthentication {
                provider: "google".into(),
                user_id: "12345678".into(),
                authentication_display_name: "testuser@example.com".to_string(),
                user_display_name: "Display Name".to_string(),
                email: Some("testuser@example.com".parse().unwrap()),
                username: Some("testuser".parse().unwrap()),
            })
            .await;

        let_assert!(Ok(user) = result);

        check!(user.data.display_name == "Display Name");
        check!(user.data.username.unwrap() == "testuser");
        check!(user.data.email.unwrap() == "testuser@example.com");

        check!(user.data.authentications.len() == 1);

        let_assert!(Some(authentication1) = user.data.authentications.get(0));
        check!(authentication1.provider == "google");
        check!(authentication1.user_id == "12345678");
        check!(authentication1.display_name == "testuser@example.com");
    }

    #[actix_rt::test]
    async fn authenticate_duplicate_email() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            email: Some("testuser@example.com".to_owned()),
            ..SeedUser::default()
        };

        test_database.seed(&test_user).await;

        let result = sut
            .authenticate(UserAuthentication {
                provider: "google".into(),
                user_id: "12345678".into(),
                authentication_display_name: "testuser@example.com".to_string(),
                user_display_name: "Display Name".to_string(),
                email: Some("testuser@example.com".parse().unwrap()),
                username: None,
            })
            .await;

        let_assert!(Err(err) = result);
        check!(SaveUserError::DuplicateEmail == err);
    }

    #[actix_rt::test]
    async fn authenticate_duplicate_username() {
        let test_database = TestDatabase::new().await;
        let sut = UsersService::new(test_database.database.clone());

        let test_user = SeedUser {
            username: Some("testuser".to_owned()),
            ..SeedUser::default()
        };

        test_database.seed(&test_user).await;

        let result = sut
            .authenticate(UserAuthentication {
                provider: "google".into(),
                user_id: "12345678".into(),
                authentication_display_name: "testuser@example.com".to_string(),
                user_display_name: "Display Name".to_string(),
                email: None,
                username: Some("testuser".parse().unwrap()),
            })
            .await;

        let_assert!(Err(err) = result);
        check!(SaveUserError::DuplicateUsername == err);
    }
}
