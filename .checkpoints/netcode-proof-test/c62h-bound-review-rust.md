# C62H-BOUND round 1: rust-reviewer notes (effort xhigh, read only)

Diff: LANE_BASE 044fab188419e8859dbca699d851563c44c078c9 .. 42b964618eed741f65ac04c1e5a06da0a82524d0 (4 commits),
worktree /home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound (branch c62h-bound-build, clean).
(No Edit tool in this agent: the notes file is rewritten whole with Write each time.)

## Plan
1. Read brief sec 0-5, 8.4, 10-11 (done), texts v3, freeze, design, rulings, step returns, build notes.
2. Read the diff per crate: l1 (lib.rs, tests.rs), lab (scenario.rs, logs.rs), judge (figures.rs, engine.rs,
   fixtures.rs).
3. Checklist per brief sec 5: HashMap order, float, overflow (u32 sums, u64 totals), docs on pub items, clippy logs
   (step1-check.log, step2-check-2.log, step3-check.log, step4-check.log), bites cfg, call-site Err handling
   (TE-CBB-7 (d)).
4. Rule on builder points (a) to (d).
5. Findings with file:line and smallest fix; verdict.

## Checklist (all read in the worktree at 42b9646)
- Numstat: 967 changed lines, 14 files, no Cargo.toml / Cargo.lock change (so deps, audit, deny unchanged).
- Clippy: check.mjs:2014-2019 lintBothBuilds = `cargo clippy --locked --workspace --all-targets -- -D warnings`, then
  the same with --all-features (check.mjs:2618); clippy-wasm32 on LEAF, npt-l1 included (check.mjs:39, :1970-1971).
  step1-check.log:6-7, step2-check-2.log:6-7, step3-check.log:6-7, step4-check.log:6-7 all PASS "measured build and
  bites build both linted"; doc PASS with rustdoc warnings as errors, both builds, host and wasm32; fmt PASS. HOLDS.
- HashMap / float: none in the added lines (grep of every + line). BTreeMap/BTreeSet only; TickRecord's s/hs are
  BTreeMap (log/src/lib.rs:108-121). Workspace: iter_over_hash_type deny, disallowed_types deny (Cargo.toml:103-107);
  lab's clippy.toml bans HashMap/HashSet. HOLDS.
- Overflow: release overflow-checks only on npt-sim and npt-proto (Cargo.toml:116-120), so l1/lab/judge wrap in
  release. l1 counters u32 over one send's new entries (lib.rs:417-431); over a whole log a wrap needs > 4e9 entries.
  lab `c.lost + c.burst` (scenario.rs:1058, :1065; link_lost :630) <= c.sent == groups after the join's check, since
  Lost and Burst are disjoint arms (lib.rs:424-428). Totals u64 (scenario.rs:803-805, :1056-1060), Counts u64
  (logs.rs:185-187). Judge counts i128 (figures.rs:679-683, :691-695). No reachable overflow. HOLDS.
- Bites: `overstated` and the overriding `let lost` are #[cfg(feature = "bites")] (scenario.rs:973-975, :1061-1072);
  LAB_BITES and Bites::of cfg'd (:214-219, :226); the field link_lost_overstated is plain data in both builds, as
  whole_as_lost is (Bites has allow(dead_code) in the measured build, :180-183). Bite tests cfg'd (:2952, :2968). HOLDS.
- TE-CBB-7 (d): scenario.rs:1051-1055 `link_lost(..).map_err(|e| fail(format!(..)))?`; fail = Failure::Failed
  (:862); main.rs:242-245 prints it and exits 1; Logs is dropped without finish (no Drop impl in log/ or lab/ except
  the test type Scratch), so no footers. No unwrap_or, no default. HOLDS.
- Docs on pub items: DatagramCounts + 3 fields (lib.rs:329-339) and server_datagrams_since (:412-416) documented;
  nothing else pub added. HOLDS. (pub(crate) group docs: RR-CB-2.)
- Panics from content data: judge `bound` indexes only constants (gaps[i], LINK_GAPS[i]); lab indexes by slot < n.
  None reachable from a log or a run.
- Determinism: lab per-slot loop in slot order (scenario.rs:1023); fate log a Vec; judge walks logs in Vec order and
  ticks strictly increasing per log (read_log enforces it: log/src/lib.rs:443, :560). No target-dependent code added
  (npt-l1 is compiled for wasm32 too; integers only). No simulated behaviour change: the reads are &self /
  RefCell::borrow (lib.rs:1918-1920) and Server::session is a find (scenario.rs:565-570).
