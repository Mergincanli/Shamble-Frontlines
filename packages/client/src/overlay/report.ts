import type { WorkerStats } from "@shamble/sim-worker";
import type { PauseRecord } from "../app/worker-link.ts";
import type { RunHeader } from "../harness/run-header.ts";
import type { PictureInfo, RenderCounters } from "../picture.ts";

/** Everything the readout and the copy report show. It never holds the match seed or the state hash. */
export interface ReportData {
  readonly header: RunHeader;
  readonly picture: PictureInfo;
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
  readonly lastPause: PauseRecord | null;
  readonly worker: WorkerStats | null;
}

const ms = (value: number): string => (Number.isFinite(value) ? `${value.toFixed(2)} ms` : "no reading yet");

/** The readout's lines: what a person checks by eye. */
export function readoutLines(data: ReportData): string[] {
  const pause = data.lastPause;
  return [
    `backend: ${data.picture.backend}${data.picture.adapter === "" ? "" : ` (${data.picture.adapter})`}`,
    `ticks: ${data.tick} at ${data.tickHz} a second · blocks: ${data.blocks} of ${data.blockBytes} bytes`,
    `frames: ${data.fps.toFixed(1)} a second · frame time ${ms(data.frameP50)} (p99 ${ms(data.frameP99)})`,
    `errors: ${data.errors}${data.lastError === "" ? "" : ` (last: ${data.lastError})`}`,
    pause === null
      ? "last pause: none yet"
      : `last pause: ${(pause.hiddenMs / 1000).toFixed(1)} s hidden · ticks while hidden: ${pause.ticksWhileHidden}`,
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
    ["timestampQuery", data.picture.timestampQuery],
    ...Object.entries(data.counters).map(([name, value]): [string, number] => [`render.${name}`, value]),
    ["tick", data.tick],
    ["tickHz", data.tickHz],
    ["blocks", data.blocks],
    ["blockBytes", data.blockBytes],
    ["inputPosts", data.posts],
    ["frames", data.frames],
    ["fps", data.fps.toFixed(2)],
    ["frameP50", ms(data.frameP50)],
    ["frameP99", ms(data.frameP99)],
    ["deliveryP50", ms(data.deliveryP50)],
    ["deliveryP95", ms(data.deliveryP95)],
    ["errors", data.errors],
    ["lastError", data.lastError === "" ? "(none)" : data.lastError],
    ["pauses", data.pauses],
    ["lastPauseHiddenMs", data.lastPause === null ? "(none)" : data.lastPause.hiddenMs.toFixed(0)],
    ["lastPauseTicksWhileHidden", data.lastPause === null ? "(none)" : data.lastPause.ticksWhileHidden],
    ...(data.worker === null
      ? [["worker", "(no report yet)"] as [string, string]]
      : Object.entries(data.worker).map(([name, value]): [string, string | number | boolean] => [
          `worker.${name}`,
          typeof value === "number" && !Number.isInteger(value) ? value.toFixed(3) : value,
        ])),
  ];
  return ["Shamble Frontlines, first page: report", ...readoutLines(data), "", ...rows.map(([name, value]) => `${name}: ${value}`)].join("\n");
}
