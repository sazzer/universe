import { HealthChecker } from '../service';
import { Router } from 'express';
import { checkHealth } from './get';

/**
 * Configuration for the healthchecker endpoints
 */
export class EndpointConfig {
  /** The actual healthchecker service */
  private healthChecker: HealthChecker;

  /**
   * Constructor for the healthchecker endpoints
   * @param healthChecker The healthchecker service
   */
  constructor(healthChecker: HealthChecker) {
    this.healthChecker = healthChecker;
  }

  /**
   * Register the endpoints for the healthchecks
   * @param router The router to register endpoints on
   */
  configure(router: Router): void {
    router.get('/health', checkHealth(this.healthChecker));
  }
}
