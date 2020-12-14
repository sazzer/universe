use actix_web::web::{get, resource, ServiceConfig};

mod get_user;
mod model;

/// Configure the endpoints for working with users.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/users/{id}").route(get().to(get_user::get_user)));
}
