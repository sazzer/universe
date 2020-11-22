import { Then } from '@cucumber/cucumber';
import { expect } from 'chai';
import { getLastResponse } from '../world/service';

Then('I get a {int} response', function (statusCode: number) {
  expect(getLastResponse().status).to.equal(statusCode);
});

Then(/I get an "application\/json" content of/, function (content: string) {
  const expected = JSON.parse(content);
  const actual = JSON.parse(getLastResponse().body);

  expect(actual).to.deep.equal(expected);
});
