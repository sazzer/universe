import * as z from "zod";

import { FormProvider, useForm } from "react-hook-form";

import { FormInput } from "../../components/form/input";
import React from "react";
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
  const { t } = useTranslation();
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

  const doSubmit = (data: AuthenticateAuthenticationForm) =>
    onSubmit(data.password);

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
            </fieldset>
          </form>
        </FormProvider>
      </div>
    </div>
  );
};
