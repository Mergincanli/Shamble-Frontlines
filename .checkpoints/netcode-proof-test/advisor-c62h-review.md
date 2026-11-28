# Advisor: ruling on C62H-REVIEW (a), the test-engineer's read of 5e563b9's 15 marks.json lines

## Plan
1. Read Mustafa's words of 2026-11-15 13:35 in history.md (whole).
2. Read my own c62h ruling (advisor-u4r-c62h-full.md, notes) and the read's notes (c62h-review-test.md).
3. Q1: check the diff of 5e563b9 on marks.json; compare with carry 62 (h) (u4s-carries.md:305-316), build4 ruling :32,
   r2run ruling :9, build1 ruling :33, u4-build-plan.md:1094-1100, ADR 0001:1605 row + its hash.
4. Q2: check the record folder (README, verdicts, cmp with diag-with-2), logs.rs bite, eval.rs lines named, P0 bites
   verdict at 93509dfd7fd1.
5. Q3: check marks.json -17 parts, figures.rs:466-481/:547, logs.rs:15-17, client lib.rs:580-607, P2/P3/B1 verdicts,
   u4r-report.md:992/:1002, l1 lib.rs:185-192, -18 checks, -58.
6. Rule on findings TE-C62H-1..3, PB-1; units ahead touching marks.json / judge; report words; for Mustafa.

## Notes (a line per point)
- Mustafa's words history.md:2113-2129 read whole; equal to the relayed request.
- 5e563b9's marks.json diff: +11 check lines (:3971-3981) + 4 bite lines (:4008-4011 area) = 15. HEAD marks sha256
  9fb5be57...; `git log 5e563b9..HEAD -- marks.json crates/judge` empty. Confirmed.
- Q1: committed check = r2run ruling's JSON (advisor-u4r-r2run-full.md:9) and u4-build-plan.md:1094-1100 readings
  line; carry 62 (h) at u4s-carries.md:305-307 still the setup form (name "setup: ...", "setup": true); its amendment
  :314-316 still says "JSON stays word for word" -> TE-C62H-2 true. Build4 :32 changed it. Bite "what" = build1 :33's
  amended wording. ADR 0001:1605 row hash (no trailing newline) 3075f638... = marks pin; row has no "incomplete",
  "lost", "group", "loses no group". plan.md :462/:475/:721 -17 rows say nothing. AGREE with "partly".
  Extra: carry 62 (h)'s ORIGINAL bite was "null and incomplete 1 for a refused group"; build1 point 4 changed the
  planted bug to "a tick decoded whole filed as lost" because no refusal occurs on P0 (0 sentinels in 3 runs).
- Q2: record folder verified: README names 5e563b9589d3, marks 9fb5be57..., judge 65acacfe... (sha256sum of the binary
  equals), --only PM-0001-2,PM-0001-17; cmp with diag-with-2 exit 0 both; whole-as-lost verdict :157-178 shown, run
  valid, mismatch pass 0, new check fail 48 (== 0); tick16: shown, 2880 fail, 720 fail. Bite site lab/src/logs.rs
  :317-330 (`bitten`, h != 0, first measured tick per slot) and :362-366 (Res::Lost) and :441-449 (null + incomplete 1).
  Unit test scenario.rs:2655-2694 (sum 16 bitten, 0 clean). P0 set 93509dfd7fd1 verdict-bites: whole-as-lost shown
  (48 fail, mismatch 0 pass), tick16 shown (12,151 + 720), client-rounding shown (878,352; new check 0 pass); clean
  s1-s3 new check 0 pass. AGREE "yes". Note: the bite plants the outcome at npt-lab's writer, not in npt-client.
- Q3: figures.rs:466-481 coverage (incomplete = resolved), :547-551 empty only at compared == 0 per run: confirmed.
  client lib.rs:580-607 "lost unless filed": confirmed. logs.rs:15-17 -57 only non-sentinel hashes: confirmed. No other
  mark checks snap_incomplete (grep marks.json: only -17's check + report). -58's parts L1 P0 wrap and L1 P3 wrap have
  no incomplete check (the read named only L1 P3). runs.txt (npt-lab lines): P2 17,260/17,089/17,532, P3
  43,295/43,210/43,454, measured client sets 864,000, stale 0, sentinels 0 on every run. B1 (64f3307d5c7f, pre-BRAWL-1
  set): npt-lab counts 31,346/31,279/31,635 measured incomplete, heals 480, stale 0, sentinels 0 -- the read said B1
  "no judged -17 value" (true for the judge) but npt-lab's count exists: about 17,280 + 10 bursts x 30 ticks x 48 =
  14,400 (less overlap) = about 31,400 -- matches by eye. My binomial estimate (1 datagram per client-tick assumed,
  unverified): P2 sd about 130, P3 about 200; no excess visible. Fate log (l1/src/lib.rs:308-321) has link, dir, flow,
  seq, send time, size, outcome: NO tick field, so PB-1's per-(client, tick) mapping needs a new lab record. Fate log
  stored only as sha256 (scenario.rs:771-776, :1192), so any new bound needs re-runs, not old logs. AGREE "not bounded".
- Judge: eval.rs:1189-1211 setup fail -> INVALID (so build4's plain form is right); :1451-1482 bite shown on any
  failing check of a valid run. figures.rs:24-40 everyone_in_view from tick.s.everyone_in_view (room log).
- PB-1 doubts: (1) its stale-drop exemption uses the client-under-test's own count -> reopens hiding place (c); staleness
  must come from the link's record (arrival order), never the client's counters. (2) ticks the room sent nothing for
  (resync gap after FullState, NC-L10; B1 after evictions) are filed lost by design and would read "unexplained" unless
  "the room sent no snapshot for it" is an explanation. (3) the fate log has no tick field: needs a new lab record.
  (4) -58's L1 P0 wrap part also unbounded. -> ADV-C62H-1 (Low, Q3 completeness), ADV-C62H-2 (Medium, PB-1 wording).
- Rulings: TE-C62H-1 Medium, follow-up -> C62H-BOUND (Mustafa) + interim report lines. TE-C62H-2 Low, fix now (one
  line in u4s-carries.md, rides the commit recording this ruling). TE-C62H-3 Low, fix now in the report ->
  C62H-KEEP (Mustafa). PB-1 -> to Mustafa as C62H-BOUND option (a), corrected; not built on the ruling.
- Units ahead touching marks.json/judge: JDG-ONLY-1 fix (judge, certain); C62H-BOUND build if chosen (marks.json +
  lab + SERIES); C62H-KEEP "remove" if chosen (marks.json); U4f (marks -47,-20,-21,-22: brief to say); U4g (-41..-62,
  -58, -18/-17 B1 reading; likely marks.json); U4h (new judge figures + fixtures: certain); readings PR if it changes a
  pinned ADR row (marks.json row_sha256). Recommend same read for marks/SERIES.md and marks/README.md (README:6).
- B1 set 64f3307d5c7f: -17 L1 B1 INVALID, -18 L1 B1 INVALID, -57 L1 B1 PASS, -2 FAIL (confirms the read).
- Interim predicted shares (my estimate, 1 datagram per client-tick assumed): P2 17,280 (sd about 130), P3 43,200 (sd
  about 200), B1 about 31,400 (16,992 outside ten 1 s bursts + 14,400 inside). Every run within 2 sd.
- Status: ruling composed; returning StructuredOutput.
