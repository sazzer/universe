import { Pool } from 'pg';

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
  }

  /**
   * Check the health of the database.
   * If unhealthy then this will return a rejected promise.
   */
  async checkHealth() {
    await this.pool.query('SELECT 1');
  }
}
