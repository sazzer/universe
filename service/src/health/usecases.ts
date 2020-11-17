import { SystemHealth } from './model';

/**
 * Use case for checking the system health
 */
export interface CheckHealthUseCase {
  /**
   * Check the health of the system
   */
  checkHealth: () => Promise<SystemHealth>;
}
