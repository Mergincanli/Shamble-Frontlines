// What the page may import from this package: the messages' types, the check of what the Worker sends, and the ring
// of samples. The Worker is started from its own entry (`@shamble/sim-worker/worker`); nothing here loads the Wasm
// or its glue.
export type { PageToWorker, WorkerStats, WorkerToPage } from "./messages.ts";
export { isWorkerToPage } from "./messages.ts";
export { Ring } from "./stats.ts";
