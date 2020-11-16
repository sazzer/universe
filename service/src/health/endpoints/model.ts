/**
 * Representation of the health of a single component
 */
export class ComponentHealth {
  /** Whether the component is healthy */
  healthy: boolean;
  /** A message about the component health */
  message?: string;

  constructor(healthy: boolean, message?: string) {
    this.healthy = healthy;
    this.message = message;
  }
}

/**
 * Representation of the overall system health
 */
export class SystemHealth {
  /** Whether the system is healthy */
  healthy: boolean;
  /** The components that make up the system */
  components: { [key: string]: ComponentHealth };

  constructor(components: { [key: string]: ComponentHealth }) {
    this.components = components;
    this.healthy = Object.values(components).every((component) => component.healthy);
  }

  /** The status code to use */
  statusCode() {
    return this.healthy ? 200 : 503;
  }
}
