# u4r-fill-check2: adversarial check of the revised u4r_fill.py against the main session's six decisions

Script under check: u4r_fill.py at HEAD ff54815 (clean tree): sha256 2276287d262b3fe1217bd95c0acff93ba3f4d52b4c1c3d8eecc80f5a12eb7aa6,
1,059 lines (committed at b4d1f0a). The task text names "2c65d78b, 985 lines": that is the FIRST version (dac2aec), the
one u4r-fill-check.md failed; the revised script is 2276287d (the writer's notes R8 say the same). Checked: 2276287d.
Read-only: git reads, bash -n, sha256sum, python3 -I dry run.
Slip note (after the restart): one print-only `python3 -I -c` sweep was piped with `> /dev/null` (a redirect to the
null device to test that it parsed; no file written or changed). Not repeated.

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
- K4 done: the dry run's hunks before section 0 (:180) are three: :29 (record, 1 -> 3 lines, each <= 120), :160
  ("(plan :885-886)" -> "(plan :886-887)"), :166 ("(:917-918)" -> "(:918-919)"); every other top line as rev 3.2's
  (the script's TOP_WANT assert, and the diff). C26's :171 and C28's :45 unchanged. Right.
- K5 done:
  - copies, 15 read against their sources by me (sed of each file's lines): P1 = carries :255-435, 181 lines,
    identical (in-memory compare of the diff's + lines); P2 history :292-293 (marker dropped, kept); P3 df :462-464;
    P4 df :465-468 with both replacements; P5 j-r2 :168-171; P8 j-r2 :298; P9 m9 :252-256; P10 m9 :259-264; P11 d2-r2
    :304-308; P12 d2-r1 :308-310; P13 carries :72-75 (rewrapped, source :72 is 122 chars; words equal); P14 :116-119
    and P15 :135-136 identical; P16 te-m3 :136-139. Every word right.
  - cites: 12 shifted plan cites compared old vs new by line (all equal); every MANUAL key read in both versions
    (:45/:405 500-501, :224 494-497, :273/:319 750-803, :276 392-504 and 835-839->835-840, :332 751-757, :390 753-755,
    :395 494-501 and 755-756, :486 756->756-757, :488 837, :525 756-757->757, :1219 498-500->499-500, :229/:1244
    carries 548-560->554-577): right. PIN :160/:166 right. L2's changed blocks (difflib): plan old 496-501, 755-757,
    838-839 (->838-840); carries old 340-343 (->+3), +3 after 520, +11 after 559.
  - own sweep: every bare `:n` token outside section 10 not in the inventory whose range would move if it were the
    plan's (>= 496) or the carries' (>= 340), 254 tokens, read by context: none is a plan or carries cite that moved
    (the plan ones are all in unchanged blocks 502-754, 758-837; :831-838 at :963 and :860-864 at :1027 are room.rs).
  - stamps: plan 1,093 1e631c55, carries 661 e8675a02, history 2,027 40ae4c30 (822c010 only since a6b3410), plan.md
    1,518 3ca4e12a; old 01ede63d 1,092, 8b7f165e 644: right.
  - R11: `git show 4e91830` (four files, five lines; lock edges npt-bots -> npt-sim at npt-bots' package, npt-lab ->
    npt-bots) and the files now (lab :32 in :23-35; bots :22 in :20-26, npt-net :25; Cargo.toml :42): every fact
    right; "2.2 and D5 say so": 2.2 :513-514 and D5 :1568-1572 do. Right.
- K6 done: the first version's dry run (git show dac2aec, run from stdin, print-only) and the revised one rebuilt in
  memory and diffed: they differ only at :29 (record), :160, :166, section 0's sentence (and its unit's rewrap), P6 in
  section 1 item 1, R11's npt-net clause. Nothing else changed. (HEAD moved to fe1460a by autosaves: P6 follows it.)
  - noted, no finding: MAIN-48's merge (7685b2b) adds a root Cargo.toml, Cargo.lock, crates/ and rust-toolchain.toml;
    the brief cites none of the merge's files by line (grep), and section 0 :207-209 names Cargo.toml, Cargo.lock and
    crate paths as spikes/netcode-proof's, so no cite reads the root's.
  - other checks: P7 sha256 349b51ad… and `bash -n` exit 0 (mine); counted.log aa02314a, its lines :2, :170, :174,
    :176, :177 give 491/561/81/0 and 11 steps; FQ4's cites in the log (:103-105, :102-104) read; no brief line cites
    history by number (grep); ast ok; no new .pyc; script 2276287d and brief 362e771e unchanged.
  - R6 (pre-existing, outside L4, reported to the advisor per the task): besides "BITE_IDS :752-758" (l1 :746-751, doc
    :744), the same cell's "the model's own bites are unit-test only (:764-773)" is stale too: MODEL_BITE_IDS' doc
    :753-755, `#[cfg(all(test, feature = "bites"))]` :756, the const :757-766; :764-773 is its last two ids, `];` and
    clock_seed. l1/src/lib.rs last changed at f3f7c2f (U4c), before the head read. Finding K6-a (Low).
- K7 result: PASS. Findings: K3-a Low (D2's "end" read as the clause; the cell goes on); K6-a Low (R6's second stale
  cite, pre-existing). Questions: D2's literal reading; R6's two cites; section 0 on section 10's L3-file cites.
