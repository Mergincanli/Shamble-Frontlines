# u3-verify3-stats: third-round skeptic, statistics in fixes.rs (test-engineer)

Branch claude/netcode-proof-test-plan-hcky5g, head 1e020a7 + checkpoint commits (ee9ad3e; spikes/ unchanged since
1e020a7). Read-only on tracked files. Worktree for diagnostics: .gate/te3s-wt (detached at ee9ad3e); target
.gate/te3s-target; scratch scripts and dumps in scratchpad/te3s/ (lib3.py, an1-an7.py, an*-out.txt, sets-*.txt,
shared-rec-12500.txt, lineage-rec-*.txt).

## Plan
1. Read prior stats notes (u3-verify2-stats.md) and build notes (u3-build-v3-harness.md) for context; what was fixed.
2. Read fixes.rs stats code: joint_dp, joint_closed, brute-force test, Wilson, block bootstrap, bar search, pinned FNV test, phase hash.
3. Run tcp_model lib tests (own target dir) and dump the 600,000-phase rows; compare to u3v3-tables1.txt.
4. Independent checks (scratch python / diag test in the worktree):
   (1) joint_dp vs my own exact formula + Monte Carlo; edge cases (median definition at even n, ties at X, Y<=X, Z).
   (2) Wilson math; bootstrap unit (blocks of 1,000 phases vs independence of sets); replicate count MC error.
   (3) bar grid, rule, ranking; sensitivity: tail +/-20 %, time scale +/-10-20 %.
   (4) bite rows realism; other broken builds (budget x2, trigger 1 s, Windows 3 s SYN).
   (5) splitmix64 phase hash, independence.
   (6) lineage correlation; shared-instant dependence for 48 clients.
   (7) pinned test soundness and runtime.
   (8) marks.json / npt-judge support for a three-part bar.
   (9) too tight / too loose.
5. Verdicts + bugs + must-correct. End with DONE.

## Method
- Worktree .gate/te3s-wt: throwaway module tcp_model/fixes/diag.rs (+ "#[cfg(test)] mod diag;" appended to its
  fixes.rs, and adaptive_limit x diag::BUDGET_MUL, default 1). With BUDGET_MUL = 1 the 49 tcp_model tests pass,
  the pin included (release, 0.32 s), so the hook is neutral. Dumps: te3s_dump_rows (600 sets of phase_at, one line
  per set, sorted within the set), te3s_shared (instants x 48 clients sharing a burst start), te3s_lineages
  (multi()). Analysis in python3 -I with numpy only; the joint is my own direct multinomial sum (lib3.py), not a DP.
- Reproduction: the dumps of today / fix 1 alone / fix 3 alone / recommended (600 sets each, 22-23 s a row on 4
  cores) print exactly the tables' 600,000 rows (u3v3-tables1.txt:120-131).
- Extra broken builds dumped (600 sets each, same phase_at): win3 (recommended with a Windows-like client whose first
  SYN timeout is 3 s), trig1s and trig600 (recommended with the trigger floor min_gap 1 s / 600 ms instead of
  300 ms), rec-budget2 (recommended with the adaptive budget x 2). fix3-budget2 came out identical to fix 3 alone
  (no adaptive budget there): the knob touches only adaptive_limit, as intended.
- Slip, recorded: one useless shell command created and at once deleted an empty file
  .checkpoints/netcode-proof-test/u3-verify3-stats.md.tmp (no content, nothing left). Notes are written with the
  Write tool only.

## Findings (a line each, as found)
- F0 (transcription): every number in the brief matches .gate/netcode-proof-test/u3v3-tables1.txt (600,000 rows
  :120-131; 60,000 rows :82-117; lineages :16-59; side effects :61-80; pass chances :132-151). No transcription error.
