use actix_http::http::header::CacheDirective;

use crate::{
    http::siren::{Action, Field, Link, SirenPayload, SirenResponse},
    http::Response,
};

/// HTTP handler to get the model for how to start authentication.
///
/// # Returns
/// The Siren model for how to start authentication
pub async fn index() -> Response<SirenPayload<()>> {
    SirenResponse {
        body: Some(
            SirenPayload::new(())
                .with_class("tag:universe,2020:classes/authentication")
                .with_link(Link::new("/authentication").with_rel("self"))
                .with_action(
                    Action::new(
                        "tag:universe,2020:actions/authentication/start",
                        "POST",
                        "/authentication",
                    )
                    .with_type_form()
                    .with_field(Field::new("username", "text")),
                ),
        ),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..SirenResponse::default()
    }
    .into()
}
