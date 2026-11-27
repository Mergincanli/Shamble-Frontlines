// The content reader's proofs in WebAssembly, under Node (content items 1 and 2): the checks of
// crates/devtools/tests/content_proofs.rs, run by the test-only module in its release and its dev profile. The
// number corpus and the tree corpus hold the reader and the writer against an engine's own JSON.parse and
// JSON.stringify; the verdicts file holds this module's verdict on every class of broken file, and on the fuzz run's
// first inputs, against the native ones, digest by digest. A missing module fails this file at its imports.
import { readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";
import * as dev from "../../../.gate/rules-module/dev/devtools.js";
import * as release from "../../../.gate/rules-module/release/devtools.js";
import { readPlainFile } from "./plain-file.ts";

// The committed vectors, each read only when it is a plain file in a plain folder (plain-file.ts).
const vectors = new URL("../../../crates/devtools/vectors/", import.meta.url);
const vector = (name) => new TextDecoder("utf-8", { fatal: true }).decode(readPlainFile(vectors, name));
const [corpus, trees, verdicts] = ["content-corpus.txt", "content-trees.txt", "content-verdicts.txt"].map(vector);
// The dev profile is not optimised: its own limit, far above what it took here (the printed lines).
const LIMIT_MS = 300_000;
// The module's memory only grows. Its heaviest input is a bundle at the 8 MiB cap, held three times at once (its
// files as the test makes them, its text, and the tree the loader reads it into): 49 MiB when this was written.
// A backstop against gross growth, and no more: the generator's own inputs set the high-water mark, so a loader
// that holds twice a string's size ends at the same number (planted by test-engineer). What the loader holds for
// each input is proven natively, by the counting allocator of crates/devtools/tests/content_fuzz.rs.
const MEMORY_CEILING = 64 * 1024 * 1024;
// What the inputs at the caps may take in WebAssembly, per profile: about twenty times what they took when this was
// written (226 ms and 3,374 ms for the seven of them; the printed lines). A backstop too: a slow machine passes and a loader that hangs
// does not; a search that walks every pair of keys stays under it, and is held natively by the test of the two key
// shapes in the same file.
const LIMITS_MS = { release: 5_000, dev: 60_000 };

describe.each([["release", release], ["dev", dev]])("the content reader's proofs, %s profile", (profile, glue) => {
  const wasm = glue.initSync({ module: readFileSync(new URL(`../../../.gate/rules-module/${profile}/devtools_bg.wasm`, import.meta.url)) });

  test("the reader gives every number's bits and the writer every number's and tree's text as the engine does", { timeout: LIMIT_MS }, () => {
    expect(glue.content_corpus(corpus)).toBe("");
    expect(glue.content_trees(trees)).toBe("");
    // The checks run in the Wasm and can fail: one changed bit of the first number, one tree's layout changed.
    expect(glue.content_corpus(corpus.replace("\n402c000000000000 14 14\n", "\n402c000000000001 14 14\n"))).toContain("the reader gives 402c000000000000");
    expect(glue.content_trees(trees.replace('\n  "i6yw": "Z]9b}",\n', '\n    "i6yw": "Z]9b}",\n'))).toContain("number 1,");
  });

  test("every class of broken file, and the fuzz run's first inputs, get the native verdicts", { timeout: LIMIT_MS }, () => {
    const started = performance.now();
    expect(glue.content_verdicts(verdicts)).toBe("");
    console.log(`the content verdicts, ${profile} profile: ${Math.round(performance.now() - started)} ms`);
    // The compare bites: a file made with another seed is not this build's, class by class.
    expect(glue.content_verdicts("shamble-content-verdicts 1 5 2\na valid file|2|0000000000000000\n")).toMatch(/^this build gives\n {2}a valid file\|2\|[0-9a-f]{16}\nand the file has\n {2}a valid file\|2\|0{16}$/);
  });

  test("at its caps the loader gives each input its verdict, in time, and the module's memory stays under its ceiling", { timeout: LIMIT_MS }, () => {
    const started = performance.now();
    expect(glue.content_limits()).toBe("");
    const took = Math.round(performance.now() - started);
    const bytes = wasm.memory.buffer.byteLength;
    console.log(`the content limits, ${profile} profile: ${took} ms, the module's memory at ${bytes} bytes`);
    expect(bytes).toBeLessThan(MEMORY_CEILING);
    expect(took).toBeLessThan(LIMITS_MS[profile]);
  });
});

test("the number corpus is this engine's own: its JSON.parse gives each line's bits, its JSON.stringify each line's text", () => {
  const view = new DataView(new ArrayBuffer(8));
  const lines = corpus.split("\n").slice(1, -1);
  expect(lines.length).toBe(26_653);
  const wrong = lines.filter((line) => {
    const [bits, text, canonical] = line.split(" ");
    view.setFloat64(0, JSON.parse(text));
    return view.getBigUint64(0).toString(16).padStart(16, "0") !== bits || JSON.stringify(JSON.parse(text)) !== canonical;
  });
  expect(wrong).toEqual([]);
});
