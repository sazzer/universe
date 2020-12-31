use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::sync::Arc;

/// Trait implemented by all components that can contribute to the Actix service.
pub trait Configurer: Send + Sync {
    /// Configure some details onto the Actix service.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` that routes and data can be wired onto.
    fn configure_server(&self, config: &mut ServiceConfig);
}

/// The wrapper around the HTTP Server
pub struct Server {
    /// The set of configurers to wire details into the server.
    pub(super) config: Vec<Arc<dyn Configurer>>,
}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    ///
    /// # Parameters
    /// - `config` - The set of configurers to wire details into the server
    ///
    /// # Returns
    /// The HTTP server, ready to use.
    pub fn new(config: Vec<Arc<dyn Configurer>>) -> Self {
        Self { config }
    }

    /// Start the Universe HTTP Server listening for requests.
    ///
    /// # Parameters
    /// - `port` - The port to listen on.
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting HTTP server");

        let config = self.config.clone();
        HttpServer::new(move || {
            let config = config.clone();

            let mut app = App::new().wrap(Logger::default()).wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
            );

            for c in &config {
                app = app.configure(move |server_config| {
                    c.configure_server(server_config);
                });
            }

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
