import { Problem as KettingProblem } from "ketting";

/**
 * Representation of an RFC-7807 Problem.
 */
export class Problem {
  readonly type: string;
  readonly title: string;
  readonly status: number;
  readonly detail?: string;
  readonly instance?: string;
  readonly extra: Record<string, any>;

  constructor(source: KettingProblem) {
    const data = source.body as Record<string, any>;

    this.type = data.type ?? "about:blank";
    this.title = data.title ?? "";
    this.status = data.status ?? source.status;
    this.detail = data.detail;
    this.instance = data.instance;

    this.extra = data;
  }
}

/**
 * Actual error that is thrown when a Problem occurs.
 */
export class ProblemError extends Error {
  readonly problem: Problem;

  constructor(source: KettingProblem) {
    super(source.message);

    this.problem = new Problem(source);
    this.name = "ProblemError";
    this.stack = source.stack;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}
