use super::model::SystemHealthModel;
use crate::{health::HealthCheckUseCase, http::Response};
use actix_web::web::Data;
use std::sync::Arc;

pub async fn check_health(
    health_service: Data<Arc<dyn HealthCheckUseCase>>,
) -> Response<SystemHealthModel> {
    let system_health = health_service.check_health().await;

    system_health.into()
}
