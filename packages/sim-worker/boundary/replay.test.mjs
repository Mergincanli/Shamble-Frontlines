// The recorded matches in WebAssembly, under Node (ADR 0004's milestone and foundation line; PM-0004-17, -19, -21):
// on the file players get, every committed recording replays to its hashes file's shared column through the
// bindings' own calls; on the test-only module, the same runner as the native one gives the whole hashes file and the
// state at the vector's tick, byte for byte the committed text. Like the other boundary tests it needs the build and
// never skips. The recordings are read from disk when the test runs, each with the frozen content its header names
// (`<name>.content.json`).
import { createHash } from "node:crypto";
import { readdirSync, readFileSync } from "node:fs";
import { expect, test } from "vitest";
import * as dev from "../../../.gate/rules-module/dev/devtools.js";
import * as release from "../../../.gate/rules-module/release/devtools.js";
import { initSync, Sim, tick_hz } from "../pkg/sim_wasm.js";
import { checkContent, firstDifference, readHashes, readRecording, replayOn, replayStep } from "../src/recording.ts";
import { readPlainFile } from "./plain-file.ts";

// The committed recordings, each read only when it is a plain file in a plain folder (plain-file.ts).
const recordings = new URL("../../../crates/devtools/recordings/", import.meta.url);
const file = (name) => Buffer.from(readPlainFile(recordings, name));
const names = readdirSync(recordings).filter((name) => name.endsWith(".rec")).map((name) => name.slice(0, -4));
const VECTOR_TICK = 900;
initSync({ module: readFileSync(new URL("../pkg/sim_wasm_bg.wasm", import.meta.url)) });
const modules = [["release", release], ["dev", dev]].map(([profile, glue]) => {
  glue.initSync({ module: readFileSync(new URL(`../../../.gate/rules-module/${profile}/devtools_bg.wasm`, import.meta.url)) });
  return [profile, glue];
});

/** A committed recording, read and checked against its hashes file's first line, with its frozen content. */
function committed(name) {
  const bytes = file(`${name}.rec`);
  const hashes = readHashes(file(`${name}.hashes`).toString("utf8"));
  expect(hashes.recordingSha256).toBe(createHash("sha256").update(bytes).digest("hex"));
  return { bytes, recording: readRecording(bytes, tick_hz()), expected: hashes.lines, content: file(`${name}.content.json`) };
}

test("the manifest lists every recording's files with their length and digest, and there is at least one", () => {
  expect(names).toContain("first-loop");
  const manifest = file("manifest.txt").toString("utf8");
  for (const name of names.flatMap((name) => [`${name}.content.json`, `${name}.rec`, `${name}.hashes`, `${name}.vector.txt`])) {
    const bytes = file(name);
    expect(manifest).toContain(`${name}: bytes ${bytes.length}  sha256 ${createHash("sha256").update(bytes).digest("hex")}\n`);
  }
});

test.each(names)("%s replays on the players' file to its hashes file, step for step", async (name) => {
  const { recording, expected, content } = committed(name);
  // The frozen content beside it is the content its header names; any other bundle is refused before a match exists.
  await checkContent(recording, content);
  const other = Uint8Array.from(content);
  other[other.length - 2] ^= 1;
  await expect(checkContent(recording, other)).rejects.toThrow("the content is not the recording's: its sha256 is not the content hash the header names");
  // The match is made on that content, as every match is: the bundle crosses once, through the constructor.
  const sim = new Sim(recording.seed, 0, 0, content);
  const got = replayOn(sim, recording);
  expect(got).toHaveLength(recording.steps.length);
  expect(firstDifference(expected, got)).toBeNull();
  sim.free();

  // The check can fail, and it names the step. A replay that leaves out the input call of step 46 differs at step 46
  // (its post brings other held keys than the step before: the script's second stretch starts there)...
  const skipping = new Sim(recording.seed, 0, 0, content);
  skipping.rebase(recording.steps[0].hostMs);
  const skipped = recording.steps.map((step, index) => {
    if (index !== 45) return replayStep(skipping, step);
    skipping.step(step.hostMs);
    return skipping.shared_hash();
  });
  expect(firstDifference(expected, skipped)).toBe(46);
  skipping.free();
  // ...and one changed digit of the file's own line is found at its step.
  const changed = expected.map((line, index) => (index === 899 ? { ...line, shared: line.shared ^ 1n } : line));
  expect(firstDifference(changed, got)).toBe(900);
});

