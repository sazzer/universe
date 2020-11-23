import { UserID, UserModel } from './model';

import { Option } from '@hqoss/monads';

/**
 * Use case for getting individual users
 */
export interface GetUserUseCase {
  /**
   * Get a single User by ID
   * @param userId The ID of the user
   */
  getUserById(userId: UserID): Promise<Option<UserModel>>;
}
