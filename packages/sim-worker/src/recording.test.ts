import { expect, test } from "vitest";
import { checkReplayable, firstDifference, readHashes, readRecording, replayOn, STEP_BYTES, type Replayable } from "./recording.ts";

const HEADER_BYTES = 87;

/** A recording's bytes, written here by hand from the format's text: three steps, no map. */
function bytesOf(change: (view: DataView, bytes: Uint8Array) => void = () => {}, steps = 3): Uint8Array {
  const bytes = new Uint8Array(HEADER_BYTES + steps * STEP_BYTES);
  const view = new DataView(bytes.buffer);
  bytes.set(new TextEncoder().encode("SHMBLREC"), 0);
  view.setUint16(8, 1, true);
  bytes.set(new TextEncoder().encode("TESTSEEDunit"), 10);
  view.setUint16(48, 30, true);
  view.setUint32(HEADER_BYTES - 4, steps, true);
  for (let index = 0; index < steps; index += 1) {
    const at = HEADER_BYTES + index * STEP_BYTES;
    view.setFloat64(at, 100 + 33.5 * index, true);
    view.setUint32(at + 8, 7 + index, true);
    view.setUint32(at + 12, 0x8001, true);
    view.setUint32(at + 16, 0x0403_0201 + index, true);
    view.setUint32(at + 28, 0xff00_0000, true);
  }
  change(view, bytes);
  return bytes;
}

test("a recording's bytes read as the format says", () => {
  const recording = readRecording(bytesOf(), 30);
  expect(recording.steps).toHaveLength(3);
  expect(new TextDecoder().decode(recording.seed.subarray(0, 12))).toBe("TESTSEEDunit");
  expect([recording.startTick, recording.firstSequence, recording.tickRate, recording.mapId]).toEqual([0, 0, 30, ""]);
  expect(recording.steps[2]).toEqual({
    hostMs: 167,
    post: 9,
    held: 0x8001,
    presses: [0x0403_0203, 0, 0, 0xff00_0000],
    movement: 0,
    ray: [0, 0, 0, 0, 0, 0],
    viewTick: 0,
    viewFraction: 0,
  });
  // The other fields have their own places: a map's id moves the steps, and each word after the presses is read.
  const full = readRecording(
    bytesOf((view, bytes) => {
      view.setUint32(42, 131_069, true);
      view.setUint16(46, 65_534, true);
      bytes.fill(0xc4, 50, 82);
      for (let word = 6; word < 15; word += 1) view.setUint32(HEADER_BYTES + 8 + 4 * word, word === 6 || word === 14 ? 0xffff : 0xa000_0000 + word, true);
    }),
    30,
  );
  expect([full.startTick, full.firstSequence, full.contentHash[31]]).toEqual([131_069, 65_534, 0xc4]);
  expect(full.steps[0]).toMatchObject({ movement: 0xffff, ray: [7, 8, 9, 10, 11, 12].map((word) => 0xa000_0000 + word), viewTick: 0xa000_000d, viewFraction: 0xffff });
});

test("bytes that are no recording of this version are refused in plain words", () => {
  const refused = (bytes: Uint8Array, tickHz = 30): string => {
    try {
      readRecording(bytes, tickHz);
    } catch (error) {
      return error instanceof Error ? error.message : String(error);
    }
    return "taken";
  };
  const changed = (change: (view: DataView, bytes: Uint8Array) => void): string => refused(bytesOf(change));
  expect(refused(bytesOf())).toBe("taken");
  expect(refused(new Uint8Array(0))).toBe("the bytes end inside the recording's header");
  expect(refused(bytesOf().subarray(0, 60))).toBe("the bytes end inside the recording's header");
  expect(changed((_, bytes) => bytes.set([0x58], 7))).toBe("the bytes do not start with SHMBLREC: this is no recording");
  expect(changed((view) => view.setUint16(8, 2, true))).toBe("the recording has format version 2; this build reads version 1 only");
  expect(changed((_, bytes) => bytes.set(new TextEncoder().encode("REALSEED"), 10))).toContain("a public test seed and never a match's own");
  expect(refused(bytesOf(), 60)).toBe("the recording was made at 30 ticks a second; this build runs at 60");
  expect(changed((view) => view.setUint8(82, 65))).toBe("the map's id is 65 bytes long; the most is 64");
  expect(changed((view) => view.setUint32(83, 0xffff_ffff, true))).toBe("the recording has 4294967295 steps; the most is 108000");
  // A count the bytes do not hold is refused by the length, before any step is read.
  expect(changed((view) => view.setUint32(83, 108_000, true))).toBe("after the header 7344000 bytes of steps should follow, and 204 do");
  expect(refused(bytesOf().subarray(0, HEADER_BYTES + 3 * STEP_BYTES - 1))).toBe("after the header 204 bytes of steps should follow, and 203 do");
  expect(changed((view) => view.setFloat64(HEADER_BYTES + STEP_BYTES, Number.NaN, true))).toBe("step 1: the host's time is not a finite number or lies before the step before");
  expect(changed((view) => view.setFloat64(HEADER_BYTES + 2 * STEP_BYTES, 99, true))).toContain("step 2: the host's time");
  expect(changed((view) => view.setUint32(HEADER_BYTES + 12, 0x1_0000, true))).toBe("step 0: the held buttons has a bit set that the format keeps at zero");
  expect(changed((view) => view.setUint32(HEADER_BYTES + STEP_BYTES + 32, 0x1_0000, true))).toBe("step 1: the move has a bit set that the format keeps at zero");
  expect(changed((view) => view.setUint32(HEADER_BYTES + 64, 0x1_0000, true))).toBe("step 0: the view time's fraction has a bit set that the format keeps at zero");
});

