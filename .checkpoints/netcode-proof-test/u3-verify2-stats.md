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
  0.414213562) x 2 s). Target dir .gate/u3v2-stats-target. Dumps and the analysis scripts an.py-an5.py in
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
  burst is evidence against independence). Also: each model phase is a fresh connection 5-7 s old; a real client's
  10 bursts come in sequence on connections shaped by the previous burst (often a takeover a few seconds earlier,
  H6). Not modelled; unclear.
- S8 (item 2, timing sensitivity, a run-design condition): fix 1 alone's over-1.5 s rate depends on the burst's
  phase against the room tick: 0.65 to 2.18 per mille across tick-phase deciles (600,000 phases, about 60,000 per
  decile: 39 to 131 phases, rising with the phase). The recommended row shows no resolvable effect (0.02-0.10 per
  mille, 1-6 counts per decile). Shared within a run it barely moves the pass chance (S6) IF the 10 bursts take
  independent uniform tick phases; if L2 starts bursts on tick boundaries or at a period that is a multiple of the
  tick, all 10 share one phase. The run must draw burst instants uniformly against the tick and log the phase.
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
  over 1/1.5 s 699/175): passes 0.869 (about 0.85-0.89): a build whose adaptive budget is broken passes 87 % of runs.
  The proposed bar checks only the tail, so it cannot see fix 1 (whose gain is the median: 435 vs 744 ms).
