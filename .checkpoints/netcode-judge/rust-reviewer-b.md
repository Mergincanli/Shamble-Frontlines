# rust-reviewer, part b (U6j), one round on 0e0f6547..a3476e6c (crates/judge)

## Plan
1. Read plan-b.md (Block 2's design, section 5) and rust-reviewer-a.md (earlier findings).
2. Read the diff file by file: ratio.rs, lib.rs, main.rs, practice.rs, eval.rs, figures.rs, then tests (fixtures.rs, cli.rs, engine.rs).
3. Per file: overflow / index / cast / determinism / cost / API notes, a line here when done.
4. Run cargo test / clippy / fmt (from spikes/netcode-proof/), and small scratch probes under .gate/ if needed.
5. Findings with severity, file:line, proposed change; verdict.

## Progress
- Cargo.toml / Cargo.lock: unchanged in the diff (git diff --name-only shows none). No new dependency.
- ratio.rs DONE: cmp_wide exact (floor/rem_euclid with den>0, reciprocal step reverses), terminates (Euclid on dens, <=~185 steps), Equal only on equal value; reduced + den>0 so consistent with derived Eq; Ord total, no panic (div_euclid by positive never overflows). OK.
- lib.rs/main.rs/practice.rs DONE: summary_line escapes is_control chars (C0, DEL, C1) OK; main passes whole file to judge_only OK.
- eval.rs DONE (diff): Missing::Fail plumbed in all 3 matches; first pass in judge_mark; `measures.chain(&mut reports)` odd but fine (Low idiom).
- figures.rs coverage/sessions read: range(from..to) never panics (starts strictly increasing; MAX..MAX is Included..Excluded = empty, no panic). Edge: last session `..u64::MAX` excludes tick u64::MAX (old code included it) -> Low. (c)'s `find` per (tick, slot) linear in sessions -> fine for real counts, quadratic on a log with thousands of admits per slot (Low).
- FINDING H1 (probe .checkpoints/netcode-judge/rr-b-sync-probe.rs, uses ratio.rs unchanged, built to .gate/rr-b/sync-probe): sync_cost_ratio figures.rs:1282-1293 panics "a fraction overflowed i128" with 4 valid runs in N=24 or N=36 at 18,000 or 54,000 ticks (3 runs: num/den ~1e34, fits; 4: Mul own*den overflows). parts = every valid run of the part across the whole folder (eval.rs:1716 serving = all runs serving the mark), not this set -> a 4th run anywhere in the folder crashes the judge.
- Profiles: judge runs via `cargo run` (dev, overflow-checks on): plain i128/u64 overflow on malformed logs = panic, not wrap; a --release build of npt-judge wraps (Cargo.toml:135-139 pins overflow-checks only for npt-sim, npt-proto).
- RAN: cargo fmt --check exit 0; cargo test -p npt-judge --locked --offline --no-fail-fast (target .gate/rr-b/target): 18+0+4+75+25 = 122 passed; clippy --all-targets -D warnings exit 0.
- figures.rs rest DONE: R2Counts dens stay <= ~2e11 at a room-hour (fits); counter_growth/crash_close slice patterns, no index panic; r2_egress_detail end-start is u64 guarded; usize->i128 casts lossless; no HashMap/float decision. crash_close (0..clients).collect() allocates by header.clients (u32): malformed header -> up to 32 GB (Low). session_memory_growth_count then_some((mid, mid+1)) eager: mid+1 overflows at a lone second u64::MAX (debug panic, Low). rss slope x*x overflows i128 if sec spans > ~1.3e19 (malformed, Low under dev profile).
- tests DONE (engine.rs diff whole, fixtures.rs, cli.rs): exact-text asserts mostly; prefixes at engine.rs sync large-sums (`starts_with("Pass 0.5")`, k=3 only), logging-off note `starts_with`, cli.rs `starts_with("PM-0001-4    PASS")`. No flaky input (no clock, no HashMap, fixed data).

## Findings (ranked)
H1 High figures.rs:1282-1293 (sync_cost_ratio). The exact mean of k unit costs needs the lcm of k denominators (each ~ the run's
   sum of relevant entities, 1e7-1e8, near coprime): Ratio's checked Add/Mul panic "a fraction overflowed i128" at k >= 4
   valid runs in N=24 or N=36 (probe: 18,000 and 54,000 ticks: k=3 ok at num/den ~1e33-1e34, k=4 PANIC; 3,600 ticks: k=5).
   `parts` is every valid run of the part in the whole folder (eval.rs:1716 `serving` = all runs serving the mark, any
   set; eval.rs:1791), so a 4th N=24 run anywhere (a rerun, a second set) aborts the judge before any verdict prints.
   The large-sums test (engine.rs, u6j_the_sync_unit_costs_gaps_and_its_large_sums) sits exactly on k=3 and asserts a prefix.
   Change: (a) never panic here: add `Ratio::checked_add` / `checked_mul` (Option, same body with `?`), use them in
   sync_cost_ratio, overflow -> Lack::Gap naming the part and run count; (b) a reading for the advisor that keeps it exact
   and bounded: the reference runs of this run's set only and exactly `min` of them (more = a gap, as logging_cost_ratio's
   "4 off runs: which three?"), or the pooled unit cost sum(ns)/sum(entities) over the part (one fraction, den ~1e8 x k);
   (c) a k=4 and a k=6 test with soak-size sums, exact expected text.
