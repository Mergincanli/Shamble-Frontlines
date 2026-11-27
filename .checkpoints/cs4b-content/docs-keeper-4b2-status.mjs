// docs-keeper, 4b-2 docs pass: STATUS and its archive. Lines about the merged #75, #76 (with #77) and #78 move to
// the archive under his OLD-LINES (a) of 2026-11-18, and lane 1's own lines whose new forms replace them, all by
// line ranges, byte for byte, in one dated block at the end of the archive's "In flight"; then the new lines; the
// archive's "From change-set 4b-2" follow-ups block, a closing pointer in 4b-1's block, and 4b-2's risks.
// `node docs-keeper-4b2-status.mjs` prints the count and writes nothing; `--write` writes, only at 120 lines or fewer.
import { readFileSync, writeFileSync } from "node:fs";

const WRITE = process.argv.includes("--write");
const S = "docs/STATUS.md", A = "docs/STATUS_ARCHIVE.md";
const s = readFileSync(S, "utf8").split("\n");
const a = readFileSync(A, "utf8").split("\n");
const L = (n) => s[n - 1];
const expect = (n, start) => { if (!L(n).startsWith(start)) throw new Error(`STATUS ${n}: ${L(n).slice(0, 70)}`); };
expect(5, "_Last updated: 2026-11-27 (ci-time: the #78 row added");
expect(13, "- **Last merged** (read with git, 2026-11-26): #74");
expect(31, "| PR | Work | Branch | State | Waiting on |");
expect(33, "| #40 |");
expect(34, "| none yet | MAIN-WALL (his words of 2026-11-25 19:36:42 UTC");
expect(35, "| #75 (draft) | change-set 4b-1, \"Content\"");
expect(36, "| #78 (draft) | CI-TIME");
expect(37, "");
expect(41, "2. v2: the netcode proof test (#40)");
expect(42, "   (a)): 4a merged; **4b-1 reviewed (#75)**;");
expect(43, "3. Lane 2 (Builder V2)");
expect(49, "- 4b-1 (#75), reviewed (its report): his go.");
expect(50, "- Later, from 4a's report: MOVE-RULES");
expect(51, "- His accounts, his words");
expect(95, "- **VISION-RULE** (his point 4c");
expect(98, "  have them on screen, hidden shooter or not), of ADR 0004 part 13 and of ADR 0003's cutaway rule.\"");
expect(99, "- **From change-set 4b-1** (the archive's block of that name; ids by its docs pass): 4B-AIM-REACH:");
expect(103, "  more ids there, among them 4B-SIGHT-MARK");
expect(104, "");
expect(113, "- **v2:** the decision drafts' risks");
expect(115, "  online play; releases worked out, not counted); 4a's");
expect(117, "  navigation; the editor and map bugs; the first-deploy tutorial;");
expect(118, "- **Preview hosting (lane 2):**");
if (s.length !== 121 || s[120] !== "") throw new Error(`STATUS length ${s.length}`);

const range = (x, y) => s.slice(x - 1, y);
const moved = { top: range(5, 5), merged: range(13, 13), rows: range(34, 36), next2: range(41, 42), wait: range(49, 50), fu: range(99, 103), risk: range(115, 117) };

