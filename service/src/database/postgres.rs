use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;

/// Configuration for the database connections.
#[derive(Debug)]
pub struct Config {
    /// The database connection url.
    pub url: String,
}

/// Wrapper around the database connections.
pub struct Database {
    /// The actual database connection pool.
    pub(super) pool: Pool,
}

impl Database {
    /// Construct a new database connection.
    pub fn new(config: &Config) -> Self {
        tracing::debug!(config = ?config, "Building database connection");

        let pg_config = tokio_postgres::Config::from_str(&config.url).unwrap();

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        Self { pool }
    }

    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    pub async fn checkout(
        &self,
    ) -> Result<Object<ClientWrapper, tokio_postgres::Error>, PoolError<tokio_postgres::Error>>
    {
        let conn = self.pool.get().await?;

        Ok(conn)
    }
}
