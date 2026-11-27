// The canonical-form helper on scratch files: the form it gives, what it refuses, and where it will not write.
import { mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterAll, beforeAll, expect, test } from "vitest";
import { canonicalText, CONTENT_PATH, run, sorted } from "./canonical.mjs";

let base;
beforeAll(() => {
  base = mkdtempSync(path.join(tmpdir(), "shamble-canonical-"));
  for (const folder of ["content/maps", "content/source", "outside"]) mkdirSync(path.join(base, folder), { recursive: true });
});
afterAll(() => rmSync(base, { recursive: true, force: true }));

const HAND = '{"spawns": [{"id": "a", "at": [0, 0.0, -5.50], "facing_deg": 90}], "bounds": {"min": [-50, 0, -50], "max": [5e1, 20, 50]},\r\n "regions": [], "id": "yard"}';
const CANONICAL = `{
  "bounds": {
    "max": [
      50,
      20,
      50
    ],
    "min": [
      -50,
      0,
      -50
    ]
  },
  "id": "yard",
  "regions": [],
  "spawns": [
    {
      "at": [
        0,
        0,
        -5.5
      ],
      "facing_deg": 90,
      "id": "a"
    }
  ]
}
`;

test("a hand-written file comes out with sorted keys at every depth, lists in their order, and each number in its shortest form", () => {
  expect(canonicalText(HAND)).toBe(CANONICAL);
  expect(canonicalText(CANONICAL)).toBe(CANONICAL);
  expect(sorted({ b: [{ d: 1, c: 2 }], a: null })).toEqual({ a: null, b: [{ c: 2, d: 1 }] });
  expect(Object.keys(sorted({ b: 1, a: 2, B: 3 }))).toEqual(["B", "a", "b"]);
  expect(() => canonicalText('{"at": [-0, 1]}')).toThrow("a minus zero");
  expect(() => canonicalText("{")).toThrow();
});

test("the file is rewritten in place once, and a second run leaves it alone", () => {
  const file = path.join(base, "content/maps/yard.json");
  writeFileSync(file, HAND);
  expect(run("maps/yard.json", base)).toBe(`content/maps/yard.json: rewritten in canonical form (${Buffer.byteLength(HAND)} bytes before, ${Buffer.byteLength(CANONICAL)} after)`);
  expect(readFileSync(file, "utf8")).toBe(CANONICAL);
  const changed = statSync(file).mtimeMs;
  expect(run("maps/yard.json", base)).toBe("content/maps/yard.json: already in canonical form");
  expect(statSync(file).mtimeMs).toBe(changed);
});

test("only a content path is taken: no file path, no source folder, no link", () => {
  for (const argument of [undefined, "", "maps/../source/export.json", "../outside/x.json", "/etc/passwd", "source/export.json", "maps/Yard.json", "maps/yard.txt", "maps/a/b.json", "weapons/smg.json", path.join(base, "content/maps/yard.json")]) {
    expect(() => run(argument, base)).toThrow("usage: canonical.mjs <maps|movement>/<id>.json");
  }
  expect(CONTENT_PATH.test("maps/test-scene.json") && CONTENT_PATH.test("movement/soldier.json")).toBe(true);
  // A link in the file's place leads outside: refused, and what it leads to is untouched.
  const outside = path.join(base, "outside/x.json");
  writeFileSync(outside, HAND);
  symlinkSync(outside, path.join(base, "content/maps/linked.json"));
  expect(() => run("maps/linked.json", base)).toThrow("is a link or not a plain file: it is not read");
  expect(readFileSync(outside, "utf8")).toBe(HAND);
});

test("the content folder as a link: nothing is read through it", () => {
  // Every folder from the base down is held to be no link (security-auditor's round 2): content/ itself as a link is
  // refused before the read, though the folder and the canonical file behind it are plain.
  const linked = mkdtempSync(path.join(tmpdir(), "shamble-canonical-linked-"));
  mkdirSync(path.join(linked, "real/maps"), { recursive: true });
  writeFileSync(path.join(linked, "real/maps/yard.json"), CANONICAL);
  symlinkSync(path.join(linked, "real"), path.join(linked, "content"));
  expect(() => run("maps/yard.json", linked)).toThrow(`${path.join(linked, "content")} is a link or not a plain folder: it is not read`);
  rmSync(linked, { recursive: true, force: true });
});
