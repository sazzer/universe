import { GetUsersRepository, buildGetUserRepository } from './get';

import { Database } from '../../database';

/** Type representing the entire user repository */
export type UserRepository = GetUsersRepository;

/**
 * Build the users repository
 * @param database The database connection
 */
export function buildUserRepository(database: Database): UserRepository {
  return {
    ...buildGetUserRepository(database)
  };
}
