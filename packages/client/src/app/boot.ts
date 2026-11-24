import SimWorker from "@shamble/sim-worker/worker?worker";
import { FrameTimes } from "../harness/frame-times.ts";
import { runHeader } from "../harness/run-header.ts";
import type { ReportData } from "../overlay/report.ts";
import { createPicture } from "../render/index.ts";
import { installErrorHook } from "./error-hook.ts";
import { readFlags } from "./flags.ts";
import { WorkerLink } from "./worker-link.ts";

// Written into the code by the build (vite.config.mjs, `define`). In a production build the first is the literal
// `false`, so everything behind it, the readout and the report among it, is cut out of the bundle; a scan of the
// built files checks that (the production check).
declare const __SHAMBLE_DEVTOOLS__: boolean;
declare const __SHAMBLE_COMMIT__: string;
const COMMIT = __SHAMBLE_COMMIT__;

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
  // A failure the page cannot go on from is said on the page itself, in every build, in plain text.
  const fail = (text: string): void => {
    if (notice === null) return;
    notice.textContent = text;
    notice.hidden = false;
  };
  const canvas = document.getElementById("view");
  if (!(canvas instanceof HTMLCanvasElement)) throw new Error("the page has no canvas to draw on");
  const flags = readFlags(location.search, __SHAMBLE_DEVTOOLS__);
  const errors = installErrorHook(window);

  const picture = createPicture();
  const info = await picture.start(canvas, { forceWebGL: flags.forceWebGL }).catch((error: unknown) => {
    fail(`This page could not start drawing: ${error instanceof Error ? error.message : String(error)}`);
    throw error;
  });
  // The backend that really runs is named in the log and on the page itself, in every build (ADR 0003 part 4).
  console.info(`Shamble Frontlines ${COMMIT}: backend ${info.backend}${info.adapter === "" ? "" : ` (${info.adapter})`}`);
  document.documentElement.dataset["backend"] = info.backend;
  document.documentElement.dataset["commit"] = COMMIT;

  const worker = new SimWorker();
  const link = new WorkerLink(worker, () => performance.timeOrigin + performance.now());
  // The link decides that the match is over and stops the Worker (`WorkerLink.end`), whichever way the problem
  // came; here the first problem is counted once and said on the page.
  const stopped = (problem: string): void => {
    errors.note(problem);
    fail(`The match stopped: ${problem}`);
  };
  worker.addEventListener("message", (event: MessageEvent<unknown>) => {
    const problem = link.receive(event);
    if (problem !== "") stopped(problem);
  });
  worker.addEventListener("error", (event) => {
    // A script that does not load gives an event with no message. Cancelled, so the window does not count it again.
    event.preventDefault();
    const problem = event.message || "the Worker failed";
    if (link.end(problem)) stopped(problem);
  });
  link.start(flags.padBytes);

  const frames = __SHAMBLE_DEVTOOLS__ ? new FrameTimes() : null;
  const resize = (): void => picture.resize(window.innerWidth, window.innerHeight, window.devicePixelRatio);
  window.addEventListener("resize", resize);
  resize();
  document.addEventListener("visibilitychange", () => {
    link.visibility(document.hidden);
    frames?.skip();
  });
  if (document.hidden) link.visibility(true);

  let readout: ((timeMs: number) => void) | null = null;
  if (__SHAMBLE_DEVTOOLS__ && frames !== null) {
    const [{ Overlay }, { formatReport, readoutLines }] = await Promise.all([
      import("../overlay/overlay.ts"),
      import("../overlay/report.ts"),
    ]);
    const data = (): ReportData => ({
      header: runHeader(window, COMMIT, flags.runLabel),
      picture: info,
      forceWebGL: flags.forceWebGL,
      webgpuOffered: "gpu" in navigator,
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
    link.frame(sequence);
    picture.draw({ newest: link.newest, previous: link.previous, timeMs });
    frames?.push(timeMs);
    readout?.(timeMs);
  });
}
