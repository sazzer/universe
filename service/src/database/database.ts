import { Pool } from 'pg';
import debug from 'debug';

const LOG = debug('universe:database');

/**
 * Wrapper around the database connection.
 */
export class Database {
  /** The actual connection pool */
  private pool: Pool;

  /**
   * Construct the database connection.
   * @param url The URL to connect to
   */
  constructor(url: string) {
    this.pool = new Pool({ connectionString: url });

    this.pool.on('error', (e) => {
      LOG('Received error from database connection pool: %o', e);
    });
  }

  /**
   * Stop the database connection pool.
   * USE WITH CARE.
   */
  async stop(): Promise<void> {
    LOG('Closing database connection pool');
    await this.pool.end();
  }

  /**
   * Check the health of the database.
   * If unhealthy then this will return a rejected promise.
   */
  async checkHealth(): Promise<void> {
    await this.pool.query('SELECT 1');
  }
}
