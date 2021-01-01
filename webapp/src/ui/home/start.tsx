import React from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/** Shape of the form submission */
interface StartAuthenticationForm {
  username: string;
}

/** Shape of the props for starting authentication */
interface StartAuthenticationProps {
  onSubmit: (username: string) => Promise<void>;
}

/**
 * Component for starting authentication.
 */
export const StartAuthentication: React.FC<StartAuthenticationProps> = ({
  onSubmit,
}) => {
  const { t } = useTranslation();
  const {
    register,
    handleSubmit,
    formState,
  } = useForm<StartAuthenticationForm>();

  const doSubmit = (data: StartAuthenticationForm) => onSubmit(data.username);

  return (
    <div className="card">
      <div className="card-header">{t("authentication.start.title")}</div>
      <div className="card-body">
        <form onSubmit={handleSubmit(doSubmit)}>
          <fieldset disabled={formState.isSubmitting}>
            <div className="mb-3">
              <label htmlFor="username" className="form-label">
                {t("authentication.fields.username.label")}
              </label>
              <input
                ref={register({ required: true })}
                type="text"
                className="form-control"
                id="username"
                name="username"
                required
                autoFocus
              />
            </div>
            <button type="submit" className="btn btn-primary">
              {t("authentication.start.submit")}
            </button>
          </fieldset>
        </form>
      </div>
    </div>
  );
};
