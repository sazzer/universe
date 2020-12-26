import { Header } from "./header";
import { HomePage } from "./home";

export const App = () => {
  return (
    <>
      <Header />
      <div className="container-fluid pt-2">
        <HomePage />
      </div>
    </>
  );
};
