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
