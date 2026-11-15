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
