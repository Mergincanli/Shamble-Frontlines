// docs-keeper, 4b-1 docs pass: move STATUS lines to the archive by line ranges, byte for byte, then put the new
// lines in their place. Run once from the worktree root: node .checkpoints/cs4b-content/docs-keeper-4b1-move.mjs
import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";

const S = "docs/STATUS.md", A = "docs/STATUS_ARCHIVE.md";
const status = readFileSync(S, "utf8").split("\n");
const archive = readFileSync(A, "utf8").split("\n");
const L = (n) => status[n - 1];
const expect = (n, start) => { if (!L(n).startsWith(start)) throw new Error(`STATUS line ${n} is not "${start}": ${L(n).slice(0, 60)}`); };
expect(5, "_Last updated: 2026-11-26 (ci-preview-handover's docs pass");
expect(6, "_Last updated: 2026-11-26 (guard-wrappers' docs pass");
expect(14, "- **Last merged** (read with git, 2026-11-26): #70, 4a");
expect(33, "| none yet (title and text in the notes) | ci-preview-handover");
expect(34, "| none yet | guard-wrappers");
expect(39, "2. v2: the netcode proof test (#40)");
expect(45, "3. Lane 2 (Builder V2)");
expect(52, "- ci-preview-handover (CI Builder): his go");
expect(53, "- guard-wrappers: his go");
expect(54, "- From 4a's report (#70, merged 2026-11-26, 2483e9a)");
expect(59, "- **RELEASE-COUNT, decided by him on 2026-11-25");
expect(62, "- His to give: ACCT-3");
expect(120, "  online play; releases worked out, not counted); 4a's (the recording tests still follow a link, 4A-S3-READ; two planted");
expect(121, "  divergences only in a stand-in or native form). **Legacy (frozen):**");

const range = (a, b) => status.slice(a - 1, b);
const moved = { header: range(5, 6), rows: range(33, 34), next2: range(39, 44), waitMerged: range(52, 53), wait4a: range(54, 61) };
const sha = (lines) => createHash("sha256").update(lines.join("\n")).digest("hex");

// The archive block, at the end of "In flight" (before "## Targets").
const block = [
  "_Moved from STATUS (2026-11-26, change-set 4b-1's docs pass, under his OLD-LINES of 2026-11-18): STATUS's rows for the merged ci-preview-handover (PR #74, merge commit 3b949c1, committed 2026-11-26 15:24:22 UTC) and guard-wrappers (PR #73, merge commit 4a62873, committed 2026-11-26 14:56:33 UTC; both times are the merge commits' own, read with `git log` on origin/main, not `merged_at`), moved by script, byte for byte, so their cells read as they stood when they left STATUS:_",
  "",
  "| PR | Work | Branch | State | Waiting on |",
  "|---|---|---|---|---|",
  ...moved.rows,
  "",
  "_And, by the same docs pass and rule, byte for byte: the two \"Last updated\" lines those two pull requests' docs passes wrote at STATUS's top, and STATUS's \"Waiting on Mustafa\" bullets for the two:_",
  "",
  ...moved.header,
  "",
  ...moved.waitMerged,
  "",
  "_And lane 1's own lines that change-set 4b-1's docs pass replaced in STATUS (lane 1 wrote them), moved by the same script, byte for byte, so their words stay on record: \"Next\" item 2 as it stood, the \"Waiting on Mustafa\" bullet from 4a's report (its answered items now point at `docs/DECISIONS.md`), and the bullet for RELEASE-COUNT (decided on 2026-11-25; its condition 3B-RELEASE-DRAFTS stays open, listed in the archive's \"Follow-ups\", the bullet \"From change-set 3b\", which STATUS's \"Merged pull requests' bullets\" points at):_",
  "",
  ...moved.next2,
  "",
  ...moved.wait4a,
  "",
];
const targets = archive.indexOf("## Targets");
if (targets < 0 || archive[targets - 1] !== "") throw new Error("archive: no \"## Targets\" after an empty line");
archive.splice(targets, 0, ...block);

