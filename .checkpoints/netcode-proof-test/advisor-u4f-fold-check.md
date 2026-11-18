# The advisor's fold check of u4f-brief.md rev 3.3, 2026-11-18 (one short check, not a round)

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read with `git show origin/main:`),
effort xhigh. Read only; no build, no run, no edit of the brief. Brief at sha256
609bfd169bacb08c024c1788e03537586f00ba4b596b44cc56d3bf9dd35bf0d0, 1,827 lines (checked); diff read from 8ca5ede; the
folder's notes u4f-brief-r33-notes.md. Old phrases below: each found once (Grep), a line break plus the next line's
indent read as one space (the fold's method). The edits are part of rev 3.3 (no new revision).

## Verdict
**ACCEPT the fold, with the edits below.** Every A1-A7 edit, the 18 Ruled lines, the conditionals, the counts, SIZE-CHECK
and E4 are as ruled. Nothing goes to Mustafa to decide (LEAD-BOUND is already with him); D3-1's fix is a choice by this
ruling under TEST-FIRST rule 2 (inside the plan, undoable, no pass mark, no money, nothing on main): the main session
names it in that day's message.

## 1 (a) A3, carry 74's reading: fold it in 2.2 too
- 2.2's pass-marks row: "the commands that arrived after their stamped tick ran" -> "the commands not on time by their
  stamp, late or never arrived". D15 names only the share: ACCEPT unchanged (my "D15" in A3 was loose).
- R23: "carry 74's option-A reading in 5.3:" -> "carry 74's option-A reading in 2.2 and 5.3:"; "**Back to the advisor:**
  D15 holds no such phrase, and 2.2's pass-marks row holds the old one but is not named" -> "D15 names only the share,
  unchanged (the fold check: N/advisor-u4f-fold-check.md)"; "| 2.3; 2.10 f34; 5.3 p12 and its closing paragraph |" ->
  "| 2.2; 2.3; 2.10 f34; 5.3 p12 and its closing paragraph |".

## 1 (b) Places L1 did not name: restate each (21 pre-runs are firm; A6 moved KL-U4F's reading)
- 5.5's room paragraph: "against about 3,607 to 3,727 MiB of logs plus the build phase." -> "against about 3,727 MiB of
  logs plus the build phase."; "about 1,855 MiB short of the sets' 3,607." -> "about 1,975 MiB short of the sets'
  3,727." (3,727 - 1,752).
- U4F-Q2's question: "(5.5: about 3,607 to 3,727 MiB)" -> "(5.5: about 3,727 MiB)".
- 12 item 6, KL-U4F's draft: "Why it matters (rev 3.2, TE31-8, from D2): U4f's sets need about 3,607 to 3,727 MiB (3.5
  to 3.6 GiB) of logs against about 1,750 MiB above the 4,096 MiB floor after the planned pack, before the shared
  target's part-clean, and" -> "Why it matters (rev 3.3, A6): the judged sets still to run need about 3,317 MiB (3.2
  GiB) of logs, plus the main folder cache's growth from their builds, against the MiB above the 4,096 MiB floor that
  D2 reads after 5.5 item 2 (filled in when the entry is sent), and" (my ruling :362-367's list).
- NA-5: "pre-runs need about 3,607 to 3,727 MiB against" -> "pre-runs need about 3,727 MiB against".
- 9.1, npt-server row: "| section 6 item 4 if a ruling opens npt-server; else U4h |" -> "| **taken:** section 6 item
  4, since U4F-Q6 (b) opens npt-server's tests (A2; D9) |".
