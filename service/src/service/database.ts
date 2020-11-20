import { Database } from '../database';
import debug from 'debug';
import { migrate } from '../database/migrate';

const LOG = debug('universe:service');

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
  LOG('Building database connection');

  const database = new Database(config.url);
  await database.checkHealth();

  await migrate(database);

  return {
    database
  };
}
