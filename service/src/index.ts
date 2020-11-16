import 'dotenv/config';

import * as env from 'env-var';

import debug from 'debug';
import { newService } from './service';

const LOG = debug('universe');

async function run() {
  const config = {
    http: {
      port: env.get('PORT').default('8000').asPortNumber()
    }
  };

  LOG('Configuration: %o', config);

  const service = await newService();
  await service.start(config.http.port);
}

run();
