use crate::http::siren::{Action, Entity, Link, SirenPayload};
use actix_web::web::{get, resource, ServiceConfig};
use index::HomeDocument;

mod index;

/// Configure the endpoints for authentication.
///
/// # Parameters
/// - `entities` - The entities to represent on the home document
/// - `actions` - The actions to represent on the home document
/// - `config` - The configuration object to register the endpoints on to.
pub fn configure(entities: &[Entity], actions: &[Action], config: &mut ServiceConfig) {
    let mut siren_payload = SirenPayload::new(())
        .with_class("tag:universe,2020:classes/home")
        .with_link(Link::new("/").with_rel("self"));

    for entity in entities {
        siren_payload = siren_payload.with_entity(entity.clone());
    }
    for action in actions {
        siren_payload = siren_payload.with_action(action.clone());
    }

    config.data(HomeDocument(siren_payload));
    config.service(resource("/").route(get().to(index::index)));
}
