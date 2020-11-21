import { EndpointConfig } from '../users/endpoints';
import debug from 'debug';

const LOG = debug('universe:service');

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
  LOG('Building users');

  return {
    endpoints: new EndpointConfig()
  };
}
