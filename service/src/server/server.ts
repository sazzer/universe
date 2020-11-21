import bodyParser from 'body-parser';
import compression from 'compression';
import cors from 'cors';
import express from 'express';
import { newLogger } from '../logger';
import responseTime from 'response-time';

const LOG = newLogger('universe:server');

/**
 * Interface that components able to contribute entpoints can implement.
 */
export interface EndpointConfig {
  configure: (router: express.Router) => void;
}

/**
 * Server represents the actual HTTP Server
 */
export class Server {
  protected app: express.Application;
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
  async start(port: number): Promise<void> {
    LOG.info({ port }, 'Starting server');
    await this.app.listen(port);
  }
}
