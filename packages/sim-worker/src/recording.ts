// A recorded match, read on the JavaScript side, and the replay of one through the bindings' own calls. Tests only:
// the boundary tests (Node) and the browser's replay spec use it; nothing the Worker or the page imports does, so no
// build holds it. The format is `crates/devtools/src/recording.rs`'s, version 1, and this reader makes the same
// checks in the same order: a count is judged against the bytes that are there before anything is read by it.

export const STEP_BYTES = 68;
export const MAX_STEPS = 108_000;
const MAX_MAP_ID_BYTES = 64;
const MAGIC = "SHMBLREC";
const TEST_SEED_MARK = "TESTSEED";

export interface RecordedStep {
  readonly hostMs: number;
  readonly post: number;
  readonly held: number;
  readonly presses: readonly [number, number, number, number];
  readonly movement: number;
  readonly ray: readonly number[];
  readonly viewTick: number;
  readonly viewFraction: number;
}

export interface Recorded {
  readonly seed: Uint8Array;
  readonly startTick: number;
  readonly firstSequence: number;
  readonly tickRate: number;
  readonly contentHash: Uint8Array;
  readonly mapId: string;
  readonly steps: readonly RecordedStep[];
}

export interface TickHashes {
  readonly tick: number;
  readonly shared: bigint;
  readonly whole: bigint;
}

/** The bindings' calls a replay makes: the players' `Sim` has exactly these. */
export interface Replayable {
  rebase(nowMs: number): number;
  input(post: number, held: number, p0: number, p1: number, p2: number, p3: number): void;
  step(nowMs: number): number;
  shared_hash(): bigint;
}

const ascii = (bytes: Uint8Array): string => String.fromCharCode(...bytes);

/** Reads a recording from any bytes; what is no recording of this version is refused in plain words. */
export function readRecording(bytes: Uint8Array, tickHz: number): Recorded {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const fixed = 8 + 2 + 32 + 4 + 2 + 2 + 32 + 1;
  const cut = (): Error => new Error("the bytes end inside the recording's header");
  if (bytes.length < 8) throw cut();
  if (ascii(bytes.subarray(0, 8)) !== MAGIC) throw new Error("the bytes do not start with SHMBLREC: this is no recording");
  if (bytes.length < 10) throw cut();
  const version = view.getUint16(8, true);
  if (version !== 1) throw new Error(`the recording has format version ${version}; this build reads version 1 only`);
  if (bytes.length < fixed) throw cut();
  const seed = bytes.slice(10, 42);
  const tickRate = view.getUint16(48, true);
  const idLength = view.getUint8(fixed - 1);
  if (idLength > MAX_MAP_ID_BYTES) throw new Error(`the map's id is ${idLength} bytes long; the most is ${MAX_MAP_ID_BYTES}`);
  const stepsAt = fixed + idLength + 4;
  if (bytes.length < stepsAt) throw cut();
  let mapId: string;
  try {
    mapId = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(fixed, fixed + idLength));
  } catch {
    throw new Error("the map's id is not UTF-8 text");
  }
  const count = view.getUint32(stepsAt - 4, true);
  if (count > MAX_STEPS) throw new Error(`the recording has ${count} steps; the most is ${MAX_STEPS}`);
  const found = bytes.length - stepsAt;
  if (found !== count * STEP_BYTES) throw new Error(`after the header ${count * STEP_BYTES} bytes of steps should follow, and ${found} do`);
  if (ascii(seed.subarray(0, 8)) !== TEST_SEED_MARK) {
    throw new Error("the seed does not start with TESTSEED: a recording holds a public test seed and never a match's own");
  }
  if (tickRate !== tickHz) throw new Error(`the recording was made at ${tickRate} ticks a second; this build runs at ${tickHz}`);
  const steps: RecordedStep[] = [];
  let before = Number.NEGATIVE_INFINITY;
  for (let index = 0; index < count; index += 1) {
    const at = stepsAt + index * STEP_BYTES;
    const word = (number: number): number => view.getUint32(at + 8 + 4 * number, true);
    const hostMs = view.getFloat64(at, true);
    if (!Number.isFinite(hostMs) || hostMs < before) {
      throw new Error(`step ${index}: the host's time is not a finite number or lies before the step before`);
    }
    before = hostMs;
    const step: RecordedStep = {
      hostMs,
      post: word(0),
      held: word(1),
      presses: [word(2), word(3), word(4), word(5)],
      movement: word(6),
      ray: [word(7), word(8), word(9), word(10), word(11), word(12)],
      viewTick: word(13),
      viewFraction: word(14),
    };
    for (const [field, value] of [["the held buttons", step.held], ["the move", step.movement], ["the view time's fraction", step.viewFraction]] as const) {
      if (value > 0xffff) throw new Error(`step ${index}: ${field} has a bit set that the format keeps at zero`);
    }
    steps.push(step);
  }
  return {
    seed,
    startTick: view.getUint32(42, true),
    firstSequence: view.getUint16(46, true),
    tickRate,
    contentHash: bytes.slice(50, 82),
    mapId,
    steps,
  };
}

