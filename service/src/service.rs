use crate::server::Server;

/// The actual Universe service.
pub struct Service {
    server: Server,
}

impl Service {
    /// Construct a new instance of the Universe service.
    pub async fn new() -> Self {
        tracing::debug!("Building Universe");
        let server = Server::new();
        tracing::debug!("Built Universe");

        Self { server }
    }

    /// Start the Universe service running.
    pub async fn start(&self) {
        tracing::debug!("Starting Universe");
        self.server.start().await;
    }
}
