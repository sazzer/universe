use crate::database::Database;
use std::sync::Arc;

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
