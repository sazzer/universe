import { Request, Response } from './testServer';
import { TestableService, newTestableService } from './testService';

import { newLogger } from '../../src/logger';

const LOG = newLogger('integration:world:service');

let service: TestableService | undefined;

let lastResponse: Response | undefined;

export async function startService(databaseUrl: string): Promise<void> {
  if (service === undefined) {
    LOG.debug('Building test service');
    service = await newTestableService({
      database: {
        url: databaseUrl
      }
    });
  }
}

export async function stopService(): Promise<void> {
  if (service !== undefined) {
    LOG.debug('Stopping test service');
    await service.shutdown();
    service = undefined;
  }
}

export async function injectRequest(request: Request): Promise<void> {
  if (service === undefined) {
    throw new Error('Service is not started');
  }

  lastResponse = await service.injectRequest(request);
}

export function getLastResponse(): Response {
  if (lastResponse === undefined) {
    throw new Error('No last response recorded');
  }

  return lastResponse;
}