// The new lines.
const top = ["_Last updated: 2026-11-27 (change-set 4b-2's docs pass, by lane 1's docs-keeper, on code 914f3b4: 4b-2 built and reviewed, its row and follow-ups; his VISION-RULE addition as pointers; the merged #75 to #78's rows, the old top line and \"Last merged\" moved to the archive by script under OLD-LINES, with lane 1's own replaced lines)._"];
const merged = ["- **Last merged** (read with git, 2026-11-27): #78, ci-time (b8f02b0); #75, 4b-1 (b9aad01); #77 and #76, main-wall (47bb79e, 2671595); #74, ci-preview-handover (3b949c1a); #73, guard-wrappers (4a62873e); #70, 4a (2483e9a); #71, ci-after-3a (07d673b)."];
const row = ["| none yet | change-set 4b-2, \"The world\" (the second of 4b's two pull requests, his SIZE-4B (b)), by lane 1 (Builder V4, then the chat he calls Builder V6): steps 6 to 11; step 12 moved to 4c by his STEP12-GO (a) (the list: REPO_MAP, ROADMAP); report `docs/reports/2026-11-27-cs4b2-world.md` | `claude/ecstatic-wozniak-5cbdzs` (stands in for `cs/4b-content`) | risk tier 1, no browser check; built (steps 6 to 11) and reviewed (two rounds); the verifier and the advisor agent's verdict on the final head are in its report | his go for the head the advisor agent approves; his Advisor opens the pull request |"];
const next2 = [
  "2. v2: the netcode proof test (#40); step 2.3's chat stays stopped; the first builder chat paused. Walk (his WALK-CUT",
  "   (a)): 4a and 4b-1 merged; **4b-2 reviewed**; then 4c (first the page's scene, moved from 4b-2), 4d, the link simulator, change-set 7 (ROADMAP, \"Next / open\").",
];
const wait = [
  "- 4b-2, built and reviewed (its report): his go. Later, from 4a's report: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; `docs/UNITS.md` §0's two lines (his unit walk).",
];
const vision = [
  "  His addition (`docs/DECISIONS.md`, 2026-11-27, 13:57:21 UTC, item 9; it sharpens version 2's point 2c; no code now) leaves open, his words: \"the exact eye height, which reaches the game through my Export, and which points of an enemy's body the line is tested against\".",
];
// 4B-AIM-REACH's words stay exactly as they are (checked below); only the bullet's head and tail change.
const fu = [
  "- **From change-sets 4b-1 and 4b-2** (the archive's blocks of those names): 4B-AIM-REACH: the loader refuses a playable map beyond",
  "  the aim's reach (x and z plus or minus 327.67 m, y 0 to 655.35 m). The aim widens in the first change-set whose map leaves that",
  "  square (none in Slice 1; Slice 2's plan says whether its maps do), at the latest in Slice 3's change-set that sets the command",
  "  codec for online play (PM-0001-51), beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet. 13 more of 4b-1's,",
  "  4B-SIGHT-MARK among them (4b-2 closed three). 4b-2's: for 4c, 4B-QUERY-RECORDING, 4B-RAY-WASM, 4B-RAY-COUNTS, 4B-WORLD-ROOM,",
  "  4B-LIMIT-IMPORT, 4B-TYPES-READER, 4B-MEMBER-KINDS; the CI Builder's 4B-CHROMIUM-TIME; hosting's 4B-CONTENT-CACHE, 4B-LICENCES;",
  "  4B-HEAPLESS; the advisor agent's 4B-FIRST-DEFAULT, 4B-UNIT-COLUMN; 4B-WALLCLOCK, 4B-SERVE-LINKS, 4B-BOOT-CALLS.",
];
const risk = [
  "  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form); 4b-1's (the fuzz run",
  "  cannot reach the caps, 4B-FUZZ-REACH); 4b-2's (four, in its report and the archive). **Legacy (frozen):** 3a's ranked list (PR #15); bot",
  "  navigation; the editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).",
];
// The AIM-REACH text, word for word, before and after (whitespace folded: the lines wrap where they did).
const fold = (lines) => lines.join(" ").replace(/\s+/g, " ");
const aim = (t) => t.slice(t.indexOf("4B-AIM-REACH:"), t.indexOf("No numbered change-set holds it yet.") + 36);
if (aim(fold(moved.fu)) !== aim(fold(fu))) throw new Error("4B-AIM-REACH's words changed");

// Bottom up.
const out = s.slice();
out.splice(115 - 1, 3, ...risk);
out.splice(99 - 1, 5, ...fu);
out.splice(98, 0, ...vision);
out.splice(49 - 1, 2, ...wait);
out.splice(41 - 1, 2, ...next2);
out.splice(34 - 1, 3, ...row);
out.splice(13 - 1, 1, ...merged);
out.splice(5 - 1, 1, ...top);
const count = out.length - 1;
console.log("STATUS lines:", count, "(was", s.length - 1, ")");

