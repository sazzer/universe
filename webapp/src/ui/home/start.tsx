import * as z from "zod";

import { FormProvider, useForm } from "react-hook-form";

import { FormInput } from "../../components/form/input";
import React from "react";
import { useTranslation } from "react-i18next";
import { zodResolver } from "@hookform/resolvers/zod";

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
  const schema = z.object({
    username: z.string().nonempty(),
  });

  const methods = useForm<StartAuthenticationForm>({
    resolver: zodResolver(schema),
  });

  const doSubmit = (data: StartAuthenticationForm) => onSubmit(data.username);

  return (
    <div className="card">
      <div className="card-header">{t("authentication.start.title")}</div>
      <div className="card-body">
        <FormProvider {...methods}>
          <form onSubmit={methods.handleSubmit(doSubmit)}>
            <fieldset disabled={methods.formState.isSubmitting}>
              <FormInput
                label={t("authentication.fields.username.label")}
                name="username"
                errorBuilder={(type) =>
                  t(`authentication.fields.username.errors.${type}`)
                }
                autoFocus
                required
              />
              <button type="submit" className="btn btn-primary">
                {t("authentication.start.submit")}
              </button>
            </fieldset>
          </form>
        </FormProvider>
      </div>
    </div>
  );
};
