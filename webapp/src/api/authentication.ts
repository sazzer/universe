import Client, { Problem } from "ketting";

import { ProblemError } from "./problem";
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
        const startAuthResource = await api.follow(
          "tag:universe,2020:rels/authentication"
        );
        const response = await startAuthResource.post({ data: { username } });
        if (
          response.links.has(
            "tag:universe,2020:rels/authentication/authenticate"
          )
        ) {
          return buildAuthenticateAction(
            response.client,
            response.links.get(
              "tag:universe,2020:rels/authentication/authenticate"
            )?.href!!,
            username
          );
        } else {
          return buildRegisterAction(
            response.client,
            response.links.get("tag:universe,2020:rels/authentication/register")
              ?.href!!,
            username
          );
        }
      },
    },
  };
}

/**
 * Helper to build the action for authenticating as an existing user
 * @param client The Ketting client to use
 * @param url The URL to send the data to
 * @param username The username to authenticate as
 */
function buildAuthenticateAction(
  client: Client,
  url: string,
  username: string
): AuthenticateAction {
  return {
    action: "AUTHENTICATE",
    username,
    authenticate: async (password) => {
      const authResource = client.go(url);

      try {
        const authResponse = await authResource.post({
          data: {
            username,
            password,
          },
        });
        console.log(authResponse);
      } catch (e) {
        if (e instanceof Problem) {
          throw new ProblemError(e);
        } else {
          throw e;
        }
      }
    },
  };
}

/**
 * Helper to build the action for registering as a new user
 * @param client The Ketting client to use
 * @param url The URL to send the data to
 * @param username The username to authenticate as
 */
function buildRegisterAction(
  client: Client,
  url: string,
  username: string
): RegisterAction {
  return {
    action: "REGISTER",
    username,
    register: async (email, displayName, password) => {
      const authResource = client.go(url);
      try {
        const authResponse = await authResource.post({
          data: {
            username,
            email,
            displayName,
            password,
          },
        });
        console.log(authResponse);
      } catch (e) {
        if (e instanceof Problem) {
          throw new ProblemError(e);
        } else {
          throw e;
        }
      }
    },
  };
}
