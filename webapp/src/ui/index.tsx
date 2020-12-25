import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { useQuery } from "react-query";
import { useApi } from "../api";

export const App = () => {
  const { t } = useTranslation();

  return (
    <div>
      {t("title")}
      <Suspense fallback={"Testing"}>
        <Authentication />
      </Suspense>
    </div>
  );
};

const Authentication = () => {
  const api = useApi().homeDocument;

  const authenticationDocument = useQuery("authenticationDocument", () =>
    api.entityLinks
      .find((link) => link.hasRel("tag:universe,2020:rels/authentication"))
      ?.fetch()
  );

  return <div>{JSON.stringify(authenticationDocument)}</div>;
};
