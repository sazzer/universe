#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod server;
mod service;

/// Main entrypoint for the Universe application.
pub async fn main() {
    tracing::info!("Starting Universe");

    let service = service::Service::new().await;
    service.start().await;
}
