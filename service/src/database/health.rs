use std::error::Error;

use super::Database;
use crate::health::Healthcheck;
use async_trait::async_trait;

#[async_trait]
impl Healthcheck for Database {
    /// Check if the database connection is healthy.
    async fn check_health(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.checkout().await?;

        conn.query_one("SELECT 1", &[]).await?;

        Ok(())
    }
}
