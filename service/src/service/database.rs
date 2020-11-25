use crate::database::{Config, Database};

/// Build the database component.
pub async fn build(config: &Config) -> Database {
    tracing::debug!(config = ?config, "Building database connection");

    let database = Database::new(config);

    database.check_health().await.expect("Database health");

    database
}
