import { useTranslation } from "react-i18next";

export const Spinner = () => {
  const { t } = useTranslation();

  return (
    <div className="position-relative">
      <div className="position-absolute start-50 translate-middle-x">
        <div
          className="spinner-border"
          style={{ width: "7rem", height: "7rem" }}
          role="status"
        ></div>
        <div className="pt-8 position-absolute start-50 translate-middle-x">
          {t("spinner.label")}
        </div>
      </div>
    </div>
  );
};
