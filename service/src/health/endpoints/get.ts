import { ComponentHealth, SystemHealthResponse } from './model';

import { CheckHealthUseCase } from '../usecases';
import { RequestHandler } from 'express';

/**
 * HTTP endpoint for checking the health of the system
 * @param checkHealthUseCase Use case for checking the system health
 */
export function checkHealth(checkHealthUseCase: CheckHealthUseCase): RequestHandler {
  return async (req, res) => {
    const result = await checkHealthUseCase.checkHealth();

    const components: { [key: string]: ComponentHealth } = {};
    for (const [key, component] of Object.entries(result.components)) {
      components[key] = component;
    }

    new SystemHealthResponse(components).send(res);
  };
}
