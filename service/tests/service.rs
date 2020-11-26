use actix_http::Request;
use universe_lib::{Config, DatabaseConfig, Service, TestResponse};
use universe_testdatabase::TestDatabase;

/// Wrapper around the service being tested
pub struct TestService {
    #[allow(dead_code)]
    db: TestDatabase,
    #[allow(dead_code)]
    service: Service,
}

impl TestService {
    /// Construct a new test service
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let db = TestDatabase::default();

        let config = Config {
            database: DatabaseConfig {
                url: db.url.clone(),
            },
        };

        let service = Service::new(config).await;

        Self { db, service }
    }

    /// Inject a request into the service and get the response back.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