// The archive.
const block = [
  "_Moved from STATUS (2026-11-27, change-set 4b-2's docs pass, by script, byte for byte): under his OLD-LINES (a) of 2026-11-18, the lines about merged pull requests: the rows of MAIN-WALL (merged as #76, 2671595, its T10 record as #77, 47bb79e), of 4b-1 (#75, b9aad01) and of CI-TIME (#78, b8f02b0), the top line (about #78 and MAIN-WALL's docs pass) and \"Last merged\" as they stood; and lane 1's own lines whose new forms STATUS now holds: \"Next\" item 2, the two \"Waiting on Mustafa\" bullets, the \"From change-set 4b-1\" follow-ups bullet and the last three lines of the \"v2\" open-risks bullet:_",
  "",
  "| PR | Work | Branch | State | Waiting on |",
  "|---|---|---|---|---|",
  ...moved.rows,
  "",
  ...moved.top,
  "",
  ...moved.merged,
  "",
  ...moved.next2,
  "",
  ...moved.wait,
  "",
  ...moved.fu,
  "",
  ...moved.risk,
  "",
];
const fuBlock = [
  "  - Closed by 4b-2 (its docs pass, 2026-11-27, read on the code 914f3b4): 4B-MEMORY-UNIT (`packages/sim-worker/boundary/content.test.mjs:19` now says \"49.2 million bytes (46.9 MiB)\"), 4B-TIME-REEXPORT (`crates/sim/src/time.rs`: `pub use content::time::UNITS_PER_SECOND`) and 4B-FACING-ZERO (`crates/content/src/map.rs:58`: \"0 faces +z, and the degrees grow toward +x\") [verified 2026-11-27 at 914f3b4]. In part: 4B-SCAN-CFG-TEST (the source scan now reads a file whose first line is `#![cfg(test)]` as test code, `crates/sim/tests/rules_scan.rs`'s `in_tests`; whether the build lints' hashed-map and clock half covers the content crate is not checked here [assumed: open]) and 4B-PLAN-4B2 (the TypeScript cap `MAX_BUNDLE_BYTES` is tested against the players' constructor in `packages/sim-worker/boundary/sim-wasm.test.mjs`, and `C::round` is on the maths allow-list, `packages/client/scripts/maths-allowed.mjs`; the bundle's outer shape not checked here [assumed: open]).",
  "- **From change-set 4b-2, \"The world\"** (lane 1, Builder V4 to step 9's third piece, then the chat he calls Builder V6;",
  "  branch `claude/ecstatic-wozniak-5cbdzs` standing in for `cs/4b-content`; the pull request is opened by his Advisor;",
  "  risk tier 1, no browser check; steps 6 to 11, step 12 moved to the start of 4c by his STEP12-GO (a); reviewed, two",
  "  rounds, every reviewer PASS by the end with Lows left; final code 914f3b4; the planted defects' whole run on it",
  "  caught 104 of 104 and held 19 controls of 19 (`.checkpoints/cs4b-content/plants-4b2.txt`); report",
  "  `docs/reports/2026-11-27-cs4b2-world.md`. Written by 4b-2's docs pass, 2026-11-27. The 4B- ids are this docs pass's,",
  "  not his; each item names who found it. The notes files named here leave the tree at the pull request's close and",
  "  stay readable at the branch's commits):",
  "  - 4B-HEAPLESS (security-auditor's dependency read; the card's step 6): the collision library's tree holds heapless",
  "    0.8.0 and rstar 0.13.1, compiled but kept out of the players' file (the built-file check's `UNLINKED_CRATES`; no",
  "    `spade` in the lockfile); heapless is fixed from 0.9.4 on: take 0.9.4 or later when rstar or parry3d next move",
  "    [verified 2026-11-27 at 914f3b4: `crates/devtools/tests/dependency_rules.rs`'s pins and its message].",
  "  - 4B-LICENCES (security-auditor, `.checkpoints/cs4b-content/security-auditor-deps-4b2.md` lines 105 to 106 and 137):",
  "    the batch's licences are all permissive; parry3d, simba and approx are Apache-2.0 only and parry3d's archive",
  "    carries no LICENSE file: the hosted build must ship the licence texts (a hosting change-set's).",
  "  - 4B-UNIT-COLUMN (the advisor agent's step-11 ruling, `.checkpoints/cs4b-content/advisor-step11.md` lines 13 and 28,",
  "    a Low): the field tables' `unit` column ships in the players' file and nothing reads it at run time.",
  "  - 4B-FIRST-DEFAULT (`advisor-step11.md`, Q1, line 27): no `default` column or keyword now; the first field with a",
  "    default (an additive change, ADR 0005 CD-4) goes to the advisor agent first, with the change to `read.rs`'s",
  "    \"nothing missing\" rule.",
  "  - 4B-QUERY-RECORDING (the advisor agent's re-read, its condition 4): no step queries the static world in 4b-2; the",
  "    first collision query from the match's step is 4c's, so a recording that covers it is 4c's.",
  "  - 4B-TYPES-READER (engine-architect, EA-3): the generated content types have no reader in 4b-2 but their tests (the",
  "    export test, and the tie of the types to the schemas); the map's reading, their one reader, moved to 4c.",
  "  - 4B-WORLD-ROOM (engine-architect, EA-7): the local match owns the authority's world (`LocalMatch`); when 4c's step",
  "    queries it inside the room, it moves to the room, and no rule yet keeps `content` out of the room's and the client",
  "    endpoint's manifests.",
  "  - 4B-LIMIT-IMPORT (engine-architect, EA-8): 4c's editor panel imports the schema limit function from product code,",
  "    never a second copy.",
  "  - 4B-CHROMIUM-TIME (infra-engineer, IE-1; a CI file, so the CI Builder's): CI's Chromium run uses 3.6 to 3.9 of its",
  "    5 minutes: a split or a new limit before 4c adds a long spec.",
  "  - 4B-CONTENT-CACHE (infra-engineer, IE-2; a hosting change-set's): `content.json` keeps a fixed name beside hashed",
  "    files: when hosting is built, serve `index.html` and `content.json` without caching, or hash the bundle's name.",
  "  - 4B-RAY-COUNTS (test-engineer, TE-4): the per-class minimum counts of the committed rays: 4c, with the casts' first",
  "    caller.",
  "  - 4B-RAY-WASM (netcode-architect, round 2): RR-1's three new paths of the ray cast (beyond the reach, a limit cut to",
  "    it, a ramp that overflows) run natively only: 4c commits one ray of each kind, with 4B-RAY-COUNTS.",
  "  - 4B-MEMBER-KINDS (engine-architect, round 2): the types-to-schemas test compares keys, corner lengths and words, not",
  "    a member's kind (number, string or which word list): 4c's reader of the types.",
  "  - 4B-BOOT-CALLS (engine-architect, round 2): `boot.ts`'s two calls of the page's tested handling",
  "    (`takeWorkerMessage`, `takeWorkerError`) are read and type-checked, not tested.",
  "  - 4B-WALLCLOCK (test-engineer, TE-6): `static-world.spec.ts`'s wall-clock bound stays as a backstop; a follow-up",
  "    only if it ever flakes.",
  "  - 4B-SERVE-LINKS (security-auditor, round 2, its L2-3; outside 4b-2's diff, from change-set 3a; about 1 line):",
  "    `packages/client/serve.test.mjs` still skips its two link tests wherever a link cannot be made, CI included.",
];
const riskBlock = [
  "- **Change-set 4b-2 of the v2 build plan, \"The world\"** (2026-11-27, no pull request number yet; the evidence: its",
  "  report `docs/reports/2026-11-27-cs4b2-world.md`, \"Risks\"; the follow-ups: the block \"From change-set 4b-2\" in",
  "  \"Follow-ups\"):",
  "  1. The static collision world is built and held but queried by nothing yet: its first real test is 4c's movement",
  "     and aim.",
  "  2. The players' file grew 4.7 times (54,514 bytes on main, 257,421 with the loader and the world's tree), mostly the",
  "     standard library's number reading and writing; recorded, not gated.",
  "  3. CI's toolchain run varies with the runner (9 min 43 s to 13 min 2 s), about 12 minutes under its 25-minute limit.",
  "  4. Two risks security-auditor left for acceptance, both on the developer's own computer: a browser that sends no",
  "     `Sec-Fetch-Site` header is served by the dev server as a tool would be; a folder swapped for a link between the",
  "     tools' check and their open is followed. Open: the advisor agent's word on them is in the report, not recorded",
  "     here [assumed: open].",
];
const targets = a.indexOf("## Targets");
const closed = a.findIndex((l) => l.startsWith("  - Closed by 4b-1, at its merge: 4A-S3-READ's read side"));
const pending = a.indexOf("## Decisions pending: the rest");
const r4a = a.findIndex((l) => l.startsWith("- **Change-set 4a of the v2 build plan, \"The recording\"**"));
if (targets < 0 || a[targets - 1] !== "" || closed < 0 || pending !== closed + 2 || r4a < pending) throw new Error("archive anchors");
if (a.some((l) => l.includes("4B-QUERY-RECORDING"))) throw new Error("archive: already run");
a.splice(r4a, 0, ...riskBlock);
a.splice(closed + 1, 0, ...fuBlock);
a.splice(targets, 0, ...block);

if (!WRITE) { console.log("dry run: nothing written"); process.exit(0); }
if (count > 120) throw new Error(`STATUS would be ${count} lines: over 120, nothing written`);
writeFileSync(S, out.join("\n"));
writeFileSync(A, a.join("\n"));
// Every moved line is in the archive, byte for byte.
const at = readFileSync(A, "utf8").split("\n");
for (const line of Object.values(moved).flat()) if (!at.includes(line)) throw new Error(`not moved: ${line.slice(0, 60)}`);
console.log("written; every moved line found in the archive");
