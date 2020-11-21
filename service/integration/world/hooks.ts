import { After, Before, setDefaultTimeout } from '@cucumber/cucumber';
import { startDatabase, stopDatabase } from '../../testing/database';
import { startService, stopService } from './service';

import { newLogger } from '../../src/logger';

const LOG = newLogger('integration:world');

setDefaultTimeout(60 * 1000);

Before(async () => {
  LOG.debug('Setting up world');
  const databaseUrl = await startDatabase();
  await startService(databaseUrl);
});

After(async () => {
  LOG.debug('Destroying world');
  try {
    await stopService();
  } finally {
    await stopDatabase();
  }
});