- 9.1, readings-PR row, its third cell from "U4f's runs rest on readings already listed" to "joins the list (plan
  :971-974)" -> "U4f's runs rest on readings already listed, plus one the rulings add: A1's reading of ADR 0001:1611's
  "moves" (2.6's closing paragraph; 5.6 B8), which joins the list (plan :971-974) by the main session's commit (the
  ruling's L3); the ruled U4F-Q3 (c), Q6 (b), Q11 (iii) (c), Q13 (b) and Q15 (c) need none (Q11 (iii) (c): its unit
  test shows the row's words hold as written)".
- Leave rev 3.2's own counts block (:74-81): it is that revision's record.
- R33's next row, new: "| R34 | the fold check (N/advisor-u4f-fold-check.md) | carry 74's option-A reading in 2.2;
  the disk total about 3,727 MiB in 5.5's room paragraph, Q2 and NA-5; KL-U4F's draft read at A6's place (about 3,317
  MiB still to run); 9.1's npt-server and readings-PR rows made firm | 2.2; 5.5; 9.1; 10 Q2; 12 item 6; NA-5 |".

## 1 (c) Placement choices: ACCEPT all
A2's fix sentence at section 6 item 4's end; A1's and E4's adds inside Q11 option (iii) (c); A3's ";" after f34; A4's
"; " join; B2's "rev 3.3 (A5):" label; Q12's "come to the advisor"; 5.5's "read by D2 after item 2 (A6's place)" as
KL-U4F's trigger; Q5's P25 substitution in section 4 and 12 item 3; Q6's f1 loss; E4's Q8 wording (walk 600 cm/s:
240 cm holds). Rewraps are whitespace only.

## 2 D3-1: Medium, confirmed; the fix
Confirmed at the head: U4r's P3 folder holds no `"reorder_permille"` and no `reorder` event (Grep count 0 over
u4r-P3-0f066338dd8d; l1-p3-s1/lab.jsonl line 1 holds `reorder_step_permille` 10); the scenario writes only the step
(lab/src/scenario.rs:1400-1403), the probe the share (lab/src/main.rs:123-139, probe.rs:212-216); the rule reads a
header param or a `reorder` event's `permille` (judge/src/eval.rs:864-881); -20 lists it (marks.json:4706-4708). The
header is written at the log's open (lab/src/logs.rs:77-88), so the form is the end-of-run event, from
`L1Net::with_fate_log` and `FateLog::reorder_counts` (l1/src/lib.rs:1918, :436): no npt-l1 change. The contract test
reads `event.reorder.permille` (contract.rs:6-9, :64-87), which no row gives (SERIES.md:406's cell does not parse), so
a new row. Caught as briefed only at p5/p6's verdicts after the merge (a stall and a second PR), hence Medium.
- 2.7's table, -20 row: "`header.params.reorder_permille` (:245, U4r's)" -> "the measured reorder share, a `reorder`
  event at the run's end in the lab log beside `fate_log` (scenario.rs:1275-1277), fields `permille` (the lower
  direction), `up_permille` and `down_permille`, from npt-l1's fate log as the -3 probe computes them
  (`FateLog::reorder_counts`, l1/src/lib.rs:436, through `L1Net::with_fate_log`, :1918; lab/src/main.rs:123-139), with
  a new SERIES.md row `event.reorder.permille`, `.up_permille`, `.down_permille` (5.6 B9): the header is written at the
  log's open (logs.rs:77-88), so the share goes in the event -20's `reorder` rule also reads (judge/src/eval.rs:864-881;
  SERIES.md:245, :406); U4r's scenario runs write only `reorder_step_permille` (scenario.rs:1400-1403; D3-1)".
- 2.10, after f39's row: "| f40 | rev 3.3, D3-1: a short L1 P3 scenario run's lab log holds one `reorder` event whose
  `permille` is above 0 and equals the lower of the two directions' shares from that run's fate log
  (`reorder_counts`: reordered x 1000 / delivered, floored, as lab/src/main.rs:123-139), with `up_permille` and
  `down_permille` beside it | lab | | 1 |".
- Counts, in this order: first "measured build at least **+118** (f38, f39," -> "measured build at least **+119** (f40
  (D3-1), f38, f39,"; then every other "+118" -> "+119" and each "+133" -> "+134" (four each in all: :45-46, :633,
  :1144, :1174-1175); "the measured 118" -> "the measured 119"; "f1-f39" -> "f1-f40"; "Made firm by the rulings, in
  both builds (counted above): f38" -> "Made firm by the rulings, in both builds (counted above): f40 (+1, D3-1); f38". Negative cases stay 67; SIZE-CHECK unchanged
  (about 40 lines, inside its range; the stop reads real lines).
