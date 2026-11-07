# u3-verify3-stats: third-round skeptic, statistics in fixes.rs (test-engineer)

Branch claude/netcode-proof-test-plan-hcky5g, head 1e020a7 + checkpoint commits (ee9ad3e; spikes/ unchanged since
1e020a7). Read-only on tracked files. Worktree for diagnostics: .gate/te3s-wt (detached at ee9ad3e); target
.gate/te3s-target; scratch scripts and dumps in scratchpad/te3s/ (lib3.py, an1.py, an2.py, an2-out.txt, sets-*.txt).

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
  0.993-0.999 is a little narrow on the low side, see F7). At 1,440: 0.976-0.997 from the max part alone.
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
