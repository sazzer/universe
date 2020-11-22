import { AuthenticationUserID, Email, ProviderID, UserID, UserModel, Username } from '../model';

/**
 * HTTP Model details for a single set of authentiaction details for a user
 */
export interface Authentication {
  /** The ID of the provider */
  provider: ProviderID;
  /** The ID of the user at this provider */
  userId: AuthenticationUserID;
  /** The display name of the user at this provider */
  displayName: string;
}

/**
 * HTTP Model details for a single user
 */
export class User {
  /** The ID of the user */
  readonly id: UserID;
  /** The display name of the user */
  readonly displayName: string;
  /** The email address of the user */
  readonly email?: Email;
  /** The username of the user */
  readonly username?: Username;
  /** The authentication details of the user */
  readonly authentications: Authentication[];

  constructor(user: UserModel) {
    this.id = user.identity.id;
    this.displayName = user.data.displayName;
    this.email = user.data.email;
    this.username = user.data.username;
    this.authentications = user.data.authentications.map((auth) => {
      return {
        provider: auth.provider,
        userId: auth.userId,
        displayName: auth.displayName
      };
    });
  }
}
