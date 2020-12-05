use super::{Authentication, UserData, UserID, UserModel};
use crate::{database::Database, model::Identity};
use serde_json::Value;
use std::sync::Arc;
use tokio_postgres::Row;

mod get_user;

/// Repository for accessing user data
pub struct UsersRepository {
    database: Arc<Database>,
}

impl UsersRepository {
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

/// Parse a database row into a `UserModel`.
pub(super) fn parse_row(row: &Row) -> UserModel {
    let id: UserID = row.get("user_id");

    let authentications_json: Value = row.get("authentications");

    let mut authentications =
        match serde_json::from_value::<Vec<Authentication>>(authentications_json) {
            Ok(a) => a,
            Err(e) => {
                tracing::warn!(id = ?id, e = ?e, "Failed to parse authentication details");
                vec![]
            }
        };

    authentications.sort_by(|a, b| a.provider.cmp(&b.provider).then(a.user_id.cmp(&b.user_id)));

    UserModel {
        identity: Identity {
            id,
            version: row.get("version"),
            created: row.get("created"),
            updated: row.get("updated"),
        },
        data: UserData {
            username: row.get("username"),
            email: row.get("email"),
            display_name: row.get("display_name"),
            authentications,
        },
    }
}
