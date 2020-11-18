import { Request, Response } from '../server/testing';
import { Service, ServiceConfig, newService } from './index';

/**
 * Extension of the Service to allow request injection for tests
 */
export interface TestableService extends Service {
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
  const service = await newService(config);

  return {
    injectRequest: async (request: Request) => {
      return await service.server.injectRequest(request);
    },
    ...service
  };
}
