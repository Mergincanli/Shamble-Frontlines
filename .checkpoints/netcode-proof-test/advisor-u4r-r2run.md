# Advisor: ruling on U4r's r2 record run (step 11) and carry 62 (h) (step 12), 2026-11-14

## Plan
1. Read advisor.md from main (done), the r2 ruling steps 11-12 (done), the merge review, build3/build4 rulings
   (points 1-4 and 8), carry 62 in u4s-carries.md.
2. Check the run's provenance: r2.txt's head (commit e59ece9, sha256s of binaries, marks.json, SERIES.md copies),
   that spikes/ is the same at 71ee13d and e59ece9 (git diff --quiet), that e59ece9 descends from 71ee13d with only
   .checkpoints/ changes; the runner log r2.log (exit 0, times, no 'stopped at' or 'exit by a signal').
3. Read c-u4r-r2.mjs: the suites, their commands, the expected lines, how measured incomplete is summed.
4. Per suite: compare what came against its expected line (r2.txt + verdict JSONs + logs), one notes line each.
5. Step 12: identify the "clean short L1 P0 suites", sum tick.s.snap_incomplete over each measured phase from the
   logs myself (python3 -I print-only reads), and rule on lag-switch's 300 from its design and logs.
6. Rule next steps, the marks.json change and the npt-judge --only PM-0001-17 check, or the cause first, and with whom.
7. What goes to Mustafa.

## Notes (a line per suite as I go)
- Provenance: e59ece9 descends from 71ee13d; `git diff --quiet 71ee13d e59ece9 -- spikes/` exits 0; tree hash of
  spikes/ b57488ad... at both; the 5 commits between touch only .checkpoints/. Binaries' sha256 (bin/r2-e59ece9ecafc/)
  and the marks.json/SERIES.md copies equal r2.txt:2-5 and the committed files. Script unchanged since e59ece9.
- r2.log: lock exclusive 15:40:36, HEAD e59ece9, run 15:40:37-15:44:52, exit 0, 'written: ...r2.txt'. No 'stopped at',
  no 'exit by', no non-zero exit in r2.log or r2.txt (grep). Provenance block empty (sh() swallows cargo output): Low note.
- p0: summary 0 incomplete; contract PASS; -2 11/11 pass, -17 0 mismatches, -57 6981/2750/88044 pass, run valid. MATCH.
- p2: contract PASS; -2 11/11, -17 (L1 P2) 0, -57 6974/2750/88061; equal to copy 3/5 (38311/112/6194/585). MATCH.
- p3: contract PASS; -2 11/11, -17 (L1 P3) 0, -57 6952/2750/88062; equal to copy 3/5 (35169/73/9139/1485). MATCH.
- b1: -2 FAIL 0.273333 (worst slot 246/900, recounted), -17 and -18 L1 B1 INVALID by pre:PM-0001-2, -57 pass
  6974/2750/88060; heals 96/96 within 500 ms, max 129.6 ms. MATCH (ruled fail, recorded, never re-run).
- forged, clamp, fast-clock: P0 SS, -2 11/11 each; incomplete 0. MATCH. (forged == fast-clock record for record in
  the summary: same audit and fate log; expected, the 'presses' and 'steady' stand-ins differ only off this path? check)
- lag-switch: P1 (50 ms, +-10 ms, 1 % loss), steady; -2 11/11; share 1.0; incomplete 300 measured (219 warm-up, 15
  after); equal to copy 3/5 (42771/222/1811/300). MATCH its expected line.
- es: -2 FAIL by ES reliable events 50.05 (<= 10.5), other 11 pass. MATCH (ruled fail).
- evstart: -2 11/11, -17 0. MATCH.
- tick16: bite shown, mismatch 2880 (>= 48); run valid (share 1.0 on 600/600); 0 breaches; contract PASS;
  after_capped false (46 after ticks); 48 resyncs all at 65596 = wrap + 60, after phase; measured 64951-65550;
  incomplete 720 = 48 x 15. MATCH (the -2 'passes' is the bite run's validity: -2 has no part for a bite run).
- tick16-clean: -2 11/11, -17 0, -57 6981/2750/88044; 0 after ticks, after_capped false; 0 breaches. MATCH.
- b1-nofallback: 0 breaches; contract PASS; heals 96, 25 within 500 ms, max 1,688,286,323 ns (> 500 ms: the max check
  fails); bite INVALID by pre:PM-0001-2 (0.453333, worst slot 408/900); 71 resyncs, measured. MATCH ruling 3 points 4
  and 8 (A) and the builder's b1-nofallback2 figure for figure.
- whole-as-lost: measured incomplete 48 (one tick x 48 clients), client hashes 43104 = 43152 - 48; -17 NOT-RUN (the
  bite id is not in marks.json yet). MATCH.
- Step 12: -17's L1 P0 part selects layer L1, scene SS, profile P0, 48 clients. Clean (role measured, no bite) short
  runs in r2 that it selects: p0, forged, clamp, fast-clock, evstart, tick16-clean. Measured sums recomputed from
  client.jsonl: 0 each (warm-up 0 too). es (scene ES) and lag-switch (profile P1) are outside L1 P0; p2/p3/b1 too.
  Ruling 4's further finding already listed the same set (plus builder p0-s1c) and left lag-switch out.
- lag-switch's 300: incomplete share 300/28800 = 1.04 % against P1's 1 % loss; p2 585/28800 = 2.03 % (2 %), p3
  1485/28800 = 5.16 % (5 %): loss-driven by design.
- forged vs fast-clock: room.jsonl and client.jsonl equal line for line (headers aside); lab.jsonl differs only by one
  counter_wrap event (press, tick 513) in forged. The presses script is wired; held fire makes press edges change no
  state. Its expected line claims only -2: met. Note, no finding.
- r2.txt's suite blocks (lines 9-278) equal r2.log lines 6-275 (diff empty). Commits: e59ece9 15:40:04 UTC, next
  5164251 15:45:51: no commit inside the run window (INF-R2-1's interim rule held).
- Judge: Rule::Pre reads the whole marks file (eval.rs:958-996), independent of --only and header marks; the check
  schema accepts series/stat/window/op/value/parts without setup (marks.rs:274-283, :505-540); Bite {id, what}.
  Tests that read the real marks.json (judge fixtures.rs on L1 P2, engine.rs -18, room/tests/bites.rs:22,
  server/tests/schedule.rs:355) are one-directional or on other parts: the change should not move them; check.mjs
  shows it. c-u4r-runs.mjs's bitesOf (:170-180) reads -17's bites at run time: P0 set gains bite-whole-as-lost
  (marks -2,-17: runBytes 56 MB per 10 min).
- SERIES.md:320's Marks cell says '-17 reports it': stale once -17 checks it; Low, at SERIES.md's next edit.

## Ruling (summary)
- Every suite matches its expected line (14 of 14). The three ruled fails (b1, es, b1-nofallback) came as ruled.
- Step 12: clean short L1 P0 suites p0, forged, clamp, fast-clock, evstart, tick16-clean: measured sums 0 each. The
  condition holds. lag-switch is P1, outside -17's L1 P0 part; its 300 is P1's 1 % loss. Main session: the marks.json
  change in its own commit (read in full), check-marks and check.mjs, then npt-judge --only PM-0001-17 on r2's
  whole-as-lost and tick16 into a new .gate folder: both bites shown.
- Lows: RUN-L1 (no build provenance in r2.log: sh() swallows cargo's output; rides U4r-F1). Nothing for Mustafa now.
