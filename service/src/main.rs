#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,
    /// The URL to connect to the database with
    pub database_url: String,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

impl Into<universe::Config> for Settings {
    fn into(self) -> universe::Config {
        universe::Config {
            database: universe::DatabaseConfig {
                url: self.database_url,
            },
        }
    }
}

/// Main entrypoint for the Universe application.
#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let settings = Settings::default();
    let port = settings.port.unwrap_or(8000);

    tracing::info!("Starting Universe");

    let service = universe::Service::new(settings.into()).await;
    service.start(port).await;
}
