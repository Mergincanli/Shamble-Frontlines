import type { WorkerStats } from "@shamble/sim-worker";
import type { RunHeader } from "../harness/run-header.ts";
import type { PictureInfo, RenderCounters } from "../picture.ts";

/** Everything the readout and the copy report show. It never holds the match seed or the state hash. */
export interface ReportData {
  readonly header: RunHeader;
  readonly picture: PictureInfo;
  /** `?backend=webgl` was given. */
  readonly forceWebGL: boolean;
  /** The browser has `navigator.gpu` at all; without it the fallback is the only way. */
  readonly webgpuOffered: boolean;
  readonly counters: RenderCounters;
  readonly tick: number;
  readonly tickHz: number;
  readonly blocks: number;
  readonly blockBytes: number;
  readonly posts: number;
  readonly frames: number;
  readonly fps: number;
  readonly frameP50: number;
  readonly frameP99: number;
  readonly deliveryP50: number;
  readonly deliveryP95: number;
  readonly errors: number;
  readonly lastError: string;
  readonly pauses: number;
  /** The last pause while the tab was hidden, as the report shows it (the shape is stated here: nothing of the app shell is imported). */
  readonly lastPause: { readonly hiddenMs: number; readonly ticksWhileHidden: number } | null;
  readonly worker: WorkerStats | null;
}

/**
 * A number with a fixed count of decimals. A value that rounds to zero prints as zero, never as "-0.00": two clocks
 * that the browser rounds can put a difference a hair under 0.
 */
export function fixed(value: number, digits: number): string {
  const text = value.toFixed(digits);
  return Number(text) === 0 ? text.replace("-", "") : text;
}

const ms = (value: number): string => (Number.isFinite(value) ? `${fixed(value, 2)} ms` : "no reading yet");

/** Why this backend runs: a fallback that was asked for reads differently from one the browser fell back to. */
function backendLine(data: ReportData): string {
  const why =
    data.picture.backend === "WebGPU"
      ? ""
      : data.forceWebGL
        ? ", asked for by ?backend=webgl"
        : data.webgpuOffered
          ? ", because WebGPU did not start"
          : ", because this browser offers no WebGPU";
  return `backend: ${data.picture.backend}${why}${data.picture.adapter === "" ? "" : ` (${data.picture.adapter})`}`;
}

/** The readout's lines: what a person checks by eye. */
export function readoutLines(data: ReportData): string[] {
  const pause = data.lastPause;
  return [
    backendLine(data),
    `ticks: ${data.tick} at ${data.tickHz} a second · blocks: ${data.blocks} of ${data.blockBytes} bytes`,
    `frames: ${fixed(data.fps, 1)} a second · frame time ${ms(data.frameP50)} (p99 ${ms(data.frameP99)})`,
    `errors: ${data.errors}${data.lastError === "" ? "" : ` (last: ${data.lastError})`}`,
    pause === null
      ? "last pause: none yet"
      : `last pause: ${fixed(pause.hiddenMs / 1000, 1)} s hidden · ticks while hidden: ${pause.ticksWhileHidden} (a match that ran on: about ${Math.round((pause.hiddenMs / 1000) * data.tickHz)})`,
    `isolation: ${data.header.crossOriginIsolated ? "yes" : "no"} · commit: ${data.header.commit}`,
  ];
}

/** The copy report: the readout's lines, then every number with its name, one per line, as plain text. */
export function formatReport(data: ReportData): string {
  const rows: [string, string | number | boolean][] = [
    ["commit", data.header.commit],
    ["run", data.header.label === "" ? "(none)" : data.header.label],
    ["at", data.header.at],
    ["browser", data.header.userAgent],
    ["display", data.header.display],
    ["hardwareConcurrency", data.header.hardwareConcurrency],
    ["crossOriginIsolated", data.header.crossOriginIsolated],
    ["backend", data.picture.backend],
    ["adapter", data.picture.adapter === "" ? "(not given)" : data.picture.adapter],
    ["forceWebGL", data.forceWebGL],
    ["webgpuOffered", data.webgpuOffered],
    ["timestampQuery", data.picture.timestampQuery],
    ...Object.entries(data.counters).map(([name, value]): [string, number] => [`render.${name}`, value]),
    ["tick", data.tick],
    ["tickHz", data.tickHz],
    ["blocks", data.blocks],
    ["blockBytes", data.blockBytes],
    ["inputPosts", data.posts],
    ["frames", data.frames],
    ["fps", fixed(data.fps, 2)],
    ["frameP50", ms(data.frameP50)],
    ["frameP99", ms(data.frameP99)],
    ["deliveryP50", ms(data.deliveryP50)],
    ["deliveryP95", ms(data.deliveryP95)],
    ["errors", data.errors],
    ["lastError", data.lastError === "" ? "(none)" : data.lastError],
    ["pauses", data.pauses],
    ["lastPauseHiddenMs", data.lastPause === null ? "(none)" : fixed(data.lastPause.hiddenMs, 0)],
    ["lastPauseTicksWhileHidden", data.lastPause === null ? "(none)" : data.lastPause.ticksWhileHidden],
    ...(data.worker === null
      ? [["worker", "(no report yet)"] as [string, string]]
      : Object.entries(data.worker).map(([name, value]): [string, string | number | boolean] => [
          `worker.${name}`,
          typeof value === "number" && !Number.isInteger(value) ? fixed(value, 3) : value,
        ])),
  ];
  return ["Shamble Frontlines, first page: report", ...readoutLines(data), "", ...rows.map(([name, value]) => `${name}: ${value}`)].join("\n");
}