- Wasm size: no shipped Wasm artifact changes by construction (deps step: npt-hash-wasm ships npt-sim only; the lane
  touches no npt-sim file). Bytes not measured (no build in this read).
- Speed: per slot per tick, two RefCell borrows and a scan of only the entries the send pushed (O(new entries)); three
  48-entry allocations per room tick (scenario.rs:1022, logs.rs:642-648), like the loop's existing sizes/sets vecs.
  Not measured by me (no cargo). Baseline for the merge step's timed `npt-lab hash --manifest
  crates/lab/precheck-recordings.json` (a runner recording re-run through scenario::run, so it exercises the join),
  same pin 1f00d2d5: real 113.1 s (.gate/u4r/brawl1/final-regen-tests.log:23), 121.1 s (final-fp-regen-tests.log:24),
  123.8 s (.gate/brawl1-merge/m3.log:57). check.mjs's wasm-step "npt-lab hash" seconds are NOT evidence: its manifest
  (crates/lab/recordings.json) is synthetic (hash.rs:396-397), so it never runs the join.

## Point rulings
- (a) logs.rs:520-523 allow(clippy::too_many_arguments, reason): ACCEPT. room_tick now has 8 inputs; the allow has a
  reason and copies the only precedent (mirror.rs:353-356); a struct to bundle the inputs would cost about 15 lines
  and touch the two test call sites. `#[expect]` (fires if the lint stops applying) would be tighter, but nothing in
  the workspace uses it; not worth a change.
- (b) figures.rs:546, :622, :645 BTreeSet<(usize, u64)>: ACCEPT. Ordered, so deterministic; counts each (slot, tick)
  pair once, as the frozen row words it ("the (slot, tick) pairs"); a duplicate is only possible across two client
  logs (read_log refuses non-increasing ticks within one: log/src/lib.rs:560), never on a valid L1 run; `whole` stays
  a Vec and cannot duplicate (slot_hashes refuses two logs at one (slot, tick): figures.rs:268-273). One side effect:
  RR-CB-1 (gap 7's location order).
- (c) engine.rs:1567-1588 BoundTick with -1 as null (`u64::try_from(h).ok()`), and engine.rs:1653
  `[fn(&mut [Log]); 8]`: ACCEPT. Test-only, documented in the type's doc; the fn-pointer array is the plain fix for
  clippy's type_complexity on `[&dyn Fn ..; 8]` (non-capturing closures coerce). `drop(..)` as a discard is fine
  (a `{ ..; }` body would do the same).
- (d) DatagramCounts and the read-only claim (lib.rs:329-339, :412-431): ACCEPT, claim HOLDS. &self over a plain
  Vec<Fate> (lib.rs:306-327); the kind is decided at the push (lib.rs:1130-1138) and the only later write to an entry
  (lib.rs:1174-1176) changes a Delivered arrival and the overtaken flag, never the kind; not-up and too-large return
  before the push (lib.rs:1117-1122), as the doc says; the only Datagram push is datagram() from send_datagram
  (lib.rs:1103, :1140-1143), so enter()'s event processing during the send adds only Ack/Stream entries, which the
  filter drops. Not filtered by link: correct today for that reason, and any extra entry fails the run through the
  join's equality check (scenario.rs:621-626).

## Findings
- RR-CB-1 (Low) figures.rs:651 vs :578: gap 7's location. `lost` is ordered (slot, tick), so `find` names the lowest
  slot's first unmatched tick, not the earliest tick, while bound's doc says "at its first tick and slot". Only the
  note's location prefix differs; the phrase, the gap and both figures are unchanged. Fix: 1 line (reword the doc for
  gap 7, or `.filter(..).min_by_key(|&&(s, t)| (t, s))`). No frozen text.
- RR-CB-2 (Low) docs: scenario.rs:800-805 and logs.rs:181-187 put one doc on the first of three fields (rustdoc shows
  it under link_sent / lost_room_sent_nothing only); scenario.rs:198-200 reads "and the link lost none written
  `tick.s.snap_link_lost_count` 1" (a comma or "is" missing). Fix: about 5 lines. pub(crate) only, so no lint.

## Verdict
APPROVE WITH FINDINGS (two Lows; neither triggers a push; 0 required lines, about 6 if folded into a fix pass).
Done: all blocks complete; nothing in flight.
