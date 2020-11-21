import { EndpointConfig } from '../users/endpoints';
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

  return {
    endpoints: new EndpointConfig()
  };
}
