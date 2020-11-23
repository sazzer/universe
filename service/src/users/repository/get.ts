import { None, Option } from '@hqoss/monads';
import { UserID, UserModel } from '../model';

import { Database } from '../../database';

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
      return None;
    }
  };
}
