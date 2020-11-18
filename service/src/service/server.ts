import { EndpointConfig } from '../server';
import { TestableServer } from '../server/testing';
import debug from 'debug';

const LOG = debug('universe:service');

/**
 * The shape of the configuration for the server.
 */
export interface ServerComponent {
  server: TestableServer;
}

/**
 * Build the configuration for the server.
 */
export function buildServer(endpoints: EndpointConfig[]): ServerComponent {
  LOG('Building HTTP server');

  return {
    server: new TestableServer(endpoints)
  };
}
