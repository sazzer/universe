import { Response } from '../../http/response';

/**
 * Representation of the health of a single component
 */
export interface ComponentHealth {
  /** Whether the component is healthy */
  healthy: boolean;
  /** A message about the component health */
  message?: string;
}

/**
 * Representation of the overall system health
 */
export interface SystemHealth {
  /** Whether the system is healthy */
  healthy: boolean;
  /** The components that make up the system */
  components: { [key: string]: ComponentHealth };
}

export class SystemHealthResponse extends Response<SystemHealth> {
  constructor(components: { [key: string]: ComponentHealth }) {
    super({
      components,
      healthy: Object.values(components).every((component) => component.healthy)
    });
    this.status = this.payload.healthy ? 200 : 503;
  }
}
