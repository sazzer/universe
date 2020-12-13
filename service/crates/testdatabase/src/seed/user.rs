use super::SeedData;
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
    pub username: Option<String>,
    pub email: Option<String>,
    pub display_name: String,
}

impl Default for SeedUser {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: Utc::now().with_nanosecond(0).unwrap(),
            updated: Utc::now().with_nanosecond(0).unwrap(),
            username: None,
            email: None,
            display_name: format!("{}", Uuid::new_v4()),
        }
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, username, email, display_name)
          VALUES ($1, $2, $3, $4, $5, $6, $7)"
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
        ]
    }
}
