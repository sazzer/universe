import React from "react";
import { useFormContext } from "react-hook-form";

export interface FormInputProps {
  label: string;
  name: string;
  type?: string;
  errorBuilder?: (type: string) => string;
  // All other props
  [x: string]: any;
}

export const FormInput: React.FC<FormInputProps> = ({
  label,
  name,
  type,
  errorBuilder,
  ...props
}) => {
  const { register, formState } = useFormContext();
  const error = formState.errors[name];
  const isError = error !== undefined;
  const errorMessageId = name + "ValidationFeedback";

  return (
    <div className="mb-3">
      <label htmlFor={name} className="form-label">
        {label}
      </label>
      <div className="input-group has-validation">
        <input
          ref={register}
          type={type ?? "text"}
          className={["form-control", isError ? "is-invalid" : ""].join(" ")}
          id={name}
          name={name}
          aria-describedby={errorMessageId}
          {...props}
        />
        {error && errorBuilder && (
          <div id={errorMessageId} className="invalid-feedback">
            {error.type === "custom_error"
              ? error.message
              : errorBuilder(error.type)}
          </div>
        )}
      </div>
    </div>
  );
};
