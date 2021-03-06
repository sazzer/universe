import {
  AuthenticationAction,
  useAuthenticationApi,
} from "../../api/authentication";
import React, { useState } from "react";

import { AuthenticateAuthentication } from "./authenticate";
import { RegisterAuthentication } from "./register";
import { StartAuthentication } from "./start";

/**
 * Component for the authentication form.
 */
export const Authentication: React.FC = () => {
  const api = useAuthenticationApi();
  const [state, setState] = useState<AuthenticationAction>(api.start);

  const doCancel = () => setState(api.start);

  switch (state.action) {
    case "START": {
      const onSubmit = async (username: string) => {
        const result = await state.start(username);
        setState(result);
      };

      return <StartAuthentication onSubmit={onSubmit} />;
    }
    case "AUTHENTICATE": {
      const onSubmit = async (password: string) => {
        await state.authenticate(password);
      };

      return (
        <AuthenticateAuthentication
          username={state.username}
          onCancel={doCancel}
          onSubmit={onSubmit}
        />
      );
    }
    case "REGISTER": {
      const onSubmit = async (
        email: string,
        displayName: string,
        password: string
      ) => {
        await state.register(email, displayName, password);
      };

      return (
        <RegisterAuthentication
          username={state.username}
          onCancel={doCancel}
          onSubmit={onSubmit}
        />
      );
    }
  }
};
