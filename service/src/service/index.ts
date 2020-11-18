import { DatabaseConfig, buildDatabase } from './database';

import { TestableServer } from '../server/testing';
import { buildHealth } from './health';
import { buildServer } from './server';
import debug from 'debug';

const LOG = debug('universe:service');

/** Configuration needed for building the service */
export interface ServiceConfig {
  database: DatabaseConfig;
}

/** Service is the actual universe service */
export interface Service {
  server: TestableServer;

  /**
   * Start the service listening
   * @param port The port to start listening on
   */
  start(port: number): Promise<void>;

  /**
   * Shutdown the service
   */
  shutdown(): Promise<void>;
}

/**
 * Create a new service
 */
export async function newService(config: ServiceConfig): Promise<Service> {
  LOG('Building universe');

  const db = await buildDatabase(config.database);
  const health = buildHealth({ db: db.database });
  const server = buildServer([health.endpoints]);

  LOG('Built universe');

  return {
    server: server.server,

    start: async (port: number) => {
      LOG('Starting service');
      await server.server.start(port);
    },

    shutdown: async () => {
      LOG('Stopping servuce');
      await db.database.stop();
    }
  };
}
