import { None, Option } from '@hqoss/monads';
import { UserID, UserModel } from '../model';

import { Database } from '../../database';
import { newLogger } from '../../logger';

/** The logger to use */
const LOG = newLogger('universe:users:repository');

export interface GetUsersRepository {
  /**
   * Get a single user from the database by ID
   * @param userId The ID of the user
   */
  getUserByID(userId: UserID): Promise<Option<UserModel>>;
}

/**
 * Build the repository for getting users by ID
 * @param database The database connection
 */
export function buildGetUserRepository(database: Database): GetUsersRepository {
  return {
    async getUserByID(userId: UserID): Promise<Option<UserModel>> {
      LOG.debug({ userId }, 'Getting user by ID');
      const row = await database.queryRow('SELECT * FROM users WHERE user_id = $1', [userId]);

      LOG.debug({ userId, row }, 'Fetched user by ID');

      return None;
    }
  };
}
