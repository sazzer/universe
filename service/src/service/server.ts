import { EndpointConfig, Server } from '../server';

import debug from 'debug';

const LOG = debug('universe:service');

/**
 * The shape of the configuration for the server.
 */
export interface ServerComponent {
  server: Server;
}

/**
 * Build the configuration for the server.
 */
export function buildServer(endpoints: EndpointConfig[]): ServerComponent {
  LOG('Building HTTP server');

  return {
    server: new Server(endpoints)
  };
}
