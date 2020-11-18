import { EndpointConfig, Server } from './server';

import debug from 'debug';
import request from 'supertest';

const LOG = debug('universe:server:testing');

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
    LOG('Injecting request: %o', req);
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
