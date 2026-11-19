/** The page's errors so far: how many, and the newest one's text. */
export interface ErrorLog {
  count: number;
  last: string;
}

/**
 * Counts every error the page does not handle: script errors and promises nobody caught. The readout shows the
 * count, and a test fails on any. Other code adds its own (a Worker's `error` message) through `note`.
 */
export function installErrorHook(target: Window): { readonly log: ErrorLog; note(message: string): void } {
  const log: ErrorLog = { count: 0, last: "" };
  const note = (message: string): void => {
    log.count += 1;
    log.last = message;
  };
  target.addEventListener("error", (event) => note(event.message));
  target.addEventListener("unhandledrejection", (event) => {
    const reason: unknown = event.reason;
    note(reason instanceof Error ? reason.message : String(reason));
  });
  return { log, note };
}
