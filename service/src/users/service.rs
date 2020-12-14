use super::repository::UsersRepository;
use crate::database::Database;
use std::sync::Arc;

mod get_user;

/// Service for interacting with Users.
pub struct UsersService {
    /// The repository of user data.
    repository: UsersRepository,
}

impl UsersService {
    /// Create a new instance of the users service.
    ///
    /// # Parameters
    /// - `database` - The database connection
    ///
    /// # Returns
    /// The users service.
    pub fn new(database: Arc<Database>) -> Self {
        let repository = UsersRepository::new(database);
        Self { repository }
    }
}
