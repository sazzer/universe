/**
 * The identity of some model
 */
export interface Identity<I> {
  /** The actual ID */
  id: I;
  /** The version tag */
  version: string;
  /** When the model was created */
  created: Date;
  /** When the model was last updated */
  updated: Date;
}

/**
 * Model representation of a persisted record
 */
export interface Model<I, D> {
  /** The identity of the record */
  identity: Identity<I>;
  /** The data of the record */
  data: D;
}
