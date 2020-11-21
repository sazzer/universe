import { ClientBase, Pool } from 'pg';

import { newLogger } from '../logger';

export { ClientBase as Client };

const LOG = newLogger('universe:database');

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
      LOG.error(e, 'Received error from database connection pool');
    });
  }

  /**
   * Stop the database connection pool.
   * USE WITH CARE.
   */
  async stop(): Promise<void> {
    LOG.info('Closing database connection pool');
    await this.pool.end();
  }

  /**
   * Check the health of the database.
   * If unhealthy then this will return a rejected promise.
   */
  async checkHealth(): Promise<void> {
    await this.pool.query('SELECT 1');
  }

  /**
   * Execute some code inside a transaction.
   * This will do a COMMIT if the callback is successful, or a ROLLBACK if it fails.
   */
  async begin<T>(callback: (client: ClientBase) => Promise<T>): Promise<T> {
    const client = await this.pool.connect();
    try {
      LOG.trace('Starting transaction');
      await client.query('BEGIN');
      const result = await callback(client);

      LOG.trace('Commiting transaction');
      await client.query('COMMIT');

      return result;
    } catch (e) {
      LOG.error(e, 'Error during transaction');

      await client.query('ROLLBACK');

      throw e;
    } finally {
      await client.release();
    }
  }
}
