/** What the address and the build ask of this run. */
export interface Flags {
  /** A development build: the readout, the copy report and the measuring flags exist. */
  readonly devtools: boolean;
  /** `?backend=webgl`: use the WebGL 2 fallback even where WebGPU exists. */
  readonly forceWebGL: boolean;
  /** `?pad=<bytes>`: pad the block for the hand-off readings. Development builds only; else 0. */
  readonly padBytes: number;
  /** `?run=<label>`: a label for this run in the report. Development builds only; else empty. */
  readonly runLabel: string;
}

/** The most padding the page asks for: the writer's own cap, 1 MiB. */
const MAX_PAD_BYTES = 1 << 20;

/** Reads the flags from the address's query (`location.search`). Anything it does not know is ignored. */
export function readFlags(search: string, devtools: boolean): Flags {
  const query = new URLSearchParams(search);
  const pad = Number(query.get("pad") ?? "0");
  const label = query.get("run") ?? "";
  return {
    devtools,
    forceWebGL: query.get("backend") === "webgl",
    padBytes: devtools && Number.isInteger(pad) && pad >= 0 && pad <= MAX_PAD_BYTES ? pad : 0,
    runLabel: devtools && /^[\w-]{1,40}$/.test(label) ? label : "",
  };
}
