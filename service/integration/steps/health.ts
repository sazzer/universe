import { When } from '@cucumber/cucumber';
import { injectRequest } from '../world/service';

When(/I get the system health/, async function () {
  await injectRequest({ method: 'GET', url: '/health' });
});
