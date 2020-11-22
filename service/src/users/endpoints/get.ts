import { NOT_FOUND_PROBLEM } from '../../http/problem';
import { RequestHandler } from 'express';
import { parseUserID } from '../model';
import { respond } from '../../http/response';

/**
 * HTTP endpoint for getting a User by ID
 */
export function getUserByID(): RequestHandler {
  return async (req, res) => {
    const userId = parseUserID(req.params['userId'] || '');
    if (userId.isNone()) {
      return respond(res, NOT_FOUND_PROBLEM);
    }

    respond(res, {});
  };
}