test("a hashes file reads as its lines and names its recording", () => {
  const digest = "ab".repeat(32);
  const text = `shamble-hashes 1 ${digest} 2\n1 0123456789abcdef 0000000000000000\n4294967295 ffffffffffffffff 00ff000000000001\n`;
  expect(readHashes(text)).toEqual({
    recordingSha256: digest,
    lines: [
      { tick: 1, shared: 0x0123_4567_89ab_cdefn, whole: 0n },
      { tick: 4_294_967_295, shared: 0xffff_ffff_ffff_ffffn, whole: 0x00ff_0000_0000_0001n },
    ],
  });
  for (const wrong of [
    "",
    `shamble-hashes 1 ${digest} 2`,
    `shamble-hashes 2 ${digest} 0\n`,
    `shamble-hashes 1 ${digest} 1\n`,
    `shamble-hashes 1 ${digest} 1\n1 0123456789ABCDEF 0000000000000000\n`,
    `shamble-hashes 1 ${digest} 1\n4294967296 0123456789abcdef 0000000000000000\n`,
    `shamble-hashes 1 ${digest} 1\n1 0123456789abcdef 0000000000000000 \n`,
  ]) {
    expect(() => readHashes(wrong), JSON.stringify(wrong)).toThrow();
  }
});

test("a replay makes the live host's calls in the live host's order, and refuses what the input call cannot take", () => {
  const calls: string[] = [];
  let hash = 0n;
  const sim: Replayable = {
    rebase: (now) => calls.push(`rebase ${now}`),
    input: (...numbers) => void calls.push(`input ${numbers.join(" ")}`),
    step: (now) => calls.push(`step ${now}`),
    shared_hash: () => {
      calls.push("shared_hash");
      hash += 1n;
      return hash;
    },
  };
  expect(replayOn(sim, readRecording(bytesOf(undefined, 2), 30))).toEqual([1n, 2n]);
  expect(calls).toEqual([
    "rebase 100",
    "input 7 32769 67305985 0 0 4278190080",
    "step 100",
    "shared_hash",
    "input 8 32769 67305986 0 0 4278190080",
    "step 133.5",
    "shared_hash",
  ]);
  const cannot = (change: (view: DataView, bytes: Uint8Array) => void): string => {
    try {
      checkReplayable(readRecording(bytesOf(change), 30));
    } catch (error) {
      return error instanceof Error ? error.message : String(error);
    }
    return "replayable";
  };
  const start = "this build cannot replay this recording's ";
  expect(cannot(() => {})).toBe("replayable");
  expect(cannot((view) => view.setUint32(42, 1, true))).toBe(`${start}start tick: a match starts at tick 0`);
  expect(cannot((view) => view.setUint16(46, 1, true))).toBe(`${start}first command sequence: a match starts at sequence 0`);
  expect(cannot((view) => view.setUint8(81, 1))).toBe(`${start}content: this build takes none`);
  expect(cannot((view) => view.setUint32(HEADER_BYTES + 32, 1, true))).toBe(`${start}move: the input call does not take it yet`);
  expect(cannot((view) => view.setUint32(HEADER_BYTES + 2 * STEP_BYTES + 56, 1, true))).toBe(`${start}aim: the input call does not take it yet`);
  expect(cannot((view) => view.setUint32(HEADER_BYTES + 60, 1, true))).toBe(`${start}view time: the input call does not take it yet`);
  expect(cannot((view) => view.setUint32(HEADER_BYTES + 64, 1, true))).toBe(`${start}view time: the input call does not take it yet`);
});

test("the first difference is the first step whose shared hash differs, or that one side lacks", () => {
  const expected = [1n, 2n, 3n].map((shared, index) => ({ tick: index + 1, shared, whole: 0n }));
  expect(firstDifference(expected, [1n, 2n, 3n])).toBeNull();
  expect(firstDifference(expected, [1n, 9n, 3n])).toBe(2);
  expect(firstDifference(expected, [1n, 2n])).toBe(3);
  expect(firstDifference(expected, [1n, 2n, 3n, 4n])).toBe(4);
  expect(firstDifference([], [])).toBeNull();
});
