use std::sync::Arc;

use universe_testdatabase::seed::SeedData;

use super::{migrate::migrate, Database};

/// Wrapper around the database and test container
pub struct TestDatabase {
    test_database: universe_testdatabase::TestDatabase,
    pub database: Arc<Database>,
}

impl TestDatabase {
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

    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        self.test_database.seed(data).await;
        self
    }
}
