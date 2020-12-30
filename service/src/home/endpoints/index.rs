use crate::{
    http::hal::{HalPayload, HalResponse},
    http::Response,
};
use actix_http::http::header::CacheDirective;
use actix_web::web::Data;

/// The actual home document.
pub struct HomeDocument(pub HalPayload<()>);

/// HTTP handler to get the home document.
///
/// # Returns
/// The Hal model for the home document.
pub async fn index(home_document: Data<HomeDocument>) -> Response<HalPayload<()>> {
    HalResponse {
        body: Some(home_document.0.clone()),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..HalResponse::default()
    }
    .into()
}
