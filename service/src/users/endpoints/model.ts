import { AuthenticationUserID, Email, ProviderID, UserID, UserModel, Username } from '../model';

import { Response } from '../../http/response';

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
export interface User {
  /** The ID of the user */
  id: UserID;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user */
  email?: Email;
  /** The username of the user */
  username?: Username;
  /** The authentication details of the user */
  authentications: Authentication[];
}

/**
 * HTTP Model details of a user
 */
export class UserResponse extends Response<User> {
  constructor(user: UserModel) {
    super({
      id: user.identity.id,
      displayName: user.data.displayName,
      email: user.data.email,
      username: user.data.username,
      authentications: user.data.authentications.map((auth) => {
        return {
          provider: auth.provider,
          userId: auth.userId,
          displayName: auth.displayName
        };
      })
    });
  }
}
