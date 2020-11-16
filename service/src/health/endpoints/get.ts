import { RequestHandler } from 'express';
import { SystemHealth } from './model';
import { respond } from '../../http/response';

export function checkHealth(): RequestHandler {
  return (req, res) => {
    const result = new SystemHealth({
      db: {
        healthy: false,
        message: 'Oops'
      }
    });
    respond(res, result);
  };
}
