// The Worker's entry: it loads the Wasm, draws the match seed and hands everything else to the host (host.ts). The
// page never touches the Wasm: it starts this Worker, posts its newest input each frame and receives one block per
// tick (ADR 0002 part 5).
import init, { panic_buffer, Sim, tick_hz } from "../pkg/sim_wasm.js";
import { SimHost } from "./host.ts";
import { isPageToWorker, MODE_LOCAL, type WorkerToPage } from "./messages.ts";
import { readPanicMessage } from "./panic-message.ts";
import type { LinearMemory } from "./read-block.ts";

let host: SimHost | undefined;
let starting = false;
let hiddenWhileLoading = false;
/** The Wasm's memory and its panic buffer, once it is loaded: a trap while the match is made is read from them. */
let loaded: { memory: LinearMemory; panicBuffer: number } | undefined;

function tell(message: WorkerToPage, transfer: ArrayBuffer[] = []): void {
  postMessage(message, transfer);
}

async function start(padBytes: number): Promise<void> {
  const wasm = await init();
  const wasmSide = { memory: wasm.memory, panicBuffer: panic_buffer() };
  loaded = wasmSide;
  // The seed is drawn here and goes nowhere else: no message carries it and nothing logs it (ADR 0004 part 8).
  const seed = crypto.getRandomValues(new Uint8Array(32));
  const sim = new Sim(seed, MODE_LOCAL, padBytes, performance.now());
  seed.fill(0);
  host = new SimHost(
    sim,
    {
      ...wasmSide,
      now: () => performance.now(),
      timeOrigin: performance.timeOrigin,
      post: tell,
      setTimer: (run, delayMs) => setTimeout(run, delayMs),
      clearTimer: (id) => clearTimeout(id),
      hardwareConcurrency: navigator.hardwareConcurrency,
      crossOriginIsolated,
    },
    tick_hz(),
  );
  host.start();
  if (hiddenWhileLoading) host.onMessage({ type: "visibility", hidden: true });
}

addEventListener("message", (event: MessageEvent<unknown>) => {
  const data = event.data;
  if (host !== undefined) {
    host.onMessage(data);
    return;
  }
  if (!isPageToWorker(data)) {
    tell({ type: "error", message: "the Worker got a message it does not know" });
  } else if (data.type === "start" && !starting) {
    starting = true;
    start(data.padBytes).catch((error: unknown) => {
      const panic = loaded === undefined ? "" : readPanicMessage(loaded.memory, loaded.panicBuffer);
      const thrown = error instanceof Error ? error.message : String(error);
      tell({ type: "error", message: panic === "" ? thrown : `the simulation stopped: ${panic}` });
    });
  } else if (data.type === "visibility") {
    // The tab may be hidden while the Wasm loads: the match then starts paused.
    hiddenWhileLoading = data.hidden;
  }
});
