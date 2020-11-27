use super::SystemHealth;

/// Use case for checking the overall system health.
pub trait HealthCheckUseCase {
    /// Check the health of the system.
    fn check_health(&self) -> SystemHealth;
}
