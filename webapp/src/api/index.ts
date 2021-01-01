import { Client, Resource } from "ketting";

import env from "@beam-australia/react-env";
import { useQuery } from "react-query";

/**
 * Get access to the API.
 */
export function useApi(): Resource<void> {
  return useQuery<Resource<void>>("rootResource", async () => {
    const client = new Client(env("URL_BASE"));
    const rootRes = client.go("/");
    await rootRes.get();

    return rootRes;
  }).data!!;
}
