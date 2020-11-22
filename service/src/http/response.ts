import { Response } from 'express';

/**
 * Interface for payloads to indicate their HTTP status code.
 */
export interface StatusCode {
  /** The status code to return */
  statusCode: () => number;
}

/**
 * Type guard for the StatusCode interface.
 * @param payload The payload to check
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isStatusCode(payload: any): payload is StatusCode {
  return 'statusCode' in payload;
}

/**
 * Interface for payloads to indicate their HTTP content type.
 */
export interface ContentType {
  /** The content type to return */
  contentType: () => string;
}

/**
 * Type guard for the ContentType interface.
 * @param payload The payload to check
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isContentType(payload: any): payload is ContentType {
  return 'contentType' in payload;
}

/**
 * Send the given payload to the response.
 * @param res The express response object
 * @param payload The payload to send to the response
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/explicit-module-boundary-types
export function respond(res: Response, payload: any): void {
  if (isStatusCode(payload)) {
    res.status(payload.statusCode());
  }
  if (isContentType(payload)) {
    res.contentType(payload.contentType());
  }

  res.json(payload);
}