- F1 (item 1, joint_dp, fixes.rs:1549-1572): state = (count at or below X capped at need, count over Y up to k); a
  client-burst over Z drops its mass (none over Z); final sum over a = need. Correct. need = ceil(n/2) (:1485-1491)
  matches the judge's nearest-rank p50 (ceil(n/2)-th value <= X <=> at least ceil(n/2) at or below X). joint_closed
  (:1577-1589) = sum_{j<=k} C(n,j) q2^j (q0+q1)^(n-j) P(Bin(n-j, q0/(q0+q1)) >= need): correct; rest = 0 gives
  outer 0 (xlny -> -inf -> exp 0), no NaN reaches the sum. Brute force (:1871-1886) enumerates 4^n sequences, n <= 7,
  with the same predicate: correct but small (n <= 7, 3 cases); the n = 480/1,440 cases only compare DP vs closed.
  "Over Z" counts toward "over Y" in the predicate (c[2] + c[3] <= k) and is moot since c[3] = 0 is required.
- F2 (item 1, a gap): counts() (:1494-1499) sets X = min(median, y); with median > y the median part silently tests at
  y. Never reached in the grid (median <= 0.7 s < y >= 0.8 s). No effect on the numbers.
- F3 (item 5, phase hash): input phase = TICK + splitmix64(seed) % TICK (fixes.rs:919); splitmix64 (:175-180) is
  Steele-Lea-Flood's finaliser with the right constants; modulo bias ~ 33.3e6 / 2^64, nil. The v2 sawtooth (B3) is
  gone. Fate streams: ChaCha8 keyed by the seed's bytes, stream (conn << 8 | dir) (tcp_model.rs:2089-2096): distinct
  seeds give distinct keys, so phase i's reconnect (conn 1) never shares phase i+1's stream. Browser draws:
  SplitMix(splitmix64(seed ^ tag)) (fixes.rs:295). Seeds: sets 1,000..600,999 (phase_at :1037-1042), lineages
  9,000,000 + r (:1368): disjoint. Burst instants: Weyl sequence j x 0.618033989, burst-vs-tick phase = (50 + 2r) mod
  TICK: equidistributed. CONFIRMED independent and unbiased.
- F4 (item 7, platform independence): the model and harness are integer-only (f64 in tcp_model.rs only in eprintln
  traces :2456-2917; in fixes.rs only in the statistics and printing): the FNV pin is platform-independent.
- F5 (item 1, the reported chances, independent check; an1.py): my multinomial sum on the dumped 600,000 gives
  recommended two-part (1.0, 1.5) 0.2273 (reported 0.227), chosen 480 bar 0.9966 (0.997), chosen 1,440 bar 0.9900
  (0.990); Monte Carlo of the judge's own reading (nearest-rank p50, count over Y, max; 40,000 runs of iid resamples):
  0.2261 +- 0.004, 0.9968 +- 0.0006, 0.9890 +- 0.0010. CONFIRMED. Classes of the chosen 480 bar (recommended):
  448,546 at or below 0.7 s (share 0.748), 149,964 in 0.7-1.3 s, 1,486 in 1.3-3.0 s, 4 over 3.0 s. So the
  correct build's 0.3 % failure chance is almost wholly the max part: P(none over 3.0 s of 480) = exp(-480 x 4 /
  600,000) = 0.9968; the count part (1.19 expected over 1.3 s per run, k = 6) and the median part (share 0.748 vs 0.5
  needed) cost about 0.0002 together. The 0.997 rests on 4 phases; Garwood 95 % on 4 of 600,000: 1.8e-6 to 1.7e-5,
  so P(none of 480) 0.992-0.999 (the reported "rates" interval 0.991-0.999 matches; the percentile bootstrap's
  0.993-0.999 is a little narrow on the low side, see F9). At 1,440: 0.976-0.997 from the max part alone.
