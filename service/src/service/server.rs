use crate::server::Server;

/// Build the HTTP Server component.
pub fn build() -> Server {
    tracing::debug!("Building HTTP Server");

    Server::new()
}
