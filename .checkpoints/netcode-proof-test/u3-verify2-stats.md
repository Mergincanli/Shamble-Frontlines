# u3-verify2-stats: skeptic on the WS-LIMIT statistics (test-engineer)

Branch claude/netcode-proof-test-plan-hcky5g, head cc72f6f + checkpoint commits (73f451d; spikes/ unchanged since
cc72f6f). Started 2026-11-07.

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

## Method
- Worktree .gate/u3v2-stats (detached at 73f451d); throwaway module tcp_model/fixes/diag.rs (+ "mod diag;" appended to
  its fixes.rs): diag_sets_dump (phase_at sets 0..599, unsorted: k i seed burst recovery) and diag_shared_dump
  (12,500 burst instants, 48 clients each with their own seeds 1,000,000 + 48j + c; burst 5 s + frac((j+1e6) x
  0.414213562) x 2 s). Target dir .gate/u3v2-stats-target. Dumps and the analysis scripts an.py-an4.py in
  scratchpad/u3v2s/ (600,000 phases per row, about 26 s each on 4 cores). Extra row "fix3" = fix 3 alone (64 KB
  budget, no adaptive budget, serial(300 ms, never abort)).
- Check: the dump of sets 0-59 reproduces the reported recommended row exactly (435/659/1,025/1,363/1,685; over 0.8,
  1, 1.2, 1.5, 2 s: 279, 63, 42, 2, 0), fix 1 alone (437/434/433/82/8 over 0.8/1/1.2/1.5/2 s; max 5,106) and today's
  6,000 (sets 0-5: 746/947/max 3,170).
