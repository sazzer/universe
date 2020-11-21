import 'dotenv/config';

import * as env from 'env-var';

import { newLogger } from './logger';
import { newService } from './service';

const LOG = newLogger('universe');

async function run() {
  const config = {
    http: {
      port: env.get('PORT').default('8000').asPortNumber()
    },
    database: {
      url: env.get('DATABASE_URL').required().asUrlString()
    }
  };

  LOG.trace({ config }, 'Configuration');

  const service = await newService(config);
  await service.start(config.http.port);
}

run();
