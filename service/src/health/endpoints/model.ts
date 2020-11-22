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

/**
 * Build the response to send
 * @param components The health of the various components of the system
 */
export function buildResponse(components: { [key: string]: ComponentHealth }): Response<SystemHealth> {
  const healthy = Object.values(components).every((component) => component.healthy);
  return {
    payload: {
      components,
      healthy
    },
    status: healthy ? 200 : 503
  };
}
