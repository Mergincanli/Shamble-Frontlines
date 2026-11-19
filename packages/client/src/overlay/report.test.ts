import { expect, test } from "vitest";
import { formatReport, readoutLines, type ReportData } from "./report.ts";

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
  counters: { drawCalls: 2, triangles: 2, lines: 202, geometries: 2, textures: 0 },
  tick: 900,
  tickHz: 30,
  blocks: 900,
  blockBytes: 8,
  posts: 1800,
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
    "backend: WebGL 2 fallback",
    "ticks: 900 at 30 a second · blocks: 900 of 8 bytes",
    "frames: 60.0 a second · frame time 16.67 ms (p99 17.20 ms)",
    "errors: 0",
    "last pause: 61.2 s hidden · ticks while hidden: 0",
    "isolation: no · commit: abc1234",
  ]);
  const webgpu = readoutLines({ ...DATA, picture: { backend: "WebGPU", adapter: "intel, gen-12", timestampQuery: true }, lastPause: null });
  expect(webgpu[0]).toBe("backend: WebGPU (intel, gen-12)");
  expect(webgpu[4]).toBe("last pause: none yet");
  expect(readoutLines({ ...DATA, errors: 2, lastError: "boom" })[3]).toBe("errors: 2 (last: boom)");
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
    "render.drawCalls: 2",
    "deliveryP95: no reading yet",
    "lastPauseTicksWhileHidden: 0",
    "worker.ticks: 900",
    "worker.latenessP99: 1.235",
  ]) {
    expect(lines, expected).toContain(expected);
  }
  expect(formatReport(DATA)).toContain("worker: (no report yet)");
});
