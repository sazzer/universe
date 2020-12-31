import React from "react";
import { StartAuthentication } from "./start";
import { useApi } from "../../api";

/**
 * Component for the authentication form.
 */
export const Authentication: React.FC = () => {
  useApi();

  return <StartAuthentication />;
};
