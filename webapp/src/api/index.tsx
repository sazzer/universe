import { Client } from "ketting";
import env from "@beam-australia/react-env";
import { useQuery } from "react-query";

/** The actual Ketting client */
const client = new Client(env("URL_BASE"));
const rootRes = client.go("/");
const rootData = rootRes.get();
/**
 * Get access to the API.
 */
export function useApi() {
  return useQuery("rootResource", async () => {
    await rootData;

    return rootRes;
  });
}
