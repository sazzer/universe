import express from 'express';

/**
 * Representation of an API resposne to send over HTTP
 */
export class Response<T> {
  /** The payload of the response */
  protected payload: T;

  /** The status code of the response */
  protected status = 200;

  /** The content type of the response */
  protected contentType = 'application/json';

  constructor(payload: T) {
    this.payload = payload;
  }

  /**
   * Send this API response to the client.
   * @param res The Express response to send this down
   */
  send(res: express.Response): void {
    res.status(this.status);
    res.contentType(this.contentType);
    res.json(this.payload);
  }
}
