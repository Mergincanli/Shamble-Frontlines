# Notes: the BRAWL-1 report-section draft (drafting agent, workflow wf_634ed418-227), 2026-11-15

## Plan
1. Read item 3b (advisor-brawl1-sets-p0-full.md:173-197), the seven rulings, both builder reports, the card's NOW block,
   u4r-report.md (:215-239, :525-526, :562-564, :614-667).
2. Check every figure against the logs (.gate/brawl1-merge/, .gate/u4r-sets/, .gate/brawl1-evidence/) and the
   results folders; the file wins over the card.
3. Write brawl1-report-draft.md: the section (3b's order), then (1) the TICK16 :562-564 replacement, then (2) the
   :215-239 "done" marker.
4. Return path, line count and the card/file differences.

## Progress
- Steps 1 done (all sources read). Step 2 in progress.

## Checked so far (file wins)
- m3.log: header HEAD b30314a, free 12950 MiB, diff/untracked e3b0; 08:38:44-08:59:43; check.mjs 538/616, self 77,
  marks 81, redact 287/0, test 691.2 s; uptime 08:53:06 up 18 min, load 2.31, 2.21, 1.54; df 8,342,093,824 then
  8,342,081,536; det-u4 234.1 s, non-cargo 219.4 + 2.5 + 0.9 + 0.0 = 222.8; regen 1f00d2d5; full test 23.90 s.
- m2-clean-B.log 08:37:41-08:37:47 (15 run lines), m2-clean-C.log 08:38:03-08:38:09 (6). M2's three df readings are
  card-only (ADV-M6-L1).
- m4-gate.log: 5a FAIL (headless_shell-1194 missing), GATE FAIL; m4-gate-no-smoke.log GATE PASS.
- M7 logs: m7-df-0 09:12:50 8,318,758,912 B; m7-clean-B 09:12:52-09:12:53; m7-df-1 09:12:54 8,722,419,712 B;
  m7-clean-C 09:13:03-09:13:09; m7-df-2 09:13:10 13,556,895,744 B. HEAD fc93733 (tree ebbfeb40). Run lines equal M2's.
- m7-ev.log: 09:15:05-09:26:03, HEAD d89cc9d (tree ebbfeb40), e3b0/e3b0; P0 538,353,013 B, P3 98,642,683 B; P3
  overflow resyncs 27, measured incomplete 43,295 (:125); verdict-P3 whole-run incomplete 44,395; judge P0 8.6 s
  1,856.3 MiB, P3 2.3 s 392.9 MiB; df 13,556,174,848 then 12,643,663,872.
- Set logs' sha256 equal report_words (cc2f626c, a24d5856, e07e882e; pre-P0 2e0bb273). redact-BRAWL1-sets.log: 7
  items, 0 findings.
- Results files' sha256 equal the rulings' (5361fd53, 21c9abbf, c2995379, 1d2ff9e5, ada5a58a, e926b812, b472784f).
  Commits: P3 fc41111, P2 93509df, P0 be41ce6.

## Card against files (differences)
- D1. M7 step 1's time: card "about 09:13-09:15 UTC"; logs 09:12:50-09:13:10. File wins.
