import express from 'express';

/** Shape of a response object to send to the client */
export interface Response<T> {
  /** The actual response payload */
  payload: T;
  /** The status code */
  status?: number;
  /** The content type */
  contentType?: string;
}

/**
 * Send a response to the client
 * @param res The express Response object
 * @param response The response to send to the client
 */
export function respond<T>(res: express.Response, response: Response<T>): void {
  if (response.status !== undefined) {
    res.status(response.status);
  }
  if (response.contentType !== undefined) {
    res.contentType(response.contentType);
  }
  res.json(response.payload);
}
