/**
 * Interface implemented by anything able to check it's health.
 */
export interface HealthCheck {
  /**
   * Check the health of the component.
   * Returns a resolved promise if healthy, or a rejected one if unhealthy.
   */
  checkHealth: () => Promise<void>;
}

/**
 * Representation of the health of a component
 */
export interface ComponentHealth {
  /** Whether the component is healthy or not */
  healthy: boolean;
  /** A message about the health */
  message?: string;
}

/**
 * Representation of the health of the entire system
 */
export interface SystemHealth {
  /** The components in the system */
  components: { [key: string]: ComponentHealth };
}
