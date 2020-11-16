import { Response } from 'express';

/**
 * Interface for payloads to indicate their HTTP status code.
 */
export interface StatusCode {
  /** The status code to return */
  statusCode: () => number;
}

/**
 * Type guard for the Status Code interface.
 * @param payload The payload to check
 */
function isStatusCode(payload: any): payload is StatusCode {
  return 'statusCode' in payload;
}

/**
 * Send the given payload to the response.
 * @param res The express response object
 * @param payload The payload to send to the response
 */
export function respond(res: Response, payload: any) {
  if (isStatusCode(payload)) {
    res.status(payload.statusCode());
  }

  res.json(payload);
}
