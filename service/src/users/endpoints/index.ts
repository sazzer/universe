import { Router } from 'express';

/**
 * Configuration for the users endpoints
 */
export class EndpointConfig {
  /**
   * Register the endpoints for working with users
   * @param router The router to register endpoints on
   */
  configure(router: Router): void {
    router.toString();
  }
}
