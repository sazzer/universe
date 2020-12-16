mod authentication;
mod authorization;
mod database;
mod health;
mod server;
pub mod testing;
mod users;

use crate::database::Config as DatabaseConfig;
use crate::server::Server;

/// Configuration needed to build the service.
#[derive(Debug)]
pub struct Config {
    /// Database connection configuration.
    pub database: DatabaseConfig,
}

/// The actual Universe service.
pub struct Service {
    /// The HTTP Server.
    server: Server,
}

impl Service {
    /// Construct a new instance of the Universe service.
    ///
    /// # Parameters
    /// - `config` - The configuration to build the service from.
    ///
    /// # Returns
    /// The service, ready to use.
    pub async fn new(config: Config) -> Self {
        tracing::debug!(config = ?config, "Building Universe");

        let db = database::build(&config.database).await;

        let _authorization = authorization::build();
        let users = users::build(db.clone());
        let authentication = authentication::build();
        let health = health::Builder::default().with_component("db", db).build();

        let server = server::Builder::default()
            .with_component(health)
            .with_component(users)
            .with_component(authentication)
            .build();

        tracing::debug!("Built Universe");

        Self { server }
    }

    /// Start the Universe service running.
    ///
    /// # Parameters
    /// - `port` - The port number to listen on.
    pub async fn start(self, port: u16) {
        tracing::debug!("Starting Universe");
        self.server.start(port).await;
    }
}
