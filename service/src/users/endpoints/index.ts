import { Router } from 'express';
import { getUserByID } from './get';

/**
 * Configuration for the users endpoints
 */
export class EndpointConfig {
  /**
   * Register the endpoints for working with users
   * @param router The router to register endpoints on
   */
  configure(router: Router): void {
    router.get('/users/:userId', getUserByID());
  }
}
