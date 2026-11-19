// What the page may import from this package: the messages' types and the statistics helpers. The Worker is
// started from its own entry (`@shamble/sim-worker/worker`); nothing here loads the Wasm or its glue.
export type { PageToWorker, WorkerStats, WorkerToPage } from "./messages.ts";
export { percentile, Ring } from "./stats.ts";
