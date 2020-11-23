import { EndpointConfig } from '../users/endpoints';
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
export function buildUsers(): UsersComponent {
  LOG.debug('Building users');

  const userService = buildUsersService();

  return {
    endpoints: new EndpointConfig(userService)
  };
}