- Slip, recorded: block S1-S3 was first appended with a python heredoc (a shell write); from S4 on the Write tool.

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
- S4 (item 3, MAJOR, 60,000 phases under-resolve the tail): 600,000 phases (sets 0-599, same phase_at) of the
  recommended row: p50 435, p99 663, p99.9 973, p99.99 1,447, max 2,094 ms; over 0.8/1/1.2/1.5/2 s: 2,734, 531,
  298, 35, 1. Over 1.5 s is 5.8e-5 (35/600,000), not 3.3e-5 (2/60,000): sets 0-59 drew a low count (Poisson 95 %
  CI for 2 is 0.24-7.2; 3.5 expected). The model's maximum is 2,094 ms, ABOVE the proposed 1.5 s cap and 24 % above
  the 1,685 ms the main session will send. A fresh 600,000 with shared timings (below): max 1,976, 27 over 1.5 s.
  Fix 1 alone 600,000: over 1/1.5/2 s 4,357/816/77, max 5,106. Today 600,000: p50 746, p99 956, max 6,262 ms;
  over 1/1.5/2 s 4,391/3,904/241 (today's 6,000-phase max of 3,170 is also not its maximum).
- S5 (item 3, the pass chance's interval): recommended at (1.0 s, 1.5 s), exact joint, independence:
  from 60,000 (63 over 1 s, 2 over 1.5 s): 0.984, 95 % interval 0.944-0.998 (Garwood Poisson bounds on both
  counts); from 1.2 M (600,000 sets + 600,000 shared; 1,131 over 1 s, 62 over 1.5 s): 0.975, 95 % 0.969-0.981.
  So the corrected figure is 0.975 (a correct build fails about 1 run in 40), not 0.984 (1 in 60). The maximum part
  decides it (P(at most 4 over 1 s) = 0.9999 at 0.45 expected per run). Other bars from 1.2 M: (0.8, 1.0) 0.615;
  (0.8, 1.5) 0.908; (1.0, 2.0) 0.9995 (95 % 0.998-1.000; 1 over 2 s in 1.2 M).
- S6 (item 2, independence, model side: CONFIRMED that shared timing does not matter in the model): 12,500 burst
  instants x 48 clients (each instant's 48 clients share the burst and the room's tick phase; their input phases also
  sit within 0.38 ms of each other, since seed x 7,919 mod tick steps 7.9 us per seed: more sharing than a real run).
  Per-instant over-bar counts vs Bin(48, p), index of dispersion (var / binomial var): recommended 1.003 (0.8 s),
  1.010 (1 s), 0.998 (1.5 s); fix 1 alone 1.005, 1.004, 1.006; all |z| < 0.8. Pass chance with 10 shared instants
  per run (exact 10-fold convolution of the per-instant (over99, overmax) distribution) vs independent: recommended
  (1.0,1.5) 0.978 vs 0.979; fix 1 alone 0.426 vs 0.426; today 0.041 vs 0.040. Across the 600 sets of 1,000 (over
  1 s): dispersion 1.018 (rec), 1.059 (fix 1), 1.122 (today, z 2.1). The tail is driven by loss draws, not timing.
  Exception: today's BODY (over 0.8 s) is overdispersed: 5.85 across sets (z 84), 1.84 across instants (z 66).
- S7 (item 2, independence, real-run side: UNCLEAR, not measurable from the model): the model gives each client its
  own path and server; the real L2 run shares LG's one process and address, RS's room tick, DelayStream, the
  per-address and new-session limits (ADR 0001:445-450; H6/H18: 48 Joins from one address after every burst) and
  the kernel. Any of these correlates the 48 clients of a burst; the binomial pass chance is then optimistic. The run
  should log each burst's over-bar count so a run can be checked for clustering (e.g. all of a run's overs in one
  burst is evidence against independence).
- S8 (item 2, timing sensitivity, a run-design condition): fix 1 alone's over-1.5 s rate depends on the burst's
  phase against the room tick: 0.65 to 2.18 per mille across tick-phase deciles (600,000 phases; 49 vs 131 counts).
  The recommended row shows no resolvable effect (0.02-0.10 per mille, 1-6 counts per decile). Shared within a run it
  barely moves the pass chance (S6) IF the 10 bursts take independent uniform tick phases; if L2 starts bursts on
  tick boundaries or at a period that is a multiple of the tick, all 10 share one phase. The run must draw burst
  instants uniformly against the tick and log the phase.
- S9 (item 5, phase_at's seeds: independent, but the input phase is a sawtooth, a BIAS for small set counts):
  seeds 1000(k+1)+i are distinct over sets 0-599 (1,000..600,999) and ChaCha8 keys each by seed (tcp_model.rs:
  645-651), so the fate streams are independent. Burst: 5 s + frac(j x 0.618033989) x 2 s, j = 1000k + i, a Weyl
  sequence (period 1e9, gcd(618,033,989, 1e9) = 1): equidistributed, unbiased over [5, 7) s. But the client's input
  phase is (seed x 7,919) mod tick (fixes.rs:169): 7.9 us per seed, one wrap per 4,209 seeds, so ONE set of 1,000
  covers 7.9 ms of the 33.3 ms tick: set 0 (S13 (e)) only input-phase deciles 2-4; sets 0-5 (the 6-set tables and
  the pinned test) cover deciles 2-6 twice as often as the rest; sets 0-59 near uniform (5,892-6,313 per decile).
  Effect: today's over-0.8 s rate ranges 132-215 per mille by input-phase decile and 133-208 by burst-vs-tick decile,
  so S13 (e)'s set 0 and the 6-set rows of ws_limit_table are biased in the body (today's per-set dispersion 5.85).
  The recommended and fix-1 tails (over 1 s, 1.5 s) show no resolvable input-phase effect, so the 60-set WS-LIMIT
  rows are not biased by it; they are under-sized (S4). Fix: derive the input phase from a hash of the seed.
- S10 (item 4, the bites of the proposed bar (1.0 s, 1.5 s), 1.2 M phases each, exact joint): recommended 0.975;
  today 0.042 (95 % 0.039-0.045): bites; fix 1 alone 0.432 (0.415-0.450): a build whose reconnect is broken passes
  43 % of runs; fix 3 alone (reconnect without the adaptive budget, 600,000 phases: p50 744, p99 884, max 2,113;
  over 1/1.5 s 699/175): passes 0.869: a build whose adaptive budget is broken passes 87 % of runs. The proposed bar
  checks only the tail, so it cannot see fix 1 (whose gain is the median: 435 vs 744 ms).
- S11 (item 4, a better-discriminating bar, search over "at most m of 480 over X, none over Y" and an optional
  nearest-rank p50 part; exact under independence, 1.2 M phases; script an4.py): with a p50 part "p50 <= 0.6 s"
  (rec's p50 435, share <= 0.6 s 0.975; today and fix 3 alone p50 744-746, share 0.008) fix 3 alone and today pass
  0.000. Candidates with rec >= 0.95: "p50 <= 0.6 s, at most 2 over 1.0 s, none over 1.6 s": rec 0.975, fix 1
  0.268, fix 3 0.000, today 0.000; "p50 <= 0.6 s, at most 2 over 1.2 s, none over 1.6 s": rec 0.984, fix 1 0.272;
  "p50 <= 0.6 s, at most 1 over 1.2 s, none over 1.5 s": rec 0.955 (too close to 0.95), fix 1 0.117. Two-part only,
  the best is "at most 2 over 0.9 s, none over 1.6 s": rec 0.953, fix 1 0.267, fix 3 0.307, today 0.001. No bar of
  480 client-bursts separates rec from fix 1 alone well: both differ only in a 0.1 % vs 0.7 % tail (0.45 vs 3.5
  expected over 1 s per run). More client-bursts per run is the lever (next finding).
