import { UserModel, parseUserID } from '../model';

import { NOT_FOUND_PROBLEM } from '../../http/problem';
import { RequestHandler } from 'express';
import { UserResponse } from './model';

/**
 * HTTP endpoint for getting a User by ID
 */
export function getUserByID(): RequestHandler {
  return async (req, res) => {
    const userId = parseUserID(req.params['userId'] || '');
    if (userId.isNone()) {
      return NOT_FOUND_PROBLEM.send(res);
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

    new UserResponse(user).send(res);
  };
}
