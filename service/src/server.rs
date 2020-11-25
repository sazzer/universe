/// The wrapper around the HTTP Server
pub struct Server {}

impl Server {
    /// Construct a new instance of the Universe HTTP Server.
    pub const fn new() -> Self {
        Self {}
    }

    /// Start the Universe HTTP Server listening for requests.
    pub async fn start(&self, port: u16) {
        tracing::debug!({ port }, "Starting HTTP Server");
    }
}
