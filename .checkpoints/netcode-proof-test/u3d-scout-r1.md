# U3d scout (test-engineer): how R1 runs, records and judges U3d's self-test

Branch claude/netcode-proof-test-plan-hcky5g, head 95dac9c. Read-only: no code, no builds, no runs, no network.
Memory: no .claude/agent-memory/test-engineer folder exists (checked 2026-11-08).

## Plan
1. u3-build-plan.md:5-12, :45-46 (U3d line, M-mapping M1, M13, M14).
2. advisor-u3q.md M1 (:233-236), M12-M15 (:265-275), course change :30; Mustafa's U3-Q1 (c) and U3-Q9 answers (checkpoint).
3. advisor-u3c-plan.md rulings 2026-11-08: condition 10, diagnostic, UP-STAMP, ImpairSocket up E ~10 us.
4. plan.md §12.1 U3, U6, U8 rows (~:1220-1230); other mentions of self-test, R1 run plan, U3-Q9.
5. Code: l2/socket.rs, socket_tests.rs (:505+), stats.rs, delay_tests.rs; crates/lab; marks/marks.json, SERIES.md; crates/log; ops/runs/.
6. results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt.
7. Synthesis: where R1 expects it; what recorded where; judged vs reported; output format; U3d's evidence here; bites; exists vs gaps.

## Findings (one line each, file:line)
