// The corpora for the content reader's proof: the committed files are what the script makes, a line holds the
// engine's own answers, and a tree's second text has its keys in byte order at every level.
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterAll, beforeAll, expect, test } from "vitest";
import { bitsOf, lineOf, run, sortedKeys, treesFile } from "./corpus-texts.mjs";

let dir;
beforeAll(() => {
  dir = mkdtempSync(path.join(tmpdir(), "shamble-corpus-"));
});
afterAll(() => rmSync(dir, { recursive: true, force: true }));

test("the committed corpora are what the script makes", () => {
  expect(run(true)).toBe("crates/devtools/vectors/content-corpus.txt and crates/devtools/vectors/content-trees.txt are what this script makes");
});

test("a line is the double's bits, the text as given, and the engine's own text of the double", () => {
  expect(lineOf("0.55")).toBe("3fe199999999999a 0.55 0.55");
  expect(lineOf("1.50")).toBe("3ff8000000000000 1.50 1.5");
  expect(lineOf("-0")).toBe("8000000000000000 -0 0");
  expect(lineOf("1E21")).toBe("444b1ae4d6e2ef50 1E21 1e+21");
  // Exactly between two shortest texts: the engine writes the even one.
  expect(lineOf("89639294043007.125")).toBe("42d461b191e0dfc8 89639294043007.125 89639294043007.12");
  expect(bitsOf(5e-324)).toBe("0000000000000001");
  expect(() => lineOf("1e400")).toThrow("is not a finite number");
  expect(() => lineOf('"1"')).toThrow("is not a finite number");
});

test("a tree is written once as it came and once with its keys in byte order at every level", () => {
  expect(JSON.stringify(sortedKeys({ b: [{ z: 1, a: 2 }], a: { d: "x", c: [] } }))).toBe('{"a":{"c":[],"d":"x"},"b":[{"a":2,"z":1}]}');
  const records = treesFile(3).split("\n\n");
  expect(records.length).toBe(3);
  expect(records[0].split("\n")[0]).toBe("shamble-content-trees 1 3");
  for (const record of records.map((record, at) => record.split("\n").slice(at === 0 ? 1 : 0))) {
    const [compact, ...pretty] = record;
    expect(JSON.stringify(sortedKeys(JSON.parse(compact)), null, 2)).toBe(pretty.join("\n").trimEnd());
  }
});

test("the check fails on a corpus that was changed, and names the way out", () => {
  const [corpus, trees] = [path.join(dir, "corpus.txt"), path.join(dir, "trees.txt")];
  expect(run(false, corpus, trees)).toMatch(/written$/);
  expect(run(true, corpus, trees)).toMatch(/are what this script makes$/);
  writeFileSync(trees, readFileSync(trees, "utf8").replace("{", "{ "));
  expect(() => run(true, corpus, trees)).toThrow("trees.txt is not what this script makes");
  expect(run(false, corpus, trees)).toMatch(/written$/);
  writeFileSync(corpus, readFileSync(corpus, "utf8").replace(" 0.55 0.55\n", " 0.55 0.550\n"));
  expect(() => run(true, corpus, trees)).toThrow("corpus.txt is not what this script makes");
});
