import { buildServer } from './server';
import debug from 'debug';

const LOG = debug('universe:service');

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
export async function newService(): Promise<Service> {
  LOG('Building universe');
  const server = buildServer();

  LOG('Built universe');

  return {
    start: async (port: number) => {
      LOG('Starting service');
      await server.server.start(port);
    }
  };
}
