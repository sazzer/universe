import { None, Option, Some } from '@hqoss/monads';
import { UserID, UserModel } from '../model';

import { GetUserUseCase } from '../usecases';

/**
 * Build the implementtation of the GetUserUseCase
 */
export function buildGetUserUseCase(): GetUserUseCase {
  return {
    async getUserById(userId: UserID): Promise<Option<UserModel>> {
      if (userId === 'a4dd5a3b-ed05-4a7a-a12c-e063ac3c25b2') {
        return None;
      }

      const user: UserModel = {
        identity: {
          id: userId,
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
      return Some(user);
    }
  };
}
