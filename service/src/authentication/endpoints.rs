use crate::http::hal::Link;
use actix_web::web::{get, post, resource, ServiceConfig};
use std::collections::HashMap;

mod start;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/authentication").route(post().to(start::start)));
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