/** Reads a hashes file: the SHA-256 (hex) of the recording it belongs to, and its lines. */
export function readHashes(text: string): { readonly recordingSha256: string; readonly lines: readonly TickHashes[] } {
  const all = text.split("\n");
  const first = /^shamble-hashes 1 ([0-9a-f]{64}) ([0-9]+)$/.exec(all[0] ?? "");
  if (first === null || all.at(-1) !== "") throw new Error("the first line is not a hashes file's, or the text does not end with a line end");
  const lines = all.slice(1, -1).map((line, index): TickHashes => {
    const parts = /^([0-9]+) ([0-9a-f]{16}) ([0-9a-f]{16})$/.exec(line);
    if (parts === null || Number(parts[1]) > 0xffff_ffff) throw new Error(`line ${index + 2} is not a tick and two hashes of 16 hex digits`);
    return { tick: Number(parts[1]), shared: BigInt(`0x${parts[2]}`), whole: BigInt(`0x${parts[3]}`) };
  });
  if (lines.length !== Number(first[2])) throw new Error(`the first line says ${first[2]} steps and ${lines.length} lines follow`);
  return { recordingSha256: first[1] ?? "", lines };
}

/** What this build's `input` call cannot take yet is refused whole: nothing recorded is silently left out. */
export function checkReplayable(recording: Recorded): void {
  const cannot = (what: string): Error => new Error(`this build cannot replay this recording's ${what}`);
  if (recording.startTick !== 0) throw cannot("start tick: a match starts at tick 0");
  if (recording.firstSequence !== 0) throw cannot("first command sequence: a match starts at sequence 0");
  if (recording.contentHash.some((byte) => byte !== 0) || recording.mapId !== "") throw cannot("content: this build takes none");
  if (recording.steps.some((step) => step.movement !== 0)) throw cannot("move: the input call does not take it yet");
  if (recording.steps.some((step) => step.ray.some((word) => word !== 0))) throw cannot("aim: the input call does not take it yet");
  if (recording.steps.some((step) => step.viewTick !== 0 || step.viewFraction !== 0)) throw cannot("view time: the input call does not take it yet");
}

/** One step's calls, as the live host makes them, then the one number a replay reads. */
export function replayStep(sim: Replayable, step: RecordedStep): bigint {
  sim.input(step.post, step.held, ...step.presses);
  sim.step(step.hostMs);
  return sim.shared_hash();
}

/** Replays a whole recording on a match just created from its seed: the shared hash after each step. */
export function replayOn(sim: Replayable, recording: Recorded): bigint[] {
  checkReplayable(recording);
  const first = recording.steps[0];
  if (first !== undefined) sim.rebase(first.hostMs);
  return recording.steps.map((step) => replayStep(sim, step));
}

/** The first step (from 1) at which the replay's hashes differ from the file's shared column, or one ends early. */
export function firstDifference(expected: readonly TickHashes[], got: readonly bigint[]): number | null {
  const steps = Math.max(expected.length, got.length);
  for (let index = 0; index < steps; index += 1) {
    if (expected[index]?.shared !== got[index]) return index + 1;
  }
  return null;
}
