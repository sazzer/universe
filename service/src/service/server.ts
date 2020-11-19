import { EndpointConfig } from '../server';
import { Server } from '../server';
import debug from 'debug';

const LOG = debug('universe:service');

/**
 * Build the server.
 */
export function buildServer<S extends Server>(endpoints: EndpointConfig[]): S {
  LOG('Building HTTP server');

  return new Server(endpoints) as S;
}
