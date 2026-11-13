# u4r_fill.py: the U4r brief's launch step (L4, section 12 items 1 to 3 and 9), on rev 3.2

Task: write .checkpoints/netcode-proof-test/u4r_fill.py (dry run by default; the main session applies it). Brief rev
3.2 (362e771e…, 1,682 lines, a6b3410). Read-only commands only: git reads, bash -n, sha256sum, python3 -I print-only
or the script's dry run. Never --apply, never cargo or check.mjs, never commit.

## Plan (blocks)
- B1. Read: section 12, section 0, section 9's table, section 10's bounds, every placeholder's home; u4d2_restamp.py
  and u4r_fold.py's helpers; the rulings L2, L4, FQ4, N7a, FQ6; L2's diff (8bcbf36) and L3's (4e91830).
- B2. Inventory the cites of the brief into the plan and the carries (outside section 10; the change lists noted):
  difflib maps for both; the cites touching a rewritten block, each read by its text, into MANUAL.
- B3. Inventory the cites of the four files L3 changed (Cargo.toml, Cargo.lock, lab/Cargo.toml, bots/Cargo.toml):
  which are old (pre-4e91830) and which the fold wrote at the head; R11's row.
- B4. The placeholders P1 to P5, P8 to P16: each source span read, its ends found once; P6 and P7 homes.
- B5. Write the script: asserts (sha256 of the brief and every source, HEAD and the spikes diff, the counted log),
  fills, re-points, stamps, R11, the words check, the log, the diff.
- B6. Dry run, read in full, fix; syntax check by ast; sha256 of the script; result.

## Progress
- B1 done: section 12, section 0, sections 1-9 and 11 read; section 10's bounds (:1308-1544); the rulings (L2 :421-424,
  L4 :428-429; fold FQ4 :101-112, N7a :89-90, FQ6 :151-168, "left as they stand" :93-95). Next: B2.
- B2 done: difflib (8bcbf36~1 -> now): plan replace old 496-501, 755-757, 838-839 (+1 from old 840); carries replace old
  340-343 (+3), insert after old 520 (+6) and after old 559 (+17). 52 plan and 4 carries cite tokens outside section
  10 and the change lists (a nearest-marker classifier cross-checks the list; one false positive, :1263's l1 BITE_IDS
  :752-758). MANUAL (by text, first and last line asserted): plan 494-497, 750-803 (x2), 392-504, 835-839 (x2),
  751-757, 753-755, 494-501, 755-756, 500-501, 756, 837, 756-757, 498-500; carries 548-560 (x2) -> 554-577. Change
  lists: left as records (fold ruling :93-95, "the change lists C1 to C31 (a record)"; C1: their cites are at fa8e335);
  the two that moved (:160 plan :885-886, :166 :917-918) are logged. Next: B3.
- B3 done: L3's files: R11 (:1268) is the one row (outside section 10 and C26's record); 2.2's (:513-514) and r1's
  Cargo.toml:107 (:550) are already the head's (asserted). R11's tally (:1303-1304) follows (18/16 -> 17/17).
  Next: B4.
- B4 done: P1 carries :255-435 (17 entries), P2 history :292-293 (line before :291), P3 df :462-464, P4 df :465-468,
  P5 j-r2 :168-171, P8 j-r2 :298, P9 m9 :252-256, P10 m9 :259-264, P11 d2-r2 :304-308, P12 d2-r1 :308-310, P13-P15
  carries :72-75, :116-119, :135-136, P16 te-m3 :136-139; homes found once each as `"<<Pn>>"` (fences for P1, P13-P15).
  P6 homes: :242, :766, :1621; P7: :257, :1625. Next: B5 (write the script).
