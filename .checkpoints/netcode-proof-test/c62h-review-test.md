# C62H-REVIEW (a): test-engineer read of marks.json lines added by 5e563b9

Seat: test-engineer. Read only. Task: Mustafa's words of 2026-11-15 13:35 (history.md:2113-2129).
(No Edit tool in this seat: this file is rewritten whole with Write at each step.)

## Plan
1. Read Mustafa's words (history.md end). Hash marks.json at HEAD. `git show 5e563b9 -- marks.json`.
2. Q1: compare check wording against ADR 0001 PM-0001-17 row, plan.md -17 rows, u4-build-plan.md s11, carry 62 (h)
   (u4s-carries.md ~289-312) + amended bite wording (advisor-u4r-build1-full.md ~33-35), ruling (advisor-u4r-r2run-full.md steps 2-4).
3. Q2: judge evaluation path (crates/judge), lab writer of tick.s.snap_incomplete + bite site; record under
   .gate/netcode-proof-test/c62h-5e563b9589d3-record/ (README + two verdicts); first try + diag beside it.
4. Q3: PM-0001-17 whole entry, -57/-58; results/u4r P2/P3/B1/P0; u4r-report.md judged-sets sections.
5. Findings, proposed bars, StructuredOutput.

## Progress (a line per answer and finding)
- marks.json HEAD sha256 9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5 (= the c62h record README's).
  `git log 5e563b9..HEAD -- marks.json crates/judge`: empty.
- Q1 (partly): committed check (marks.json:3972-3981) == r2run ruling JSON (advisor-u4r-r2run-full.md:9; python json
  compare True); name in u4-build-plan.md s11 (:1094-1100, a36babd). vs carry 62 (h) JSON (u4s-carries.md:305-309):
  2 diffs, name prefix "setup: " and "setup": true, both dropped by build ruling 4 (advisor-u4r-build4-full.md:32:
  setup form would make the tick16 and whole-as-lost bite runs INVALID, eval.rs:1189-1208) then r2run. Carry's own
  amendment (u4s-carries.md:314-316) still says "JSON stays word for word". Bite "what" (marks.json:4011) == build1
  amended wording (advisor-u4r-build1-full.md:33) exactly. ADR 0001:1605 has no such check or bite (row sha
  3075f638... unchanged = marks draft.row_sha256): a stricter reading pending Mustafa's go (readings PR before R5).
- Q2 (yes): bite site lab/src/logs.rs:317-330 + :362-366 -> Res::Lost -> snap_incomplete 1 (:441-449). Judge:
  eval.rs:1451-1458 (checks of matched part L1 P0), :1209-1211, :282 sum, :123 measured, :1477-1478 shown.
  Record whole-as-lost-verdict.json:157-178: shown, run valid, mismatch pass 0, new check fail 48 (== 0). tick16: 2880
  and 720 fail. cmp with diag-with-2 equal (sha256 0ee24942..., da62fe7f...). Judged P0 set 93509dfd7fd1
  verdict-bites.json: whole-as-lost shown 48 again; clean s1-s3 read 0. Only this check catches the bite.
- Q3 (not bounded): new check parts ["L1 P0"] only; P2/P3/B1 only the report (window all, marks.json:3993-3998).
  figures.rs:466-481 coverage resolves a tick by incomplete 1 as well as by hash; :547 empty only at 0 compared in the
  whole run (not per slot). Client files "lost unless filed" when a tick leaves the ring (client/src/lib.rs:580-607),
  so any filing slip defaults to lost. -57 errors only for ticks filed with a non-sentinel hash (logs.rs:15-17). -2
  everyone-in-view is room-side. -18 heal bounds only first decode after a burst on B1 (INVALID today). P2 measured
  incomplete 17,260/17,089/17,532, P3 43,295/43,210/43,454 (u4r-report.md:992,1002), ~2.0 % and ~5.0 % of ~864,000 =
  the profiles' loss (l1/src/lib.rs:185-192). B1: no judged -17 value (64f3307d5c7f all 3 runs INVALID by pre:-2).
- Findings: TE-C62H-1 Medium (no bound on P2/P3/B1), TE-C62H-2 Low (carries file stale JSON), TE-C62H-3 Low
  (bar built on ruling, go pending; report wording).
- Proposed bars: PB-1 (unexplained incomplete == 0 on L1 P2/P3/B1, from L1 fate log, with bite on P2).
- Status: DONE; result returned by StructuredOutput.
