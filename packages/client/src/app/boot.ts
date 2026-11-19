import SimWorker from "@shamble/sim-worker/worker?worker";
import { FrameTimes } from "../harness/frame-times.ts";
import { runHeader } from "../harness/run-header.ts";
import type { ReportData } from "../overlay/report.ts";
import { createThreePicture } from "../render/three-picture.ts";
import { installErrorHook } from "./error-hook.ts";
import { readFlags } from "./flags.ts";
import { WorkerLink } from "./worker-link.ts";

// Set by the build (scripts/build.mjs). In a production build the first is false, and everything behind it,
// the readout and the report among it, is left out of the bundle.
const DEVTOOLS = import.meta.env["VITE_SHAMBLE_DEVTOOLS"] === "1";
const COMMIT: string = import.meta.env["VITE_SHAMBLE_COMMIT"] ?? "unknown";

/** The readout is rewritten this often, never every frame. */
const READOUT_EVERY_MS = 250;

/**
 * The app shell's boot order: the picture first (`await renderer.init()` before the first frame), then the Worker,
 * then the frame loop. Each drawn frame posts the newest input once and draws from the two newest blocks. The page
 * never loads or calls the Wasm: the Worker does.
 */
export async function boot(): Promise<void> {
  const notice = document.getElementById("notice");
  if (notice !== null) notice.hidden = true;
  const canvas = document.getElementById("view");
  if (!(canvas instanceof HTMLCanvasElement)) throw new Error("the page has no canvas to draw on");
  const flags = readFlags(location.search, DEVTOOLS);
  const errors = installErrorHook(window);

  const picture = createThreePicture();
  const info = await picture.start(canvas, { forceWebGL: flags.forceWebGL });
  // The backend that really runs is named in the log and on the page itself, in every build (ADR 0003 part 4).
  console.info(`Shamble Frontlines ${COMMIT}: backend ${info.backend}${info.adapter === "" ? "" : ` (${info.adapter})`}`);
  document.documentElement.dataset["backend"] = info.backend;
  document.documentElement.dataset["commit"] = COMMIT;

  const worker = new SimWorker();
  const link = new WorkerLink(worker, () => performance.timeOrigin + performance.now());
  worker.addEventListener("message", (event: MessageEvent<unknown>) => {
    const problem = link.onMessage(event.data);
    if (problem !== "") errors.note(problem);
  });
  worker.addEventListener("error", (event) => errors.note(event.message === "" ? "the Worker failed" : event.message));
  link.start(flags.padBytes);

  const frames = DEVTOOLS ? new FrameTimes() : null;
  const resize = (): void => picture.resize(window.innerWidth, window.innerHeight, Math.min(window.devicePixelRatio, 2));
  window.addEventListener("resize", resize);
  resize();
  document.addEventListener("visibilitychange", () => {
    link.visibility(document.hidden);
    frames?.skip();
  });
  if (document.hidden) link.visibility(true);

  let readout: ((timeMs: number) => void) | null = null;
  if (DEVTOOLS && frames !== null) {
    const [{ Overlay }, { formatReport, readoutLines }] = await Promise.all([
      import("../overlay/overlay.ts"),
      import("../overlay/report.ts"),
    ]);
    const data = (): ReportData => ({
      header: runHeader(window, COMMIT, flags.runLabel),
      picture: info,
      counters: picture.counters(),
      tick: link.tick,
      tickHz: link.tickHz,
      blocks: link.blocks,
      blockBytes: link.blockBytes,
      posts: link.posts,
      frames: frames.frames,
      fps: frames.fps(),
      frameP50: frames.percentile(50),
      frameP99: frames.percentile(99),
      deliveryP50: link.delivery.percentile(50),
      deliveryP95: link.delivery.percentile(95),
      errors: errors.log.count,
      lastError: errors.log.last,
      pauses: link.pauses,
      lastPause: link.lastPause,
      worker: link.stats,
    });
    const overlay = new Overlay(document.body, () => formatReport(data()));
    // The browser tests read the same numbers the readout shows.
    (window as unknown as { __shambleReport: () => ReportData }).__shambleReport = data;
    let next = 0;
    readout = (timeMs): void => {
      if (timeMs < next) return;
      next = timeMs + READOUT_EVERY_MS;
      link.requestReport();
      overlay.update(readoutLines(data()));
    };
  }

  let sequence = 0;
  picture.loop((timeMs) => {
    sequence = (sequence + 1) >>> 0;
    link.frame(sequence, timeMs);
    picture.draw({ newest: link.newest, previous: link.previous, timeMs });
    frames?.push(timeMs);
    readout?.(timeMs);
  });
}