// The new STATUS lines.
const header = [
  "_Last updated: 2026-11-26 (change-set 4b-1's docs pass, first part, by lane 1's docs-keeper, on its branch with main 3b949c1 merged in: 4b-1 in flight, in review round 1; VISION-RULE's follow-up and pointers; 4b-1's follow-ups and the builder's two questions; the merged #73's and #74's rows, waiting bullets and top lines moved to the archive by script under OLD-LINES, with lane 1's replaced lines; the reviews' outcomes come in the main session's own pass)._",
];
const lastMerged = "- **Last merged** (read with git, 2026-11-26): #74, ci-preview-handover (3b949c1, 2026-11-26); #73, guard-wrappers (4a62873, 2026-11-26); #70, 4a (2483e9a); #71, ci-after-3a (07d673b); #67 (c1da1bd).";
const row = "| none yet | change-set 4b-1, \"Content\" (the first of 4b's two pull requests, his SIZE-4B (b)), by lane 1 (the chat he calls Builder V4): the content crate `crates/content` with no dependency (one reader for a cut grammar of JSON, the canonical writer, the bundle, the movement kind, the map kind with its own exact rules and the aim's reach, the time unit's conversion), the simulation's time unit, the reader's proofs and a bounded fuzz run natively, under Node and in the browsers, the old tuning's numbers and durations extracted by script, the time unit's rounding report, and the links check (his LINKS-CHECK (c)); 6,591 written lines against the stop line 7,450 | `claude/cool-pasteur-oiqk1r` (stands in for `cs/4b-content`) | risk tier 1, no browser check; built (code head ef2edec, main 3b949c1 merged in), in review round 1 (rust-reviewer, engine-architect, security-auditor; test-engineer follows) | the reviews and their fix pass, the verifier; the advisor agent's verdict on the final head; then his go for that head |";
const next2 = [
  "2. v2: the netcode proof test (#40); step 2.3's chat stays stopped. Walk is four change-sets (his WALK-CUT (a); no",
  "   change-set 3c: his ORDER-3C (c) of 2026-11-25 in `docs/DECISIONS.md`): 4a merged (#70); **4b-1 in review**; then 4b-2",
  "   (steps 6 to 12: the collision library, the built-file check, his Export's data with DIFF-LIST, content to the",
  "   simulation, the static collision world, the page's scene; after a short re-read by the advisor agent); 4c the first",
  "   soldier (he plays; an early look inside it); 4d sights; then the link simulator, merged before change-set 7 starts.",
  "   The first builder chat stays paused; Slice 3 waits for #40 and picks how commands are timed.",
];
const wait = [
  "- Change-set 4b-1: his go for the head the advisor agent approves, after the reviews (no browser check: SIZE-4B).",
  "- From 4a's report (#70, merged) and 4b's start, answered (`docs/DECISIONS.md`, 2026-11-26, 12:21:17, 13:05:03 and",
  "  14:09:13 UTC): CONTENT-SOURCE, DEPART-4, EARLY-LOOK-ORDER, LINKS-CHECK, SOURCE-GO, SIZE-4B, FEATURE-COMPARE;",
  "  FIREFOX-REPLAY (carried by #74, merged). STEP-60: his Export at 30 steps is in (15:07:24 UTC; saved whole in the chat's",
  "  notes); DIFF-LIST is asked at 4b-2's data step (step 8). Later: MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME,",
  "  SPRINT-DELAY-4; the two `docs/UNITS.md` §0 lines the Hornet's entry clashes with (his unit walk).",
  "- The builder's two questions from 4b-1 (for its report): **SIGHT-FLAG**: should 4b already give each map object a",
  "  \"blocks sight\" flag? The builder and engine-architect recommend not in 4b: it comes in Slice 2 with the sight check",
  "  (4b-2's collision world is to keep each collider's shape id, so no rework). **DUR-1**: which fire rate counts as the",
  "  old game's? The old code never carried a timer's leftover time, so the Shabzon fired about 720 shots a minute at 60",
  "  frames a second where its table value of 0.072 s gives 833; it binds change-set 8; game-designer recommends the table's",
  "  value now, tuned by play (`.checkpoints/cs4b-content/game-designer-durations.md`: it leaves the tree at the pull",
  "  request's close and stays readable at the branch's commits).",
];
const risks = [
  "  online play; releases worked out, not counted); 4a's (two planted divergences only in a stand-in or native form; the",
  "  recording tests' link read, 4A-S3-READ, closed on 4b-1's branch, at its merge). **Legacy (frozen):**",
];
// line 121 continues after "**Legacy (frozen):**"
const rest121 = L(121).slice(L(121).indexOf("**Legacy (frozen):**") + "**Legacy (frozen):**".length);
risks[1] += rest121;

// Follow-ups: two bullets before the empty line that ends the section.
const fuEnd = status.findIndex((l, i) => i > 100 && l.startsWith("## Open risks")) - 1; // the empty line
if (status[fuEnd] !== "") throw new Error("no empty line before Open risks");
const followups = [
  "- **VISION-RULE** (his point 4c, `docs/DECISIONS.md`, 2026-11-26, 15:51:38 UTC; no ADR text rewritten now), his words:",
  "  \"for the next edit of ADR 0001 (part 9 (c) and its table Q14: hiding enemies by line of sight is now asked for, so",
  "  R2-INT-OQ5's cost measurement at 48 players becomes due; part 9 (f): shots and their traces reach every player who can",
  "  have them on screen, hidden shooter or not), of ADR 0004 part 13 and of ADR 0003's cutaway rule.\"",
  "- **From change-set 4b-1** (in review; the archive's block \"From change-set 4b-1\"; the 4B- ids are its docs pass's):",
  "  4B-AIM-REACH: the loader refuses a playable map beyond the aim's reach (x and z plus or minus 327.67 m, y 0 to",
  "  655.35 m). The aim widens in the first change-set whose map leaves that square (none in Slice 1; Slice 2's plan says",
  "  whether its maps do), at the latest in Slice 3's change-set that sets the command codec for online play (PM-0001-51),",
  "  beside 3B-ACKS and 3B-OUTAGE-256. No numbered change-set holds it yet. ADR 0002's next revision: 4B-FEATURE-POINTER;",
  "  the drafts' next revision: 4B-DEPART-3, 4B-PM5-3 (his acceptance); the CI Builder: 4B-CI-CONTENT; 4d's import and",
  "  change-set 8: 4B-OLD-TIMERS. Closed by 4b-1's step 4, at its merge: 4A-S3-READ's read side.",
];

// Apply from the bottom up so earlier line numbers hold.
if (fuEnd >= 120 - 1) throw new Error("order: follow-ups end after the risks line");
status.splice(120 - 1, 2, ...risks);
status.splice(fuEnd, 0, ...followups);
status.splice(52 - 1, 10, ...wait);
status.splice(39 - 1, 6, ...next2);
status.splice(33 - 1, 2, row);
status[14 - 1] = lastMerged;
status.splice(5 - 1, 2, ...header);

writeFileSync(S, status.join("\n"));
writeFileSync(A, archive.join("\n"));
console.log("moved, sha256 of each range:", Object.fromEntries(Object.entries(moved).map(([k, v]) => [k, `${v.length} lines ${sha(v)}`])));
console.log("STATUS lines now:", status.length - (status.at(-1) === "" ? 1 : 0));
