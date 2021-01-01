import { useApi } from ".";

/**
 * Base interface for all of the possible API Actions.
 */
export interface Action {
  action: string;
}

/**
 * Interface for the action to register a new user
 */
export interface RegisterAction extends Action {
  action: "REGISTER";
  username: string;
  register: (
    email: string,
    displayName: string,
    password: string
  ) => Promise<void>;
}

/**
 * Interface for the action to authenticate an existng user
 */
export interface AuthenticateAction extends Action {
  action: "AUTHENTICATE";
  username: string;
  authenticate: (password: string) => Promise<void>;
}

/**
 * Interface for the action to start authentication.
 */
export interface StartAction extends Action {
  action: "START";
  start: (username: string) => Promise<AuthenticateAction | RegisterAction>;
}

/** Possible actions for authentication */
export type AuthenticationAction =
  | StartAction
  | RegisterAction
  | AuthenticateAction;

/**
 * Interface for the API to authenticate users
 */
export interface AuthenticationApi {
  start: StartAction;
}

export function useAuthenticationApi(): AuthenticationApi {
  const api = useApi();

  return {
    start: {
      action: "START",
      start: async (username) => {
        const authResource = await api.follow(
          "tag:universe,2020:rels/authentication"
        );
        const response = await authResource.post({ data: { username } });

        if (
          response.links.has(
            "tag:universe,2020:rels/authentication/authenticate"
          )
        ) {
          // Known user
          return {
            action: "AUTHENTICATE",
            username,
            authenticate: async (password) => {
              console.log("Authenticating");
            },
          };
        } else {
          // Unknown user
          return {
            action: "REGISTER",
            username,
            register: async (email, displayName, password) => {
              console.log("Registering");
            },
          };
        }
      },
    },
  };
}
