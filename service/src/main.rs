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

    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_auth_uri: Option<String>,
    pub google_token_uri: Option<String>,
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

impl Into<universe_lib::Config> for Settings {
    fn into(self) -> universe_lib::Config {
        universe_lib::Config {
            database: universe_lib::DatabaseConfig {
                url: self.database_url,
            },
            google: universe_lib::GoogleConfig {
                google_client_id: self.google_client_id,
                google_client_secret: self.google_client_secret,
                google_auth_uri: self.google_auth_uri,
                google_token_uri: self.google_token_uri,
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

    let service = universe_lib::Service::new(settings.into()).await;
    service.start(port).await;
}
