# u4r-fill-check2: adversarial check of the revised u4r_fill.py against the main session's six decisions

Script under check: u4r_fill.py at HEAD ff54815 (clean tree): sha256 2276287d262b3fe1217bd95c0acff93ba3f4d52b4c1c3d8eecc80f5a12eb7aa6,
1,059 lines (committed at b4d1f0a). The task text names "2c65d78b, 985 lines": that is the FIRST version (dac2aec), the
one u4r-fill-check.md failed; the revised script is 2276287d (the writer's notes R8 say the same). Checked: 2276287d.
Read-only: git reads, bash -n, sha256sum, python3 -I dry run.

## Plan (blocks)
- K1. Read section 12 (items 1, 2, 3, 9), section 0, title block, change lists, FQ4; the first check; the notes.
- K2. Read the script whole; the diff 2c65d78b -> 2276287d (what the revision changed); the dry run, output read whole.
- K3. Each decision 1-6: met in its words? asserts extended or loosened?
- K4. Top changes: every top line changed vs rev 3.2 (all of them).
- K5. Sample >= 10 copies and cites against their sources; stamps; R11 vs `git show 4e91830` and the files.
- K6. Any other change (section 10, elsewhere) vs the first version's result.
- K7. Result.

## Progress
- K1 done: brief title (:1-29), change lists (:31-178), section 0 (:180-237), section 12 (:1619-1681) read; first
  check and writer's notes read (R1-R8 revision log).
- K2 done: script read whole (1,059 lines); diff dac2aec..b4d1f0a read (the revision's changes: MAIN48, PIN, CITES
  :45/:160/:166 + MANUAL :45, TOP_INV, NEW_R11, KEPT10/SENT0, TITLE_END/SENT_TITLE, P6_1_NEW, TOPCITE/TOP_WANT asserts).
  Dry run at HEAD ff54815: exit 0, 687 lines, 66 changes, 1,682 -> 1,909, result sha256 28d90673…; no .pyc written.
- Resumed (fresh copy after the 19:00 cut) at HEAD 9627723 (checkpoint commits only since ff54815: the root card and
  this file); script still 2276287d, brief 362e771e. Dry run re-read whole at 9627723: exit 0, 687 lines, 66 changes,
  1,682 -> 1,909, result sha256 9370a6ad… (differs from 28d90673 only by `<base>`'s value, as it must).
- K3 done (re-read the script whole and the diff dac2aec..b4d1f0a again after the restart):
  - D1 met: CITES gains :45 (MANUAL, held), :160 and :166 (PIN: text asserted in 8bcbf36~1 :885/:886, :917/:918 and
    now :886/:887, :918/:919; read by me: right); each found once (:1673's and :1675's own words differ); the old
    `L[:H0] == ORIG[:H0]` replaced by TOP_WANT (ORIG's top with the two cites replaced) checked line for line outside
    the record (L[:TR], L[TR+KT:J0]); TOP_INV asserted exactly :45/:160/:166; C26's bots/Cargo.toml:24 at :171 logged
    and held by the top assert. Dry run: the top's hunks are :29 (record), :160, :166 only.
  - D2: the clause reads "npt-bots still names npt-net (bots/Cargo.toml:25) (C26)", asserted, "(:" absent. But the
    cell goes on ". 2.2 and D5 say so; re-walked at launch (section 12 item 9)" (as in the first version): the decision
    says R11's new words END at "(C26)". Literal reading not met -> finding K3-a (Low), question for the advisor.
  - D3 met: P6_1_NEW = the first version's minus "; your worktree may start ... empty", ends "all merged by cc8595d).",
    asserted (no "worktree"). U4d2's P6 (u4d2_restamp.py:141-143) was the full sha plus a description: same form.
  - D4 met: SENT0 = first version minus "and the change lists above", "stay" -> "stays" (agreement), KEPT10's words
    after "(records)" exactly the decision's, asserted whole; one sentence asserted.
  - D5 met: :145-148, rev-parse --verify then merge-base --is-ancestor 4d21c1a BASE, any non-zero stops; right after
    BASE is read. 7685b2b's second parent is 4d21c1a (git log): right.
  - D6 met: SENT_TITLE = the decision's words exactly (compared), after TITLE_END on rev 3.2 :29; paragraph bounds
    asserted (:16-29, blank :15 and :30); result :29-31, the paragraph's other lines unchanged.
