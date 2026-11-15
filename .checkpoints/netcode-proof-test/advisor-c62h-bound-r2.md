# Advisor: C62H-BOUND round 2 (rule the test-engineer's read, check texts v2, final size and paste blocks)

Seat: advisor, from main's .claude/agents/advisor.md (read 2026-11-15 via git show origin/main). Read only. Nothing built.

## Plan
1. Read Mustafa's words (history.md:2113-2146), my earlier read (advisor-c62h-bound-full.json: C1-C11, Q rulings),
   the design (c62h-bound-design-full.json), the test-engineer read (c62h-bound-test-full.json), texts v2.
2. Rule TE-CB-1..10 and TE conditions 1-6 (accept / merge / decline), with overlaps TE-CB-4~C7, TE-CB-5~C2,
   TE-CB-8~C1+C4, TE-CB-3~C9. Confirm points against spikes/netcode-proof/ code where needed.
3. Check texts v2 against C1, C2, C4, C8: applied exactly, nowhere else; C2's place; the inherited-gaps row.
4. Exact replacements (block, old once in block, new) for accepted changes to frozen texts.
5. Final planned size by file, total, range, SIZE-CHECK stop line.
6. Paste blocks: differs, still unbounded, ADR -17 row line, run numbers (C10), freeze line (C9 + TE-CB-3).
7. Anything else before the paste; to_mustafa paragraph.

## Notes (one line per ruling, appended as I go)
- Block 1 done (read): texts v2 word-diffed against the design by script: only C1 (2 names, plain words, README),
  C2 phrase (row 1 Gaps), C4 (plain words), C8 (:320 Marks cell) changed; each frozen block occurs once in its
  placement block. Facts: crates unchanged since fc411116c1f1 and fd4612050cd6 (git log empty); marks.json sha256
  9fb5be57...; runner sha256 99dd070e...; free 7,638 MiB (df -m, 2026-11-15 evening); P2 stops 30/24/23, P3 87/62/59;
  projections P2 2,129 MiB, P3 1,530 MiB (runs.txt:11), on-disk peaks 1,758 / 982 MiB (:14); room_tick writes
  hs.server_hash on every room tick record (logs.rs set_hashes unconditional); client writer keeps per-(tick, slot)
  room-hash flags in `logged` (removed at write) -> TE-CB-2's tally is buildable there.
- Code checks: eval.rs:1226-1231 a failing check keeps gaps in out.gaps and the run valid; :1475-1478 a bite is shown
  when any check fails -> bite evidence must also require an empty gaps list. Only logs.rs:620 writes room tick records,
  always with hs.server_hash -> C2's broad gap is right. Client log tick records always carry snap_incomplete
  (logs.rs:449) -> TE-CB-5 (ii) gap is a real defect signal. Runner helper tests live only in .gate scratch harnesses
  (u4r-f2, u4r-f3), none tracked. P0 bites today: 6 (-17: tick16, client-rounding, whole-as-lost; -57: 3).
- Option (a) as chosen (advisor-c62h-review-full.md:79) already said "B1 comes with U4g's judged B1 runs": my C3 (v) is
  not a difference -> moved to "still unbounded".
- Block 2 rulings: TE-CB-1 accept (pure fn + 5 tests); TE-CB-2 accept (report-only tally; neither vs figure A; P3 s1 E1);
  TE-CB-3 merge C9 (+ runs.txt sha256, + what a figure computes); TE-CB-4 merge C7 (+ its test via TE-CB-1);
  TE-CB-5 (i) merge C2 (broad wording stands), (ii) accept -> row 1 Gaps text change; TE-CB-6 accept (2 fixtures);
  TE-CB-7 accept (1)-(4) as expected lines, L from counts; (5),(6) merge C10; + bite stop rule; TE-CB-8 merge C1+C4
  (already applied; TE's own phrasing declined, same meaning); TE-CB-9 merge C6 (dry print all 4 profiles);
  TE-CB-10 merge C11 (no JDG-ONLY-1 files: C5). TE conds 1-6 follow.
- Block 3 own addition: :320 row "each 1 must be explained" -> "each measured 1" (C1's scope, missed in my first read).
  Replacements tested by script against v2: each old string once per block; rows still equal inside series-placement.
- Block 4 size: by file low 543 / high 754 -> about 650 (545 to 755); stop stays 1,050 (numstat of
  spikes/netcode-proof/, added+deleted; 1.6x the plan, 1.4x the range top).
- Block 5 paste blocks written in the structured result: differs (6 lines; C3 (v) moved), still unbounded (5), ADR line,
  run numbers (time 1,735 + 1,020 = 2,755 s; start checks 4,983.6 / 4,620.4 MiB; free 7,638; projections 2,129 / 552 /
  1,530 MiB; stops 30/24/23 and 87/62/59), freeze line (C9 + TE-CB-3 + bite and lab-count stop rules).
- Block 6 other changes: texts v3 by script; C10/C5 corrections in the paste; Q1-Q5 shown as ruled; build-brief items;
  disk and PR #53 order. Verdict: READY AFTER THE CHANGES BELOW. DONE.