- S11 (item 4, a better-discriminating bar at 480, search over "at most m of 480 over X, none over Y" and an optional
  nearest-rank p50 part; exact under independence, 1.2 M phases; script an4.py): with a p50 part "p50 <= 0.6 s"
  (rec's p50 435, share <= 0.6 s 0.975; today and fix 3 alone p50 744-746, share 0.008) fix 3 alone and today pass
  0.000. Candidates with rec >= 0.95: "p50 <= 0.6 s, at most 2 over 1.0 s, none over 1.6 s": rec 0.975, fix 1
  0.268, fix 3 0.000, today 0.000; "p50 <= 0.6 s, at most 1 over 1.2 s, none over 2.0 s": rec 0.974, fix 1 0.136;
  "p50 <= 0.6 s, at most 1 over 1.2 s, none over 1.5 s": rec 0.955 (too close to 0.95), fix 1 0.117. Two-part only,
  the best is "at most 2 over 0.9 s, none over 1.6 s": rec 0.953, fix 1 0.267, fix 3 0.307, today 0.001. No bar of
  480 client-bursts separates rec from fix 1 alone well: both differ only in a 0.1 % vs 0.7 % tail (0.45 vs 3.5
  expected over 1 s per run). More client-bursts per run is the lever (S12).
- S12 (item 4/7, more bursts per run; an5.py, rec >= 0.97 required): 20 bursts (960): "p50 <= 0.6 s, at most 2 over
  1.2 s, none over 2.0 s": rec 0.986, fix 1 0.029, fix 3 0.000, today 0.000. 30 bursts (1,440): "p50 <= 0.6 s, at
  most 3 over 1.2 s (nearest-rank p99.75), none over 2.0 s": rec 0.992, fix 1 0.007, fix 3 0.000, today 0.000.
  50 bursts (2,400): "p50 <= 0.6 s, at most 4 over 1.2 s, none over 2.0 s": rec 0.990, fix 1 0.000. The proposed
  form at 1,440 (99 % within 1.0 s = at most 14 over, none over 1.5 s): rec 0.928 (worse: a fixed 1.5 s max loses
  as N grows), fix 1 0.131. So: more bursts plus a p50 part plus a 2.0 s maximum discriminates; the 1.5 s maximum
  does not scale.
- S13 (item 7, 60,000-phase maxima are noise; WS-FIX's row choice): maxima of 10 disjoint 60,000 blocks (ms):
  never abort 1,516-2,094 (sets 0-59's 1,685 is the 3rd lowest); abort 500 ms 1,510-2,204; two endpoints 1,819-2,920.
  At 600,000: never abort over 1/1.5 s 531/35, max 2,094; abort 500 ms 688/32, max 2,204; two endpoints 332/51, max
  2,920. "Abort at 500 ms: max 1,546" vs "never abort: max 1,685" is not a difference; the over-1 s count is (531 vs
  688, z 4.5, never abort better); two endpoints has the best over-1 s count and the worst tail.
- S14 (item 6, the pinned test fixes.rs:624-633): runs in check.mjs (cargo test --locked --workspace, debug, twice:
  check.mjs:198-200): 3.5 s per run in debug here (4 cores): fast enough. The model is integer-only (no f32/f64 in
  tcp_model.rs; f64 only in at_most and the tail's printing), so the exact pin is platform-independent: a sound
  REGRESSION pin. But its first assertion, "max <= 1.5 s && p99 < 1 s" (fixes.rs:628), and the doc "Mustafa's goal
  ... pinned" (fixes.rs:617-619) over-claim: of 100 disjoint 6,000-phase blocks of the same row, only 71 have max
  <= 1.5 s (block maxima 1,101-2,094, median 1,434). Sets 0-5 pass by luck; any harmless model change that re-rolls
  the phases has a 29 % chance of failing the "goal" with no regression. It pins 3 order statistics only (a change
  between them passes); a pin of counts over 0.8/1/1.5 s or a hash of the sorted vector is tighter.
- S15 (item 7, tight/loose summary): TOO TIGHT: the 1.5 s maximum sits below the model's own maximum (2,094 and
  1,976 in two 600,000 samples); a correct build fails 2.5 % of runs in the model; the advisor's earlier rule
  (model max x 1.25, rounded up to 0.1 s, advisor-u3q.md:195) would give 2.7 s from 600,000. TOO LOOSE: the 99 %
  part is free for a correct build (0.9999) and the whole bite lives in the maximum; it misses a broken reconnect
  (fix 1 alone) in 43 % of runs and a broken adaptive budget (fix 3 alone) in 87 %. The pass chance is a model
  figure: L2 cannot reproduce the modelled reconnect (H17: ~2 RTT faster), so in L2 the bar is easier still; a real
  browser on Windows with a 3 s SYN timer (H11) behaves like fix 1 alone. Flake rule: with a designed 2.5 %
  false-fail rate a fail must be root-caused, never re-run; the run needs per-connection TCP_INFO (retransmits, rto,
  srtt) at each client-burst so an over-bar burst can be matched to the modelled tail mechanism.

## Verdicts (finished block)
1. at_most() binomial sum: CONFIRMED (S1). "99 % of 480" = at most 4 over: CONFIRMED against the judge (S2).
   The product with the maximum's chance: REFUTED as exact (S3): it understates the joint; small for the
   recommendation, 0.379 -> 0.438 for fix 1 alone at (1.0, 1.5).
2. Independence of the 480 client-bursts: CONFIRMED in the model (S6: dispersion 1.00-1.01, shared-timing pass
   chance = independent within 0.001); UNCLEAR for the real run (S7, S8).
3. 60,000 phases resolve the 1.5 s maximum: REFUTED (S4, S5): 2 of 60,000 was a low draw; 600,000 give 35 (5.8e-5),
   pass chance 0.975 (0.969-0.981), not 0.984 (60,000's own interval 0.944-0.998); the model's maximum is 2,094 ms.
4. Bites of (1.0, 1.5): today CONFIRMED (0.042 pass); fix 1 alone weak (0.432 pass); fix 3 alone not caught (0.869
   pass): REFUTED as a check that shows a broken fallback. Better bars exist (S11, S12).
5. Seed sets 0-59 independent and unbiased: fates independent CONFIRMED; burst instants unbiased CONFIRMED; input
   phase is a per-seed sawtooth: set 0 and the 6-set tables are biased in the body (S9); the 60-set tail is not.
6. Pinned test: sound as a deterministic regression pin and fast (3.5 s debug); its "goal" assertion is seed luck
   (71 of 100 blocks pass) and its name/doc over-claim (S14).
7. The bar is both too tight (maximum below the model's own) and too loose (blind to a broken adaptive budget, weak
   on a broken reconnect) (S15).
Numbers the main session will send, checked: today 746/947/3,170 (6,000) CONFIRMED as computed, not its maximum
(600,000: 6,262); fix 1 alone row CONFIRMED as computed; recommended row CONFIRMED as computed but REFUTED as the
model's tail (600,000: p99.9 973, p99.99 1,447, max 2,094; 35 over 1.5 s); pinned 437/665/1,318 CONFIRMED
(PINNED_RECOMMENDED); the abort/two-endpoint maxima are 60,000-phase noise (S13); the pass chances: arithmetic
CONFIRMED to 3 decimals (u3fix-tail1.txt) but the product is approximate (S3) and the inputs under-resolved (S5).

## Bugs (finished block)
- B1 (S3) fixes.rs:699: product in place of the joint; effect: fix 1 alone's (1.0,1.5) pass 0.379 reported, 0.438
  exact (60,000) / 0.432 (1.2 M); recommended (0.8,1.0) 0.558 vs 0.585.
- B2 (S4/S5) fixes.rs:673 (sweep_sets(o, 60)): 60,000 phases for a bar set at the 1-in-17,000 level; effect:
  recommended (1.0,1.5) 0.984 reported, 0.975 at 1.2 M; max 1,685 reported, 2,094 at 600,000; over 1.5 s 2 vs 35
  per 600,000. Fix: 600 sets (27 s release) and report the interval.
- B3 (S9) fixes.rs:169 + :388-393: input phase = (seed x 7,919) mod tick, a sawtooth: a 1,000-phase set covers 24 %
  of the tick; effect: S13 (e)'s set 0 and the 6-set rows biased in the body (today over 0.8 s 132-215 per mille by
  decile); no resolvable effect on the WS-LIMIT tail.
- B4 (S14) fixes.rs:617-628: the pinned test asserts the goal on a lucky 6,000 (71 % of blocks pass); effect: none on
  the numbers; a false "meets the goal" claim and a 29 % spurious failure on any re-pin.

## Must correct before Mustafa decides (finished block)
1. Replace the 60,000-phase tail by 600,000+ (S4): recommended max 2,094 ms (not 1,685), 35 per 600,000 over 1.5 s,
   p99.9 973, p99.99 1,447; the pass chance of (1.0 s, 1.5 s) is 0.975 (95 % 0.969-0.981), not 0.984; give
   intervals for every pass chance.
2. Use the exact joint, not the product (S3): fix 1 alone at (1.0, 1.5) passes 0.43, not 0.38.
3. Tell him what the proposed bar does NOT catch (S10): a build with the reconnect broken passes 43 % of runs; with
   the adaptive budget broken (fix 3 alone: p50 744, max 2,113) 87 %. Offer a bar that does (S11/S12), e.g. 30
   bursts per run (1,440 client-bursts): "half within 0.6 s, at most 3 over 1.2 s, none over 2.0 s": correct 0.99,
   reconnect broken 0.007, budget broken 0.000, today 0.000; or at 10 bursts "half within 0.6 s, at most 1 over
   1.2 s, none over 2.0 s": correct 0.974, reconnect broken 0.136, budget broken 0.000.
4. Say that the 1.5 s maximum is below the model's own worst case (2,094 ms), so a correct build fails about 1 run in
   40, and that a fail must be root-caused, never re-run; require per-burst TCP_INFO logging so it can be (S15).
5. State the pass chance as a model figure resting on independence the model supports (S6) but the real run may not
   (S7: shared address, rate limits, one LG process); require uniform burst phases against the tick and per-burst
   over-bar counts in the run's record (S8).
6. Drop "max 1,685 vs 1,546 vs 1,918" as row comparisons (S13); compare rows by over-1 s and over-1.5 s counts at
   600,000.
7. Fix the pinned test's claim (S14): keep the exact pin, drop or relabel the goal assertion; and seed the input
   phase from a hash (S9) before the next re-pin.

Cleanup: the diag edits were reverted (fixes.rs restored from HEAD with git show, diag.rs deleted) and the worktree
.gate/u3v2-stats removed (git worktree list: main folder only). The build output .gate/u3v2-stats-target (563 MB,
gitignored) is left: rm -rf was denied; the session may delete it. Scratch outputs kept:
scratchpad/u3v2s/ (dumps, an.py-an5.py). Nothing committed or pushed.

DONE
