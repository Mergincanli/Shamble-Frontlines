import type { WorkerStats } from "@shamble/sim-worker";
import type { RunHeader } from "../harness/run-header.ts";
import type { PictureInfo, RenderCounters } from "../picture.ts";

/** A content file as the readout names it (the page's `ContentFile`; its shape is stated here: nothing of the app shell is imported). */
export interface ContentFileRow {
  readonly path: string;
  readonly format: string;
  readonly version: number;
}

/** The page's content: its copy still held for the loader's answer, let go unread (the match ended first), or read. */
export type ContentRows = "holding" | "none" | readonly ContentFileRow[];

/** The drawn scene, counted once when the map was drawn: the map's id and its blocks and patches of ground. */
export interface SceneRows {
  readonly map: string;
  readonly boxes: number;
  readonly ramps: number;
  readonly regions: number;
}

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
  /** The newest block's count of messages the endpoint sent and received and of messages either side refused. */
  readonly messages: { readonly sent: number; readonly received: number; readonly refused: number };
  /** The number of the post from which the last command the server ran was built. */
  readonly postUsed: number;
  /** The keys the page reads, by name. */
  readonly keys: string;
  /** Ticks that ran an older post than the tick before them: the keys out of order (0 on a sound Worker). */
  readonly olderPosts: number;
  /** Ticks whose echo held other keys than the post the tick used (0 on a sound Worker). */
  readonly wrongKeys: number;
  /**
   * Presses and releases the page made and the server's echo brought back. `lost` and `doubled` presses are
   * counted button by button once the keys have been quiet for a moment, and are `null` while the newest echo may
   * still be on its way. A release's `doubled` counts a button the echo showed as held again with no press
   * (`EchoTally.heldWithoutPress`).
   */
  readonly presses: { readonly made: number; readonly echoed: number; readonly lost: number | null; readonly doubled: number | null };
  readonly releases: { readonly made: number; readonly echoed: number; readonly doubled: number };
  /** From a key change to the start of the tick that took it up: the median, the 95th percentile and how many were timed. */
  readonly keyToTickP50: number;
  readonly keyToTickP95: number;
  readonly keyToTickCount: number;
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
  readonly content: ContentRows;
  /** `null` while the empty floor is drawn. */
  readonly scene: SceneRows | null;
}

/** The content in words, for the readout. */
function contentWords(content: ContentRows): string {
  if (content === "holding") return "waiting for the loader";
  if (content === "none") return "not read, the match stopped first";
  return content.map(({ path, format, version }) => `${path} ${format} ${version}`).join(" · ");
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

/** Presses that did not come back or came back unmade, in words; none are named before the keys have been quiet for a moment. */
function lostWords(lost: number | null, doubled: number | null): string {
  return lost === null || doubled === null ? "(counting)" : `${lost} lost · ${doubled} doubled`;
}

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
    `messages: sent ${data.messages.sent} · received ${data.messages.received} · refused ${data.messages.refused}`,
    `content: ${contentWords(data.content)}`,
    data.scene === null
      ? "scene: empty"
      : `scene: ${data.scene.map} · ${data.scene.boxes} boxes · ${data.scene.ramps} ramps · ${data.scene.regions} regions`,
    `presses: ${data.presses.made} made · ${data.presses.echoed} echoed · ${lostWords(data.presses.lost, data.presses.doubled)}`,
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
    ["messages.sent", data.messages.sent],
    ["messages.received", data.messages.received],
    ["messages.refused", data.messages.refused],
    ["postUsed", data.postUsed],
    ["olderPosts", data.olderPosts],
    ["wrongKeys", data.wrongKeys],
    ["keys.bound", data.keys],
    ["presses.made", data.presses.made],
    ["presses.echoed", data.presses.echoed],
    ["presses.lost", data.presses.lost ?? "(counting)"],
    ["presses.doubled", data.presses.doubled ?? "(counting)"],
    ["releases.made", data.releases.made],
    ["releases.echoed", data.releases.echoed],
    ["releases.doubled", data.releases.doubled],
    ["keyToTickP50", ms(data.keyToTickP50)],
    ["keyToTickP95", ms(data.keyToTickP95)],
    ["keyToTickCount", data.keyToTickCount],
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
    ...(typeof data.content === "string"
      ? [["content", data.content] as [string, string]]
      : data.content.map(({ path, format, version }): [string, string] => [`content.${path}`, `${format} ${version}`])),
    ...(data.scene === null
      ? [["scene", "(empty)"] as [string, string]]
      : [
          ["scene.map", data.scene.map] as [string, string],
          ["scene.boxes", data.scene.boxes] as [string, number],
          ["scene.ramps", data.scene.ramps] as [string, number],
          ["scene.regions", data.scene.regions] as [string, number],
        ]),
    ...(data.worker === null
      ? [["worker", "(no report yet)"] as [string, string]]
      : Object.entries(data.worker).map(([name, value]): [string, string | number | boolean] => [
          `worker.${name}`,
          typeof value === "number" && !Number.isInteger(value) ? fixed(value, 3) : value,
        ])),
  ];
  return ["Shamble Frontlines, first page: report", ...readoutLines(data), "", ...rows.map(([name, value]) => `${name}: ${value}`)].join("\n");
}
