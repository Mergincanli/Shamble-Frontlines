# Advisor: rulings on U1's open questions (2026-11-06)

Input: `.checkpoints/netcode-proof-test/u1-questions.md` (untracked, read at branch head 5ac4042).
Plan: `.checkpoints/netcode-proof-test/plan.md` rev 3.2. Drafts: `.gate/npt-readings` (1b4094a).

## Plan of work
1. Read the questions file, the plan's relevant sections, the catalogue §8, marks README, marks.json, SERIES.md,
   test-engineer and netcode-architect notes.
2. Check each C item against the code (eval.rs, redact-check.mjs, check.mjs, Cargo.toml).
3. Rule each item; flag any that move a number or meaning (R-PR route).
4. Verdict on U1.

## Notes (as I go)
- Read: questions file, plan §4.1-4.7, §6.1-6.3, §10.2-10.6, §12, §15; catalogue (whole); marks README (whole);
  eval.rs (whole); netcode-architect and test-engineer notes.
- C2 finding: README §2.3 (line 100-101) says "An empty series makes the check fail, never pass"; eval.rs collect()
  returns Err -> Missing::Gap -> Outcome::Gap -> run INVALID (eval.rs:221-229, 369, 888-893). Contract and code
  disagree; the code matches C2. README must change. Also: any check on a count that may legitimately be 0 must not
  read an event series (no events -> gap -> INVALID). To check in marks.json.
- C5: eval.rs:1237-1263 matches README §3.5. eval_part:953 makes a part INVALID if ANY member run is invalid, even
  with min valid passing runs: an old invalid run left in the folder voids the part forever. Check ADR V-S wording.
- B6: eval.rs:1051-1067: no matched part -> checks without parts; none -> Undecided -> NOT-RUN. README §5 lacks the
  "undecided" sentence.
- C2 more: marks.json's zero-expected checks read summary events paired with a "ran" count (good). But SERIES.md has
  no rule that counters/flags are written on every record (0 included): a sparse writer would make clean runs INVALID
  (e.g. tick.v.overrun sum == 0, late-zero's tick.v.late_count). Needs a SERIES.md rule.
- C5 gap (not asked): ADR 0001:1521 "An invalid run ... its cause is found and written down before it is measured
  again". eval_part:953 keeps any invalid run blocking the part forever; the judge has no record of a written cause
  or a superseded run. Needed before the first counted run (U8), not for U1.
- C8: npt-log also exports SCHEMA; npt-log and npt-judge depend on serde_json (plan says "serde only"); check.mjs
  allows serde_json for the judge. Read "serde" as serde + serde_json; name it in plan text.
- C10/C11: check.mjs:36-91 and :99-112 match the session's readings.
- C6/C7: redact-check.mjs:31-46, 101-118 match. Prefix forms only catch prefixed text: U6 must never print a token,
  capability or bind secret in any other form (hex, bare base64).
- Prior rulings: R-15 already reads PM-0004-67's jitter from the first 36,000 measured ticks (Q22 is in R-PR).
- Reproduced: `node spikes/netcode-proof/check.mjs` at 5ac4042: all 9 steps PASS (44 cargo tests, 9 self, 10 redact;
  275 tracked files, 81 marks, rows at 1b4094a unchanged).
- C12: marks.json's 81 entries equal marks-draft.jsonl's 81 objects (0 differ); SERIES.md differs from
  series-draft.md in 4 lines (title, the 2-line provenance, the units rule): hand edits after the copy, fine.
- C2 REPRODUCED (scratch fixture, scratchpad/c2demo): a bite run whose defect FAILs one check ("queue held at the
  cap") and removes an event (count check -> gap) comes out INVALID; the mark INVALID; the bite is never shown.
  eval.rs judge_run:885-896 pushes a gap into invalid_why; eval_bite:1059 checks validity first. Same at run level:
  a measured run with a FAIL and a gap is INVALID, not FAIL (eval_part:949-953).
- C4: `params.killed` also listed for -59 and PM-0006-22 (SERIES.md:351). -59's catch should write its own footer.
