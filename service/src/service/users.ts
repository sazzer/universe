import { Database } from '../database';
import { EndpointConfig } from '../users/endpoints';
import { buildUserRepository } from '../users/repository';
import { buildUsersService } from '../users/service';
import { newLogger } from '../logger';

const LOG = newLogger('universe:service');

/**
 * Service component for working with users
 */
export interface UsersComponent {
  endpoints: EndpointConfig;
}

/**
 * Build the users component.
 */
export function buildUsers(database: Database): UsersComponent {
  LOG.debug('Building users');

  const userRepository = buildUserRepository(database);
  const userService = buildUsersService(userRepository);

  return {
    endpoints: new EndpointConfig(userService)
  };
}
