# U4a + U4b review, round 1 (test-engineer)

Started 2026-11-10. U4a: .claude/worktrees/agent-a9297644e7c0354f3 (base dffef0e); U4b: .gate/u4b-wt (base 4b5812a).
Only plants in place (one at a time, restored, sha256-checked). No commit. Build rules: plan §7.1 (shared target,
offline, touch crates/ before and every 2 s during a counted run, wait for no other cargo/rustc, df before each build).
Run helper: scratchpad te-run.sh (waits 8 s idle, df, touches, toucher every 2 s inside the same command, stops it).

## Plan
1. sha256 of every changed file in both copies (U4b's four given; U4a's against its notes' baseline).
2. Read each plan test and bite (§5 U4a a1-a9, catalogue items 1/2/3/6, carry 4; §5 U4b's six scenarios + 3 bites)
   and check each tests what it claims. For doubtful power: plant in place, show the named test failing, restore,
   sha256. Findings a probe can settle without touching a copy (N1, SEC-2, SEC-5's tally, SEC-6/N6, S1's width) go
   through one scratch probe crate under .gate/te-probe (a path dependency on U4a's copy; nothing written in the copy).
3. Carries U4a 1-5, 34; U4b 6, 35 against code and tests.
4. check.mjs once per copy at the end, fresh touch; record counts vs deltas (U4a 296/321/52, U4b 285/308/48).
5. Final sha256 of both copies = step 1.

## Progress
- Step 1 done (both copies, before any plant). U4a, 18 files, every one = the builder's baseline (first 16 hex):
  bits 4a390bdd0679dd1c, control 390c1fa059c7d336, error ce66d7b58375f789, hash 5a24907f63b6340f, input
  75b167fd93d08bf0, lib a8603264db0cf0c3, limits 08e80997cab3809e, msg 63677d4f00e3e151, quant 0d41e906a41f1c4d,
  snapshot ef9e601309e0a88d, wrap 9f2613c747b35c9c, tests/bites 9d7cb2f874485439, catalogue 14313b7ba5bb1f38, wire
  8675723a2b32b636, common 17bee55e5d501f55, clippy.toml 30ab9b2103d6b4e0, check.mjs 7eddc285abef4580,
  check.test.mjs da63983fc622b546. U4b, 4 files = given: clippy.toml 5a363a03b44ef029, lib.rs 2b62afca56b2c964,
  schedule.rs f9076ff81b453e4d, tests/schedule.rs dfb7da2cbac98c07. Full hashes: scratchpad u4a-sha-before.txt,
  u4b-sha-before.txt.
- Step 2-3 reading done (both copies, every src and test file; check.mjs/check.test.mjs diffs; seam.rs at base;
  catalogue §1-§3). Read the three reviewers' notes (netcode N1-N8/L1-L2, security SEC-1..7, rust F-A1..8/F-B1..3:
  the rust notes are complete, "DONE").
- Process note (honest record): the Edit tool is not available in this run (as the rust-reviewer found). Two progress
  lines above were first appended to this file with a shell heredoc before I noticed; the file is rewritten whole
  with Write from here on, and plants are made by Write of the whole file, restored by Write, sha256-checked.
- 10:20 UTC U4b: my first plant (last_due without "- 1") was NOT applied (the Edit call failed); the run that followed
  was therefore a clean bites-build run: 16 passed, 0 failed (matches u4b-build.md's 16). sha256 of all four U4b
  files re-checked: unchanged. Not counted as a plant.
- 10:23 UTC probe crate .gate/te-probe (Cargo.toml with an empty [workspace], path deps on U4a's proto, sim, link;
  src/main.rs prints measurements; shared target, offline, U4a's crates touched before and every 2 s): log scratchpad
  te-probe-1.log, exit 0. Results:
  - N1 CONFIRMED by run: 49 soldier Adds encode to 1,016 B, `encode` Ok, above Limits::SNAPSHOT (1,000 B, the send
    cap); the largest encode Ok is 55 soldiers, 1,138 B; 56 is TooLarge (the 1,150 B receive limit). The client
    decodes the 1,016 B datagram. msg.rs:219-231 (content_limit) used by encode_into msg.rs:343.
  - SEC-2 CONFIRMED: an Effect Add in `entities` encodes (22 B) and decodes; a Vehicle Add in `own` encodes (30 B)
    and decodes. GROUPS rows listing Soldier: own state (Owner) and remote soldiers (Spatial).
  - S1 MEASURED: the Soldier layout is 126 bits of fields, [16,16,12,16,10,16,16,16,8], three of them grid::VELOCITY
    (48 bits); one soldier Add costs 20 B in `entities`, so every remote soldier carries velocity today.
  - SEC-6/N6 CONFIRMED: Input {ack_tick 5, ack_groups 0} decodes and round-trips, as does {0, 0}: two values that
    both mean "no snapshot decoded" (input.rs:24, :198-199). Semantic, not byte, non-canonicality (a2's re-encode
    rule cannot see it).
  - SEC-5 MEASURED: with the prefix kept (a2's prefix half, wire.rs:180-189), every cut of every stream type and of
    RoomControl is BadLength or Truncated inside the framing: FullState 55 BadLength + 5 Truncated, Join 66 + 3,
    JoinAccepted 77 + 3, SessionControl(RecordStep, 40 B note) 61 + 3, SessionStatus 13 + 3, RoomControl 21 + 2, the
    small ones alike; no cut reaches a body. With the prefix rewritten to the cut, every cut reaches the body and is
    Truncated (and none panics). So a2's prefix half proves only the length check for 12 of 16 types (11 stream types
    + RoomControl); bodies' truncation is reached only by a2's random half (half its buffers carry a matching prefix)
    and its bit flips. By arithmetic the flips likely catch an unchecked `take` (about 17 RecordStep cases per run
    whose flipped length runs past the end), so the gap is in what the prefix half claims, not in a2 as a whole.
- 10:24 UTC U4b plant B-P3 (clock ban live?): server lib.rs + `pub fn te_plant_clock() -> std::time::Instant {
  std::time::Instant::now() }`; `cargo clippy --locked -p npt-server --all-targets -- -D warnings` exit 101: "use of
  a disallowed type `std::time::Instant`" (lib.rs:17:28, :18:5) and "use of a disallowed method
  `std::time::Instant::now`" (lib.rs:18:5), lib and lib test. The record's "the std bans are live now" holds. Restored
  by Write; sha256 of the four U4b files = baseline. (Log te-b-p3.log.)
- 10:25 UTC U4b plant B-P1 (the closed form's exact boundary): schedule.rs:140 `... * tick_hz as u128 - 1) /` ->
  `... * tick_hz as u128) /` (claims tick k due 1 ns early whenever k x 10^9 / hz is an integer: at 30 Hz a wake at
  99,999,999 ns runs tick 3). npt-server recompiled ("Compiling npt-server"), bites build: 16 passed, 0 failed. The
  mutant SURVIVES every test, the 120,000-wake reference comparison included: no test wakes at at(k) - 1 for an exact
  at(k) (the early-wake test uses at(1) - 1, and 10^9/30 and 10^9/60 are not integers). Restored by Write; sha256 =
  baseline. Finding T-B1 (Low). (Log te-b-p1b.log.)
- Not run, by reasoning: B-P2 (the base_ns guard, schedule.rs:156): random_wakes reaches it after a drop whenever the
  next gap is shorter than (base - now), about 0.5% of drop wakes, so a few hundred times per run; F-A6 (hash's never-0
  guard): no input reaches a computed 0 (p about 5e-15 per view), so removing it passes every test (rust F-A6 stands).
- By grep (not planted): no test names an EncodeError of Snapshot or FullState (tests/*.rs: only Input's TooMany,
  NotCanonical and OutOfRange, wire.rs:471-531); the snapshot encoder's refusals are untested (T-A5).
- 10:27-10:30 UTC check.mjs, U4b copy (te-u4b-check.log): all 10 steps PASS; test: measured 285, bites 308, 0 failed,
  0 skips (= 275 + 10, 292 + 16); self 48 of 48; doc both builds; test step 107.1 s. df 12 G free.
- 10:32-10:35 UTC check.mjs, U4a copy, run 1: 10 PASS but bites build counted 304 (+12, not +29; measured 296 held):
  another copy's artifact ran (U4c/U4d lanes' cargo test in flight before and after). NOT REPORTED as evidence.
- 10:38-10:40 UTC check.mjs, U4a copy, run 2 (te-u4a-check2.log; idle 117 s first): all 10 steps PASS; test: measured
  296, bites 321, 0 failed, 0 skips (= 275 + 21, 292 + 29); self 52 of 52; doc "host and wasm32"; test step 123.9 s.
- Step 5: both copies byte-identical to step 1 (U4a 18 of 18 OK, U4b 4 of 4 OK); `git status` file sets unchanged.
  Left outside the copies: .gate/te-probe (probe crate, gitignored); scratchpad scripts and logs.

## Carries
- U4a 1 met (GROUPS and the mapping from layout() for every kind in ALL; sizes from each Field; KIND_BITS from ALL;
  VELOCITY != POSITION_XY: bites.rs:119-121). S1 is a consequence of the given, not a breach of carry 1.
- U4a 2 met (Quant's own per-instance bite; ids only in npt-proto's BITES, check.mjs bite-home grep green; a4 at the
  ends, worst = half the step: quant.rs:372-422; the builder's P17). Reading to state: at a non-wrapping end whose
  nearest multiple is outside the range the error is under one step (the test's `ends < step`).
- U4a 3 met (linkUseProblems, the rename refusal, self-tests; the builder's P10). 4 met (quant.rs:312-334; P4).
  5 met (text); crates/proto/Cargo.toml:13-14's bites comment is still stale (entry 41, the next manifest edit's).
- U4a 34 met (all four, each pinned by check.test.mjs's two new tests; P13-P16).
- U4b 6 met for the type and the 60 Hz tests (rates() from loop_catchup_at; no literal cap in a run test); its
  "server's schedule" assertion (tests/schedule.rs:209-217) reads back its own argument and cannot fail: the binding
  half is U6's and U4r's callers (T-B3). U4b 35 met (only NS_PER_S; nothing derived from TICK_HZ).

## Findings (ranked)
- T-A1 Medium, U4a, before merge (= N1, SEC-7, F-A3): msg.rs:219-231 + :343; limits.rs:23-24; error.rs:184. Shown.
- T-A2 Medium, U4a (S1; with SEC-1, F-A1): snapshot.rs:429-454, :481-483; seam.rs:114-116; the table's test
  snapshot.rs:637-652 cannot see it. Not a U4a merge blocker (condition 3 freezes the seam); gate before U4e's brief.
- T-A3 Low, U4a (SEC-2, F-A2): GROUPS not enforced; shown; folds into T-A2's fix.
- T-A4 Low, U4a (SEC-5): a2's prefix half vacuous for 12 of 16 types; measured. wire.rs:180-201.
- T-A5 Low, U4a: the snapshot encoder's refusals untested (snapshot.rs:457-468, :175-177, :245-254, :267-268, :591).
- T-A6 Low, U4a (SEC-6, N6): input.rs:198-199; shown.
- T-A7 Low, U4a (F-A6): hash.rs:96 never-0 guard untestable as written; wire.rs:643-649 cannot fail.
- T-A8 Low, U4a (F-A7): a8's vectors native only; to U4d2's Wasm-in-Node step.
- T-A9 Low, U4a (N8): catalogue §6 item 3 note and the Input row's sizes, in U4a's change-set per plan §5 U4a; the
  catalogue tests re-run on the changed doc.
- T-B1 Low, U4b: schedule.rs:139-141 boundary; mutant survives (B-P1).
- T-B2 Low, U4b: tests/schedule.rs:283 purity assertion cannot fail; the property is held by the type and the clock
  ban (B-P3 shows the std half live).
- T-B3 Low, U4b: carry 6's read-back test is tautological (tests/schedule.rs:209-217).
- T-B4 Low, U4b (F-B1): the tokio entries (clippy.toml:7, :12) cannot be shown live until tokio resolves; plant then.

## Verdicts
- U4a: REVISE, for T-A1 alone (a public encode contract U4e's splitter builds on, and a3's encoder half cannot see
  it); the rest follow-ups, T-A2 gating U4e's brief.
- U4b: APPROVE WITH FINDINGS (four Lows, follow-ups).
DONE 10:42 UTC.
