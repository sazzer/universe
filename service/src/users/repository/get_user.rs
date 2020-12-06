use serde_json::json;

use super::UsersRepository;
use crate::users::{ProviderID, ProviderUserID, UserID, UserModel};

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

    pub async fn find_authenticated_user(
        &self,
        provider: &ProviderID,
        user_id: &ProviderUserID,
    ) -> Option<UserModel> {
        let conn = self.database.checkout().await.unwrap();

        let query = json!(
            [
                {
                    "provider": provider,
                    "user_id": user_id
                }
            ]
        );

        let user = conn
            .query_opt("SELECT * FROM users WHERE authentications @> $1", &[&query])
            .await
            .unwrap()
            .map(|row| super::parse_row(&row));

        tracing::info!(provider = ?provider, user_id = ?user_id, user = ?user, "Found user");

        user
    }
}
