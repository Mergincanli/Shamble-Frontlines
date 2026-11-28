import SimWorker from "@shamble/sim-worker/worker?worker";
import { FrameTimes } from "../harness/frame-times.ts";
import { BUTTON_NAMES, KeyboardInput } from "../input/keyboard.ts";
import { runHeader } from "../harness/run-header.ts";
import type { ReportData, SceneRows } from "../overlay/report.ts";
import { createPicture } from "../render/index.ts";
import { type ContentFile, fetchContent, PageContent, takeWorkerError, takeWorkerMessage } from "./content.ts";
import { installErrorHook } from "./error-hook.ts";
import { readFlags } from "./flags.ts";
import { lostAndDoubled } from "./press-tally.ts";
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
 * The app shell's boot order: the picture first (`await renderer.init()` before the first frame), then the content
 * (one fetch; no Worker is made when it fails), then the Worker started on it, then the frame loop. The page reads its
 * copy of the content only after the Worker's `ready` (content.ts). Each drawn frame posts the newest input once (the
 * keys held and the running press counts) and draws from the two newest blocks. The page never loads or calls the
 * Wasm: the Worker does.
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

  // The content, fetched once before any Worker exists: a failure is said on the page in fixed words, and nothing starts.
  const fetched = await fetchContent((url) => fetch(url));
  if (!fetched.ok) {
    errors.note(fetched.problem);
    fail(`The match did not start: ${fetched.problem}`);
    return;
  }
  // What the page read of the content, once, on the Worker's `ready`: the map goes to the picture, and in development
  // the files and the drawn scene's counts are kept for the readout (counted here once, never in a frame).
  let delivered: { readonly files: readonly ContentFile[]; readonly scene: SceneRows } | null = null;
  const content = new PageContent((read) => {
    picture.setContent(read.picture);
    if (__SHAMBLE_DEVTOOLS__) {
      const drawn = picture.inventory() ?? [];
      const count = (kind: string): number => drawn.filter((object) => object.kind === kind).length;
      delivered = { files: read.files, scene: { map: read.picture.map.id, boxes: count("box"), ramps: count("ramp"), regions: count("region") } };
    }
  });

  // The clock both sides share.
  const clock = (): number => performance.timeOrigin + performance.now();
  const worker = new SimWorker();
  const link = new WorkerLink(worker, clock);
  // The game's keys, wherever the focus is; in a text box only a key that comes up counts (`KeyboardInput.key`).
  // When the window loses the focus or the tab is hidden, no key-up will come: every key counts as let go.
  const keys = new KeyboardInput();
  const onKey = (event: KeyboardEvent): void => {
    const typing = event.target instanceof Element && event.target.closest("textarea, input") !== null;
    // The key's own time, on the clock both sides share; an event without one gets the handler's.
    const at = event.timeStamp > 0 ? performance.timeOrigin + event.timeStamp : clock();
    if (keys.key(event.code, event.type === "keydown", event.repeat, at, typing)) event.preventDefault();
  };
  window.addEventListener("keydown", onKey);
  window.addEventListener("keyup", onKey);
  window.addEventListener("blur", () => keys.clear(clock()));
  // The link decides that the match is over and stops the Worker (`WorkerLink.end`), whichever way the problem
  // came; here the first problem is counted once and said on the page.
  const stopped = (problem: string): void => {
    errors.note(problem);
    fail(`The match stopped: ${problem}`);
  };
  // A method with a name of its own, so that a heap profile can charge the listener's whole body to it. The page's
  // copy of the bundle is read on the Worker's `ready` and let go as soon as the match ends (content.ts).
  const listener = {
    blockArrived(event: MessageEvent<unknown>): void {
      const problem = takeWorkerMessage(link, content, event);
      if (problem !== "") stopped(problem);
    },
  };
  worker.addEventListener("message", listener.blockArrived);
  worker.addEventListener("error", (event) => {
    // A script that does not load gives an event with no message. Cancelled, so the window does not count it again.
    event.preventDefault();
    const problem = takeWorkerError(link, content, event.message);
    if (problem !== "") stopped(problem);
  });
  content.start(fetched.value, link, flags.padBytes);

  const frames = __SHAMBLE_DEVTOOLS__ ? new FrameTimes() : null;
  const resize = (): void => picture.resize(window.innerWidth, window.innerHeight, window.devicePixelRatio);
  window.addEventListener("resize", resize);
  resize();
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) keys.clear(clock());
    link.visibility(document.hidden);
    frames?.skip();
  });
  if (document.hidden) link.visibility(true);

  let readout: ((timeMs: number) => void) | null = null;
  if (__SHAMBLE_DEVTOOLS__ && frames !== null) {
    const [{ Overlay }, { formatReport, readoutLines }, { KeyDiagram, KeyLights }] = await Promise.all([
      import("../overlay/overlay.ts"),
      import("../overlay/report.ts"),
      import("../overlay/keys.ts"),
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
      messages: { sent: link.sent, received: link.received, refused: link.refused },
      postUsed: link.postUsed,
      keys: BUTTON_NAMES.join(" "),
      presses: {
        made: keys.presses,
        echoed: link.echo.presses,
        ...(lostAndDoubled(keys.pressesByButton, link.echo.pressesByButton, keys.changedAt, clock()) ?? { lost: null, doubled: null }),
      },
      releases: { made: keys.releases, echoed: link.echo.releases, doubled: link.echo.heldWithoutPress },
      olderPosts: link.olderPosts,
      wrongKeys: link.wrongKeys,
      keyToTickP50: link.keyToTick.percentile(50),
      keyToTickP95: link.keyToTick.percentile(95),
      keyToTickCount: link.keyToTick.count,
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
      content: delivered !== null ? delivered.files : content.holding ? "holding" : content.refused ? "refused" : "none",
      scene: delivered?.scene ?? null,
    });
    const overlay = new Overlay(document.body, () => formatReport(data()));
    // The browser tests read the same numbers the readout shows, and the scene as drawn.
    (window as unknown as { __shambleReport: () => ReportData }).__shambleReport = data;
    (window as unknown as { __shambleScene: () => ReturnType<typeof picture.inventory> }).__shambleScene = () => picture.inventory();
    const diagram = new KeyDiagram(document.body, BUTTON_NAMES, new KeyLights());
    let next = 0;
    readout = (timeMs): void => {
      // The key diagram follows the server's echo every frame; the readout's text is rewritten less often.
      diagram.follow(link.echo.takePressed(), link.echoHeld, timeMs);
      if (timeMs < next) return;
      next = timeMs + READOUT_EVERY_MS;
      link.requestReport();
      overlay.update(readoutLines(data()));
    };
  }

  picture.loop((timeMs) => {
    link.frame(keys);
    picture.draw({ newest: link.newest, previous: link.previous, timeMs });
    frames?.push(timeMs);
    readout?.(timeMs);
  });
}
