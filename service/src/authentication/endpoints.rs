use actix_web::web::{get, resource, ServiceConfig};

mod index;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/authentication").route(get().to(index::index)));
}
