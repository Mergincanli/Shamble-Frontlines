Task: security-auditor DEP-HOLD binding check of U8's browser package (PR #40, lane
claude/netcode-proof-test-plan-hcky5g), 2026-11-22. Earlier read: security-r1-deps.md B5 (:143-180), verdict
(:241-247), U8 binding check (:256-257). Read only: the held lock, registry metadata, OSV. No installs, no builds,
no npm audit.

Files held: .gate/u8-deps/package.json sha256 8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90,
.gate/u8-deps/package-lock.json sha256 a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524
(both re-hashed by me: match).

## Plan (blocks)
- C0: the lock's facts (lockfileVersion, count, resolved/integrity, install scripts, os/cpu/optional entries).
- C1: binding check: resolve webdriverio 10.2.0 from registry packuments with the cut-off 2026-11-15 (npm --before
  semantics: newest version satisfying the range with publish time <= cut-off), compare with the lock package by
  package; explain 167 vs B5's 162.
- C2: OSV querybatch on every (name, version) in the lock; identify the 4 moderates; rule on each.
- C3: C2's substitute for the npm rule (ignore-scripts): confirm or amend.
- C4: verdict and dependency-check.md lines; DONE.

## Notes
