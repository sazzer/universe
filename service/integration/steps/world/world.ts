import { Request, Response } from '../../../src/server/testing';
import { TestableService, newTestableService } from '../../../src/service/testing';
import { after, before, binding } from 'cucumber-tsflow';

import { TestDatabase } from './database';

/**
 * The World for the cucumber scenarios.
 */
export class World {
  /** The test database */
  private database?: TestDatabase;

  /** The service being tested */
  private service?: TestableService;

  /** The last response received from the server */
  private lastResponse?: Response;

  /**
   * Start the world
   */
  async start(): Promise<void> {
    this.database = new TestDatabase();
    await this.database?.start();
    this.service = await newTestableService({
      database: {
        url: this.database?.url
      }
    });
  }

  /**
   * Stop the world
   */
  async stop(): Promise<void> {
    await this.service?.shutdown();
    await this.database?.stop();
  }

  /**
   * Make an HTTP request to the service.
   * @param request The details of the request to make
   */
  async request(request: Request): Promise<void> {
    this.lastResponse = await this.service?.injectRequest(request);
  }

  /**
   * Get the most recently received response from the server
   */
  get response(): Response | undefined {
    return this.lastResponse;
  }
}

/**
 * Cucumber steps for managing the world
 */
@binding([World])
export class WorldSteps {
  /** The world */
  private world: World;

  constructor(world: World) {
    this.world = world;
  }

  /**
   * Start the world
   */
  @before()
  async setupWorld(): Promise<void> {
    await this.world.start();
  }

  /**
   * Stop the world
   */
  @after()
  async tearDownWorld(): Promise<void> {
    await this.world.stop();
  }
}
