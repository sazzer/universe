import React from "react";

/**
 * Props for the Alert component
 */
export interface AlertProps {
  type?:
    | "primary"
    | "secondary"
    | "success"
    | "danger"
    | "warniing"
    | "info"
    | "light"
    | "dark";
  message: string;
  heading?: string;
}

/**
 * Component for displaying an alert widget.
 */
export const Alert: React.FC<AlertProps> = ({ type, message, heading }) => {
  return (
    <div className={`alert alert-${type ?? "danger"}`} role="alert">
      {heading !== undefined && <h4 className="alert-heading">{heading}</h4>}
      {message}
    </div>
  );
};
