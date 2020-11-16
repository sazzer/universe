import 'dotenv/config';

import * as env from 'env-var';

import debug from 'debug';
import { newService } from './service';

const LOG = debug('universe');

async function run() {
  const config = {
    http: {
      port: env.get('PORT').default('8000').asPortNumber()
    },
    database: {
      url: env.get('DATABASE_URL').required().asUrlString()
    }
  };

  LOG('Configuration: %o', config);

  const service = await newService(config);
  await service.start(config.http.port);
}

run();
