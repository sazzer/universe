use super::Database;

impl Database {
    /// Check if the database connection is healthy.
    pub async fn check_health(&self) -> Result<(), String> {
        let conn = self
            .checkout()
            .await
            .map_err(|e| format!("Failed to checkout connection: {}", e))?;

        conn.query_one("SELECT 1", &[])
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        Ok(())
    }
}
