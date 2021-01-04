import * as z from "zod";

import { FormProvider, useForm } from "react-hook-form";
import React, { useState } from "react";

import { Alert } from "../../components/alerts";
import { FormInput } from "../../components/form/input";
import { ProblemError } from "../../api/problem";
import { useTranslation } from "react-i18next";
import { zodResolver } from "@hookform/resolvers/zod";

/** Shape of the form submission */
interface AuthenticateAuthenticationForm {
  username: string;
  password: string;
}

/** Shape of the props for authenticating as a known user */
interface AuthenticateAuthenticationProps {
  username: string;
  onCancel: () => void;
  onSubmit: (password: string) => Promise<void>;
}

/**
 * Component for authenticating as a known user.
 */
export const AuthenticateAuthentication: React.FC<AuthenticateAuthenticationProps> = ({
  username,
  onCancel,
  onSubmit,
}) => {
  const { t, i18n } = useTranslation();
  const [error, setError] = useState<string | null>(null);

  const schema = z.object({
    username: z.string().nonempty(),
    password: z.string().nonempty(),
  });

  const methods = useForm<AuthenticateAuthenticationForm>({
    resolver: zodResolver(schema),
    defaultValues: {
      username,
    },
  });

  const doSubmit = async (data: AuthenticateAuthenticationForm) => {
    setError(null);
    try {
      await onSubmit(data.password);
    } catch (e) {
      let errorCode = null;

      if (e instanceof ProblemError) {
        errorCode = `authentication.errors.${e.problem.type}`;
      }

      if (errorCode === null || !i18n.exists(errorCode)) {
        errorCode = "authentication.errors.unexpected_error";
      }

      setError(errorCode);
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        {t("authentication.authenticate.title")}
      </div>
      <div className="card-body">
        <FormProvider {...methods}>
          <form onSubmit={methods.handleSubmit(doSubmit)}>
            <fieldset disabled={methods.formState.isSubmitting}>
              <FormInput
                label={t("authentication.fields.username.label")}
                name="username"
                readOnly
              />
              <FormInput
                label={t("authentication.fields.password.label")}
                name="password"
                errorBuilder={(type) =>
                  t(`authentication.fields.password.errors.${type}`)
                }
                autoFocus
                required
              />
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
              {error && (
                <div className="mt-3">
                  <Alert message={t(error)} />
                </div>
              )}
            </fieldset>
          </form>
        </FormProvider>
      </div>
    </div>
  );
};
