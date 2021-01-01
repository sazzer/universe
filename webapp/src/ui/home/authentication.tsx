import {
  AuthenticationAction,
  useAuthenticationApi,
} from "../../api/authentication";
import React, { useState } from "react";

import { AuthenticateAuthentication } from "./authenticate";
import { StartAuthentication } from "./start";

/**
 * Component for the authentication form.
 */
export const Authentication: React.FC = () => {
  const api = useAuthenticationApi();
  const [state, setState] = useState<AuthenticationAction>(api.start);

  const doCancel = () => setState(api.start);

  if (state.action === "START") {
    const onSubmit = async (username: string) => {
      const result = await state.start(username);
      setState(result);
    };

    return <StartAuthentication onSubmit={onSubmit} />;
  } else if (state.action === "AUTHENTICATE") {
    return (
      <AuthenticateAuthentication
        username={state.username}
        onCancel={doCancel}
      />
    );
  } else {
    return <div>{state.action}</div>;
  }
};
