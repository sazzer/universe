use super::{parse_row, SaveUserError, UsersRepository};
use crate::{
    model::Identity,
    users::{UserData, UserID, UserModel},
};

impl UsersRepository {
    /// Create a new user with the given data.
    ///
    /// # Parameters
    /// - `data` - The data for the new user
    ///
    /// # Returns
    /// The newly created user.
    /// If the user creation fails then rerturn an indication as to why.
    pub async fn create_user(&self, data: UserData) -> Result<UserModel, SaveUserError> {
        let conn = self.database.checkout().await.unwrap();

        let identity: Identity<UserID> = Identity::default();

        let user = conn.query_one(
            "INSERT INTO users(user_id, version, created, updated, username, email, display_name) VALUES ($1, $2, $3, $3, $4, $5, $6) RETURNING *",
            &[
                &identity.id,
                &identity.version,
                &identity.created,
                &data.username,
                &data.email,
                &data.display_name
            ],
        )
        .await
        .map(|row| parse_row(&row))?;

        tracing::debug!(user = ?user, "Created user");

        Ok(user)
    }
}
