mod authentication;
mod authorization;
mod database;
mod health;
mod server;
pub mod testing;
mod users;

pub use self::authentication::GoogleConfig;
use crate::database::Config as DatabaseConfig;
use crate::server::Server;

/// Configuration needed to build the service
#[derive(Debug)]
pub struct Config {
    /// Database connection configuration.
    pub database: DatabaseConfig,

    /// Google Authentication configuration.
    pub google: GoogleConfig,
}

/// The actual Universe service.
pub struct Service {
    server: Server,
}

impl Service {
    /// Construct a new instance of the Universe service.
    pub async fn new(config: Config) -> Self {
        tracing::debug!(config = ?config, "Building Universe");

        let db = database::build(&config.database).await;

        let _authorization = authorization::build();
        let users = users::build(db.clone());
        let authentication = authentication::builder().with_google(config.google).build();
        let health = health::builder().with_component("db", db).build();

        let server = server::builder()
            .with_component(health)
            .with_component(users)
            .with_component(authentication)
            .build();

        tracing::debug!("Built Universe");

        Self { server }
    }

    /// Start the Universe service running.
    pub async fn start(self, port: u16) {
        tracing::debug!("Starting Universe");
        self.server.start(port).await;
    }
}
