import React from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/** Shape of the form submission */
interface AuthenticateAuthenticationForm {
  username: string;
  password: string;
}

/** Shape of the props for authenticating as a known user */
interface AuthenticateAuthenticationProps {
  username: string;
  onCancel: () => void;
}

/**
 * Component for authenticating as a known user.
 */
export const AuthenticateAuthentication: React.FC<AuthenticateAuthenticationProps> = ({
  username,
  onCancel,
}) => {
  const { t } = useTranslation();
  const {
    register,
    handleSubmit,
    formState,
  } = useForm<AuthenticateAuthenticationForm>({
    defaultValues: {
      username,
    },
  });

  const doSubmit = (data: AuthenticateAuthenticationForm) => console.log(data);

  return (
    <div className="card">
      <div className="card-header">
        {t("authentication.authenticate.title")}
      </div>
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
                readOnly
              />
            </div>
            <div className="mb-3">
              <label htmlFor="password" className="form-label">
                {t("authentication.fields.password.label")}
              </label>
              <input
                ref={register({ required: true })}
                type="password"
                className="form-control"
                id="password"
                name="password"
                required
                autoFocus
              />
            </div>
            <div className="btn-group" role="group">
              <button type="submit" className="btn btn-primary">
                {t("authentication.authenticate.submit")}
              </button>
              <button
                type="button"
                className="btn btn-outline-secondary"
                onClick={onCancel}
              >
                {t("authentication.authenticate.cancel")}
              </button>
            </div>
          </fieldset>
        </form>
      </div>
    </div>
  );
};
