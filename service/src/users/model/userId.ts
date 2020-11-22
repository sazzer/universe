import * as uuid from 'uuid';

import { None, Option, Some } from '@hqoss/monads';

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
    // Invalid
    return None;
  }

  return Some(processed as UserID);
}
