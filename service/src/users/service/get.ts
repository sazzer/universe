import { UserID, UserModel } from '../model';

import { GetUserUseCase } from '../usecases';
import { GetUsersRepository } from '../repository/get';
import { Option } from '@hqoss/monads';

/**
 * Build the implementtation of the GetUserUseCase
 */
export function buildGetUserUseCase(userRepository: GetUsersRepository): GetUserUseCase {
  return {
    async getUserById(userId: UserID): Promise<Option<UserModel>> {
      return await userRepository.getUserByID(userId);
    }
  };
}
