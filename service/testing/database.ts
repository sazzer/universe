import { GenericContainer, StartedTestContainer } from 'testcontainers';

import debug from 'debug';

const LOG = debug('integration:world:database');

const container = new GenericContainer('postgres', '12.4-alpine')
  .withExposedPorts(5432)
  .withEnv('POSTGRES_DB', 'universe-test')
  .withEnv('POSTGRES_USER', 'universe-test')
  .withEnv('POSTGRES_PASSWORD', 'universe-test');

let startedContainer: StartedTestContainer | undefined;

export async function startDatabase(): Promise<string> {
  if (startedContainer === undefined) {
    LOG('Starting test database');

    startedContainer = await container.start();
  }

  const host = startedContainer?.getHost();
  const port = startedContainer?.getMappedPort(5432);

  const url = `postgres://universe-test:universe-test@${host}:${port}/universe-test`;

  LOG('Started database: %s', url);

  return url;
}

export async function stopDatabase(): Promise<void> {
  if (startedContainer !== undefined) {
    LOG('Stopping test database');
    await startedContainer.stop();

    startedContainer = undefined;
  }
}
