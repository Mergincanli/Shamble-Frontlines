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
