use crate::http::hal::{HalPayload, Link};
use actix_web::web::{get, resource, ServiceConfig};
use index::HomeDocument;
use std::collections::HashMap;

mod index;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `config` - The configuration object to register the endpoints on to.
/// - `links` - The links to represent on the home document
pub fn configure(config: &mut ServiceConfig, links: &[HashMap<String, Link>]) {
    let mut hal_payload = HalPayload::new(());

    for l in links {
        for (name, link) in l {
            hal_payload = hal_payload.with_link(name.clone(), link.clone());
        }
    }

    config.data(HomeDocument(hal_payload));
    config.service(resource("/").route(get().to(index::index)));
}
