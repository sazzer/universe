use super::UsersRepository;
use crate::{
    model::Identity,
    users::{UserData, UserID, UserModel},
};

impl UsersRepository {
    pub async fn get_user(&self, user_id: &UserID) -> Option<UserModel> {
        let conn = self.database.checkout().await.unwrap();

        let user = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[user_id])
            .await
            .unwrap()
            .map(|row| UserModel {
                identity: Identity {
                    id: row.get("user_id"),
                    version: row.get("version"),
                    created: row.get("created"),
                    updated: row.get("updated"),
                },
                data: UserData {
                    username: None,
                    email: None,
                    display_name: row.get("display_name"),
                    authentications: vec![],
                },
            });

        tracing::info!(user_id = ?user_id, user = ?user, "Found user");

        user
    }
}
