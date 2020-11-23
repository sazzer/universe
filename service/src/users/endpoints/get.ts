import { UserModel, parseUserID } from '../model';

import { NOT_FOUND_PROBLEM } from '../../http/problem';
import { RequestHandler } from 'express';
import { User } from './model';
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

    const user: UserModel = {
      identity: {
        id: userId.unwrap(),
        version: '287d73ef-674f-42b9-85c7-0ab986431f6a',
        created: new Date(),
        updated: new Date()
      },
      data: {
        displayName: 'Test User',
        email: 'testuser@example.com',
        username: 'testuser',
        authentications: [
          {
            provider: 'google',
            userId: '12345678',
            displayName: 'testuser@example.com'
          },
          {
            provider: 'twitter',
            userId: 'abcdefgh',
            displayName: '@testuser'
          }
        ]
      }
    };

    respond(res, {
      payload: new User(user),
      cachePeriod: 3600,
      etag: `"${user.identity.version}"`
    });
  };
}
