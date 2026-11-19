# Firefox: what was read, not run (the last pass of change-set 2)

_Written 2026-11-19T13:47:18Z from the clock. Firefox was not run: this container has no Firefox (`command -v firefox`: nothing; Playwright's browsers here: chromium chromium-1243 chromium_headless_shell-1243 ffmpeg-1011 ), and fetching one is code from the internet (his TOOL-SOURCE: it comes back to him)._

## The search
Our own sources of the page and the Worker (tests left out), for calls that not every browser has:

```
$ grep -rnE '<names>' packages/client/src packages/sim-worker/src packages/client/index.html (no *.test.ts)
packages/client/src/overlay/report.ts:12:  /** The browser has `navigator.gpu` at all; without it the fallback is the only way. */
packages/client/src/overlay/report.ts:59:    `isolation: ${data.header.crossOriginIsolated ? "yes" : "no"} · commit: ${data.header.commit}`,
packages/client/src/overlay/report.ts:72:    ["crossOriginIsolated", data.header.crossOriginIsolated],
packages/client/src/overlay/overlay.ts:21:      navigator.clipboard.writeText(report).then(
packages/client/src/harness/run-header.ts:9:  readonly crossOriginIsolated: boolean;
packages/client/src/harness/run-header.ts:19:    userAgent: navigator.userAgent,
packages/client/src/harness/run-header.ts:20:    hardwareConcurrency: navigator.hardwareConcurrency,
packages/client/src/harness/run-header.ts:22:    crossOriginIsolated: target.crossOriginIsolated,
packages/client/src/app/boot.ts:50:  const link = new WorkerLink(worker, () => performance.timeOrigin + performance.now());
packages/client/src/app/boot.ts:73:  document.addEventListener("visibilitychange", () => {
packages/client/src/app/boot.ts:89:      webgpuOffered: "gpu" in navigator,
packages/client/src/app/error-hook.ts:20:  target.addEventListener("securitypolicyviolation", (event) =>
packages/client/src/app/worker-link.ts:44:  /** `now` is the clock both sides share: `performance.timeOrigin + performance.now()`. */
packages/client/src/render/three-picture.ts:8:  device?: { adapterInfo?: { vendor?: string; architecture?: string; description?: string } };
packages/client/src/render/three-picture.ts:17:  const info = backend.device?.adapterInfo;
packages/client/src/render/three-picture.ts:20:  const names = gl?.getExtension("WEBGL_debug_renderer_info") ?? null;
packages/client/src/render/three-picture.ts:29:  if (type === "error") reportError(new Error(message));
packages/sim-worker/src/worker.ts:19:  const seed = crypto.getRandomValues(new Uint8Array(32));
packages/sim-worker/src/worker.ts:27:      timeOrigin: performance.timeOrigin,
packages/sim-worker/src/worker.ts:31:      hardwareConcurrency: navigator.hardwareConcurrency,
packages/sim-worker/src/worker.ts:32:      crossOriginIsolated,
packages/sim-worker/src/host.ts:21:  /** `performance.timeOrigin`: with `now()` it gives the clock the page shares. */
packages/sim-worker/src/host.ts:22:  readonly timeOrigin: number;
packages/sim-worker/src/host.ts:27:  readonly crossOriginIsolated: boolean;
packages/sim-worker/src/host.ts:121:      crossOriginIsolated: this.#env.crossOriginIsolated,
packages/sim-worker/src/host.ts:148:    this.#env.post({ type: "block", buffer, doneAt: this.#env.timeOrigin + before }, [buffer]);
packages/sim-worker/src/messages.ts:30:   * (`performance.timeOrigin + performance.now()`).
packages/sim-worker/src/messages.ts:58:  readonly crossOriginIsolated: boolean;
```

## What the search says (reading only)
- None of the names that only Chromium has is called by the page or the Worker: no `performance.memory`, no
  `navigator.userAgentData`, no `scheduler.`, no `requestIdleCallback`, no `Atomics`, `SharedArrayBuffer` or
  `OffscreenCanvas`. What is called (`navigator.clipboard.writeText` behind a click with a text-box fallback,
  `reportError`, `crypto.getRandomValues`, `performance.timeOrigin`, a module Worker, `visibilitychange`,
  `securitypolicyviolation`) is written against the web's standards, not against one browser.
- Three points only a run can answer; they are steps F2 and F3 of the zip's README:
  1. What the built page does when Firefox opens it as a file. In Chromium the stylesheet and the script are refused
     (origin `null`) and the page's own line of text stays. Firefox treats file pages by its own rules; if it runs the
     script, the page hides that line and then stops at the Worker or the Wasm with "The match stopped: ...".
  2. Whether three.js's WebGPU backend starts in his Firefox, or the page falls back to WebGL 2 and says why.
  3. The adapter's name on the fallback is read through `WEBGL_debug_renderer_info` (three-picture.ts:20). If Firefox
     gives no such extension the report says "(not given)"; if it gives it with a console warning, that is a warning,
     not an error (the error count listens to error events, refused policy and unhandled rejections only:
     error-hook.ts:17-25). On WebGPU the name comes from `device.adapterInfo` (three-picture.ts:17); a browser without
     that field also gives "(not given)".
- The browser tests are Chromium-only today: the Playwright config has one browser project (`chromium`), and the allocation spec uses
  Chromium's own profiler. A Firefox project needs Playwright's Firefox build (a download: his word first) and
  belongs with the CI pull request's text.
