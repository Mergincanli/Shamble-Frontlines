// docs-keeper, 4b-1 docs pass: ROADMAP. Fold the four oldest "Recent work" entries into "Build history" by line
// ranges, byte for byte; add 4b-1's entry, two merge pointers and a "Next / open" pointer. Run once from the root.
import { readFileSync, writeFileSync } from "node:fs";

const F = "docs/ROADMAP.md";
const r = readFileSync(F, "utf8").split("\n");
const L = (n) => r[n - 1];
const expect = (n, start) => { if (!L(n).startsWith(start)) throw new Error(`line ${n}: ${L(n).slice(0, 70)}`); };
expect(219, "");
expect(220, "- **Kit: KEPT-LOGS (2026-11-15):**");
expect(250, "  4a the recording, 4b content and the world");
expect(251, "- **Superseded by v2**");
expect(268, "*(Newest first, a few lines each.");
expect(270, "**The preview hand-over step [2026-11-26, in flight]**");
expect(277, "`docs/reports/2026-11-26-ci-preview-handover.md`");
expect(279, "**The second guard fix: nothing in front of git or gh unread");
expect(286, "then the review tracks. Report `docs/reports/2026-11-26-guard-wrappers.md`.");
expect(278, ""); expect(287, ""); expect(377, "UTC).");
expect(379, "**CI builds the WebAssembly and runs the tests that need it");
expect(390, "**Change-set 2 of the build plan, First page");
expect(401, "**The kit hold and two CI edits");
expect(413, "**Change-set 1 of the build plan, Types");
if (r.length !== 422 && !(r.length === 423 && r[422] === "")) throw new Error(`length ${r.length}`);
const end = r[r.length - 1] === "" ? r.length - 1 : r.length; // last content line count

const block = (a, b) => r.slice(a - 1, b); // inclusive, 1-based
// Entries: [first, last content line]; each is followed by one empty line except the last of the file.
const ciWasm = block(379, 388), cs2 = block(390, 399), kitHold = block(401, 411), cs1 = block(413, end);
for (const [name, b, next] of [["ci-wasm", ciWasm, 389], ["cs2", cs2, 400], ["kit-hold", kitHold, 412]]) {
  if (L(next) !== "") throw new Error(`${name}: line ${next} not empty`);
  if (b.at(-1) === "") throw new Error(`${name}: ends empty`);
}

const entry = [
  "**Change-set 4b-1 of the build plan, Content [2026-11-26, in review]** — cs4b-content, the first of 4b's two pull",
  "requests (his SIZE-4B (b)), by lane 1, the chat he calls Builder V4: the content crate with no dependency (one reader",
  "for a cut grammar of JSON that refuses everything else, the canonical writer, the bundle with the movement and map",
  "kinds, the map's exact rules and the aim's reach, the time unit of 3,000 a second), held against a browser engine's",
  "own numbers and trees and by a bounded fuzz run, natively, under Node and in the browsers; the links check (his",
  "LINKS-CHECK (c)). 6,591 written lines against the stop line 7,450; no browser check. In review round 1. Next: 4b-2.",
  "",
];
const ptr74 = "_Pointer (2026-11-26, 4b-1's docs pass): merged as PR #74, merge commit 3b949c1 (its own time 15:24:22 UTC, read with `git log`)._";
const ptr73 = "_Pointer (2026-11-26, 4b-1's docs pass): merged as PR #73, merge commit 4a62873 (its own time 14:56:33 UTC, read with `git log`)._";
const next = [
  "- **Pointer, 2026-11-26 (change-set 4b-1's docs pass; not his words):** change-set 4a is merged (PR #70); 4b is two",
  "  pull requests by his SIZE-4B (b) (`docs/DECISIONS.md`, 2026-11-26, 14:09:13 UTC): 4b-1, Content, is in review; next",
  "  is 4b-2 (the collision library and the static collision world, the built-file check, his Export's values into the",
  "  content and the simulation, the page drawing the scene from the map), after a short re-read by the advisor agent;",
  "  then 4c, 4d and the link simulator (`docs/STATUS.md`, \"Next\").",
];

// Bottom up.
const out = r.slice(0, 378);              // through line 378 (the empty line before ci-wasm stays as line 378)
// drop lines 379..end (the four folded entries); keep the file's last empty line
if (out[377] !== "") throw new Error("line 378 not empty");
out.pop();                                 // no empty line at the very end of Recent work's last entry before EOF
out.push("");                              // file ends with a newline
out.splice(286, 0, ptr73);                 // after line 286
out.splice(277, 0, ptr74);                 // after line 277
out.splice(269, 0, ...entry);              // before line 270
out.splice(250, 0, ...next);               // after line 250
out.splice(219, 0, ...cs1, "", ...kitHold, "", ...cs2, "", ...ciWasm, ""); // before line 220, after the empty 219
writeFileSync(F, out.join("\n"));
const recent = out.slice(out.indexOf("## Recent work")).filter((l) => l.startsWith("**")).length;
console.log("Recent work entries:", recent, "lines:", out.length - 1);
