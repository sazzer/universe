use super::model::SystemHealthModel;
use crate::{health::HealthCheckUseCase, http::Response};
use actix_web::web::Data;
use std::sync::Arc;

/// HTTP handler to get the health of the system.
///
/// # Parameters
/// - `health_service` - The health service to get the system health with.
///
/// # Returns
/// The representation of the health of the system.
pub async fn check_health(
    health_service: Data<Arc<dyn HealthCheckUseCase>>,
) -> Response<SystemHealthModel> {
    let system_health = health_service.check_health().await;

    system_health.into()
}
