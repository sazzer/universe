/// The actual Universe service.
pub struct Service {}

impl Service {
    /// Construct a new instance of the Universe service.
    pub fn new() -> Self {
        tracing::debug!("Building Universe");
        tracing::debug!("Built Universe");
        Self {}
    }

    /// Start the Universe service running.
    pub fn start(&self) {
        tracing::debug!("Starting Universe");
    }
}
