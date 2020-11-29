use std::sync::Arc;

use actix_web::web::{Data, Json};

use crate::health::HealthCheckUseCase;

use super::model::SystemHealthModel;

pub async fn check_health(
    health_service: Data<Arc<dyn HealthCheckUseCase>>,
) -> Json<SystemHealthModel> {
    let system_health = health_service.check_health().await;

    Json(system_health.into())
}
