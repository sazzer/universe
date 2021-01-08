use crate::http::hal::Link;
use actix_web::web::{post, resource, ServiceConfig};
use std::collections::HashMap;

mod authenticate;
mod model;
mod register;
mod start;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/authentication").route(post().to(start::start)));
    config.service(
        resource("/authentication/authenticate").route(post().to(authenticate::authenticate)),
    );
    config.service(resource("/authentication/register").route(post().to(register::register)));
}

/// Return the links that should contribute to the home document for authentication
pub fn home_document_links() -> HashMap<String, Link> {
    let mut links = HashMap::new();
    links.insert(
        "tag:universe,2020:rels/authentication".to_string(),
        "/authentication".into(),
    );

    links
}
