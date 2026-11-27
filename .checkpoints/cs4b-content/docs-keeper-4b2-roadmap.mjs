// docs-keeper, 4b-2 docs pass: ROADMAP. Fold the two oldest "Recent work" entries (ci-wasm, cs3a) into "Build
// history" after cs2's entry, byte for byte, by line ranges; add 4b-2's entry at the top of Recent work, merge
// pointers (lines added, no word changed) under the #78, #75 and MAIN-WALL entries, and a dated "Next / open"
// pointer. Run once from the root.
import { readFileSync, writeFileSync } from "node:fs";

const F = "docs/ROADMAP.md";
const r = readFileSync(F, "utf8").split("\n");
const L = (n) => r[n - 1];
const expect = (n, start) => { if (!L(n).startsWith(start)) throw new Error(`line ${n}: ${L(n).slice(0, 70)}`); };
expect(243, "**Change-set 2 of the build plan, First page");
expect(252, "main (gate run 934 with smoke, toolchain run 232).");
expect(253, ""); expect(254, "- **Kit: KEPT-LOGS (2026-11-15):**");
expect(285, "- **Pointer, 2026-11-26 (change-set 4b-1's docs pass; not his words):**");
expect(289, "  then 4c, 4d and the link simulator");
expect(290, "- **Superseded by v2**");
expect(306, "## Recent work"); expect(308, "");
expect(309, "**CI-TIME [2026-11-27, reviewed]**");
expect(312, "approved; it merges on his go. Report: `docs/reports/2026-11-27-ci-time.md`.");
expect(313, "");
expect(314, "**Change-set 4b-1 of the build plan, Content [2026-11-26, reviewed]**");
expect(321, "planted defects 35 of 35 caught.");
expect(322, "");
expect(323, "**Main's wall [2026-11-26, the wall on; docs in flight]**");
expect(329, "`docs/reports/2026-11-26-main-wall.md`; TECHNICAL §8c.");
expect(330, "");
expect(424, "");
expect(425, "**Change-set 3a of the build plan, Rules and boundaries");
expect(440, "UTC).");
expect(441, "");
expect(442, "**CI builds the WebAssembly and runs the tests that need it");
expect(451, "run 45373057216 with smoke, toolchain run 45373056953).");
if (r.length !== 452 || r[451] !== "") throw new Error(`length ${r.length}`);

const range = (a, b) => r.slice(a - 1, b); // inclusive, 1-based
const cs3a = range(425, 440), ciWasm = range(442, 451);

const entry = [
  "**Change-set 4b-2 of the build plan, The world [2026-11-26 to 11-27, reviewed]** — cs4b-content, the second of 4b's",
  "two pull requests (his SIZE-4B (b)), by lane 1 (Builder V4, then the chat he calls Builder V6), steps 6 to 11: the",
  "collision library (parry3d `=0.31.1`, behind one module of `sim`) with its pinned batch and dependency rules; the",
  "built-file check on the players' WebAssembly (no fused multiply-add, vector instructions, host maths or foreign",
  "imports); his Testlines Export at 30 steps as the soldier's movement table, written by script (his DIFF-LIST (a)),",
  "and the test scene's map; one content bundle per match, judged by the game's loader in the constructor; the test scene's static",
  "collision world, the same tree and rays natively, under Node and in the browsers; the content kinds' TypeScript",
  "types and JSON Schemas. Step 12, the page's scene with the map's reading field by field, moved to the start of 4c",
  "(his STEP12-GO (a)). 8,408 written lines against the stop line 8,450; nothing a player sees changes, no browser",
  "check. Two review rounds, Lows left as follow-ups (STATUS); planted defects 104 of 104 caught, 19 controls held.",
  "",
];
const ptr78 = "_Pointer (2026-11-27, 4b-2's docs pass): merged as PR #78, merge commit b8f02b0 (its own time 13:55:47 UTC, read with `git log`)._";
const ptr75 = "_Pointer (2026-11-27, 4b-2's docs pass): merged as PR #75, merge commit b9aad01 (its own time 2026-11-26 18:53:13 UTC, read with `git log`)._";
const ptrWall = "_Pointer (2026-11-27, 4b-2's docs pass): its docs merged as PR #76, merge commit 2671595 (2026-11-26 18:00:25 UTC), and T10's record as PR #77, 47bb79e (18:52:17 UTC), read with `git log`._";
const next = [
  "- **Pointer, 2026-11-27 (change-set 4b-2's docs pass; not his words):** change-set 4b-1 is merged (PR #75); 4b-2,",
  "  the world, is built (steps 6 to 11) and reviewed; the page drawing the scene from the map, with the map's reading",
  "  field by field, moved to the start of 4c by his STEP12-GO (a) (`docs/DECISIONS.md`, 2026-11-27, 20:03:18 UTC);",
  "  then 4c, 4d and the link simulator (`docs/STATUS.md`, \"Next\").",
];

// Bottom up.
const out = r.slice(0, 423);               // through line 423; lines 424..451 (the two folded entries) go
out.push("");                               // the file ends with one newline after line 423
out.splice(329, 0, ptrWall);                // after line 329
out.splice(321, 0, ptr75);                  // after line 321
out.splice(312, 0, ptr78);                  // after line 312
out.splice(308, 0, ...entry);               // before line 309
out.splice(289, 0, ...next);                // after line 289
out.splice(253, 0, ...ciWasm, "", ...cs3a, ""); // before line 254, after the empty 253
if (out[out.length - 2] === "") throw new Error("trailing empty line");
writeFileSync(F, out.join("\n"));

// Byte-for-byte check of the folds, and the count.
const text = out.join("\n");
for (const [name, b] of [["ci-wasm", ciWasm], ["cs3a", cs3a]]) {
  const block = b.join("\n");
  if (text.split(block).length !== 2) throw new Error(`${name} not exactly once`);
  if (text.indexOf(block) > text.indexOf("## Current state")) throw new Error(`${name} not in Build history`);
}
const recent = out.slice(out.indexOf("## Recent work")).filter((l) => l.startsWith("**")).length;
console.log("Recent work entries:", recent, "lines:", out.length - 1);
