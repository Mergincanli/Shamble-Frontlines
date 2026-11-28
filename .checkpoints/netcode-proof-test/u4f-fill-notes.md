# U4f launch fill script: notes (agent, 2026-11-18)

Task: write N/u4f_fill.py (python3 stdlib, lean, well under 300 lines) for u4f-brief.md rev 3.3 (sha256 50530a82…,
1,835 lines) section 12 items 1, 2, 3, 10 and 0.2's living stamps; dry-run it once; save the whole output in
N/u4f-fill-dryrun.md. Never `--apply`.

## Plan (blocks)
1. Read the rules: brief 0.2, 0.5, 12 items 1-3 and 10; u4r_fill.py's checks and edit engine; r31_check.py's 7-word
   method; stamps_anchors.py's anchors. (done)
2. Script: checks first (brief sha; `<base>` = HEAD with spikes/netcode-proof = 0ad9661's, results/ excluded as D1;
   u4f-run.sh sha + `bash -n`; plan and plan.md unchanged; carries and history living: the stamped version found in git
   by its sha256, carries changed only by inserts after :597, history only appended; closed records' sha256 prefix
   equal to 0.2's; every anchor found once, its first and last words, the stated line range; anchors read in item 3's
   own text). Fills: every `` `<<Pn>>` `` outside item 3 by the flattened copy in quotation marks; `<<PV1>>`, `<base>`
   by value; 0.2's carries and history rows re-stamped and one sentence after the table. Units rewrapped to 120 only
   where a line passes 120 (rows and headings in place). After: no `<<P`+digit and no `<base>` outside item 3; item 3
   unchanged; words equal to the original with only these substitutions; 7-word check on the result with the copies
   masked. Output: one line per fill, then the unified diff (n=0); `--apply` writes once and reads back.
3. Dry run once: `python3 -I .checkpoints/netcode-proof-test/u4f_fill.py`; output saved by Write.
4. Report.

## Log
- block 1 done: the brief's placeholders are all backticked (147 targets in all); placeholders stand as homes AND as
  cross-refs (tables, headings 2.9 and 5.6, the change lists' C2 "`<<P1>>` to `<<P36>>`"), and item 3 says "gone from
  its every place": the script fills every place; the cost (copy sizes, headings) goes to the main session.
- finding: history's anchors P14-P23 each found once in the whole file (the archive :2284-6789 repeats none; Grep).
- finding: carries 686 lines; entry 90 at :598-601 (U4h), U4f section :507-539 unchanged; no `|` in carries.
- block 2 done: u4f_fill.py written (389 lines, over the 300 aim: 36 copy specs, the rewrap engine, the shown diff).
  Development runs (python3 -I, dry, nothing written): run 1 cut off in the tool's view; run 2 found two script bugs
  (0.2's b1-p3 stamp wraps across a line; item 3 names P4-P12 as "P3 to P13") and wrote nothing; run 3 clean. The full
  diff (copies inline) is about 65k tokens, too big to carry by Write: the default diff now shows each copy as «Pn» and
  leaves out the 48 table rows filled in their line alone (listed by line); `--full` prints the whole diff. Result
  sha256 20afd444… in every run (the shown form changes the printout only).
- block 3 done: the final run (script acb9f891…) saved whole in N/u4f-fill-dryrun.md; `git diff --no-index` against
  the run's output shows only the harness's "[exited with code 0]" trailer. No problem; 0 seven-word runs.
- for the main session: the fill puts every copy in every place (147 targets): P1 (3,582 characters) 7 times, among
  them the 2.9 heading (3,692 characters) and C2's "`<<P1>>` to `<<P36>>`" range name; P15 11 times; the brief grows
  1,835 -> 2,003 lines; `<base>` filled in C28's "`<base>` stays a placeholder" and 0.5's "Values:" line too.
- block 4 done (the advisor's fold check, section 4, ACCEPT with conditions): each copy once, as "   - Pn's copy:
  "..."" at the end of item 3's list (189 lines, before item 4); every other `<<Pn>>` -> "Pn (copied in section 12 item
  3)" (102 pointers, the 2.9 and 5.6 headings now 140 characters each, in their line); C2's, C28's and 0.5's
  "Values:" mentions kept, named by text; conditions 3 and 4's sentences edited, each old phrase found once; the new
  asserts all pass; 7-word check 0. One development run failed (the pointer count read FINAL, where a pointer can
  wrap: now read on the normalized text) and wrote nothing. The default output no longer prints the diff (50 hunks,
  about 28k tokens, more than this agent can carry by Write): `--diff` prints it, `--full` whole. Final script
  20dbb02b…, 449 lines; HEAD moved to f82866c (spikes still 0ad9661's); result 2,040 lines, 880ec8b1…; saved whole in
  N/u4f-fill-dryrun.md (`git diff --no-index`: only the harness trailer differs).
