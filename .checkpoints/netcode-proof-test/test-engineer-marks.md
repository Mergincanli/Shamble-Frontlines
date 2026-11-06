# test-engineer: marks.json draft (SPIKE_PLAN item 1, plan §6.1)

Task (main session, 2026-11-06): draft every pass mark as data against `spikes/netcode-proof/marks/README.md`
(the contract; never changed), plus the series registry. Notes only; no code, no spikes/ or docs/ edits, no web.

## Plan
1. Read the contract (done), plan §4.3, §4.7, §6.1-6.3, §15; digests (netcode §A, infra §A, mine §D-§F).
2. Hash every row at 16763a724b741a40615dc8e7c2ef25c5ea3e4b30 (.gate/npt-readings) with a small script in
   `scratch-te/` -> `scratch-te/row-hashes.json`.
3. Read the ADR keys (V-S, Bites, Soaks, profiles; ADR 0006 run rules) and each row.
4. Draft marks in blocks of ~10, append each block to `marks-draft.jsonl` at once:
   - B1 PM-0001-1..10, B2 -11..20, B3 -21..30, B4 -31..40, B5 -41..50, B6 -51..63, B7 PM-0004-67,
     B8 PM-0006-11..19, B9 PM-0006-20..27.
5. Keep a notes line per question (below). Series names go into `series-draft.md` as used.
6. Validate the jsonl (parse, ids in order, hashes match, verdict kinds, no floats) with a small script.
7. Hand back: paths, counts per verdict, figures, questions.

## Progress
- Step 1 done: contract read, re-read after the coordinator's 4 additions (footer + `complete`, `machine.role`,
  check `from`, V-S on `second.s.inputs_count` and LG's `second.v.cpu_permille`). Plan §4.3, §4.7, §6.1-6.3, §15 read;
  digests read (mine §D-F, netcode §A, infra §A: all pre-R-PR, so the rows at the head rule).
- Step 2 done: 81 rows hashed at 16763a72 -> `scratch-te/row-hashes.json` (script `scratch-te/row_hashes.py`).
- Block 1 done (PM-0001-1..10): 10 lines in marks-draft.jsonl, built by scratch-te/b1.py (helpers in mk.py).
- Coordinator's 2nd note read: events carry `ph`; bite runs are judged by the checks of the parts they match. End-of-run summary events use `window: all`.
- Block 2 done (-11..-16, -17, -57, -58, -18) and block 3 done (-19..-26, -59, -27): 30 lines.
- Block 4 done (-28..-38, -39, -40, -63 outside): 44 lines. Scripts now run `python3 -I -B` (coordinator: no __pycache__ in the notes folder).
- Block 5 done (-41, -42, -60, -43..-46, -61, -47, -48): 54 lines.
- Block 6 done (-49..-52, -62, -53, -54..-56, PM-0004-67): 64 lines.
- Resumed after the usage limit (draft at 64 lines, PM-0004-67 last). Block 7 done (PM-0006-11..-19): 73 lines.
- Block 8 done (PM-0006-20..-27): 81 lines, all rows. fix1 (-27 times-of-day figure; room CPU share renamed proc_cpu_permille), fix2 (status/exit code to flags), fix3 (-58 reads -41's untraced count: a max over every correction would FAIL at P3; -19 reuses decoded_hash_mismatch_count; -14 reads -36's mds_read). validate.py: 0 errors.

