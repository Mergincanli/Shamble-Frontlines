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
