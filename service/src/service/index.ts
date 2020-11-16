import { DatabaseConfig, buildDatabase } from './database';

import { buildServer } from './server';
import debug from 'debug';

const LOG = debug('universe:service');

/** Configuration needed for building the service */
export interface ServiceConfig {
  database: DatabaseConfig;
}

/** Service is the actual universe service */
export interface Service {
  /**
   * Start the service listening
   * @param port The port to start listening on
   */
  start(port: number): Promise<void>;
}

/**
 * Create a new service
 */
export async function newService(config: ServiceConfig): Promise<Service> {
  LOG('Building universe');

  const db = await buildDatabase(config.database);
  const server = buildServer();

  LOG('Built universe');

  return {
    start: async (port: number) => {
      LOG('Starting service');
      await server.server.start(port);
    }
  };
}
