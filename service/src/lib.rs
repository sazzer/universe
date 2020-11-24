#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod service;

/// Main entrypoint for the Universe application.
pub fn main() {
    tracing::info!("Starting Universe");

    let service = service::Service::new();
    service.start();
}