- F6 (items 3 and 9, MAJOR: the median part is fragile; an2.py, an2-out.txt): the recommended row's CDF is steep at
  the median (share at or below 0.6 s 0.327, 0.654 s 0.498, 0.7 s 0.748), and the bar's median sits 46 ms (7 %) above
  the model's p50. Scaling every recovery by s: the chosen 480 bar passes the correct build 0.998 (s 0.9), 0.997
  (1.0), 0.993 (1.05), 0.505 (1.07), 0.000 (1.10); the 1,440 bar 0.993/0.990/0.972/0.465/0.000. Adding d ms to every
  recovery: +30 ms 0.996, +40 ms 0.915, +50 ms 0.189, +70 ms 0.000. Pushing only the values above the median out by
  10 % (v x 1.1 for v > p50): 0.511. By contrast the tail parts are robust: tail values above 1 s stretched x 1.2:
  0.992 (480) / 0.978 (1,440); rates over Y and Z x 1.2: 0.995/0.985; x 2: 0.983/0.811. So the "+/-20 % tail" test
  passes, but a +7 % shift of the body (or +45 ms) turns the 0.997 into a coin toss, and the model's p50 already moved
  +50 % (435 -> 654 ms) between v2 and v3. The median part is the one the model cannot yet support.
- F7 (item 3, MAJOR: the search's choice is the grid's corner, not a property of the data): the chosen 480 bar has
  the grid's loosest median (0.7 s: grid 0.5-0.7, fixes.rs:1735), its loosest count (k = 6: grid 0..=6, :1734) and
  its loosest max (3.0 s: grid 1.5-3.0, :1737); the 1,440 bar likewise (0.7 s, k = 16 = the top of 0..=16, 3.0 s).
  Cause: the ranking (:1761-1765) sorts first by the broken builds' best chance rounded to thousandths; every
  candidate that fails all three bite rows ties at 0, so the next key (the correct build's chance, also rounded) and
  the tie-break (the loosest median) push every parameter to its grid edge. The ranking has no term for margin
  against model error: the correct build's point chance is flat (0.997) for any median from 0.68 to about 0.8 s, so
  the search cannot see that 0.7 s leaves 7 % margin. The neighbourhood (an2-out.txt): median 0.65 s -> correct
  build 0.19; 0.68 s -> 0.990; 0.70 s -> 0.997; 0.75 s -> 0.997, while fix 3 alone stays at ~1e-175 at 0.75 s (its
  p50 is 844 ms). The 0.000 for the bite rows is believable (fix 1 alone ~6e-6 at the chosen bar, fix 3 alone ~1e-248,
  today ~1e-319) but it is far below what any choice needs, so it carries no information about the choice.
  Re-run in python (an3.py, an3-out.txt): my copy of the ranking on the agent's grid picks the same bar (700, 1.3 s,
  k 6, 3.0 s); on a wider grid (median to 0.85 s, k to 12, max to 5.0 s) it walks to the new corner, "half within
  0.8 s, at most 8 over 1.2 s, none over 5.0 s" (correct 0.9996, worst bite 2e-8). CONFIRMED: the choice is set by
  the grid's edges.
- F8 (item 4, other broken builds; an3-out.txt; 600,000 phases each): p50/p99/max ms and the chosen 480 bar's pass:
  Windows-like client with a 3 s first SYN timeout 646/2,031/6,468 (over 1.3 s 33 per mille): passes 0.004 (caught);
  trigger floor 1 s 500/1,396/4,896 (p50 BETTER than the recommendation: the 1 s trigger fires at B1's 1 s return,
  its SYN is not lost; over 1.3 s 28 per mille): 0.021 (caught); trigger floor 600 ms 654/1,218/4,496 (over 1.3 s
  5.4 per mille, 2.6 expected per run): passes 0.980 at 480 and 0.519 at 1,440 (NOT caught); budget x 2 771/1,110/
  4,770 (share at or below 0.7 s 0.254): 0.000 at 0.7 s, caught by the median part only, and only while the
  model's time scale is within about -9 % (771 x 0.91 = 700): at a 0.75 s median it is caught only within -3 %, at
  0.8 s it passes (share 0.694). Bite rows today / fix 1 alone / fix 3 alone are gross breaks (fix 3: p50 +29 %; fix
  1: 49 per mille over 1.3 s, 20x the recommendation); none of them probes the margin. Realistic subtle breaks (a
  trigger twice too slow, a budget twice too large) sit inside the model's uncertainty band: no median bar can
  separate budget x 2 (p50 +18 %) from a correct build with an 18 % model error. A wider search with these rows as
  bites (an3.py) finds at best "half within 0.7-0.75 s, at most 9 of 480 over 1.0 s, none over 3.5-5.0 s": correct
  0.95, worst bite 0.008 (trig600): the bar cannot be both robust and sharp at 480.
