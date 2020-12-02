use actix_http::{
    http::header::CacheControl,
    http::{
        header::{CacheDirective, ContentType},
        StatusCode,
    },
    Error, Response as HttpResponse,
};
use actix_web::Responder;
use futures::future::{ok, Ready};
use serde::Serialize;

/// Friendly representation of the HTTP Response to send back.
pub struct Response<T>
where
    T: Serialize,
{
    pub status: StatusCode,
    pub cache_control: Vec<CacheDirective>,
    pub content_type: ContentType,
    pub body: Option<T>,
}

impl<T> Default for Response<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            cache_control: vec![CacheDirective::NoCache],
            content_type: ContentType::json(),
            body: None,
        }
    }
}

impl<T> Responder for Response<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        let response = HttpResponse::build(self.status)
            .set(CacheControl(self.cache_control))
            .set(self.content_type)
            .json(self.body);

        ok(response)
    }
}
