mod create_user;
mod errors;
mod get_user;

use super::{UserData, UserID, UserModel};
use crate::{database::Database, model::Identity};
pub use errors::*;
use std::sync::Arc;
use tokio_postgres::Row;

/// Repository for accessing user data.
pub struct UsersRepository {
    /// The database connection.
    database: Arc<Database>,
}

impl UsersRepository {
    /// Creates a new users repository.
    ///
    /// # Parameters
    /// - `database` - The database connection.
    ///
    /// # Returns
    /// The users repository.
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

/// Parse a database row into a `UserModel`.
///
/// # Parameters
/// - `row` - The database row.
///
/// # Returns
/// The user model that this row represents.
pub(super) fn parse_row(row: &Row) -> UserModel {
    let id: UserID = row.get("user_id");

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
            password: row.get("password"),
        },
    }
}
