import { startDatabase, stopDatabase } from '../../testing/database';

import { Database } from './database';
import { migrate } from './migrate';

describe('Database Migrations', () => {
  let db: Database | undefined;

  beforeEach(async () => {
    const url = await startDatabase();
    db = new Database(url);
  }, 60000);

  afterEach(async () => {
    db?.stop();
    await stopDatabase();
  }, 60000);

  test('Migrate database', async () => {
    if (db === undefined) {
      throw new Error('Database not available');
    }
    await migrate(db);
  });
});
