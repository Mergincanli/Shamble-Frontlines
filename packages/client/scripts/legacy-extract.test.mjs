// The extract of the old tuning file's numbers: the committed file is what the script makes, and the script reads a
// number as its text is written.
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterAll, beforeAll, expect, test } from "vitest";
import { numbersFile, numberTexts, readPlain, run, writePlain } from "./legacy-extract.mjs";

let dir;
beforeAll(() => {
  dir = mkdtempSync(path.join(tmpdir(), "shamble-legacy-"));
});
afterAll(() => rmSync(dir, { recursive: true, force: true }));

test("the committed extract is what the script makes from the committed tuning file", () => {
  expect(run(true)).toBe("crates/devtools/vectors/legacy-numbers.txt is what this script makes");
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
  const tuning = path.join(dir, "tuning.json");
  const numbers = path.join(dir, "numbers.txt");
  writeFileSync(tuning, '{"a": 1, "b": 2.5}');
  expect(run(false, tuning, numbers)).toMatch(/2 numbers written$/);
  expect(readFileSync(numbers, "utf8").split("\n").slice(1)).toEqual(["1", "2.5", ""]);
  expect(run(true, tuning, numbers)).toMatch(/is what this script makes$/);
  writeFileSync(numbers, readFileSync(numbers, "utf8").replace("2.5", "2.50"));
  expect(() => run(true, tuning, numbers)).toThrow("is not what this script makes");
  // A tuning file that changed: the first line's digest no longer matches.
  expect(run(false, tuning, numbers)).toMatch(/written$/);
  writeFileSync(tuning, '{"a": 1, "b": 2.5} ');
  expect(() => run(true, tuning, numbers)).toThrow("is not what this script makes");
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
