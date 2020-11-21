import bunyan from 'bunyan';

/**
 * Build a logger to log with
 * @param name The name of the logger
 */
export function newLogger(name: string) {
  let logLevelString = process.env.LOG_LEVEL;
  let logLevel: number | undefined = undefined;

  if (logLevelString !== undefined) {
    logLevel = bunyan.resolveLevel(logLevelString as bunyan.LogLevelString);
  }

  if (logLevel === undefined) {
    if (process.env.NODE_ENV !== 'production') {
      logLevel = bunyan.DEBUG;
    } else {
      logLevel = bunyan.INFO;
    }
  }

  return bunyan.createLogger({
    name,
    level: logLevel,
    src: true
  });
}
