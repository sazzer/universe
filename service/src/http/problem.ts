import { Response } from './response';

export interface Problem {
  type: string;
  title: string;
  status: number;
}

export class ProblemResponse extends Response<Problem> {
  constructor(type: string, title: string, status: number) {
    super({
      type,
      title,
      status
    });
    this.status = status;
    this.contentType = 'application/problem+json';
  }
}

/** Problem for when a resource is not found */
export const NOT_FOUND_PROBLEM = new ProblemResponse(
  'tag:universe,2020:problems/not_found',
  'The requested resource was not found',
  404
);