- 5.6, after B8: "| B9 | a new SERIES.md row, `event.reorder.permille` with `.up_permille` and `.down_permille`
  (D3-1): the event route SERIES.md:406 already names for the `reorder` rule, in the contract test's form
  (contract.rs:64-87), its meaning SERIES.md:245's | SERIES.md (a new row only: U4F-Q1 (a)) | not a bar: marks.json's
  -20 validity (:4706-4708), the rule (judge/src/eval.rs:864-881) and every existing row unchanged | built | nothing
  (named in that day's message: TEST-FIRST rule 2) |". Its "So:": "and adds only report-only SERIES.md rows;" -> "and
  adds only report-only SERIES.md rows and B9's row (a name the `reorder` rule already reads);".
- 11, D3's row whole -> "| D3 | L1 P3's measured reorder share is above 0, so -20's runs meet the `reorder` rule |
  **read** (the main session, 2026-11-18; D3-1): U4r's P3 logs hold only `reorder_step_permille` (10, the step's
  setting: u4r-P3-0f066338dd8d/l1-p3-s1/lab.jsonl line 1), never the measured share; the scenario runner writes only
  the step (lab/src/scenario.rs:1400-1403), the -3 probe alone the share (lab/src/main.rs:131-139); U4f's lab writes it
  (2.7's -20 row; f40) | 5.3's p5 and p6 (-20 P3, A and B), judged in 5.4's form (D15's calls): neither verdict gives
  "reorder: no reorder share above 0 recorded" (judge/src/eval.rs:879-881), and each lab log's `reorder` event has
  `permille` above 0, read by the Grep tool (pattern `"permille":[0-9]+`, path the run's folder, output mode content
  with `-o`), the values read by hand; a miss stops -20's set and comes to the advisor | after the merge: at p5 and p6,
  before -20's set |".
- 12 item 13: "D3 read first; then, with no build and no set running," -> "with no build and no set running (D3 is read
  at 5.3's p5 and p6, after the merge: D3-1),". This replaces the D3 part of my ruling's L4 item 13: the u4r-P3 pack
  waits for nothing of D3.
- Change list, after R34: "| R35 | D3-1 (Medium; the main session's D3 run; the fold check) | U4f's lab writes the
  measured reorder share for scenario runs (a `reorder` event at the run's end, from npt-l1's fate log as the -3 probe
  computes it; U4r's runs wrote only the step, so -20's runs would be INVALID by the `reorder` rule); its new SERIES.md
  row (B9); f40 (+1, both builds); D3 read at 5.3's p5 and p6 | 2.7; 2.10 f40; 5.6 B9; 11 D3; 12 item 13 |".
- **Not a bar change for Mustafa** (the main session's reading holds): it writes the series SERIES.md:245 defines (writer
  "npt-lab (L1, from npt-l1's fate log)", reader -20) by the route :406 names; marks.json, the judge, every check and
  every existing row stay; no threshold moves. The test-engineer's panel read of SERIES.md's new rows covers B9.

## 3 adv_words_check.py's 36 hits: PASS when limited to his words
A read-only `python3 -I -c` run of adv_words_check.py's method (8-word runs, punctuation ignored, section 12 item 3
excluded) against his quoted words only: the "Mustafa ... verbatim" bullets and their fences in the 15 "## Decision:
Mustafa" blocks (history.md:1927-2282; 276 lines): **0 hits**; against the 147 verbatim lines of the older "Decisions
taken" section (:280-527): **0 hits**. Against the 15 blocks' whole text: one hit, :2212 (PACK-READ's option (a), the
session's text he agreed to, as at rev 3.2). The other 35 are session text: :414 (the session's "How it is applied"
under his :411-412), :1913, and 33 in the archived card (:2284-6789). L4 item 10's re-run at launch reads its hits the
same way: a hit passes when its history line lies outside his quoted words.

## Decisions needed
None. (LEAD-BOUND, the lead bound's first value, is already with Mustafa.)

## For Mustafa
Nothing.
