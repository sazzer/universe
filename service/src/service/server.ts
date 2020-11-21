import { EndpointConfig } from '../server';
import { Server } from '../server';
import { newLogger } from '../logger';

const LOG = newLogger('universe:service');

/**
 * Build the server.
 */
export function buildServer<S extends Server>(endpoints: EndpointConfig[]): S {
  LOG.debug('Building HTTP server');

  return new Server(endpoints) as S;
}
