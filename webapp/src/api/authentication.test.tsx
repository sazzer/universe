import * as sut from "./authentication";

import { QueryClient, QueryClientProvider } from "react-query";
import React, { Suspense } from "react";
import { act, renderHook } from "@testing-library/react-hooks";

import nock from "nock";

test("Initial Action", async () => {
  const optionsScope = nock("http://universe.example.com")
    .options("/")
    .reply(200, undefined, {
      "Access-Control-Allow-Methods":
        "PUT, OPTIONS, CONNECT, PATCH, GET, HEAD, POST, DELETE, TRACE",
      "Access-Control-Allow-Origin": "http://localhost",
      "Access-Control-Expose-Headers": "link, etag, location",
      "Access-Control-Allow-Headers": "user-agent",
    });

  const getScope = nock("http://universe.example.com")
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
      {
        "Access-Control-Allow-Methods":
          "PUT, OPTIONS, CONNECT, PATCH, GET, HEAD, POST, DELETE, TRACE",
        "Access-Control-Allow-Origin": "http://localhost",
        "Access-Control-Expose-Headers": "link, etag, location",
        "Access-Control-Allow-Headers": "user-agent",
      }
    );

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        suspense: true,
      },
    },
  });
  const { result, waitFor } = renderHook(sut.useAuthenticationApi, {
    wrapper: ({ children }) => (
      <QueryClientProvider client={queryClient}>
        <Suspense fallback={""}>{children}</Suspense>
      </QueryClientProvider>
    ),
  });

  await waitFor(() => {
    expect(getScope.isDone()).toBe(true);
  });

  expect(optionsScope.isDone()).toBe(true);
  expect(getScope.isDone()).toBe(true);

  expect(result.current.start.action === "START");
});
