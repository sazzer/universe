import bodyParser from 'body-parser';
import compression from 'compression';
import cors from 'cors';
import debug from 'debug';
import express from 'express';
import responseTime from 'response-time';

const LOG = debug('universe:server');

export interface EndpointConfig {
  configure: (router: express.Router) => void;
}

/**
 * Server represents the actual HTTP Server
 */
export class Server {
  private app: express.Application;
  /**
   * Construct the server
   */
  constructor(routes: EndpointConfig[]) {
    const app = express();

    app.use(responseTime());
    app.use(bodyParser.urlencoded({ extended: false }));
    app.use(bodyParser.json());
    app.use(compression());
    app.use(cors());

    routes.forEach((config) => config.configure(app));

    this.app = app;
  }

  /**
   * Start the HTTP server.
   * @param port The port to listen on
   */
  async start(port: number) {
    LOG('Starting server on port %d', port);
    await this.app.listen(port);
  }
}
