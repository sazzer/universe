use std::sync::Arc;

use crate::database::Database;

/// Repository for accessing user data
pub struct UsersRepository {
    database: Arc<Database>,
}

impl UsersRepository {
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
