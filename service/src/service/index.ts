import { DatabaseConfig, buildDatabase } from './database';
import { EndpointConfig, Server } from '../server';

import { buildHealth } from './health';
import { buildServer } from './server';
import { buildUsers } from './users';
import debug from 'debug';

const LOG = debug('universe:service');

/** Configuration needed for building the service */
export interface ServiceConfig {
  database: DatabaseConfig;
}

/** Service is the actual universe service */
export interface Service<S extends Server> {
  server: S;

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
export async function newService<S extends Server>(
  config: ServiceConfig,
  buildServerFunc: (endpoints: EndpointConfig[]) => S = buildServer
): Promise<Service<S>> {
  LOG('Building universe');

  const db = await buildDatabase(config.database);
  const health = buildHealth({ db: db.database });
  const users = buildUsers();
  const server = buildServerFunc([health.endpoints, users.endpoints]);

  LOG('Built universe');

  return {
    server,

    start: async (port: number) => {
      LOG('Starting service');
      await server.start(port);
    },

    shutdown: async () => {
      LOG('Stopping service');
      await db.database.stop();
    }
  };
}
