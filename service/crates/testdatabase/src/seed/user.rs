use super::SeedData;
use argonautica::Hasher;
use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use uuid::Uuid;

/// Representation of a user ready to seed into the test database.
#[derive(Debug)]
pub struct SeedUser {
    pub user_id: Uuid,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub password: String,
}

impl Default for SeedUser {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: Utc::now().with_nanosecond(0).unwrap(),
            updated: Utc::now().with_nanosecond(0).unwrap(),
            username: format!("{}", Uuid::new_v4()),
            email: format!("{}", Uuid::new_v4()),
            display_name: format!("{}", Uuid::new_v4()),
            password: "-".to_string(),
        }
    }
}

impl SeedUser {
    pub fn with_password<S>(mut self, password: S) -> Self
    where
        S: Into<String>,
    {
        self.password = Hasher::default()
            .opt_out_of_secret_key(true)
            .with_password(password.into())
            .hash()
            .unwrap();

        self
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, username, email, display_name, password)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    }

    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.username,
            &self.email,
            &self.display_name,
            &self.password,
        ]
    }
}
