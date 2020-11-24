/// The wrapper around the HTTP Server
pub struct Server {}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    pub fn new() -> Self {
        tracing::debug!("Built HTTP Server");
        Self {}
    }

    /// Start the Universe HTTP Server listening for requests.
    pub fn start(&self) {
        tracing::debug!("Starting HTTP Server");
    }
}