- F9 (item 2, the bootstrap; an4.py, an4-out.txt): the per-set (block of 1,000) dispersion index of the class counts
  is 0.97 (at or below 0.7 s), 0.93 (1.3-3.0 s), 0.995 (over 3.0 s): no within-block dependence, so the block is a
  harmless unit (phase-level resampling gives the same interval: 0.9933-0.9990 vs blocks 0.9933-0.9990). 2,000
  replicates are enough: 10 repeats with fresh seeds put the lower end at 0.9927-0.9934 (Monte Carlo spread 0.0007),
  the upper at 0.9990. But the percentile bootstrap measures only the model's sampling noise (600,000 phases), not
  the model's error (F6) or the real run's dependence (F10): the rule "the correct build's bootstrap lower end at
  least 0.95" (fixes.rs:1779-1782) guards against the wrong risk. With 4 events driving the max part the percentile
  bootstrap is also slightly narrow at the low end (0.993 vs Garwood-based 0.992); the "rates" (Wilson) bounds
  (fixes.rs:1653-1667) are the better of the two here; Wilson's 4-of-600,000 upper bound 1.71e-5 equals Garwood's.
- F10 (item 6, dependence; an5.py/an6.py, an5-out.txt/an6-out.txt): (a) 12,500 instants x 48 clients sharing the
  burst start: the tail counts are binomial (dispersion 1.005 over 1.2 s, 0.997 over 1.3 s, 0.994 over 2.0 s) but the
  BODY is not: the count at or below 0.7 s has dispersion 1.39 (z +31): the burst's phase against the room's tick
  moves the p50 from 616 to 665 ms and the share at or below 0.7 s from 0.824 to 0.705 by decile. (b) Lineages (one
  client, bursts 20 s apart): bursts 2+ have p50 661-666 ms and share at or below 0.7 s about 0.714 (burst 1 0.75),
  over 1.0 s 13.0 vs 11.3 per mille single, over 1.3 s 2.40 vs 2.48; per-lineage dispersion of the body count 1.74
  (10 bursts) and 3.44 (30 bursts), z +40/+77: a client's bursts are strongly alike. Why: 20 s is 600.000006 ticks,
  so every burst of a lineage meets the room's tick at the same phase (drift 200 ns a burst), and the client's input
  phase is fixed per seed. A real run's 48 clients share the burst instants, so with 20 s spacing all 480 (or 1,440)
  client-bursts share ONE tick phase. At the point this costs little (480: 0.9995 vs 0.9998 independent; 1,440:
  0.9970 vs 0.9994; 48 whole lineages per run at 1,440: 0.9966 vs 0.9981), but it multiplies F6's fragility: with
  +30 ms on every recovery, 0.984 (one shared phase) vs 0.999 (independent); +40 ms 0.66 vs 0.92 (480); 1,440 at
  +40 ms 0.63 vs 0.99. The pass chance reported (independent client-bursts) overstates a real run's under any body
  shift; the lineage rates also cut the margin (share 0.714 vs 0.748: about 9 ms of the 46). Second sample note: the
  shared-instant 600,000 had 0 over 3.0 s (max 2,945) vs 4 in phase_at's 600,000: pooled 4 in 1.2 M, P(none of 480)
  0.998, of 1,440 0.995; the max part's chance is known only to about a factor of 3.
- F11 (item 6, BUG: stale bar in the lineage table): three_part() (fixes.rs:1525-1534) still encodes round 2's bar
  (half within 0.6 s, 1 / 3 over 1.2 s, none over 2.0 s) and multi_burst_rows (:1460-1469) and ws_limit_tail
  (:1715) evaluate only that one; the v3 recommended row fails it with certainty (p50 654 > 600), so every lineage
  "pass" line prints 0.000 (u3v3-tables1.txt:20-21, 25-26, ...; :135, :139) and the chosen bars were never evaluated
  on lineages. My evaluation (an5-out.txt): 48 whole lineages per run, chosen 480 bar 0.9998, chosen 1,440 bar
  0.9966 (60,000 lineage bursts hold 0 over 3.0 s, so the max part is unresolved there).
