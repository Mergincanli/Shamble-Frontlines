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

## Questions for the main session
- Q1: `select` lists `machine.name`/`machine.arch` but not `machine.role`; `from` uses `machine.role`. May a part's select use `machine.role`? (I use it in parts only where a part must be one machine kind.)

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
