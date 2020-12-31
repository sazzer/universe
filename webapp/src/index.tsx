import "./i18n";

import { QueryClient, QueryClientProvider } from "react-query";
import React, { Suspense } from "react";

import { App } from "./ui";
import { BrowserRouter } from "react-router-dom";
import ReactDOM from "react-dom";
import reportWebVitals from "./reportWebVitals";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      suspense: true,
    },
  },
});

ReactDOM.render(
  <React.StrictMode>
    <Suspense fallback={"Loading..."}>
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </QueryClientProvider>
    </Suspense>
  </React.StrictMode>,
  document.getElementById("root")
);

reportWebVitals();
