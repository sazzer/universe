import { useTranslation } from "react-i18next";

export const Authentication = () => {
  const { t } = useTranslation();

  return (
    <div className="card">
      <div className="card-header">{t("authentication.start.title")}</div>
      <div className="card-body">
        <form>
          <div className="mb-3">
            <label htmlFor="username" className="form-label">
              {t("authentication.fields.username.label")}
            </label>
            <input
              type="text"
              className="form-control"
              id="username"
              autoFocus
            />
          </div>
          <button type="submit" className="btn btn-primary">
            {t("authentication.start.submit")}
          </button>
        </form>
      </div>
    </div>
  );
};
