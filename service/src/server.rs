use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::{ops::Deref, sync::Arc};

/// A function that is able to contribute configuration to the Actix server when it is being constructed.
type FnConfig = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// The wrapper around the HTTP Server
pub struct Server {
    /// The set of configuration traits for the HTTP server.
    pub(super) configs: Vec<FnConfig>,
}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    pub fn new() -> Self {
        Self { configs: vec![] }
    }

    /// Start the Universe HTTP Server listening for requests.
    pub async fn start(&self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting HTTP server");

        let configs = self.configs.clone();
        HttpServer::new(move || {
            let configs = configs.clone();

            let mut app = App::new().wrap(Logger::default());

            for config in &configs {
                app = app.configure(config.deref());
            }

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
