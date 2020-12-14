use std::sync::Arc;

use universe_testdatabase::seed::SeedData;

use super::{migrate::migrate, Database};

/// Wrapper around the database and test container
pub struct TestDatabase {
    /// The test database container
    test_database: universe_testdatabase::TestDatabase,
    /// The database connection
    pub database: Arc<Database>,
}

impl TestDatabase {
    /// Create a new test database
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let test_database = universe_testdatabase::TestDatabase::default();
        let database = Arc::new(Database::new(&crate::database::Config {
            url: test_database.url.clone(),
        }));

        migrate(&database).await;

        Self {
            test_database,
            database,
        }
    }

    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        self.test_database.seed(data).await;
        self
    }
}
