import { Router } from 'express';
import { checkHealth } from './get';

export class EndpointConfig {
  configure(router: Router) {
    router.get('/health', checkHealth());
  }
}
