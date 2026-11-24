# test-engineer, part a (MOVED), one review round on b22c30cd
2026-11-24. Reviewer notes; text only. Question: is it proven?

## Plan
1. Run: cargo test -p npt-judge --locked --no-fail-fast; clippy -D warnings; fmt --check; --check-marks. Paste lines.
2. marks.json diff: script base vs head (strip `moved`, undo -7/-56 edits, compare); count labels; check `to`s against
   moved-block.md and moved-table.mjs; MARK-NOTES sentences word for word; rough twin wording (cond. 8).
3. Conditions 1-9 against code and named tests.
4. Hunt unproven behaviour; plant defects in a scratch copy under .gate/ (never in worktree source).
5. Contract text (marks/README.md 2.1, 2.2, 2.3, 2.6, 3, 5, 6; fixtures/README.md) vs code.
6. The three changed existing assertions: forced? anything stale?

## Progress
- (lines appended per block / finding)
- Block 1 DONE (ran on b22c30cd, worktree source == b22c30cd): cargo test 17+0+3+55+23+0 = 98 passed, 0 failed, EXIT 0;
  clippy -D warnings EXIT 0; fmt --check EXIT 0; --check-marks "81 marks read and checked (drafts at 626df678…)" EXIT 0.
- Block 2 DONE: te-marks-diff.mjs (this folder): strip `moved` + undo -56 added check/notes + -7 notes => equal to base.
  Counts 26 marks, 21 parts, 2 checks, 1 bite; one `by`. -7 and -56 notes = MARK-NOTES word for word (tag prefix added).
  Destinations match moved-block.md (-35 to Slice 3 per moved-block/MARK-NOTES, table said RP: moved-block wins).
- FINDING (Medium, data): PM-0001-3's kept bite `PM-0001-3/not-net-of-base` ("fails MP's 100 +- 5 ms") belongs to the
  moved part "MP at P2" (u6c-brief.md:300 "not-net-of-base is MP's part, moved with R7"; u8-stopb-netcode.md:218).
  Unlabelled: its runs match only the moved part -> left out -> bite NOT-RUN -> -3 held at NOT-RUN for ever.
- Note: kept checks naming only moved parts (-3 "MP: server-measured RTT median…" x2; -19 "setup: at least 2 Worker
  traps injected", "one timed rejoin per Worker trap") are silently never judged and not named in the MOVED line.
- Block 3 in progress: plants in scratch copy .gate/te-plant/netcode-proof (own target dir); baseline copy 98 pass.
- PLANT P1 SURVIVED (all 98 pass): eval.rs:1494 `c.moved.is_none() &&` removed (a moved mark-wide check still judged
  on a bite run matching no part). FINDING Medium: condition 4's bite-run place has two branches; only the matched one
  is tested (a_moved_check_is_skipped_wherever_checks_are_chosen's bite case matches S30).
- PLANT P2 SURVIVED: a bite run matching a kept AND a moved part takes the moved part's rules/checks (then_some(matched)).
  FINDING Low: the mixed case is untested.
- PLANT P3 caught by a_bite_run_on_moved_parts_only_is_left_out.
- Plants caught: P4 (why line drops checks) a_moved_check_is_skipped…; P6 (pre refusal forgets checks)
  a_precondition_with_a_moved_piece_is_refused; P7 (pooled bypasses applies) a_moved_check_is_skipped…; P8 (moved part
  judged then relabelled) a_moved_part_is_never_judged…; P9 (ws_bound moved allowed) and P10 (blank not trimmed)
  moved_labels_are_refused…; P11 (MOVED above kept-part NOT-RUN) a_moved_part_is_never_judged… +
  the_real_marks_file_labels…; P12 (any_moved forgets bites) a_moved_bite_is_never_evaluated + a_label_anywhere….
- PLANT P5 SURVIVED: outside refusal narrowed to the mark's own label (a moved bite on an outside mark accepted).
  FINDING Low: README §2.6 says an outside mark takes no label; only the mark-level case is tested.
- FINDING Medium (cond. 8 honesty): -19/-31 "ML* Chrome (rough, reported)" labels carry by = "NPT-TRIM, 2026-11-21
  (the re-plan, approved 2026-11-22)" and a why that does not say the move is the advisor's reading; the file's own text
  attributes it to the approved re-plan, whose table does not name these parts.
- FINDING Low/Medium (cond. 9 spirit): -56 moved check why "MARK-NOTES: the record counts 4 items here, not 10" is a
  paraphrase under MARK-NOTES' name (his: "-56's record counts 4 items, not 10").
- Contract: §5 "a bite run that fails only moved checks … so its bite is missed" is wrong when no kept non-setup check
  applies (all(setup) on empty -> Undecided -> NOT-RUN). Low.
- Comment eval.rs:964 says every check of a precondition counts, yet `applies` (eval.rs:1177-1179) skips moved checks in
  the matched branch and not in the no-match branch (eval.rs:970-971): unreachable (loader refuses), Low.
