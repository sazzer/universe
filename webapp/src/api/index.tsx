import { newClient, Resource } from "butes";
import { useQuery } from "react-query";

/** The Butes client to access the API with */
const client = newClient(async (url, request) => {
  console.log(url);
  console.log(request);
  try {
    return await fetch(url, request);
  } catch (e) {
    console.log(e);
    return null;
  }
});

/** The shape of the hook for accessing the API */
export interface ApiHook {
  homeDocument: Resource<void>;
}

/**
 * Hook function to get access to the API.
 */
export function useApi(): ApiHook {
  const homeDocument = useQuery("homeDocument", () =>
    client.get<void>("https://universe-cd.herokuapp.com")
  );

  return {
    homeDocument: homeDocument.data!!,
  };
}
