// docs-keeper, 4b-1 part 2: STATUS's lane-1 lines shortened. The long forms move to the archive by script, byte for
// byte (end of "In flight", one dated block); the new short lines take their places. Run once from the root.
import { readFileSync, writeFileSync } from "node:fs";

const S = "docs/STATUS.md", A = "docs/STATUS_ARCHIVE.md";
const s = readFileSync(S, "utf8").split("\n");
const a = readFileSync(A, "utf8").split("\n");
const L = (n) => s[n - 1];
const expect = (n, start) => { if (!L(n).startsWith(start)) throw new Error(`STATUS ${n}: ${L(n).slice(0, 70)}`); };
expect(5, "_Last updated: 2026-11-26 (change-set 4b-1's docs pass, first part");
expect(32, "| none yet | change-set 4b-1, \"Content\"");
expect(37, "2. v2: the netcode proof test (#40)");
expect(41, "3. Lane 2 (Builder V2)");
expect(48, "- Change-set 4b-1 (no browser check)");
expect(55, "- Answered on 2026-11-26");
expect(60, "- His to give: ACCT-3");
expect(112, "- **From change-set 4b-1** (in review;");
expect(118, "  change-set 8: 4B-OLD-TIMERS.");
expect(119, "");
expect(127, "- **v2:** the decision drafts' risks");
expect(131, "  editor and map bugs; the first-deploy tutorial;");
expect(132, "- **Preview hosting (lane 2):**");

const range = (x, y) => s.slice(x - 1, y);
const moved = { header: range(5, 5), row: range(32, 32), next2: range(37, 40), wait: range(48, 59), fu: range(112, 118), risk: range(127, 131) };

const block = [
  "_Moved from STATUS (2026-11-26, change-set 4b-1's docs pass, second part, to bring STATUS toward his 120 lines): lane 1's own lines (lane 1 wrote them), moved by script, byte for byte, where STATUS now has short forms: its first part's \"Last updated\" line, 4b-1's in-flight row as it stood before the reviews, \"Next\" item 2, the \"Waiting on Mustafa\" bullets with the long form of the builder's questions SIGHT-FLAG and DUR-1 and what he answered on 2026-11-26, the \"From change-set 4b-1\" follow-ups bullet and the \"v2\" open-risks bullet as they stood:_",
  "",
  "| PR | Work | Branch | State | Waiting on |",
  "|---|---|---|---|---|",
  ...moved.row,
  "",
  ...moved.header,
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
const targets = a.indexOf("## Targets");
if (targets < 0 || a[targets - 1] !== "") throw new Error("archive: \"## Targets\"");
a.splice(targets, 0, ...block);

const header = ["_Last updated: 2026-11-26 (change-set 4b-1's docs pass, second part, by lane 1's docs-keeper, on code 2039e3c: 4b-1 reviewed; the reviews' follow-ups; lane 1's lines shortened, their long form moved to the archive by script; its first part moved the merged #73's and #74's lines there under OLD-LINES)._"];
const row = "| none yet | change-set 4b-1, \"Content\" (the first of 4b's two pull requests, his SIZE-4B (b)), by lane 1 (the chat he calls Builder V4): the content crate, the time unit, the reader's proofs and fuzz run, the links check (the list: REPO_MAP, ROADMAP) | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4b-content`) | risk tier 1, no browser check; reviewed (two rounds, no Critical or High); the verifier and the advisor agent's verdict on the final head are in its report | his go for the head the advisor agent approves |";
const next2 = [
  "2. v2: the netcode proof test (#40); step 2.3's chat stays stopped; the first builder chat paused. Walk (his WALK-CUT",
  "   (a)): 4a merged; **4b-1 reviewed**; then 4b-2, 4c, 4d, the link simulator, change-set 7 (ROADMAP, \"Next / open\").",
];
const wait = [
  "- Change-set 4b-1, reviewed (two rounds, no Critical or High; the verifier and the advisor agent's verdict on the final",
  "  head: its report): his go (no browser check); the builder's SIGHT-FLAG (a \"blocks sight\" flag in 4b?) and DUR-1 (which",
  "  fire rate is the old game's?) in its \"Decisions needed\"; long form, his answers of 2026-11-26: the archive's \"In flight\".",
  "- Later, from 4a's report: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME, SPRINT-DELAY-4; the two `docs/UNITS.md` §0",
  "  lines the Hornet's entry clashes with (his unit walk); DIFF-LIST at 4b-2's data step (his Export at 30 steps is in).",
];
const fu = [
  "- **From change-set 4b-1** (the archive's block \"From change-set 4b-1\"; the 4B- ids are its docs pass's):",
  "  4B-AIM-REACH: the loader refuses a playable map beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to",
  "  655.35 m). The aim widens in the first change-set whose map leaves that square (none in Slice 1; Slice 2's plan says",
  "  whether its maps do), at the latest in Slice 3's change-set that sets the command codec for online play (PM-0001-51),",
  "  beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet. Also 4B-FEATURE-POINTER, 4B-DEPART-3, 4B-PM5-3,",
  "  4B-CI-CONTENT (the CI Builder), 4B-OLD-TIMERS, 4B-TIME-REEXPORT, 4B-FACING-ZERO, 4B-SPAWN-SLOTS, 4B-PLAN-4B2,",
  "  4B-FUZZ-REACH, 4B-REFUSAL-TABLE, 4B-SCAN-CFG-TEST, 4B-OLD-READERS. Closed at its merge: 4A-S3-READ's read side.",
];
const risk = [
  "- **v2:** the decision drafts' risks (the asset pipeline and several LOOK_V2 techniques not researched); R1-R6's (R3-R6",
  "  spot-checked; unread sources); change-set 3a's (MUL-ADD first); 3b's (no bound yet on how late a command runs, before",
  "  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form);",
  "  4b-1's (the fuzz run cannot reach the caps, 4B-FUZZ-REACH). **Legacy (frozen):** 3a's ranked list (PR #15); bot",
  "  navigation; the editor and map bugs; the first-deploy tutorial; the wall-clock reads (an input for ADR 0004 part 4).",
];

// Bottom up.
s.splice(127 - 1, 5, ...risk);
s.splice(112 - 1, 7, ...fu);
s.splice(48 - 1, 12, ...wait);
s.splice(37 - 1, 4, ...next2);
s.splice(32 - 1, 1, row);
s.splice(5 - 1, 1, ...header);
writeFileSync(S, s.join("\n"));
writeFileSync(A, a.join("\n"));
console.log("STATUS lines:", s.length - (s.at(-1) === "" ? 1 : 0));
