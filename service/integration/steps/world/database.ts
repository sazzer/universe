import { GenericContainer, StartedTestContainer } from 'testcontainers';

import debug from 'debug';

const LOG = debug('integration:world:database');

/**
 * Wrapper around a Postgres database for testing.
 */
export class TestDatabase {
  /** The docker container for the database */
  private container: GenericContainer;

  /** The started container */
  private startedContainer?: StartedTestContainer;

  constructor() {
    this.container = new GenericContainer('postgres', '12.4-alpine')
      .withExposedPorts(5432)
      .withEnv('POSTGRES_DB', 'universe-test')
      .withEnv('POSTGRES_USER', 'universe-test')
      .withEnv('POSTGRES_PASSWORD', 'universe-test');
  }
  /**
   * Start the database
   */
  async start(): Promise<void> {
    LOG('Starting test database');

    this.startedContainer = await this.container.start();
    LOG('Started database: %s', this.url);
  }

  /**
   * Stop the database
   */
  async stop(): Promise<void> {
    LOG('Stopping test database');

    await this.startedContainer?.stop();
  }

  /**
   * Get the URL to connect to the database on.
   */
  get url(): string {
    const host = this.startedContainer?.getHost();
    const port = this.startedContainer?.getMappedPort(5432);

    return `postgres://universe-test:universe-test@${host}:${port}/universe-test`;
  }
}
