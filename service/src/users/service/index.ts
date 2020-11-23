import { GetUserUseCase } from '../usecases';
import { buildGetUserUseCase } from './get';

export type UsersService = GetUserUseCase;

/**
 * Build the Users Service, which is the amalgamation of the various use cases
 */
export function buildUsersService(): UsersService {
  return {
    ...buildGetUserUseCase()
  };
}
