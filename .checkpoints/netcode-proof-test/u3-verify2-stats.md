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
- S1 (item 1, at_most): fixes.rs:636-644 is the binomial CDF P(X <= k), X ~ Bin(n, r), by the term ratio
  (n-i+1)/i * r/(1-r): correct for 0 <= r < 1 (r = 1 gives 0 * inf = NaN; never reached). Recomputed with exact
  comb() sums (python): identical to 4 decimals.
- S2 (item 1, "99 % of 480" = at most 4 over): the judge's p99 is the nearest rank ceil(0.99 x 480) = 476th value
  (crates/judge/src/eval.rs:260-285, exact Ratio), so "p99 <= bar" <=> at most 4 of 480 over the bar. CONFIRMED.
- S3 (item 1, BUG, the product): fixes.rs:699 multiplies P(at most 4 over bar99) by P(none over barmax). The two
  events are positively dependent (every phase over barmax is also over bar99), so the product UNDERSTATES the joint
  chance. Exact joint under the same independence (multinomial: none over max, at most 4 in (bar99, barmax]):
  recommended (0.8,1.0) 0.585 not 0.558; (0.8,1.5) 0.912 not 0.910; (1.0,1.5) 0.984 same; fix 1 alone (1.0,1.5)
  0.438 not 0.379; (1.0,2.0) 0.697 not 0.686. Effect: the bite (fix 1 alone failing) is weaker than reported.
