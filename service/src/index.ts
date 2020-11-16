import 'dotenv/config';

import * as env from 'env-var';

import debug from 'debug';

const LOG = debug('universe');

const config = {
  http: {
    port: env.get('PORT').default('8000').asPortNumber()
  }
};

LOG('Configuration: %o', config);
