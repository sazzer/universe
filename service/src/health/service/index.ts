import { ComponentHealth, HealthCheck, SystemHealth } from '../model';

import { newLogger } from '../../logger';

const LOG = newLogger('universe:health:service');

/**
 * The HealthChecker is the service implementation for checking the health of the system.
 */
export class HealthChecker {
  /** The components to check */
  private components: { [key: string]: HealthCheck };

  /**
   * Construct the health checker.
   * @param components The components to check
   */
  constructor(components: { [key: string]: HealthCheck }) {
    this.components = components;
  }

  /**
   * Check the health of the entire system
   */
  async checkHealth(): Promise<SystemHealth> {
    const result: { [key: string]: ComponentHealth } = {};

    for (const [key, component] of Object.entries(this.components)) {
      try {
        await component.checkHealth();

        LOG.info({ key }, 'Component is healthy');

        result[key] = {
          healthy: true
        };
      } catch (e) {
        LOG.warn({ err: e, key }, 'Component is unhealthy');

        result[key] = {
          healthy: false,
          message: e?.message
        };
      }
    }

    return {
      components: result
    };
  }
}
