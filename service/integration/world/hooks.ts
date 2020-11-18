import { After, Before, setDefaultTimeout } from '@cucumber/cucumber';
import { startDatabase, stopDatabase } from './database';
import { startService, stopService } from './service';

import debug from 'debug';

const LOG = debug('integration:world');

setDefaultTimeout(60 * 1000);

Before(async () => {
  LOG('Setting up world');
  const databaseUrl = await startDatabase();
  await startService(databaseUrl);
});

After(async () => {
  LOG('Destroying world');
  try {
    await stopService();
  } finally {
    await stopDatabase();
  }
});
