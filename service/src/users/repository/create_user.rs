use chrono::Utc;
use uuid::Uuid;

use super::UsersRepository;
use crate::users::{UserData, UserID, UserModel};

impl UsersRepository {
    pub async fn create(&self, data: UserData) -> Result<UserModel, ()> {
        let user_id = UserID::default();
        let version = Uuid::new_v4();
        let now = Utc::now();

        let authentications = serde_json::to_value(data.authentications).unwrap();

        let conn = self.database.checkout().await.unwrap();

        conn.query_one(
        "INSERT INTO users(user_id, version, created, updated, username, email, display_name, authentications)
          VALUES ($1, $2, $3, $3, $4, $5, $6, $7)
          RETURNING *",
          &[&user_id, &version, &now, &data.username, &data.email, &data.display_name, &authentications]).await
            .map(|row| super::parse_row(&row))
            .map_err(|e| {
              tracing::warn!(e = ?e, "Error creating user");

              ()
            })
    }
}
