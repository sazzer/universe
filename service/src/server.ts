import debug from 'debug';

const LOG = debug('universe:server');

/**
 * Server represents the actual HTTP Server
 */
export class Server {
  /**
   * Start the HTTP server.
   * @param port The port to listen on
   */
  async start(port: number) {
    LOG('Starting server on port %d', port);
  }
}
