import { EndpointConfig, Server } from '../../src/server';

import { newLogger } from '../../src/logger';
import request from 'supertest';
import template from 'url-template';

const LOG = newLogger('universe:server:testing');

/**
 * Details of an HTTP request that we can inject as part of a test.
 */
export interface Request {
  method: 'GET';
  url: string;
  params?: { [key: string]: any }; // eslint-disable-line @typescript-eslint/no-explicit-any
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

    const url = template.parse(req.url).expand(req.params);

    if (req.method === 'GET') {
      return await client.get(url).then((res) => {
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
