use actix_web::web::{get, resource, ServiceConfig};

mod check_health;
mod model;

/// Configure the endpoints for checking the system health.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/health").route(get().to(check_health::check_health)));
}
