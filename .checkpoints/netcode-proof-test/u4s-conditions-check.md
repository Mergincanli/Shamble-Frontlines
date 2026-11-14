# U4s conditions check: test-engineer notes (2026-11-09)

Task: check only that the advisor's U4s ruling (advisor-u4.md:236-383) is applied. Read-only on every other file.

## Plan
1. Hashes: HEAD's u4-build-plan.md = f7961cfe…; working files' sha256 vs the ones recorded in u4s-conditions-notes.md.
2. Diff u4-build-plan.md HEAD vs working: each hunk traced to condition 1 (a)-(d); nothing else; D-4 word for word
   (advisor-u4.md:355-360); freeze word for word (condition 3, :328-333).
3. u4s-carries.md vs condition 2's list and O1: every item present.
4. Verdict and exact fixes.

## Progress
- Step 1: HEAD:u4-build-plan.md sha256 f7961cfee8cdff10d9bd64ac0df7859ec59cd8d9724367e713f994e298f33302 (matches the
  expected f7961cfe…). Working u4-build-plan.md 7ec1b324287faefc7b1968184b599c8da691fcde9f992c888f195ed57579ff87 (1,031
  lines); working u4s-carries.md 0b4b896b792d111cccd26d2700b000e1ebc8b5cdadb0c151540e20749ca2ee13 (230 lines);
  HEAD:u4s-carries.md 1500ca04425f9872cdbe00464356addece691fcc130dcb90a6bd5fbab2856f41. All four equal what
  u4s-conditions-notes.md records. Both edited files are uncommitted (git status: M). Step 1 done.
- Step 2 (git diff HEAD, word-diff on non-space tokens): 12 hunks, +36 -17, 12 markers (grep); every word change maps
  to condition 1: §4.1 npt-sim row :186 and npt-client row :191 (a, carry 30 = HEAD carries :145-150); §4.2 :237-241
  (b, "non-finite components," removed, rest identical, rewrapped); §4.3 :246-247 (a, carry 32 = :154-155); §4.14
  :443, :448 (a, carry 31 = :152-153); §4.15 :463 "move 4" -> "move 2" and :466-467 (a, carry 33 = :156-157; widths
  1+4+2+2+4+2+12+4+1 + 1+4+1+1 = 39 B, x 48 = 1,872 B); §5 a4 :527-529 (a, carries 31 and 5 = :151-152, :55-57);
  D-4 :597-603 (b); Lanes :822-829 (d, inside the wave-1 bullet at :818, HEAD :808); §7.2 :878-880 (c, phrase exact).
  No other word changed. Step 2 done.
- D-4: whitespace-normalized plan :597-603 begins with the ruling's quoted D-4 (advisor-u4.md:355-360) exactly; the
  only remainder is the tail "The words are the ruling's new D-4; carry 15 is u4s-carries.md's (the advisor's U4s
  ruling, condition 1 (b))." Freeze: plan :822-829 begins with condition 3 (advisor-u4.md:328-333) exactly; remainder
  "The words are the ruling's condition 3; the carries are u4s-carries.md's (the advisor's U4s ruling, condition 1
  (d))." Both tails are provenance, not rule text (N1: a sentence where every other edit has a one-parenthesis marker).
- (b) §4.1:183 at HEAD = the seam-data row (`InputReports` and `SimCore::reports`): no "non-finite", no `non_finite`;
  HEAD has "non-finite" only at :236 and :593 and `non_finite` nowhere: nothing to drop at :183 (author's claim holds).
  New file: "non-finite" only in D-4 :599; `non_finite` only in the freeze :825.
- G1 (consequence of (a)): §4.15 Size :466-468 says 1,872 B a tick but keeps "about 17.8 MB per 9,000 ticks and 53 MB
  for three" and "about 4.4 million calls", all from the old 1,973 B (1,973 x 9,000 = 17.76 MB; /4 = 4.44 M). At
  1,872 B: 16.8 MB, 50.5 MB, 4.2 million. No other plan line uses them (grep). Beyond (a)-(d): the advisor's call (F1).
- G2: condition 1's "Record the plan's new sha256 here, under this ruling" is not done: advisor-u4.md unchanged vs HEAD,
  no 7ec1b324 in it. The main session's step (with the commit of both edited files).
- Step 3 (carries diff vs HEAD): condition 2 items: s6 exhaustive match = 36 (U4d; seam.rs:440 verified
  `assert_eq!(EntityKind::ALL.len(), 7);`); TickInput integer-only = 37 (U4d); StateDump = 38 (U4d; cites :83-84,
  condition 1's rule, verified); carry 7 reworded, ruling's phrase and carry-8 note present; tick-count rule = 35 (U4b
  section) and 39 (U4e section), rule text equals the ruling's (check.mjs:627 verified: `/\bTICK_HZ\b/.test(init)`);
  four check.mjs nits = 34 (U4a), all four, faithful to u4s-fix2-review-rust.md:90-93, homes U4a then U4d2 (42), never
  the main session; proto Cargo.toml comment = 41 (U4d2 section; proto Cargo.toml:13-14 verified); report lines = 43
  (u4s-report.md:58-59 verified) and 44 (:38-40 verified; list equals advisor-u4.md:268-270). O1 = 40 (new U4r
  section; u4-conditions-check.md:42-45 verified; "0 view-time replacements and 0 clipped shots on both legs" =
  advisor-u4.md:107; plan HEAD :678-679 U4e `rewind_at` and :787 U4h verified). Every item present. Step 3 done.
- G3: entries 35 and 39 add, outside the quoted rule, "a U4b (U4e) builder that needs such a constant asks the main
  session, which places it". Unruled: in wave 1 crates/sim is U4d's lane and check.mjs U4a's (plan :819 "No file is
  shared"); in wave 2 check.mjs is U4d2's. That route makes the main session a second editor of a lane's files, and
  it pre-empts the author's own open F2. Fix: route to the advisor instead.
- N2: entry 34 "by a fix or a stated deferral in its report" + entry 42 "if U4a's report does not list entry 34 as
  met": a deferral could be listed as met and the nits then have no second home. Low.
- N3 (not on condition 2's list, optional): carries :3-7 still say the build is uncommitted in a worktree (merged at
  f080551); :32 and :159-173 ("For the advisor", NF-D4) still read as open, with option letters the ruling says are
  swapped. Not entries, so not carried; a one-line "ruled" note would stop a brief writer reading them as live.
- Step 4: verdict GAPS (G1 needs the advisor; G2 the main session; G3 a text fix). Done.
