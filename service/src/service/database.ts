import debug from 'debug';

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
export interface DatabaseComponent {}

/**
 * Build the database component.
 * @param config The database configuration
 */
export function buildDatabase(config: DatabaseConfig): DatabaseComponent {
  LOG('Building database connection');

  return {};
}
