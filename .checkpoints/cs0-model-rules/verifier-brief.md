# Brief: the verifier's pass on change-set 0 of the build plan (effort max)

Task name: cs0-verifier. Docs only; no game code. Verdict PASS or FAIL with proof, on the one severity scale of
CLAUDE.md. You edit nothing but your notes. Pull request #56 (a draft), branch `claude/wonderful-hamilton-jdmlnf`, base
main c94c81b. Head when this was written: cacea55 (docs-keeper's pass and this change-set's report follow; you will be
asked for a short second look at what they change, so do not wait for them).

## What the change-set is
Mustafa's words of 2026-11-17 (`docs/DECISIONS.md`, the entry of 19:10:44 UTC, CHANGE 1; and of 20:39:23 UTC,
SIZE-0): change-set 0 records his decisions, carries the build plan to main, and fixes on paper the rules a model
file must follow. `git log --oneline c94c81b..HEAD -- docs` lists its commits: his records (3747bb3, 4a1b0d7), the
plan's carry (212f55b), the rules and the joint list (5f356ef, 65c8fe8, cacea55).

## Check, with proof for each
**A. His words, byte for byte.** Run from the repository root (they only read):
`node .checkpoints/cs0-model-rules/decisions-insert.mjs --check` (five entries in DECISIONS equal the saved messages;
the appendix holds the same bytes), `node .checkpoints/cs0-model-rules/appendix-verify.mjs HEAD` (every block's sha256
and size as git stores it), `node .checkpoints/cs0-model-rules/pointers-insert.mjs --check` (the dated pointers; each
quote in them is in his saved messages), `node .checkpoints/cs0-model-rules/report-build.mjs --check` (the plan in the
report equals commit 68addf9). Then check the checkers: (1) that the saved messages are what the transcript holds:
re-extract his five messages yourself from the session's transcript
(/root/.claude/projects/-home-user-Shamble-Frontlines/f707c56b-26b4-5848-b1e1-237a99a6a881.jsonl; the extract scripts
in `.checkpoints/build-plan-slice-1/` and `.checkpoints/cs0-model-rules/` show how; a message sent while a turn ran
is a `queue-operation` record) and compare sha256; (2) that each entry header's times and message ids match the
transcript's records; (3) that no entry header and no pointer says more or less than his words (read the headers and
the six pointers in DECISIONS, the three in SPIKE_PLAN, the one in ADR 0002). Open nothing under
/root/.claude/uploads/ (his words of 2026-11-17: the Advisor chat's log there is not an input and is not read); the
Art lead's texts are compared from `.checkpoints/build-plan-slice-1/inputs/`.
**B. His answers written into the drafts.** Each against his message (`docs/DECISIONS.md`, the entry of 2026-11-17,
19:10:44 UTC): OQ-0002-6 (ADR 0002 and the register), OQ-0006-18 (ADR 0006 and the register), OQ-0003-1 further in
part (the register), the three "do soldiers block" rows (the register), PM-0004-34's row (ADR 0004). The register's
arithmetic: 98 open became 96; ADR 0002's row 8 open became 7; ADR 0006's 17 became 16. Nothing recorded as decided
that is open; nothing of the Art lead's recorded as Mustafa's.
**C. The two research files** (`docs/research/asset-export-rules.md`, 397 lines; `docs/research/soldier-joint-list.md`,
224 lines). Spot-check at least 20 key claims against their sources, at least 6 of them in the joint list: the glTF
specification, Blender's exporter and three.js's loader as the rules file cites them (official sources first; web
access through the documentation domains the settings list; say which claims you could not reach); the legacy code
lines the joint list cites (`legacy/shamble-core.js`, `legacy/shamble-game.js`), `docs/LEGACY_INVENTORY.md` rows GA1
to GA15, ADR 0003 and ADR 0005 lines. Check that every "[inferred]" is one, that no rule states as proven what only
a real export can show, and that both files agree with each other (the 20 joints, `root`, the sockets' axes, the
hands, `palm`) and with ADR 0005's rows PM-0005-47 to -56. Give the count for the research index's status cell
("spot-checked (N of M key claims)").
**D. ADR 0005's rows.** The ten rows and the section's texts as committed against test-engineer's file
(`.checkpoints/cs0-model-rules/adr0005-marks.draft.md`, sections 1 and 3): pasted whole, nothing else in the section
changed, no existing mark loosened (diff the section against c94c81b). The two open-question entries (AS3, AS4) say
"answered in part" and that no export has gone through the rules.
**E. The plan's report.** `docs/reports/2026-11-17-build-plan-slice-1.md`: "Since the plan" against his messages (run
`node .checkpoints/cs0-model-rules/quotes-check.mjs` on the lines above "## The plan as approved"; then read it for
anything said beyond his words); its section 5 against the two research files.
**F. The rest.** `node tools/gate.mjs --no-smoke` (the smoke check cannot start here: the gate's pinned Playwright
looks for Chromium build 1194, the container has 1243; say so, do not install a browser); `node
.claude/scripts/repo-map-check.mjs`; no CR characters and no `.checkpoints/` citation in any file under `docs/` that
this change-set adds (the report's copied plan cites its own notes: that part is copied, not written); LOOK_V2's one
changed line against his words of 2026-11-03 as SPIKE_PLAN quotes them; the size: `node
.checkpoints/cs0-model-rules/size-count.mjs` against his stop line of 1,200 written lines (check that its rule for
"copied" is fair: his quoted words in DECISIONS, the appendix's blocks, the plan copied from 68addf9).

## Your answer (at most 40 lines)
PASS or FAIL; a table A to F with the result and the proof (command output in short, file:line); the findings,
numbered, each with severity, place, evidence and the fix; the spot-check count; what you could not verify.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/verifier-cs0.md (you may always write there). Look there
first: if the first line names another task, move the file into
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/old/ and start fresh; if the notes end with DONE, write
your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never
reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming
your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group
run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find
it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done,
written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/: text
and small scripts only; builds and binaries go under .gate/cs0-model-rules/. This folder is pushed to a public
repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a
note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives
or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.
