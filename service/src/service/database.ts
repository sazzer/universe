import { Database } from '../database';
import { migrate } from '../database/migrate';
import { newLogger } from '../logger';

const LOG = newLogger('universe:service');

/**
 * Required configuration settings for the database
 */
export interface DatabaseConfig {
  url: string;
}

/**
 * Service component for the database
 */
export interface DatabaseComponent {
  database: Database;
}

/**
 * Build the database component.
 * @param config The database configuration
 */
export async function buildDatabase(config: DatabaseConfig): Promise<DatabaseComponent> {
  LOG.debug('Building database connection');

  const database = new Database(config.url);
  await database.checkHealth();

  await migrate(database);

  return {
    database
  };
}
