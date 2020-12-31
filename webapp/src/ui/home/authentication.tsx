import { Action } from "butes";
import { useApi } from "../../api";
import { useQuery } from "react-query";
import { useState } from "react";
import { useTranslation } from "react-i18next";

/** Mapping of the action name to the fields that are understood in the order to display them */
const ORDERED_FIELDS: Record<string, string[]> = {
  "tag;universe,2020:actions/authentication/start": ["username"],
};

/**
 * Shape of the props that the AuthenticationForm component can handle
 */
interface AuthenticationFormProps {
  /** The name of the action */
  actionName: string;
  /** The action itself */
  action: Action;
  /** Callback to trigger when submitting the form */
  onSubmit: (values: Record<string, string>) => void;
}

/**
 * Component for rendering the authentication form itself, based off of a Butes Action provided to it.
 */
const AuthenticationForm: React.FC<AuthenticationFormProps> = ({
  actionName,
  action,
  onSubmit,
}) => {
  const { t } = useTranslation();

  const fields = action.fields;

  const fieldElements = ORDERED_FIELDS[actionName]
    ?.filter((field) => fields[field] !== undefined)
    ?.map((field, index) => {
      return (
        <div className="mb-3">
          <label htmlFor={field} className="form-label">
            {t(`authentication.fields.${field}.label`)}
          </label>
          <input
            type={fields[field].type}
            className="form-control"
            id={field}
            autoFocus={index === 0}
          />
        </div>
      );
    });

  return (
    <div className="card">
      <div className="card-header">
        {t(`authentication.${actionName}.title`)}
      </div>
      <div className="card-body">
        <form>
          {fieldElements}
          <button
            type="button"
            className="btn btn-primary"
            onClick={() => {
              onSubmit({ username: "graham" });
              return false;
            }}
          >
            {t(`authentication.${actionName}.submit`)}
          </button>
        </form>
      </div>
    </div>
  );
};

/**
 * Component to render the sidebar of the homepage for authenticating with the server.
 * Retrieves the appropriate resources from the server to describe the action and renders as appropriate.
 */
export const Authentication = () => {
  const api = useApi();

  const authenticationResource = useQuery("authenticationResource", () =>
    api.homeDocument.entityLinks
      .find((link) => link.hasRel("tag:universe,2020:rels/authentication"))
      ?.fetch()
  );

  const [actionName] = useState(
    "tag;universe,2020:actions/authentication/start"
  );
  const [action] = useState(
    authenticationResource.data?.actions[
      "tag:universe,2020:actions/authentication/start"
    ]
  );
  const submitCallback = (values: Record<string, string>) => {
    action?.submit(values).then((result) => {
      console.log(result);
    });
  };

  if (action !== undefined) {
    return (
      <AuthenticationForm
        action={action}
        actionName={actionName}
        onSubmit={submitCallback}
      />
    );
  }
  return null;
};
