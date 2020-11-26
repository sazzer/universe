use actix_web::{middleware::Logger, App, HttpServer};

/// The wrapper around the HTTP Server
pub struct Server {}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    pub const fn new() -> Self {
        Self {}
    }

    /// Start the Universe HTTP Server listening for requests.
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting HTTP server");

        HttpServer::new(move || {
            let app = App::new().wrap(Logger::default());

            tracing::trace!("Built listener");

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