- B5 done: u4r_fill.py written (edit engine word by word, rewrap only past 120, refill's safe wrap from u4r_fold.py;
  the words check; the classifier cross-check). Found while writing: history.md moved at 822c010 (16 lines appended:
  MAIN-48, Mustafa's words recorded): the script now asserts rev 3.2's history (a6b3410, 88bc195f) is a prefix of the
  current one, re-stamps history too and says so in the one sentence; no brief line cites history by line (P2 is a
  text anchor). Carries entry 6's first line is 122 characters: the P13 copy rewraps that entry within 120 in its own
  indent (item 3's "rewrapped to 120"; white space only, asserted). Docs the brief cites by line (ADR 0001, the
  catalogue) asserted unchanged since a6b3410. Next: B6.
- B6 done: dry run exit 0, last line "dry run: nothing written"; 63 changes; brief 1,682 -> 1,908 lines; read in full
  (log, the main session's words, the diff). Script sha256 2c65d78b00164f71d6d815600e7b571343a4d0606aeae6a50a90a01b3e0d616c.
  `<base>` at my last dry run: 209e35de5cc4a56880f75b681166bdc7042342df (an autosave commit; the main session's run
  reads its own HEAD). No .pyc written (ast check only). Next: the result.
- Question for the result: MAIN-48 (history, 18:14 UTC) merges main after L4's commit and before L6; `<base>` would
  then predate main's merge and the builder's `git diff --stat <base>` (section 6 item 1) would list main's 42 files;
  recommend merging main first and then running the script (its asserts hold after that merge: main's merge-base diff
  touches no spikes/, .checkpoints/ or cited docs file).

## Revision for the main session's decisions

Task: revise u4r_fill.py (2c65d78b…, 985 lines) for the main session's six decisions on u4r-fill-check.md (F1 to F5 and
the writer's question); keep everything else; asserts and the words check extended, never loosened. Edit tool only;
read-only commands and the dry run only. Branch claude/netcode-proof-test-plan-hcky5g, HEAD 86951ce at the start (after
the merge 7685b2b, main's 4d21c1a).

### Plan (blocks)
- R1. Read: the notes, the script whole, the check, section 12, the title block (:1-29), the change lists (:157-168),
  section 0 (:180-237), section 1 item 1, FQ4, u4d2_restamp.py's P6 and launch sentence; plan :885-886/:917-918 at
  8bcbf36~1 and :886-887/:918-919 now; a first dry run at HEAD.
- R2. Decision 1: the top's two moved plan cites into the inventory (C22 :160 "(plan :885-886), for the advisor to
  confirm.", C24 :166 "plan §7.1 item 6 (:917-918)"), each with a text pin asserted in both plan versions; C28's :45
  "(plan §4.15 :500-501)" into it too as a held cite by its text (MANUAL), so the classifier no longer exempts the top
  (every moved cite outside section 10 must be in the inventory); the top assert: rev 3.2's top with exactly the two
  cites and decision 6's record changed; C26's L3 cite stays logged. Logs, comments and docstring follow.
- R3. Decision 2: R11's "(:{bots_net})" -> "(bots/Cargo.toml:{bots_net})" and the old R11 assert unchanged.
- R4. Decision 3: P6_1_NEW ends at "all merged by cc8595d)." (the worktree clause dropped).
- R5. Decision 4: SENT0 (both branches): "except in section 10, which stays as written (records): its cites read the
  plan and the carries at the old stamps 01ede63d… and 8b7f165e… (`git show 8bcbf36~1:<plan>`, and the same for
  u4s-carries.md)"; one sentence.
- R6. Decision 5: after BASE, stop unless main's 4d21c1a7… is an ancestor of HEAD; logged.
- R7. Decision 6: the launch record appended to the rev 3.2 paragraph (after "values (section 12 items 1 to 3,
  next)."), an item through the edit engine (rewrap within 120), in the words check, the top assert, MAIN and WHERE.
- R8. Syntax check (ast), dry run read whole, every assert holds; sha256; result.

### Progress
- R1 done: all read; first dry run at HEAD 8e64c53: exit 0 (63 changes, 1,682 -> 1,908; head-limited read). The rev 3.2
  paragraph ends at :29 (the task's "after" phrase found once there); "(plan :885-886)" stands twice (:160 and :1673,
  the latter already re-pointed), so :160's cite is found by "(plan :885-886), for the advisor to confirm." (once);
  "plan §7.1 item 6 (:917-918)" once (:166); "(plan §4.15 :500-501)" once (:45). Plan old :885-886 = new :886-887 and
  old :917-918 = new :918-919 (text equal); old :500-501 / new :500-501 hold "The determinism step" ... "pre-check
  only." Next: R2.
- R2 done: CITES gains :45 (MANUAL (45, plan, 500-501), held), :160 and :166 (PIN, a new dict: first and last words
  asserted in 8bcbf36~1's and the new plan's lines, on top of the shift path's line-equality assert; USEDPIN == PIN);
  the classifier no longer exempts the top (LEFT_TOP gone; TOP_INV asserted to be exactly :45, :160, :166, each in the
  inventory); the top assert is TOP_WANT (rev 3.2's top, the two tokens replaced in their lines) with the record's line
  rewrapped (KT lines, its words = the old line + the record). C26's bots/Cargo.toml:24 stays in L3LEFT's log. Next: R3.
- R3 done: NEW_R11 "(bots/Cargo.toml:{bots_net})"; asserts: the phrase once, no "(:" in R11, BOTN :25 is npt-net.
- R4 done: P6_1_NEW ends "all merged by cc8595d)."; asserted (no "worktree").
- R5 done: KEPT10 (the old stamps from OLDSTAMP, the path from L2C and PLAN), asserted equal to the decision's words;
  both SENT0 branches use it; asserts: no "change lists", one sentence (no ". " inside, ends with ".").
- R6 done: MAIN48 constant; `git rev-parse --verify <sha>^{commit}` equal, then `merge-base --is-ancestor`, else stop;
  in the first LOG line.
- R7 done: TITLE_END found once at rev 3.2 :29 (the rev 3.2 paragraph :16-29 asserted); SENT_TITLE with `<base>`'s
  sha7, history's re-stamp asserted first; an item through edit() (tag "record (title)"; "record" added to the log
  and WHERE filters); MAIN gains it. First run stopped on my own paragraph-start assert (off by one: :16, not :15);
  fixed. Dry run now exit 0. Next: R8 (read the output whole).
