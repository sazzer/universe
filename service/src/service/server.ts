import { Server } from '../server';

/**
 * The shape of the configuration for the server.
 */
export interface ServerComponent {
  server: Server;
}

/**
 * Build the configuration for the server.
 */
export function buildServer(): ServerComponent {
  return {
    server: new Server()
  };
}
