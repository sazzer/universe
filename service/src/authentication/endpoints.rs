mod list;
mod model;

use actix_web::web::{get, resource, ServiceConfig};

/// Configure the endpoints for authentication
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/authentication").route(get().to(list::list_providers)));
}
