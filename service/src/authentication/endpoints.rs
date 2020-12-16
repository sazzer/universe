use actix_web::web::{get, post, resource, ServiceConfig};

mod index;
mod start;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(
        resource("/authentication")
            .route(get().to(index::index))
            .route(post().to(start::start)),
    );
}