test("two matches on one module instance, stepped in turn, each give the hashes they give alone", () => {
  const { recording, expected, content } = committed("first-loop");
  // A second match of another seed and another input stream, on the same content: the first one's steps from step
  // 600 on, on the first one's own times.
  const turned = recording.steps.map((step, index) => ({ ...recording.steps[(index + 600) % recording.steps.length], hostMs: step.hostMs }));
  const second = { ...recording, seed: Uint8Array.from(recording.seed, (byte, at) => (at < 8 ? byte : byte ^ 0x5a)), steps: turned };
  const alone = new Sim(second.seed, 0, 0, content);
  const secondAlone = replayOn(alone, second);
  alone.free();
  expect(firstDifference(expected, secondAlone)).not.toBeNull();

  const [a, b] = [new Sim(recording.seed, 0, 0, content), new Sim(second.seed, 0, 0, content)];
  a.rebase(recording.steps[0].hostMs);
  b.rebase(second.steps[0].hostMs);
  const together = [[], []];
  let next = 0;
  for (const [index, step] of recording.steps.entries()) {
    together[0].push(replayStep(a, step));
    // Unevenly: none, one or two steps of the second match after each step of the first.
    for (let more = 0; more < index % 3 && next < second.steps.length; more += 1) together[1].push(replayStep(b, second.steps[next++]));
  }
  while (next < second.steps.length) together[1].push(replayStep(b, second.steps[next++]));
  expect(firstDifference(expected, together[0])).toBeNull();
  expect(together[1]).toEqual(secondAlone);
});

test.each(modules)("the test-only module, %s profile, gives the committed hashes and the state at the vector's tick", (_, glue) => {
  for (const name of names) {
    const bytes = file(`${name}.rec`);
    const content = file(`${name}.content.json`);
    // Both columns: the whole state's hash too, which the players' file never gives.
    expect(glue.replay_hashes(bytes, content)).toBe(file(`${name}.hashes`).toString("utf8"));
    // The state field by field and its shared bytes (the word-size vector): the same text as the native one.
    expect(glue.replay_state_at(bytes, content, VECTOR_TICK)).toBe(file(`${name}.vector.txt`).toString("utf8"));
    expect(glue.replay_state_at(bytes, content, 0)).toContain("tick = 0\nplayer.buttons = 0\n");
    expect(glue.replay_state_at(bytes, content, 4_000_000)).toBe("error: the recording never reaches tick 4000000");
  }
  // What is no recording, or content that is not the recording's, comes back as a reason, never a trap.
  const content = file("first-loop.content.json");
  expect(glue.replay_hashes(new Uint8Array(5), content)).toBe("error: the bytes end inside the recording's header");
  const real = Uint8Array.from(file("first-loop.rec"));
  expect(glue.replay_hashes(real, new Uint8Array(0))).toBe("error: the content is not the recording's: its sha256 is not the content hash the header names");
  real.set(new TextEncoder().encode("REALSEED"), 10);
  expect(glue.replay_hashes(real, content)).toContain("error: the seed does not start with TESTSEED");
  expect(glue.replay_state_at(real, content, 1)).toContain("error: the seed does not start with TESTSEED");
});

test("the state hash's cost in WebAssembly is read and printed (PM-0004-21's report: a reading)", () => {
  const rounds = 20_000;
  const lines = [];
  for (const length of [22, 54, 4_096, 16_384]) {
    release.fnv1a_64_rounds(length, 200);
    const start = performance.now();
    const folded = release.fnv1a_64_rounds(length, rounds);
    const nanos = ((performance.now() - start) * 1e6) / rounds;
    lines.push(`  fnv1a_64 over ${String(length).padStart(6)} bytes: mean ${nanos.toFixed(1).padStart(9)} ns`);
    expect(release.fnv1a_64_rounds(length, rounds)).toBe(folded);
  }
  console.log(`state hash cost, WebAssembly in Node ${process.version}, release module (a reading):\n${lines.join("\n")}`);
});
