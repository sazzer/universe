use super::Service;
use actix_http::Request;
use actix_web::App;

impl Service {
    /// Inject a request into the server. Only used for testing
    pub async fn inject(&self, req: Request) -> TestResponse {
        let app = App::new();

        let mut test_service = actix_web::test::init_service(app).await;
        let response = actix_web::test::call_service(&mut test_service, req).await;

        let status = response.status();
        let headers = response.headers().clone();
        let body = actix_web::test::read_body(response).await;

        TestResponse {
            status,
            headers,
            body,
        }
    }
}

/// Representation of the response to injecting a test request
pub struct TestResponse {
    /// The status code
    pub status: actix_http::http::StatusCode,
    /// The set of headers
    pub headers: actix_http::http::HeaderMap,
    /// The response body
    pub body: bytes::Bytes,
}

impl TestResponse {
    /// Get the value of the header with the given name
    ///
    /// # Parameters
    /// - `name` - The name of the header
    ///
    /// # Returns
    /// The header, if present. `None` if it wasn't present.
    pub fn header<S>(&self, name: S) -> Option<&actix_http::http::HeaderValue>
    where
        S: Into<String>,
    {
        self.headers.get(name.into())
    }
}