L1 Low figures.rs:1218-1221 `then_some((mid, mid + 1))` is eager: a lone measured second numbered u64::MAX panics (debug) on
   `mid + 1`. Change: `(last > first).then(|| (mid, mid + 1))`.
L2 Low figures.rs:1153-1158 rss slope: `x * x`, `n * sxy` are plain i128; fine at real sizes (2 h x 1 GB ~1e23) but overflow
   when measured `sec` spans > ~1.3e19 (malformed): debug panic, release wrap (npt-judge has no release overflow-checks,
   Cargo.toml:135-139). Change: checked_* with a Gap, or a gap when `last - first` > 10^9 s.
L3 Low figures.rs:1518 crash_close_max_ns `(0..clients).filter(..).collect()` sized by header.clients (u32, no rule bounds
   it): a malformed header asks up to 32 GB. Change: `let left = clients - closes.len() as u64;` and
   `(0..clients).find(|s| !closes.contains_key(s))` (stops within closes.len()+1 steps, no Vec).
L4 Low figures.rs:589-591 sessions(): the last session ends at `..u64::MAX` exclusive, so a hash at tick u64::MAX is in no
   session and escapes (c) and (d); the old span included it. Change: `to: Option<u64>` and
   `r.range((Included(from), to.map_or(Unbounded, Excluded)))` (same in (d)'s `ticks.range`).
L5 Low figures.rs:654-656 coverage (c) `v.iter().find(..)` per (tick, slot): linear in a slot's sessions (fine: 54,000 x 48 x
   a few); quadratic on a log with thousands of admits for one slot. Change: `partition_point(|x| x.from <= *t)`.
L6 Low figures.rs:964-971 only_from: the `_ =>` arm reads any other kind as tick.s (a typo'd kind passes silently); each call
   scans every non-room log whole (~2.6M lookups on an L2 run with 48 client logs x 54,000 ticks), 1-3 per figure, per run,
   per check, again for reports: fine for a CLI. Change: an enum Kind { SecondV, SecondS, TickV, TickS, Event }.
L7 Low eval.rs:1022-1027 (Rule::Pre) and eval.rs:1262-1267 (setup): a Missing::Fail prints "= ?, needs" and drops its reason;
   fail_lines got the new form, these did not. Unreachable today (crash_close_max_ns is neither a pre nor a setup check).
L8 Low figures.rs:1404-1411 r2_egress_detail says "not timed (no ns on both)" also for 3 reads or a backwards pair.
L9 Low figures.rs:28-36, 1664, 1676: Inputs (pub fields), Parts, Detail, Figure.detail are public API via `pub mod figures`;
   only eval.rs uses them (pub(crate), with Compute/Figure/FIGURES so private_interfaces stays quiet). Optional.
L10 Low eval.rs:1765-1769 `let mut reports` + `chain(&mut reports)`: plain `chain(mark.report.iter().map(..))`.
L11 Low tests: the prefix asserts above where the exact text is cheap.
Info: ratio.rs cmp_wide correct (see progress); no HashMap, no float decision, all usize->i128 casts lossless; kill_p99_ratio
   turns another run's disagreeing params.killed into this run's gap (run_param `?`, figures.rs:1632): conservative.

## Verdict: REVISE (H1 only; the Lows are optional or follow-ups)
