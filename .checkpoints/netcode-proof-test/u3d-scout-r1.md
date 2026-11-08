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
- F1 U3d's line: u3-build-plan.md:45-46 "expected excess lateness beyond 0.25 ms at 20,000 releases/s, inbound measured where quinn polls, p50/p99/p99.9/max reported; here it is reported only (RS judges at R1, U3-Q9 facts read)". Mapping U3d = M1, M13, M14 (:12-13). S14 kernel check carried to R1's run plan (:47).
- F2 Mustafa's answers recorded: checkpoint .checkpoints/netcode-proof-test.md:468-481 ("Decisions taken"; "So: Q1 (c) ... under about 10 us at the tightest profile, at 20,000 releases/s on RS, percentiles reported" :473-474; "Q9 RS's kernel, HZ, cpuidle and clocksource read at R1 (R1's run plan)" :481). His literal words: "I agree with the recommendations on U3-Q1, ... and Q9" (:468-469); the "So:" is the session's record of those recommendations. Original Q list: u3-scout-summary.md:65-77 (Q1 :66-67, Q9 :77).
- F3 M1 (advisor-u3q.md:233-236): inbound at poll_recv's return, outbound at try_send's return, arrival stamped at kernel drain; both directions E[(L-L0-0.25ms)+] beside p50/p99/p99.9/max; "on loopback here it is a dry run, RS decides (Q1, Q9)". M13 (:266-268): 10,000/s each way sustained, jitter+loss+reorder on, release thread CPU recorded (0.27 core at lead 0). M14 (:269-271): slack 1 ns via /proc/<tid>, read back and tested, RT priority a recorded switch, no spin lead; T3 lead self-test dropped. M12 (:263-265) pass-through same path; header records mode. M15 (:272-274) SERIES rows for slack, sched policy, lateness per direction, writing binary named.
- F4 advisor-u3c-plan.md 2026-11-08: DelayStream self-test carried to R1 (:101-104): RS, 48 conns, P2, room running, >=10,000 samples/dir; JUDGED E < ~10 us AND p99 <= 1 ms per direction; p99.9 and max reported; max > 10 ms to advisor (:114-116). UP-STAMP item 5 (:191-192): steal from /proc/stat before and after each DelayStream AND ImpairSocket self-test run, nothing in measured path. ImpairSocket up E 11.6/16.9 us here "U3d reports it, RS judges it" (:163-165). E does not add across parts (:166-167). Fallback order for a DelayStream miss (:177-180).
