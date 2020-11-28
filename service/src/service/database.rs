use crate::database::{migrate::migrate, Config, Database};
use crate::health::Healthcheck;
use std::sync::Arc;

/// Build the database component.
pub async fn build(config: &Config) -> Arc<Database> {
    tracing::debug!(config = ?config, "Building database connection");

    let database = Database::new(config);

    database.check_health().await.expect("Database health");

    migrate(&database).await;

    Arc::new(database)
}
