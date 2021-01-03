import * as z from "zod";

import { FormProvider, useForm } from "react-hook-form";

import { FormInput } from "../../components/form/input";
import React from "react";
import { useTranslation } from "react-i18next";
import { zodResolver } from "@hookform/resolvers/zod";

/** Shape of the form submission */
interface RegisterAuthenticationForm {
  username: string;
  email: string;
  displayName: string;
  password: string;
  password2: string;
}

/** Shape of the props for registering as a new user */
interface RegisterAuthenticationProps {
  username: string;
  onCancel: () => void;
}

/**
 * Component for registering as a new user.
 */
export const RegisterAuthentication: React.FC<RegisterAuthenticationProps> = ({
  username,
  onCancel,
}) => {
  const { t } = useTranslation();
  const schema = z
    .object({
      username: z.string().nonempty(),
      email: z.string().nonempty().email(),
      displayName: z.string().nonempty(),
      password: z.string().nonempty(),
      password2: z.string().nonempty(),
    })
    .refine((val) => val.password === val.password2, {
      message: t("authentication.fields.password2.errors.mismatch"),
      path: ["password2"],
    });

  const methods = useForm<RegisterAuthenticationForm>({
    resolver: zodResolver(schema),
    defaultValues: {
      username,
    },
  });

  const doSubmit = (data: RegisterAuthenticationForm) => console.log(data);

  return (
    <div className="card">
      <div className="card-header">{t("authentication.register.title")}</div>
      <div className="card-body">
        <FormProvider {...methods}>
          <form onSubmit={methods.handleSubmit(doSubmit)}>
            <fieldset disabled={methods.formState.isSubmitting}>
              <FormInput
                label={t("authentication.fields.username.label")}
                name="username"
                type="text"
                readOnly
              />
              <FormInput
                label={t("authentication.fields.email.label")}
                name="email"
                type="email"
                errorBuilder={(type) =>
                  t(`authentication.fields.email.errors.${type}`)
                }
                required
                autoFocus
              />
              <FormInput
                label={t("authentication.fields.displayName.label")}
                name="displayName"
                type="displayName"
                errorBuilder={(type) =>
                  t(`authentication.fields.displayName.errors.${type}`)
                }
                required
              />
              <FormInput
                label={t("authentication.fields.password.label")}
                name="password"
                type="password"
                errorBuilder={(type) =>
                  t(`authentication.fields.password.errors.${type}`)
                }
                required
              />
              <FormInput
                label={t("authentication.fields.password2.label")}
                name="password2"
                type="password"
                errorBuilder={(type) =>
                  t(`authentication.fields.password2.errors.${type}`)
                }
                required
              />
              <div className="btn-group" role="group">
                <button type="submit" className="btn btn-primary">
                  {t("authentication.register.submit")}
                </button>
                <button
                  type="button"
                  className="btn btn-outline-secondary"
                  onClick={onCancel}
                >
                  {t("authentication.register.cancel")}
                </button>
              </div>
            </fieldset>
          </form>
        </FormProvider>
      </div>
    </div>
  );
};
