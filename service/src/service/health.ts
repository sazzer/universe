import { EndpointConfig } from '../health/endpoints';
import { HealthCheck } from '../health';
import { HealthChecker } from '../health/service';
import { newLogger } from '../logger';

const LOG = newLogger('universe:service');

/**
 * Service component for the healthchecks
 */
export interface HealthComponent {
  endpoints: EndpointConfig;
}

/**
 * Build the healthchecks component.
 */
export function buildHealth(components: { [key: string]: HealthCheck }): HealthComponent {
  LOG.debug('Building healthchecks');

  const healthChecker = new HealthChecker(components);

  return {
    endpoints: new EndpointConfig(healthChecker)
  };
}
