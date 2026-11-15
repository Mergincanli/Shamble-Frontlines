Task: night 2026-11-16 item 7 (npt-u4f-brief), test-engineer check of rev 1 of U4f's build brief (tests, negative cases, bites, runs, counts, carries).

(No Edit tool in this agent: the notes file is rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1: read the approved plan (git show 7812127:.checkpoints/netcode-proof-test/night-2026-11-16.md). DONE.
- B2: read the draft u4f-brief-draft.md (rev 1, 837 lines) in full. DONE.
- B3: read the drafter's notes netcode-architect-u4f.md (NA-1..NA-12, counts). DONE.
- B4: read the scout's map N/night-u4f-scout-full.md. DONE.
- B5: read U4f's sources in N/u4-build-plan.md. DONE.
- B6: read the carries at N/u4s-carries.md (46, 80 a-e, 86, 57, 74, 21, 27, 18, 39, 50, 61, 58, 76, 77; 63/64/67 conditions).
- B7: read marks.json's -20/-21/-22/-47 rows (bites, run counts) and marks/README.md:285-290, :317-326; ADR 0001 rows 1610-1612, 1644.
- B8: read the advisor sources the draft leans on for tests and runs (advisor-u4.md:78-124, 147, 334-335, 477-479).
- B9: cross-check: tests/negative cases/bites/runs/carries present and testable; counts and budgets add up; scope creep.
- B10: write findings (TE-n) and summary; DONE.

## Notes (one line per finding / block)
- B1 done.
- B2 done (draft: 2.10 has f1-f30, claims measured +60 / bites +74 / neg 44; 2.11 17 bites, 14 new unit proofs; 5.1 36 measured + 12 twins + 17 bites = 65; 5.5 totals 65 + pre-runs, ~3,750 MB, 5.5-7 h).
- B3 done (drafter: NA-1..NA-12; counts as the draft; drafter did not read client lib.rs:810-839, logs.rs:495, contract.rs, bots :404).
- B4 done (scout: carries list incl. 62 (h) at :289-321; condition 5 at advisor-u4.md:119-124 asks the precondition pass on short runs of -20 P3 and EACH of -21's EIGHT suites; Disk: 7,689 MiB free, no figures for -47/-21 suites/option A/never-acker).
- B5 done. Facts from plan at 11fd190:
  - §9 :962-963 run counts: -20 6, -21 24 + 12 twins, -22 3, -47 3 (draft 5.1 measured 36 = 6+24+3+3: OK; twins 12 OK).
  - §5 preamble :507-512: counts as deltas; bite must fail the named test or check; pub items listed.
  - §7.1 :914-915 item 3: "a bite run shows its own test failing by name"; :916 touch after last bite.
  - §4.11 :399-401: a test runs a bite instance and a clean instance on parallel threads (draft f30, 1 test for client+room).
  - §4.11 :402-403: Send asserts Room<World>, Decoder, ClientEndpoint (draft: "kept, not new").
  - §4.10 :389-390: twin test fate logs equal every flow + snapshot group counts every tick (draft f16: forged A, forged B, clamp A, clamp B = 4).
  - §4.10 :365-366: unit test clamp-then-grid idempotent for every fire origin below TUNE-2 bound (U4d's? check whether exists; draft does not list it).
  - §4.13 :437-447: shot band per suite; "U4r's contract test shows the band per suite on its short runs".
  - §4.12 :412-415: contract test shows every check of every U4 L1 part resolves, every name has SERIES row, validity inputs exist (admit events, footer, tick.v.late_count wherever late-zero applies).
  - §4.14 :462 not-older-than: fast-clock suite, option A; :463 no-catchup-cap: forged, option A, two datagrams in one tick; :475 -22/unbounded: full S30.
  - §4.14 :467 -48/no-buffer-wait reads -20's logged depth at P3 option B (U4h's; depends on U4f's depth series).
  - §5 U4f :810-815: -21's 8 parts with twins and 13 bites "each bite run in the suite and option §4.14 or marks.json's notes name".
  - §5 U4r :758-761: contract test shows PM-0001-2 passing on short runs of -20's P3 and EACH of -21's eight suites with open-loop scripts.
  - §4.7 :333: FullState unacked within ticks_for_ms(2_000) (60 ticks) or overflow again -> Kick(overflow).
- Next: B6.
