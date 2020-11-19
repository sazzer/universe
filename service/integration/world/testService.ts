import { Request, Response, TestableServer } from './testServer';
import { Service, ServiceConfig, newService } from '../../src/service';

/**
 * Extension of the Service to allow request injection for tests
 */
export interface TestableService extends Service<TestableServer> {
  /**
   * Inject a request into the server and get the response.
   * @param request The request to inject
   */
  injectRequest(request: Request): Promise<Response>;
}

/**
 * Create a new testable service
 */
export async function newTestableService(config: ServiceConfig): Promise<TestableService> {
  const service = await newService(config, (endpoints) => new TestableServer(endpoints));

  return {
    injectRequest: async (request: Request) => {
      return await service.server.injectRequest(request);
    },
    ...service
  };
}
