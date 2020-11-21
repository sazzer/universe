import { Client, Database } from './database';

import fs from 'fs';
import { newLogger } from '../logger';
import path from 'path';

const LOG = newLogger('universe:database:migrate');

const MIGRATION_BASE = './migrations';

/**
 * Migrate the database to the latest schema version
 * @param database The database to migrate
 */
export async function migrate(database: Database): Promise<void> {
  LOG.info('Migrating database');

  // Get a new database transaction
  await database.begin(async (client) => {
    // Create the migrations table
    await lockMigrationsTable(client);

    // List the migration files that have been run
    const applied = await listAppliedMigrations(client);
    LOG.trace({ applied }, 'Applied files');

    // List the migration files to run
    const available = await listAvailableMigrations();
    LOG.trace({ available }, 'Available files');

    const toApply = available.filter((file) => applied.indexOf(file) === -1);
    LOG.trace({ toApply }, 'Files to apply');

    // Actually run the files that haven't yet been run
    for (const file of toApply) {
      LOG.trace({ file }, 'Applying file');
      await applyFile(client, file);
    }
  });

  LOG.info('Migrated database');
}

/**
 * Create and lock the migrations table used to record which migrations have been applied
 * @param client The database client
 */
async function lockMigrationsTable(client: Client) {
  await client.query(`CREATE TABLE IF NOT EXISTS __migrations(
    migration_file TEXT PRIMARY KEY,
    sequence SERIAL NOT NULL,
    executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    executed_from TEXT NOT NULL DEFAULT inet_client_addr()
  )`);

  LOG.trace('Locking migrations table');
  await client.query('LOCK TABLE __migrations IN EXCLUSIVE MODE');
  LOG.trace('Locked migrations table');
}

/**
 * Generate a list of migrations that have previously been applied
 * @param client The database client
 */
async function listAppliedMigrations(client: Client): Promise<string[]> {
  const result = await client.query('SELECT migration_file FROM __migrations');
  const rows = result.rows.map((row) => row['migration_file']);
  return rows;
}

/**
 * Generate a list of migrations that are available to apply
 */
async function listAvailableMigrations(): Promise<string[]> {
  const files = await fs.promises.readdir(MIGRATION_BASE);
  files.sort();
  return files;
}

/**
 * Actually apply a migration file
 * @param client The database client
 * @param file The name of the migration file
 */
async function applyFile(client: Client, file: string): Promise<void> {
  const fullFile = path.join(MIGRATION_BASE, file);
  const contents = await fs.promises.readFile(fullFile, {
    flag: 'r',
    encoding: 'utf-8'
  });

  await client.query(contents);
  await client.query('INSERT INTO __migrations(migration_file) VALUES ($1)', [file]);
}