- F12 (item 7, the pin; fixes.rs:1821-1851): FNV-1a over the 6,000 sorted recoveries' LE bytes plus the counts over
  0.8/1.0/1.2/1.5 s [467, 77, 32, 10] (consistent with the 600,000 rates: 470/68/27/10 expected) and the side counts
  [5,998, 3,999, 946, 1,056]. Integer-only inputs (F4), so exact across platforms; any change to any of the 6,000
  recoveries or side counts fails it, and the failure prints summary(), so a re-pin is informed. Runtime: 3.3 s
  debug, 0.3 s release (4 cores; check.mjs runs debug twice: about 7 s; CI with 2 cores about twice that): fast
  enough. Sound as a regression guard for the recommended Chromium/Linux path. Gaps: it pins no bite row (today,
  fix 1 alone, fix 3 alone), no Firefox or Windows-like row, no lineage; the bars' meaning rests on those rows, so a
  change that moved only fix 3 alone's p50 (the row the median part exists to catch) passes CI unseen. The goal
  assertion of v2 is gone (round 2 B4 fixed). Cheap fix: pin 1,000 phases of each bite row and one lineage.
- F13 (item 8, marks.json and npt-judge): PM-0001-18 (spikes/netcode-proof/marks/marks.json) today has two checks on
  event.burst_heal.after_link_ns (max <= 500,000,000; count >= 480) over parts "L1 B1" (min 3) and "L2 B1" (min 1).
  The judge already supports all three parts with no code change: stat "p50" (nearest rank, eval.rs:260-285;
  marks.rs:419-433), stat "count" with a where {"op": ">", "value": 1300000000} and op "<=" 6 (only count and share
  take a where: marks.rs:466-478; ">" parsed at marks.rs:157), and stat "max" "<=" 3000000000. The judge's semantics
  match the evaluator's classes (p50 <= X <=> at least ceil(n/2) at or below X; count of v > Y; max <= Z). The mark
  needs a WebSocket part: a select with "params.transport": "ws" (used by PM-0001-25/-50) and "parts" on the three
  new checks (PM-0001-3's pattern), so the 500 ms max stays on WebTransport only; plus the count >= 480 check (a
  client that never heals writes no record, and both "at most k over Y" and "max" ignore a missing record). If L1
  also runs a WebSocket B1 part, its recoveries come from E15's stream model (plan.md:419), not the TCP model: the
  bar was not derived for it. Every run that must pass multiplies the false-fail chance (four runs at 0.997: 0.988).
  The event's L2 time uses the client's server-clock estimate (PM-0001-47: p99 error <= 5 ms): another 5 ms of the
  median part's 46 ms margin.
- F14 (items 3/9, the margin windows; an7.py, an7-out.txt): with the count and max parts as chosen, the range of a
  common time scale s (all builds x s) over which the correct build passes >= 0.95 AND fix 3 alone fails (<= 0.05):
  480, median 0.70 s: 0.84-1.05 (centred on s = 0.94: it bets the real system is faster than the model); 0.72 s:
  0.86-1.08; 0.75 s: 0.90-1.13 (about centred; 0.75 s is near the geometric mean of 654 and 844 ms, 743 ms);
  0.78 s: 0.93-1.14. Budget x 2 fails only for s >= 0.92 (0.70 s), >= 0.99 (0.75 s): it cannot be caught robustly
  by any median bar. At 1,440 the correct build's upper end is 1.06 for every median: the 1,440 bar's count part (16
  over 1.2 s) and max part give way first, so the 1,440 bar is fragile under a body shift whatever the median.
