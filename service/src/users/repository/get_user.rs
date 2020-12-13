use super::UsersRepository;
use crate::users::{UserID, UserModel};

impl UsersRepository {
    pub async fn get_user(&self, user_id: &UserID) -> Option<UserModel> {
        let conn = self.database.checkout().await.unwrap();

        let user = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[user_id])
            .await
            .unwrap()
            .map(|row| super::parse_row(&row));

        tracing::info!(user_id = ?user_id, user = ?user, "Found user");

        user
    }
}
