import { Response, respond } from '../../http/response';

import { GetUserUseCase } from '../usecases';
import { NOT_FOUND_PROBLEM } from '../../http/problem';
import { RequestHandler } from 'express';
import { User } from './model';
import { parseUserID } from '../model';

/**
 * HTTP endpoint for getting a User by ID
 */
export function getUserByID(getUserUseCase: GetUserUseCase): RequestHandler {
  return async (req, res) => {
    const userId = parseUserID(req.params['userId'] || '');
    if (userId.isNone()) {
      return respond(res, NOT_FOUND_PROBLEM);
    }

    const user = await getUserUseCase.getUserById(userId.unwrap());

    const response = user
      .map((user) => {
        return {
          payload: new User(user),
          cachePeriod: 3600,
          etag: `"${user.identity.version}"`
        } as Response;
      })
      .unwrapOr(NOT_FOUND_PROBLEM);

    return respond(res, response);
  };
}
