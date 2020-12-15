use super::UsersRepository;
use crate::users::{UserID, UserModel, Username};

impl UsersRepository {
    /// Get the user that has the given User Id.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the User to retrieve.
    ///
    /// # Returns
    /// The user with the given ID. If no user was found then returns `None`.
    pub async fn get_user_by_id(&self, user_id: &UserID) -> Option<UserModel> {
        let conn = self.database.checkout().await.unwrap();

        let user = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[user_id])
            .await
            .unwrap()
            .map(|row| super::parse_row(&row));

        tracing::info!(user_id = ?user_id, user = ?user, "Found user");

        user
    }

    /// Get the user that has the given Username.
    ///
    /// # Parameters
    /// - `username` - The username of the User to retrieve.
    ///
    /// # Returns
    /// The user with the given username. If no user was found then returns `None`.
    pub async fn get_user_by_username(&self, username: &Username) -> Option<UserModel> {
        let conn = self.database.checkout().await.unwrap();

        let user = conn
            .query_opt("SELECT * FROM users WHERE username = $1", &[username])
            .await
            .unwrap()
            .map(|row| super::parse_row(&row));

        tracing::info!(username = ?username, user = ?user, "Found user");

        user
    }
}
