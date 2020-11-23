import { Response } from './response';

export interface Problem {
  type: string;
  title: string;
  status: number;
}

/** Problem for when a resource is not found */
export const NOT_FOUND_PROBLEM: Response = {
  payload: {
    type: 'tag:universe,2020:problems/not_found',
    title: 'The requested resource was not found',
    status: 404
  },
  status: 404,
  contentType: 'application/problem+json'
};
