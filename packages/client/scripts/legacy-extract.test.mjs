// The extracts of the old tuning file: the committed files are what the script makes, the script reads a number as
// its text is written, and a duration is a field of the reviewed list, never a guess from a name.
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterAll, beforeAll, expect, test } from "vitest";
import { durationsFile, numberPaths, numbersFile, numberTexts, readPlain, run, writePlain } from "./legacy-extract.mjs";

let dir;
beforeAll(() => {
  dir = mkdtempSync(path.join(tmpdir(), "shamble-legacy-"));
});
afterAll(() => rmSync(dir, { recursive: true, force: true }));

test("the committed extracts are what the script makes from the committed tuning file and the reviewed list", () => {
  expect(run(true)).toBe("crates/devtools/vectors/legacy-numbers.txt (2038) and crates/devtools/vectors/legacy-durations.txt (69) are what this script makes");
});

test("a duration is a field of the reviewed list, at every place the tuning file has it, as written", () => {
  const tuning = '{"T": {"fuse": 1.50, "speed": 6.3}, "W": [{"reload": 2, "mag": 30}, {"reload": 0.072}], "MS": 280}';
  expect(numberPaths(tuning)).toEqual([["T.fuse", "1.50"], ["T.speed", "6.3"], ["W[0].reload", "2"], ["W[0].mag", "30"], ["W[1].reload", "0.072"], ["MS", "280"]]);
  const list = "# a comment\nT.fuse\ts\trules\tgame.js:1\nW[*].reload\ts\trules\tgame.js:2\nMS\tms\tlook\tcore.js:3\n";
  const made = durationsFile(Buffer.from(tuning), list).split("\n");
  expect(made[0]).toMatch(/^shamble-legacy-durations 1 [0-9a-f]{64} 4$/);
  expect(made.slice(1)).toEqual(["T.fuse\ts\trules\t1.50", "W[0].reload\ts\trules\t2", "W[1].reload\ts\trules\t0.072", "MS\tms\tlook\t280", ""]);
  // A list that names a field the file does not have, a unit or a kind that is none, or a field twice: refused.
  expect(() => durationsFile(Buffer.from(tuning), "T.fuze\ts\trules\tx\n")).toThrow("names fields the tuning file does not have: T.fuze");
  expect(() => durationsFile(Buffer.from(tuning), "T.fuse\tseconds\trules\tx\n")).toThrow("no field, unit and kind");
  expect(() => durationsFile(Buffer.from(tuning), "T.fuse\ts\tplay\tx\n")).toThrow("no field, unit and kind");
  expect(() => durationsFile(Buffer.from(tuning), "T.fuse\ts\trules\tx\nT.fuse\ts\tlook\tx\n")).toThrow("a field twice");
});

test("a number is taken as its text is written, in the text's order, and nothing else is taken", () => {
  const text = '{"a": 1.50, "b": [1e5, -0, 0.1], "c": {"d": "7", "e": true, "f": null, "g": 6.3}, "h": 1E-7}';
  expect(numberTexts(text)).toEqual(["1.50", "1e5", "-0", "0.1", "6.3", "1E-7"]);
  const small = '{"a": 2, "b": 0.5}';
  const digest = createHash("sha256").update(small).digest("hex");
  expect(numbersFile(Buffer.from(small))).toBe(`shamble-legacy-numbers 1 ${digest} 2\n2\n0.5\n`);
  // The digest in the first line is the tuning file's own: one byte more changes it.
  const first = (bytes) => numbersFile(Buffer.from(bytes)).split("\n")[0];
  expect(first('{"a": 2}')).toMatch(/^shamble-legacy-numbers 1 [0-9a-f]{64} 1$/);
  expect(first('{"a": 2} ')).not.toBe(first('{"a": 2}'));
});

test("the check fails on a changed extract, and names the way out", () => {
  const [tuning, numbers, paths, durations] = ["tuning.json", "numbers.txt", "paths.txt", "durations.txt"].map((name) => path.join(dir, name));
  const files = [tuning, numbers, paths, durations];
  writeFileSync(tuning, '{"a": 1, "b": 2.5}');
  writeFileSync(paths, "b\ts\trules\tthe test's own\n");
  expect(run(false, ...files)).toMatch(/numbers\.txt \(2\) and .*durations\.txt \(1\) written$/);
  expect(readFileSync(numbers, "utf8").split("\n").slice(1)).toEqual(["1", "2.5", ""]);
  expect(readFileSync(durations, "utf8").split("\n").slice(1)).toEqual(["b\ts\trules\t2.5", ""]);
  expect(run(true, ...files)).toMatch(/are what this script makes$/);
  writeFileSync(numbers, readFileSync(numbers, "utf8").replace("2.5", "2.50"));
  expect(() => run(true, ...files)).toThrow("numbers.txt is not what this script makes");
  // A reviewed list that changed: the durations extract is no longer what the script makes.
  expect(run(false, ...files)).toMatch(/written$/);
  writeFileSync(paths, "b\ts\tlook\tthe test's own\n");
  expect(() => run(true, ...files)).toThrow("durations.txt is not what this script makes");
  // A tuning file that changed: the first line's digest no longer matches.
  expect(run(false, ...files)).toMatch(/written$/);
  writeFileSync(tuning, '{"a": 1, "b": 2.5} ');
  expect(() => run(true, ...files)).toThrow("is not what this script makes");
});

test("neither side is read or written through a link", () => {
  const target = path.join(dir, "outside.txt");
  writeFileSync(target, "kept");
  const link = path.join(dir, "link.txt");
  symlinkSync(target, link);
  expect(() => readPlain(link)).toThrow("is not a plain file");
  expect(() => writePlain(link, "changed")).toThrow("is not a plain file");
  expect(() => readPlain(dir)).toThrow("is not a plain file");
  expect(readFileSync(target, "utf8")).toBe("kept");
});
