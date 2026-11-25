import { expect, test } from "vitest";
import { fixed, formatReport, readoutLines, type ReportData } from "./report.ts";

const DATA: ReportData = {
  header: {
    commit: "abc1234",
    label: "",
    userAgent: "TestBrowser/1.0",
    hardwareConcurrency: 8,
    display: "1920x1080 at 1x, 24 bit",
    crossOriginIsolated: false,
    at: "2026-11-19T00:00:00.000Z",
  },
  picture: { backend: "WebGL 2 fallback", adapter: "", timestampQuery: false },
  forceWebGL: false,
  webgpuOffered: false,
  counters: { drawCalls: 2, triangles: 2, lines: 202, geometries: 2, textures: 0, programs: 2 },
  tick: 900,
  tickHz: 30,
  blocks: 900,
  blockBytes: 44,
  posts: 1800,
  messages: { sent: 899, received: 900, refused: 0 },
  postUsed: 1799,
  olderPosts: 0,
  wrongKeys: 0,
  keys: "W A S D Space Shift",
  presses: { made: 12, echoed: 12, lost: 0, doubled: 0 },
  releases: { made: 11, echoed: 11, doubled: 0 },
  keyToTickP50: 21.5,
  keyToTickP95: Number.NaN,
  keyToTickCount: 7,
  frames: 1800,
  fps: 60,
  frameP50: 16.67,
  frameP99: 17.2,
  deliveryP50: 0.4,
  deliveryP95: Number.NaN,
  errors: 0,
  lastError: "",
  pauses: 1,
  lastPause: { hiddenMs: 61_234, ticksWhileHidden: 0 },
  worker: null,
};

test("the readout names the backend, the ticks, the errors, the last pause and the commit", () => {
  expect(readoutLines(DATA)).toEqual([
    "backend: WebGL 2 fallback, because this browser offers no WebGPU",
    "ticks: 900 at 30 a second · blocks: 900 of 44 bytes",
    "frames: 60.0 a second · frame time 16.67 ms (p99 17.20 ms)",
    "messages: sent 899 · received 900 · refused 0",
    "presses: 12 made · 12 echoed · 0 lost · 0 doubled",
    "errors: 0",
    "last pause: 61.2 s hidden · ticks while hidden: 0 (a match that ran on: about 1837)",
    "isolation: no · commit: abc1234",
  ]);
  const webgpu = readoutLines({ ...DATA, picture: { backend: "WebGPU", adapter: "intel, gen-12", timestampQuery: true }, lastPause: null });
  expect(webgpu[0]).toBe("backend: WebGPU (intel, gen-12)");
  // A fallback that was asked for, and one the browser fell back to, read differently; the adapter follows.
  const fallback = { ...DATA, picture: { ...DATA.picture, adapter: "ANGLE (SwiftShader)" } };
  expect(readoutLines({ ...fallback, forceWebGL: true, webgpuOffered: true })[0]).toBe(
    "backend: WebGL 2 fallback, asked for by ?backend=webgl (ANGLE (SwiftShader))",
  );
  expect(readoutLines({ ...fallback, webgpuOffered: true })[0]).toBe(
    "backend: WebGL 2 fallback, because WebGPU did not start (ANGLE (SwiftShader))",
  );
  expect(webgpu[6]).toBe("last pause: none yet");
  expect(readoutLines({ ...DATA, errors: 2, lastError: "boom" })[5]).toBe("errors: 2 (last: boom)");
  // A press is not called lost while its echo may still be on its way; afterwards it is, and one that came back
  // unmade is doubled, each with its own number.
  const presses = (echoed: number, lost: number | null, doubled: number | null) =>
    readoutLines({ ...DATA, presses: { made: 12, echoed, lost, doubled } })[4];
  expect(presses(11, null, null)).toBe("presses: 12 made · 11 echoed · (counting)");
  expect(presses(10, 2, 0)).toBe("presses: 12 made · 10 echoed · 2 lost · 0 doubled");
  expect(presses(12, 1, 1)).toBe("presses: 12 made · 12 echoed · 1 lost · 1 doubled");
});

test("the report is plain text with one named value per line", () => {
  const report = formatReport({
    ...DATA,
    worker: { ticks: 900, latenessP99: 1.23456 } as unknown as ReportData["worker"],
  });
  const lines = report.split("\n");
  expect(lines[0]).toBe("Shamble Frontlines, first page: report");
  for (const expected of [
    "commit: abc1234",
    "run: (none)",
    "browser: TestBrowser/1.0",
    "backend: WebGL 2 fallback",
    "adapter: (not given)",
    "forceWebGL: false",
    "webgpuOffered: false",
    "render.drawCalls: 2",
    "deliveryP95: no reading yet",
    "messages.sent: 899",
    "messages.received: 900",
    "messages.refused: 0",
    "postUsed: 1799",
    "olderPosts: 0",
    "wrongKeys: 0",
    "keys.bound: W A S D Space Shift",
    "presses.made: 12",
    "presses.echoed: 12",
    "presses.lost: 0",
    "presses.doubled: 0",
    "releases.made: 11",
    "releases.echoed: 11",
    "releases.doubled: 0",
    "keyToTickP50: 21.50 ms",
    "keyToTickP95: no reading yet",
    "keyToTickCount: 7",
    "lastPauseTicksWhileHidden: 0",
    "worker.ticks: 900",
    "worker.latenessP99: 1.235",
  ]) {
    expect(lines, expected).toContain(expected);
  }
  expect(formatReport(DATA)).toContain("worker: (no report yet)");
  const counting = formatReport({ ...DATA, presses: { made: 1, echoed: 0, lost: null, doubled: null } });
  expect(counting).toContain("presses.lost: (counting)");
  expect(counting).toContain("presses.doubled: (counting)");
});

test("a value that rounds to zero prints as zero, never with a minus sign", () => {
  expect(fixed(-0.001, 2)).toBe("0.00");
  expect(fixed(-0, 1)).toBe("0.0");
  expect(fixed(-0.4, 0)).toBe("0");
  expect(fixed(0.004, 2)).toBe("0.00");
  // A value that does not round to zero keeps its sign.
  expect(fixed(-0.005, 2)).toBe("-0.01");
  expect(fixed(-12.345, 1)).toBe("-12.3");
  expect(fixed(16.666, 2)).toBe("16.67");
  // His Brave report of 2026-11-19 held "deliveryP50: -0.00 ms": the two sides' clocks, each rounded by the browser.
  const lines = formatReport({
    ...DATA,
    deliveryP50: -0.0004,
    frameP50: -0.004,
    worker: { latenessP50: -0.0002 } as unknown as ReportData["worker"],
  }).split("\n");
  expect(lines).toContain("deliveryP50: 0.00 ms");
  expect(lines).toContain("frameP50: 0.00 ms");
  expect(lines).toContain("worker.latenessP50: 0.000");
  expect(lines.join("\n")).not.toMatch(/-0\.0+( |$)/m);
});