## Questions for the main session
- Q1: `select` lists `machine.name`/`machine.arch` but not `machine.role`; `from` uses `machine.role`. May a part's select use `machine.role`? (I use it in parts only where a part must be one machine kind.)
- Q9: setup conditions (tick_start, ticks_below_wrap, queue depths) are checks named 'setup: ...', so a wrong setup FAILs; INVALID would be honest. Proposal: a `setup` validity rule reading header params.
- Q10: the jitter of -3 is read as both delay-range ends and the uniform's p90 within 5 % of the half jitter (the p90 catches a normal jitter with a matching range). Confirm, or name the statistic.
- Q11: per-client bandwidth means (-12 ES, -13): the draft checks the largest per-client mean (figures `down_client_mean_max_bits`, `up_client_mean_max_bits`); a pooled mean would be weaker.
- Q12: -19's '100 joins' read as LG's; the ML Chrome leg (with Worker-trap rejoins) has no count in the row: the draft asks >= 1.
- Q13: -25's IPv6 /64 cycler runs only where LG has IPv6 ('recorded if it has not'): as a part selected by `params.lg_ipv6`, without IPv6 it and its bite stay NOT-RUN. Optional parts?
- Q14: -25's 'closed 2 s after accept': checked as `<= 2 s` exactly; the sweep's granularity needs a tolerance the row lacks.
- Q15: -27's 'within 5 %' after the SIGKILL read as not worse by more than 5 % (after/before <= 1.05); and -27 names no bite: may PM-0006-13's busy-wait bite run list -27 too?
- Q16: string-valued records (-28's negotiated draft, browser versions, -2's stand-in names, the clock resolution in `machine`) can't be read by any stat. Proposal: the verdict detail prints named header params and machine fields.
- Q17: -30's second bite expects INVALID (a late command must void the run); an invalid bite run makes the mark INVALID under the contract, so it is left out of `bites`: a judge fixture instead?
- Q18: -28's control (a wrong certificate hash fails 100 of 100) is a part with role `control` and a sum == 0 check, not a bite (a bite only needs to fail one check).
- Q19: -42's number is at P3; plan §6.2 runs it at P2 to P4 under both options: the draft judges P3 (both options) and reports P2 and P4.
- Q20: -48's false-hit share: denominator read as view misses ('a miss in the view counted as a hit').
- Q21: -49's second 'bite' (a client delaying every pong by 150 ms must still meet the cap) must PASS, so it rides in the rule-cap runs' cheat set, not in `bites`.
- Q22: PM-0004-67 reads the jitter from the FIRST 36,000 measured ticks of each S30; the contract's windows have no first-N form.
- Q23: -56's item (1) asks both crates' results on -11 and -25, whose rows don't ask for both crates: the run plans must run them per crate.
- Q24: reports without `parts`: the draft assumes the judge computes them per part.
- Q25: ADR 0006's key (the advisor's reading, 2026-11-06): npt-judge checks the 3-sessions rule (>= 6 h apart, one 18:00-23:00 Amsterdam) from the results' UTC times. The header has `date` only: it needs a UTC start time (`params.started_utc`, passed in by the caller). The draft uses a figure `times_of_day_rule` on -27, PM-0006-13 and -14.
- Q26: PM-0006-19 compares POOLED p99s on both sides; `against` combines per-run statistics, so the draft uses a figure `logging_cost_ratio`.
- Q27: PM-0006-19's validity (each 'off' run's p99 within 1 % of the pooled 'off' p99) is no README §4 rule.
- Q28: money (PM-0006-18) has no unit suffix; PM-0006-18's numbers ride in the person event's steps.
- Q29: PM-0006-11 is listed by every run, but the density runs are unpinned (no affinity events): the sibling check needs those runs excluded.

## Notes
- Q2: a part's `select` on per-log fields (`machine.*`, `process`): does a run match when ANY of its logs matches? (I assume yes.)
- Q3: README §1's unit suffixes have no length or angle. -41, -42, -46, -51, -55, -57, -58, -60 need lengths, -57 angles. Proposal: `_um` (micrometres) and `_microdeg` (millionths of a degree); the draft uses them, pending this answer.
- Q4: validity is per mark, but -58's `reorder` rule applies to its P3 parts only (P0 has no reordering). Proposal: an optional `validity` on a part. The draft leaves it in -58's notes.
- Q5: may a `report` entry take `parts`, `from`, `window` and `where` like a check? Rough legs (ML*) and the P4 legs of -13 are report-only parts.
- Q6: stand-in vs named machine: the draft selects the named laptop as `machine.name` "ML", the stand-in as "ML*", the runner "MAC-r", the borrowed Mac "MAC-b". RD has no `machine.role` in README §1 (RS, LG, MV, C, ML, MP, MAC): add `RD`?
- Q7: preconditions (-1's machine header, -2's load) are their own marks; the judge does not chain them into -4 to -27. A run whose load fell short could pass -4. Proposal: a validity rule `load` (-2's counts) on every SS/ES timed mark, or chain -2's run verdict.
- Q8: B-S / B-N bite sizes (mark − measured p99 + 10 % of mark) are not checked by the judge. Proposal: the bite run header carries `params.bite_size_ns` / `params.bite_pad_bytes`, reported beside the bite.

## Notes
- Thresholds are the rows' literals in base units (16.67 ms = 16,670,000 ns, not 1/60 s; MB = 10^6 B, kbps = 1,000 bit/s, key 1495-1496).
- Key 1477-1478: p99 reads measured ticks; max, overrun and dropped-time counts read the whole run (window `all`).
- ES (key 1436-1441): 48 soldiers, 15 big vehicles, all 47 + 15 in view on >= 99 % of snapshots, 72 shots/s, 10 reliable events/s, 32 live projectiles at 0.44 spawns per player per second (21.12/s); rates within +-5 % (proposed).
- -1's bite must fail -1's own affinity check, so -1 must NOT list V-S (a V-S failure would make the bite run INVALID, not FAIL).
