import * as sut from "./authentication";

import { QueryCache, QueryClient, QueryClientProvider } from "react-query";

import React from "react";
import nock from "nock";
import { renderHook } from "@testing-library/react-hooks";

const corsHeaders = {
  "Access-Control-Allow-Methods":
    "PUT, OPTIONS, CONNECT, PATCH, GET, HEAD, POST, DELETE, TRACE",
  "Access-Control-Allow-Origin": "http://localhost",
  "Access-Control-Expose-Headers": "link, etag, location",
  "Access-Control-Allow-Headers": "user-agent",
};

function setupHomeDocument() {
  return nock("http://universe.example.com")
    .options("/")
    .reply(200, undefined, corsHeaders)
    .get("/")
    .reply(
      200,
      {
        _links: {
          "tag:universe,2020:rels/authentication": {
            href: "/authentication",
          },
        },
      },
      corsHeaders
    );
}

test("Initial Action", async () => {
  const getScope = setupHomeDocument();

  const queryCache = new QueryCache();
  queryCache.clear();
  const queryClient = new QueryClient({ queryCache });
  const wrapper: React.FC = ({ children }) => {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );
  };
  const { result, waitFor } = renderHook(sut.useAuthenticationApi, { wrapper });

  await waitFor(() => {
    expect(getScope.isDone()).toBe(true);
  });

  expect(result.current.start.action).toBe("START");
});

test("Submit unknown user", async () => {
  const getScope = setupHomeDocument();

  const submitScope = nock("http://universe.example.com")
    .options("/authentication")
    .reply(200, undefined, corsHeaders)
    .post("/authentication", { username: "unknownUser" })
    .reply(
      200,
      {
        _links: {
          self: {
            href: "/authentication",
          },
          "tag:universe,2020:rels/authentication/register": {
            href: "/authentication/register",
          },
        },
      },
      corsHeaders
    );

  const queryCache = new QueryCache();
  queryCache.clear();
  const queryClient = new QueryClient({ queryCache });
  const wrapper: React.FC = ({ children }) => {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );
  };
  const { result, waitFor } = renderHook(sut.useAuthenticationApi, { wrapper });

  await waitFor(() => {
    expect(getScope.isDone()).toBe(true);
  });

  expect(result.current.start.action).toBe("START");

  const action = await result.current.start.start("unknownUser");
  expect(submitScope.isDone()).toBe(true);

  expect(action.action).toBe("REGISTER");
  expect(action.username).toBe("unknownUser");
});

test("Submit known user", async () => {
  const getScope = setupHomeDocument();

  const submitScope = nock("http://universe.example.com")
    .options("/authentication")
    .reply(200, undefined, corsHeaders)
    .post("/authentication", { username: "knownUser" })
    .reply(
      200,
      {
        _links: {
          self: {
            href: "/authentication",
          },
          "tag:universe,2020:rels/authentication/authenticate": {
            href: "/authentication/authenticate",
          },
        },
      },
      corsHeaders
    );

  const queryCache = new QueryCache();
  queryCache.clear();
  const queryClient = new QueryClient({ queryCache });
  const wrapper: React.FC = ({ children }) => {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );
  };
  const { result, waitFor } = renderHook(sut.useAuthenticationApi, { wrapper });

  await waitFor(() => {
    expect(getScope.isDone()).toBe(true);
  });

  expect(result.current.start.action).toBe("START");

  const action = await result.current.start.start("knownUser");
  expect(submitScope.isDone()).toBe(true);

  expect(action.action).toBe("AUTHENTICATE");
  expect(action.username).toBe("knownUser");
});
