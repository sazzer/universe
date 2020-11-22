import { Authentication } from './authentication';
import { Model } from '../../model';
import { UserID } from './userId';

/** An email address of a user */
export type Email = string;

/** The username of a user */
export type Username = string;

/** The actual data for a user */
export interface UserData {
  /** The username of the user */
  username?: Username;
  /** The email address of the user */
  email?: Email;
  /** The display name of the user */
  displayName: string;
  /** The authentication details of the user */
  authentications: Authentication[];
}

/** Model representation of a persisted user */
export type UserModel = Model<UserID, UserData>;
