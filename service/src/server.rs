use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::sync::Arc;

/// Trait implemented by all components that can contribute to the Actix service.
pub trait Configurer: Send + Sync {
    /// Configure some details onto the Actix service.
    fn configure_server(&self, config: &mut ServiceConfig);
}

/// The wrapper around the HTTP Server
pub struct Server {
    pub(super) config: Vec<Arc<dyn Configurer>>,
}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    pub fn new(config: Vec<Arc<dyn Configurer>>) -> Self {
        Self { config }
    }

    /// Start the Universe HTTP Server listening for requests.
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting HTTP server");

        let config = self.config.clone();
        HttpServer::new(move || {
            let config = config.clone();

            let mut app = App::new().wrap(Logger::default());

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
