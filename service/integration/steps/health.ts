import { binding, when } from 'cucumber-tsflow';

import { World } from './world';

/**
 * Cucumber steps for working with healthchecks
 */
@binding([World])
export class HealthSteps {
  /** The world */
  private world: World;

  constructor(world: World) {
    this.world = world;
  }

  @when(/I get the system health/)
  async checkHealth(): Promise<void> {
    await this.world.request({ method: 'GET', url: '/health' });
  }
}
