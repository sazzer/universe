import { Model } from '../model';

/** The ID of a user */
export type UserID = string;

/** The ID of an authentication provider */
export type ProviderID = string;

/** The ID of the user at the authentication provider */
export type AuthenticationUserID = string;

/** The authentication details of a user */
export interface Authentication {
  /** The provider that these details are for */
  provider: ProviderID;
  /** The ID of the user at this provider */
  userId: AuthenticationUserID;
  /** The display name of the user at this provider */
  displayName: string;
}

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
