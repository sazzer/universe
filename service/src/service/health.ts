import { EndpointConfig } from '../health/endpoints';
import debug from 'debug';

const LOG = debug('universe:service');

/**
 * Service component for the healthchecks
 */
export interface HealthComponent {
  endpoints: EndpointConfig;
}

/**
 * Build the healthchecks component.
 */
export function buildHealth(): HealthComponent {
  LOG('Building healthchecks');

  return {
    endpoints: new EndpointConfig()
  };
}
