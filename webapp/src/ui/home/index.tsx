import { Authentication } from "./authentication";
import { Landing } from "./landing";

export const HomePage = () => {
  return (
    <Landing>
      <Authentication />
    </Landing>
  );
};