- F15 (item 9, too tight / too loose, summary): TOO TIGHT where it matters least to players: the median part (7 %
  margin, +45 ms or one shared tick phase plus +30 ms breaks a correct build), and at 1,440 the whole bar above
  s = 1.06. TOO LOOSE where the search had no data to push against: the max at 3.0 s and k = 6 over 1.3 s are the
  grid's corner (F7); "none over 3.0 s" is 2.9x the recommendation's p99; a reconnect with a trigger twice too slow
  passes 0.98 (480). The bar is calibrated on the model's one burst length (B1's 1 s) and a Linux client: the 1 s
  trigger row beats the recommendation's median (500 vs 654 ms) only because its SYN leaves at B1's return; a
  different burst length reorders the rows. Its bites are gross breaks only. And L2 measures another system
  (round 2 H17 and round 3 V19, u3-verify3-harness.md:187-195: DelayStream above TLS, LG not a browser), so neither
  the 0.997 nor the 0.000s transfer to L2 without an L2 calibration run. Better split: keep the tail parts (robust;
  they catch fix 1 alone, today and a 3 s SYN client), and catch mechanism breaks with mechanism tests that do not
  depend on the model's time scale: unit and property tests of the budget formula and the trigger (catch budget x 2
  and trigger x 2 exactly), and per-burst L2 series (bytes outstanding at the link's return against the adaptive
  limit; the silence at each attempt's start against the trigger). Then a median part is unnecessary; if kept, put
  it near 0.75 s, the midpoint, and state its window (s 0.90-1.13). Without a median part and without mechanism
  checks, fix 3 alone passes "at most 6 over 1.3 s, none over 3.0 s" 0.253 and budget x 2 0.989 (an3-out.txt).

## Verdicts (finished block)
1. joint_dp / joint_closed exact for "median at or below X, at most k over Y, none over Z" under independence:
   CONFIRMED (F1, F5: my multinomial sum and a Monte Carlo of the judge's reading agree to 0.001); the brute-force
   test is correct but small (n <= 7).
2. Wilson intervals: CONFIRMED (formula, 5/100 and 0/100 values; at 4 of 600,000 equal to Garwood). Block bootstrap
   over 600 blocks: a valid unit (no within-block dependence, F9) and 2,000 replicates enough (Monte Carlo spread
   0.0007); but it measures only the model's sampling noise: the "lower end >= 0.95" rule does not guard the risk
   that matters (model error, run dependence).
3. The bar search: arithmetic CONFIRMED; the choice is REFUTED as robust: it sits at the grid's corner in median,
   k and max (F7), and the median part breaks a correct build at +7 % time scale or +45 ms (F6, F14). The tail
   parts survive +/-20 % (CONFIRMED robust).
4. Bite rows: today, fix 1 alone, fix 3 alone failing with 0.000: CONFIRMED and believable (gross breaks, 1e-6 to
   1e-300); REFUTED as a sufficient set: a trigger twice too slow passes 0.98 (480), a budget twice too large is
   caught only inside a narrow time-scale window, and a 3 s SYN client is caught (0.004) (F8).
5. Hashed input phase and independence of the 600 sets: CONFIRMED (F3, F9).
6. Multi-burst lineages and run dependence: tails CONFIRMED near-binomial; the body is strongly dependent (lineage
   dispersion 1.7-3.4; one tick phase per run with 20 s spacing) and lineage bursts 2+ are slower in the body
   (share at or below 0.7 s 0.714 vs 0.748); at the point the pass chance barely moves, under a body shift it falls
   much faster than the independent formula says (F10). The lineage table's pass lines use a stale bar (F11).
7. Pinned test: CONFIRMED a sound, fast regression guard for the recommended path; gaps: no bite row pinned (F12).
8. marks.json: CONFIRMED expressible with today's judge (p50, count-with-where, max); needs a ws part and parts on
   the checks (F13).
9. Too tight in the median (and at 1,440 overall), too loose in the corner-chosen tail parts against subtle breaks;
   model-only calibration (F15).
Numbers in the brief: all CONFIRMED as computed (F0); the 0.997 [0.993-0.999] is CONFIRMED as the model's sampling
interval but rests on 4 phases over 3.0 s (a second 600,000 shows 0: pooled 0.998), and does not include model
error, which dominates.

## Bugs (finished block)
- B1 (F7) fixes.rs:1734-1765: the search's ranking ties every bar that fails all bite rows at 0.000 and then pushes
  each parameter to the grid's edge; effect: the chosen bars ("half within 0.7 s, at most 6 of 480 over 1.3 s, none
  over 3.0 s"; at 1,440 "... 16 over 1.2 s ...") are the grid's corner; a wider grid picks "0.8 s, 8 over 1.2 s,
  none over 5.0 s" with the same 0.000 bites. The reported bars are not a finding about the fallback.
- B2 (F6/F14) the selection rule fixes.rs:1779-1782 uses the bootstrap's lower end (sampling noise only); effect:
  a 0.7 s median 7 % above the model's p50 passes the rule; under +7 % time scale the correct build passes 0.505.
- B3 (F11) fixes.rs:1525-1534 with :1460-1469 and :1715: stale three_part() bar; effect: the lineage table's and the
  tail's three-part lines print 0.000 for the correct build (u3v3-tables1.txt:20, 25, 135, 139), and the chosen bars
  were never run on lineages (mine: 0.9998 / 0.9966).
- B4 (F10) the pass chance assumes independent client-bursts; with 20 s spacing (600 ticks) a run's bursts share one
  tick phase and a lineage's bursts are alike; effect: none at the point (0.9995 vs 0.9998), large under a body shift
  (+40 ms: 0.66 vs 0.92 at 480; 0.63 vs 0.99 at 1,440).
- B5 (F12) minor: the pin covers the recommended row only; effect: a regression in a bite row passes CI.
- B6 (F2) minor: Bar::counts silently tests the median at Y when median > Y; no effect today.

## Must correct before Mustafa decides (finished block)
1. Do not present "half within 0.7 s, at most 6 of 480 over 1.3 s, none over 3.0 s" (and the 1,440 twin) as
   searched-out optimal: they are the search grid's corner (F7). Say what each part is for and its margin.
2. Give the margin with the pass chance: the correct build passes 0.997 only while the real recoveries are within
   -16 %/+5 % of the model's time scale (+45 ms flips it to a coin toss); the model's p50 moved +50 % between v2 and
   v3 (F6, F14). A run's shared tick phase makes it worse (+30 ms: 0.98; +40 ms: 0.66) (F10). The 0.997's interval
   (0.993-0.999) is sampling noise of 4 events, not model error.
3. Offer the robust alternative: tail parts as the product bar (k over Y, none over Z: robust to +/-20 %), and the
   mechanism breaks caught by mechanism checks (unit/property tests of the budget and the trigger; per-burst L2
   series for bytes outstanding at the return and the silence at each attempt's start). If a median part stays, put
   it near 0.75 s (window 0.90-1.13) and name budget x 2 as not caught robustly.
4. Name what the bar does not catch: a trigger twice too slow (passes 0.98 at 480, 0.52 at 1,440); a budget twice too
   large (caught only within a narrow time-scale window) (F8).
5. Re-run the lineage table with the chosen bars (B3) and require burst phases spread uniformly against the tick in
   L2 (spacing not a multiple of 33.3 ms, e.g. 20 s plus a random 0-33 ms) with per-burst over-bar counts and tick
   phase in the record (F10).
6. Say the pass chances are model figures on a Linux client with 1 s bursts and that L2 measures another system
   (H17/V19); an L2 calibration run of the correct build (not judged) should precede freezing any median value.
7. The mark: three checks on a "params.transport": "ws" part plus count >= 480; state how many runs must pass and
   the joint false-fail chance (four runs at 0.997: 0.988) (F13).

Cleanup: diag edits reverted in the worktree (fixes.rs from git show HEAD, diag.rs deleted), worktree .gate/te3s-wt
removed (git worktree list: main folder and the two agent worktrees only). The build output .gate/te3s-target
(582 MB, gitignored) is left: rm -rf was denied; the session may delete it. Scratch kept in scratchpad/te3s/.
Nothing committed or pushed.

DONE
