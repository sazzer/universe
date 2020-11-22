import * as uuid from 'uuid';

import { None, Option, Some } from '@hqoss/monads';

import { newLogger } from '../../logger';

const LOG = newLogger('universe:users:model');

/** The ID of a user */
export type UserID = string;

/**
 * Generate a new User ID
 */
export function newUserID(): UserID {
  return uuid.v4();
}

/**
 * Parse an input string as a User ID, failing if it's not valid
 * @param input The input to parse
 */
export function parseUserID(input: string): Option<UserID> {
  const processed = input.trim().toLowerCase();

  if (!uuid.validate(processed)) {
    LOG.warn({ input }, 'Malformed User ID');
    return None;
  }

  return Some(processed as UserID);
}
