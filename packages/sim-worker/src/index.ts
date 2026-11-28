// What the page may import from this package: the messages' types, the check of what the Worker sends, the cap on a
// content bundle, the map a local match is played on, the cut of a text to the bound the Worker keeps, and the ring of
// samples. The Worker is started from its own entry (`@shamble/sim-worker/worker`); nothing here loads the Wasm or its
// glue.
export { LOCAL_MAP } from "./local-map.ts";
export type { PageToWorker, WorkerStats, WorkerToPage } from "./messages.ts";
export { isWorkerToPage, MAX_BUNDLE_BYTES } from "./messages.ts";
export { cutText } from "./panic-message.ts";
export { Ring } from "./stats.ts";
