/**
 * The messages between the page and the Worker. They are structured-clone data, checked where they arrive. No
 * message carries the match seed: the Worker draws it and keeps it (ADR 0004 part 8).
 */

/** The local match: the client endpoint and a local server in one Worker (ADR 0002 part 6). The only mode so far. */
export const MODE_LOCAL = 0;

/** From the page to the Worker. */
export type PageToWorker =
  /** Start the match. `padBytes` pads the block for the hand-off readings; the game itself sends 0. */
  | { readonly type: "start"; readonly padBytes: number }
  /**
   * The newest input, once per drawn frame: its sequence number, and the buffers the page is done with, which ride
   * this post back to the pool (they are in its transfer list). The view time of the drawn frame joins with the
   * change-set that uses it, as a tick and a fraction, never as the page's milliseconds.
   */
  | { readonly type: "input"; readonly sequence: number; readonly returned: ArrayBuffer[] }
  /** The tab was hidden or shown: a hidden tab pauses the local match. */
  | { readonly type: "visibility"; readonly hidden: boolean }
  /** Ask for the Worker's statistics. */
  | { readonly type: "report-request" };

/** From the Worker to the page. */
export type WorkerToPage =
  /** The Wasm is loaded and the match runs; every block will be `blockBytes` long. */
  | { readonly type: "ready"; readonly blockBytes: number; readonly tickHz: number }
  /**
   * One tick's block, transferred. `doneAt` is when the Worker finished the step, on the clock both sides share
   * (`performance.timeOrigin + performance.now()`).
   */
  | { readonly type: "block"; readonly buffer: ArrayBuffer; readonly doneAt: number }
  /** The Worker cannot go on: the Wasm trapped, or a message it got made no sense. */
  | { readonly type: "error"; readonly message: string }
  | { readonly type: "report"; readonly stats: WorkerStats };

/** The Worker's own numbers, for the readout and the copy report. Times are in milliseconds. */
export interface WorkerStats {
  readonly ticks: number;
  readonly wakes: number;
  readonly maxTicksInOneWake: number;
  readonly drops: number;
  readonly droppedMs: number;
  readonly pauses: number;
  /** How late tick starts were against their due times: the median, the 99th percentile and the worst kept. */
  readonly latenessP50: number;
  readonly latenessP99: number;
  readonly latenessMax: number;
  /** What the one copy of the block cost. */
  readonly copyP50: number;
  readonly copyP99: number;
  readonly blockBytes: number;
  readonly poolMisses: number;
  readonly poolRefused: number;
  /** Blocks whose buffer was still attached after it was posted: the transfer list was missing (PM-0002-6). */
  readonly undetached: number;
  readonly hardwareConcurrency: number;
  readonly crossOriginIsolated: boolean;
}

/**
 * Every field of `WorkerStats` with its kind: a field that joins the interface does not compile until it is named
 * here, so the check below cannot fall behind it.
 */
const STATS_FIELDS: { readonly [Name in keyof WorkerStats]: WorkerStats[Name] extends boolean ? "boolean" : "number" } = {
  ticks: "number",
  wakes: "number",
  maxTicksInOneWake: "number",
  drops: "number",
  droppedMs: "number",
  pauses: "number",
  latenessP50: "number",
  latenessP99: "number",
  latenessMax: "number",
  copyP50: "number",
  copyP99: "number",
  blockBytes: "number",
  poolMisses: "number",
  poolRefused: "number",
  undetached: "number",
  hardwareConcurrency: "number",
  crossOriginIsolated: "boolean",
};
/**
 * The same list as pairs, made once: the check runs on the page for every report and allocates nothing. Marked as
 * free of side effects, so the Worker's chunk, which never runs this check, does not carry the list.
 */
const STATS_KINDS: readonly (readonly [string, string])[] = /* @__PURE__ */ Object.entries(STATS_FIELDS);

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isCount(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0 && value <= 0xffff_ffff;
}

/**
 * Whether a message the Worker received is one the page may send. Anything else is answered with an `error`
 * message and never reaches the Wasm.
 */
export function isPageToWorker(value: unknown): value is PageToWorker {
  if (!isObject(value)) return false;
  switch (value["type"]) {
    case "start":
      return isCount(value["padBytes"]);
    case "input":
      return (
        isCount(value["sequence"]) &&
        Array.isArray(value["returned"]) &&
        value["returned"].every((buffer) => buffer instanceof ArrayBuffer)
      );
    case "visibility":
      return typeof value["hidden"] === "boolean";
    case "report-request":
      return true;
    default:
      return false;
  }
}

/** Whether a value is the Worker's statistics: every field there, each of its kind. A reading may be `NaN`. */
export function isWorkerStats(value: unknown): value is WorkerStats {
  if (!isObject(value)) return false;
  for (let i = 0; i < STATS_KINDS.length; i += 1) {
    const field = STATS_KINDS[i];
    if (field === undefined || typeof value[field[0]] !== field[1]) return false;
  }
  return true;
}

/**
 * Whether a message the page received is one the Worker may send. Anything else ends the match on the page: the
 * two sides are then from different builds, or the Worker's script is not ours.
 */
export function isWorkerToPage(value: unknown): value is WorkerToPage {
  if (!isObject(value)) return false;
  switch (value["type"]) {
    case "ready":
      return isCount(value["blockBytes"]) && isCount(value["tickHz"]);
    case "block":
      return value["buffer"] instanceof ArrayBuffer && typeof value["doneAt"] === "number" && Number.isFinite(value["doneAt"]);
    case "error":
      return typeof value["message"] === "string";
    case "report":
      return isWorkerStats(value["stats"]);
    default:
      return false;
  }
}
