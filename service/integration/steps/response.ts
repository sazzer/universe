import { binding, then } from 'cucumber-tsflow';

import { World } from './world';
import { expect } from 'chai';

@binding([World])
export class HttpResponseSteps {
  /** The world */
  private world: World;

  constructor(world: World) {
    this.world = world;
  }

  @then(/I get an "OK" response/)
  checkStatusCode(): void {
    expect(this.world.response?.status).to.equal(200);
  }

  @then(/I get an "application\/json" content of/)
  checkResponseBody(content: string): void {
    const expected = JSON.parse(content);
    const actual = JSON.parse(this.world.response?.body || '');

    expect(actual).to.deep.equal(expected);
  }
}
