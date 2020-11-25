mod database;
mod server;

use crate::database::Config as DatabaseConfig;
use crate::server::Server;

/// Configuration needed to build the service
#[derive(Debug)]
pub struct Config {
    /// Database connection configuration.
    pub database: DatabaseConfig,
}

/// The actual Universe service.
pub struct Service {
    server: Server,
}

impl Service {
    /// Construct a new instance of the Universe service.
    pub async fn new(config: Config) -> Self {
        tracing::debug!(config = ?config, "Building Universe");

        database::build(&config.database).await;

        let server = server::build();

        tracing::debug!("Built Universe");

        Self { server }
    }

    /// Start the Universe service running.
    pub async fn start(&self, port: u16) {
        tracing::debug!("Starting Universe");
        self.server.start(port).await;
    }
}
