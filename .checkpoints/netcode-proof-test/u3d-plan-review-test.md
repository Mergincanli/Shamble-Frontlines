# test-engineer: adversarial review of u3d-plan.md rev 1 (U3d, the timing self-test)

Started 2026-11-09 on branch claude/netcode-proof-test-plan-hcky5g, HEAD f6041cb (95dac9c + checkpoint-only commits;
code as at 768d00b). Read-only: no code edits, builds, test runs or network.

## Plan
1. Read the plan (rev 1), scouts' notes (u3d-scout-req.md, u3d-scout-r1.md), plan-notes (u3d-plan-notes.md).
2. Check every cited source: stats.rs (E definition, L0, windows), impair.rs, socket.rs, socket_tests.rs (M13 run),
   delay_tests.rs, config.rs, npt-l1 lib.rs (P1), marks.json PM-0001-3, marks/README.md, SERIES.md, plan.md rows,
   advisor-u3q.md M1/M13/M14, advisor-u3c-plan.md rulings, u3-build-plan.md, u3-verify-timing.md, c10 result file.
3. Per test T1-T4: can it fail? does the bite fail it for its own reason? any timing dependence?
4. Statistics: E against L0 + 0.25 ms as defined; the L0=0 form; T2's 1,000 ns tolerance; the 5-sd loss band.
5. Dry-run honesty: what is claimed vs what the run can show; vacuous checks.
6. R1 handoff: everything R1 needs is carried (U6/U8 rows), nothing judged here that only R1 can judge.
7. Findings list + verdict.

## Findings log (one line per finding as found)
