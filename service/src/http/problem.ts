import { timeStamp } from 'console';

/**
 * Representation of an RFC-7807 Problem
 */
export class Problem {
  /** The problem type */
  readonly type: string;

  /** The problem title */
  readonly title: string;

  /** The problem status code */
  readonly status: number;

  constructor(type: string, title: string, status: number) {
    this.type = type;
    this.title = title;
    this.status = status;
  }

  /** The status code to use */
  statusCode(): number {
    return this.status;
  }

  /** The content-type to use */
  contentType(): string {
    return 'application/problem+json';
  }
}

/** Problem for when a resource is not found */
export const NOT_FOUND_PROBLEM = new Problem(
  'tag:universe,2020:problems/not_found',
  'The requested resource was not found',
  404
);
