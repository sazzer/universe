import { Router } from 'express';
import { UsersService } from '../service';
import { getUserByID } from './get';

/**
 * Configuration for the users endpoints
 */
export class EndpointConfig {
  /** The users service */
  private usersService: UsersService;

  constructor(usersService: UsersService) {
    this.usersService = usersService;
  }

  /**
   * Register the endpoints for working with users
   * @param router The router to register endpoints on
   */
  configure(router: Router): void {
    router.get('/users/:userId', getUserByID(this.usersService));
  }
}
