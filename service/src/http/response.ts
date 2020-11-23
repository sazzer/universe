import express from 'express';

/** Shape of a response object to send to the client */
export interface Response<T> {
  /** The actual response payload */
  payload: T;
  /** The status code */
  status?: number;
  /** The content type */
  contentType?: string;
  /** The period of time to cache for */
  cachePeriod?: number;
  /** The etag to use */
  etag?: string;
}

/**
 * Send a response to the client
 * @param res The express Response object
 * @param response The response to send to the client
 */
export function respond<T>(res: express.Response, response: Response<T>): void {
  if (response.etag !== undefined) {
    if (res.req?.header('if-none-match') === response.etag) {
      res.status(304);
      res.end();
      return;
    } else {
      res.setHeader('etag', response.etag);
    }
  }

  if (response.status !== undefined) {
    res.status(response.status);
  }
  if (response.contentType !== undefined) {
    res.contentType(response.contentType);
  }
  if (response.cachePeriod !== undefined) {
    res.setHeader('cache-control', `public,max-age=${response.cachePeriod}`);
  }
  res.json(response.payload);
}
