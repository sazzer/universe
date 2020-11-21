import { EndpointConfig, Server } from '../../src/server';

import { newLogger } from '../../src/logger';
import request from 'supertest';

const LOG = newLogger('universe:server:testing');

/**
 * Details of an HTTP request that we can inject as part of a test.
 */
export interface Request {
  method: 'GET';
  url: string;
}

/**
 * Details of an HTTP response received as part of a test.
 */
export interface Response {
  status: number;
  body: string;
}

export class TestableServer extends Server {
  constructor(routes: EndpointConfig[]) {
    super(routes);
  }

  /**
   * Inject a request into the server and get the response.
   * @param req The request to inject
   */
  async injectRequest(req: Request): Promise<Response> {
    LOG.trace({ req }, 'Injecting request');
    const client = request(this.app);

    if (req.method === 'GET') {
      return await client.get(req.url).then((res) => {
        return {
          status: res.status,
          body: res.text
        };
      });
    } else {
      throw new Error('Unsupported HTTP method');
    }
  }
}
