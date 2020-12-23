use crate::{
    http::siren::{SirenPayload, SirenResponse},
    http::Response,
};
use actix_http::http::header::CacheDirective;
use actix_web::web::Data;

/// The actual home document.
pub struct HomeDocument(pub SirenPayload<()>);

/// HTTP handler to get the home document.
///
/// # Returns
/// The Siren model for the home document.
pub async fn index(home_document: Data<HomeDocument>) -> Response<SirenPayload<()>> {
    SirenResponse {
        body: Some(home_document.0.clone()),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..SirenResponse::default()
    }
    .into()
}
