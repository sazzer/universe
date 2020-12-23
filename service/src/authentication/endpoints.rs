use crate::http::siren::Entity;
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

/// Return the entities that should contribute to the home document for authentication
pub fn home_document_entities() -> Vec<Entity> {
    vec![Entity::new_link("/authentication").with_rel("tag:universe,2020:rels/authentication")]
}
