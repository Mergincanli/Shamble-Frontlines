# u3-verify2-stats: skeptic on the WS-LIMIT statistics (test-engineer)

Branch claude/netcode-proof-test-plan-hcky5g, head cc72f6f + checkpoint commits. Started 2026-11-07.

## Plan
1. Read fixes.rs: at_most(), ws_limit_tail, phase_at, PINNED_RECOMMENDED, the set loop; tests.rs S13 pins.
2. (1) Check the binomial sum and "99 % of 480 -> at most 4" and the product with the max chance, by hand (python).
3. (2) Independence: per-set over-1 s counts vs binomial dispersion; then a worktree copy that runs 480 client-bursts
   sharing 10 burst timings (48 clients each) and checks the over-bar count's distribution.
4. (3) Resolution of the 1.5 s max bar with 2/60,000 over: CI on p, CI on the pass chance.
5. (4) Bites: pass chance of today / fix 1 alone under (1.0, 1.5) with intervals; search for a better two-part bar.
6. (5) Seed sets: phase_at formula, golden-ratio spread, overlap/bias.
7. (6) Pinned test: what it checks, tolerance, runtime, check.mjs wiring.
8. (7) Anything else too tight / too loose.
Write a line after each finding; DONE at the end.

## Findings
