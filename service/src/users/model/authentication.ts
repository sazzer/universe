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
