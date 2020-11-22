import { When } from '@cucumber/cucumber';
import { injectRequest } from '../world/service';

When('I get the user with ID {string}', async function (userId: string) {
  await injectRequest({ method: 'GET', url: '/users/{userId}', params: { userId } });
});
