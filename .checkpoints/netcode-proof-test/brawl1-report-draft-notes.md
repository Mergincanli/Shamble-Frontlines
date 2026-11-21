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
- Steps 1-3 done. brawl1-report-draft.md written (604 lines). Step 4 (return) next.
- Checked by print-only python: the re-run sets block equals advisor-brawl1-sets-p0-full.md:212-253 except :218 (the
  ruling's own NOTE at :266); "Advisor corrections" :260-264, KEPT-LOGS :278-284, plain words :271-275 and item 1 :287
  equal their source lines; every inline quote (condition 18, ADV-BP-3 and its masking rule, ADV-BP-4, TE-B1-8, FP-9
  item 5, ADV-R1-1/-2, the P3 evidence line, TICK16-WORDING) is a verbatim substring of its source after whitespace
  folding; the three "old text" blocks equal u4r-report.md:562-564, :215-216 and :525-526.
- Commits checked: 926a6cb, 7402dac, 888ecbb, b30314a (08:09:41), f7dff4b, 8f0939f, d89cc9d, fc93733, 352a245;
  trees ebbfeb40 at b30314a, fc93733 and d89cc9d; abad0635 at 352a245; spikes/ 63de1632 at b30314a^.

## Choices made in the draft
- The section is laid out in item 3b's order, with KEEP markers for the three existing subsections (round 2 at
  :616-655, EVQ-LOAD :657-661, TICK16 :663-667) and INSERT markers for the new text.
- The re-run sets are copied from report_words with one change, ordered by the ruling itself (:266).
- The follow-ups add round 1's three (RR-B1-7 (1), EA-B1-1/NA-B1-3, TE-B1-8's glue test: FP-9 item 6) and -17 L1 B1
  beside the four item 3b names.
- Item 3b also lists the :525-526 update ("tick16's shown rests on r2's 30 s record"); the task's list left it out, so
  it is drafted as an optional third item.
- No line counts are given (the r2 ruling's lesson); identities are sha256 values.

## Noticed, outside the draft
- u4r-report.md:269-273 ("The next npt-lab lane: N2-1 = TE-R2-1, TE-R2-2, RR4-L2, RR4-L1, ... EA-U4R-1 ... EA-U4R-2
  ... RR3-L2 (a)") was carried by BRAWL-1's Part 1 (brawl1-plan.md:4-6; brawl1-build-report.md:10-66), merged at
  b30314a; EA-U4R-1 in the narrow form, no one-loop driver. It may also want a done marker; for the main session and
  the advisor's read.
- u4r-report.md:261-268 (the check.mjs follow-up) does not list ADV-PT-L3, whose home it is.

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

- Raw logs, print-only: tick16 lab.jsonl 94 overflow_resync, 47 at 65,611 / 46 at 65,729 / 1 at 65,791, all "after",
  48 slots, slots 17 and 5 once. l1-p3-s1 27 resyncs, 25 slots, 76,910-77,844, measured; l1-p3-s3 1 (slot 35,
  76,803); evidence brawl1-ev-p3-s1 the same 27 / 25 / 76,910-77,844.
- -57 in the six results folders: P2 p99 5,257 / 5,280 / 5,283 (U4r) to 5,111 x3: s3 -3.256 % = -3.26 % (ADV-P0-2).
  P3 max 6,952 / 6,963 / 6,970 against 6,984; P0 p99 5,737 / 5,736 / 5,739 against 5,719 / 5,718 / 5,715; bites
  34,856 / 46,692 / 440,313 against 34,830 / 46,692 / 440,312.
- -2 and -17 in the results folders match the report words (everyone in view, compared, incomplete, shots).

## Card against files (differences)
- D1. M7 step 1's time: card "about 09:13-09:15 UTC"; logs 09:12:50-09:13:10. File wins.
- D2. P2's record (card :322) still says "-57's nine values equal U4r's ... largest change is 0.0 %". The files give
  p99 5,257 / 5,280 / 5,283 to 5,111, largest -3.26 % (ADV-P0-2). The card records ADV-P0-2 lower down (:349-351) but
  item 3a's "correct P2's record" is not done in place.
- D3. Item 3a asks the card to record EVQ-LOAD and the corrected TICK16-WORDING word for word. The card holds neither
  text; it points to the files (:357-359). They are word for word in u4r-report.md:661 and :667.
- D4. Line counts on the card differ from `wc -l`: advisor-brawl1-points-full.md "82" (wc 107); brawl1-review-r1-full.md
  "189" (wc 224); advisor-brawl1-plan-full.md "118" (wc 129); u4g-brief-carry.md "15" (wc 14); r2 "181" (180), m6
  "88" (87), p3 "213" (212), p0 "288" (287). The sha256 values all match. Per the r2 ruling's lesson, the draft gives
  no line counts.
- D5. P3's pre-launch free space: card "free 12,058 MiB"; the set log's first line says 12057 MiB (a card-only reading,
  the P3 ruling's unnamed Low). Not used in the draft.
- Not a difference: U4r's P2 bytes "778.9" (report words; u4r-f3-plan.md:141, the folder) against u4r-report.md:495's
  "778.6 MiB of .jsonl".
- Not a difference: copy 1's size 992 (git diff --stat, 9 files) against 998 (numstat, 10 files with SERIES.md's 6).
