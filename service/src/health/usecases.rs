use super::SystemHealth;
use async_trait::async_trait;

/// Use case for checking the overall system health.
#[async_trait]
pub trait HealthCheckUseCase {
    /// Check the health of the system.
    ///
    /// # Returns
    /// The status of the system health.
    async fn check_health(&self) -> SystemHealth;
}
