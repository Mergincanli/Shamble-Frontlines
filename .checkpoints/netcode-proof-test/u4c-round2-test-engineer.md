# U4c review round 2 (test-engineer), on what changed only

Started 2026-11-11. Pre-fix: /home/user/Shamble-Frontlines/.gate/u4c-prefix. Fixed copy:
/home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (uncommitted, base 097f897). Round 1 notes:
u4c-review-test.md (mine: M1, M2, L1-L4, N1, O1, O2), u4c-review-netcode.md (F1-F7), u4c-review-rust.md. Rust round 2:
u4c-round2-rust-reviewer.md. Fixer's section: worktree .checkpoints/netcode-proof-test/u4c-build.md:163-346.
Rules (plan §7.1): `cd <copy> || exit 1` first line of every command; CARGO_TARGET_DIR=.gate/shared-target,
CARGO_NET_OFFLINE=true; wait for no cargo/rustc before each counted run; touch crates/ before and every 2 s during it
(loop inside the same command); df before each build (stop below 2 GB); files written with Write/Edit only; plants in
place one at a time, restored, sha256-checked; git only inside the copy, no -C/-c; commit nothing. Scratch:
u4c-round2-test/ (runner script, logs, hash lists).
DEVIATION (stated): this agent has no Edit tool (the call fails "No such tool available: Edit"), so plants cannot go
through Edit, and rewriting lib.rs (68 KB) whole with Write twice per plant is not a sound method. Plants go through a
scratch plant script written with Write (u4c-round2-test/r2-plant.py, loading round 1's u4c-test-plant.py by path;
r2-plant2.py adds one plant): the method plan §7.2 names ("the bites planted by a scratch script under §7.1") and round
1 used. Exact-match edits with occurrence counts, in place, one at a time, original bytes restored, sha256 checked
before and after each; backups in the session scratchpad (u4c-r2-backup-*). The script waits for quiet BEFORE planting
and stops unplanted if none comes in 240 s. Notes rewritten whole with Write.

## Plan
1. sha256 of every file under spikes/netcode-proof in the copy (start state); the three files vs the fix notes and the
   pre-fix copy.
2. diff -ru crates/l1 (pre-fix vs fixed); read every hunk.
3. Per claimed fix, judge closed or not: M1 (drive/woken, L-c5 event mode, host test), M2 (packet threshold), L1
   (time threshold), L2 (acks lost only in bursts), L3 (lc6 link 3 first), L4 (burst comment + no-PTO assertion), N1
   (fails()), F1 doc line.
4. check.mjs once, counted (quiet wait, touch loop): record counts (expected 289 / 316).
5. For each new or changed test, plant the defect it must catch, in place, one at a time, restore, sha256: round 1's
   exact gap plants (the closure proof) and new plants of mine (not the fixer's). Show each failing by name.
6. Defects added by the fix (vacuity, timing dependence, wrong oracles, docs).
7. Final sha256 = step 1; findings; verdict.

## Progress
- 09:57Z Step 1: sha256 of all 158 spike files (excluding target/) -> u4c-round2-test/sha-before.txt. Fixed copy: lib.rs
  d6e6e130, tests.rs e8014c67, tests/l1.rs 9c23f9cc (= fix notes :327); pre-fix copy 1647e75d / c21a362f / 639ab9c5
  (= round 1). git status = round 1's 3 files + fixer notes/logs; HEAD 097f897. df 12 GB. Fixer's r2-check-3 (21:04Z
  yesterday): log 15 B, no run2.sh/check.mjs of this copy running: stale, not running. Foreign lanes busy at 09:58Z
  (agent-a3b8 check.mjs + cargo test --workspace; e0d-1 cargo test -p npt-sim), all in the shared target.
- 09:58Z Step 2: diff -ru -> u4c-round2-test/diff.txt (771 lines), read whole. lib.rs: doc lines :54-57 only (F1).
  tests.rs: Rig.at; drive(); woken(); Lc5 struct; L-c5 event mode on drive with sends_at; stamp/stamped/fixed;
  host_at_next_event (M1); packet_threshold (M2); time_threshold (L1); lc6 links 2/3 fixed(0), link 3 first, acks sent
  at 70 ms (L3); fails(r, lc, check) (N1). tests/l1.rs: burst comment reworded + no stream resend in [2.2 s, 3 s) (L4);
  acknowledgements_are_lost_only_inside_bursts (L2).
- 10:10Z Read checks: Profile jitter_ns is the round trip's ± (lib.rs:149-150, :156; delay ± half): the time-threshold
  test's doc "±20 ms of jitter (50 ± 10 ms each way)" (tests.rs:1242-1243) is wrong for jitter_ns 40 ms (= ±40 RT,
  50 ± 20 each way): rust round 2's Low A confirmed, doc only. Plant anchors counted (read-only): all 15 occur once.
- Plant list (r2-plant.py): round 1's gap plants next-event-none, next-event-no-timers, next-event-no-ready,
  no-packet-threshold, packet-threshold-2, time-threshold-srtt-only, acks-lost-at-profile-rate, event-order-link-first,
  pto-from-oldest-unacked; new: m1-gate-blind, m1-late-1ns, m2-off-by-one, l1-latest-only, l1-time-5-4,
  l2-acks-never-lost, l4-send-no-restart; control clean.
- 10:02:46-10:07:34Z Step 4, check.mjs COUNTED (r2-plant.py check; 20 s quiet reached after 131 s; touch before and
  every 2 s; df 12 GB; exit 0): all 10 PASS. fmt 0.4 s; clippy 7.7; clippy-wasm32 0.7; doc 23.9; test 118.4 s
  measured 289 passed 0 failed 0 skip, bites 316 passed 0 failed 0 skip (= 285 + 4, 312 + 4: the deltas the fix notes
  expect, :332-334); deps 0.5; refs 0.4 (562); marks 2.5 (81, drafts 626df678); self 1.3 (48/48); redact 0.8 (10/10,
  158 items, 0 findings). Foreign: 3 samples from 10:07:29Z, wf_f54246ff-e0d-1's `cargo check -p npt-sim` (host, then
  wasm32), at the test step's end / the last steps (the steps' durations sum to 10:07:28Z for the test step's end); a
  cargo check of npt-sim writes no npt-l1 test binary, and the counts are the expected deltas: counted. Logs:
  u4c-round2-test/r2-check.log, r2-check.foreign.log. sha256 -c after: ALL 158 MATCH.
- 10:07:51Z Plant run "clean": STOP before planting (no 10 s quiet in 240 s: e0d-1 `cargo test --workspace`); sha256 -c
  ALL MATCH. Retried.
- Step 5, plants (every run: 10 s quiet first, df 11 GB, marker lines present, "Compiling npt-l1 (this copy's path)",
  no foreign cargo/rustc during it, restored: lib.rs d6e6e130, tests.rs e8014c67, l1.rs 9c23f9cc after each). Logs:
  u4c-round2-test/<name>.log. Measured build unless said.
  - 10:12Z clean (marker only): lib 16/16, tests/l1.rs 16/16 pass.
  - M1, round 1's gaps (round 1: next-event-none passed L-c5; -no-timers and -no-ready passed all 28):
    next-event-none: FAILS tests::a_host_called_only_at_next_event_ns_... ("link 0, end 1: 31 of 33 items handed out
    when the host stopped at 3001.000 ms"), tests::lc5_... ("L-c5: link 0: an item that arrived at 51.757190 ms was
    handed out at 67.000000 ms"), next_event_ns_names_... (tests/l1.rs). next-event-no-timers: FAILS host test ("32
    of 33 ... 3101.000 ms") and lc5 ("never called at 23 instants with an internal event, the first 112.781568 ms").
    next-event-no-ready: FAILS host test ("link 1: an item that arrived at 20.000000 ms was handed out at 30.000000
    ms, not in its instant, 20.000001 ms").
  - M1, new plants: m1-gate-blind (ready instant = arrival, no gate + 1): FAILS host test only ("it named an instant at
    or before the call that asked it, 2 times"). m1-late-1ns (every instant 1 ns late): FAILS host test ("link 0: an
    item that arrived at 51.000000 ms was handed out at 51.000001 ms"), lc5 ("... 51.757190 ms was handed out at
    51.757191 ms"), next_event_ns_names_... ("nothing before it").
  - M2: no-packet-threshold (1,000,000): FAILS tests::the_packet_threshold_... only ("declared at [1112.5] ms (by count
    [false])"); packet-threshold-2: FAILS it only ("[1102.0] ms (by count [true])"); m2-off-by-one (`>` for `>=`):
    FAILS it only ("[1104.0] ms (by count [true])"). Round 1: the first two passed all 28.
  - L1: time-threshold-srtt-only: FAILS tests::the_time_threshold_... only ("trial 1: declared lost Some(110.514853) ms
    after its send, not 9/8 x max(98.235, 100.619) = 113.196 ms"). l1-latest-only: FAILS it ("trial 0: ... 88.930971
    ms ..., not 9/8 x max(96.363, 79.050) = 108.408 ms": a step-down trial), also lc1 and the estimator test.
    l1-time-5-4: FAILS it ("trial 0: 120.453577 ms ... 108.408 ms"), also the host test (its path check: "time
    threshold false"), lc7, the estimator test.
  - L2: acks-lost-at-profile-rate: FAILS acknowledgements_are_lost_only_inside_bursts only ("361 acknowledgements with
    the wrong fate, the first (kind, sent ns) Some((2, 69879691))"; round 1 passed all 28). l2-acks-never-lost (`let
    lost = false`): FAILS it ("222 acknowledgements with the wrong fate, the first Some((0, 1000241224))") and lc3.
  - L3: event-order-link-first: FAILS tests::lc6_... only ("the two arrivals at 70 ms were not acknowledged in
    fate-log order, link 3's first (links 2 and 3 at Some(6), Some(7))"; round 1 passed all 28).
  - L4: pto-from-oldest-unacked: FAILS streams_lose_nothing_... at tests/l1.rs:417 (the new assertion: "2 stream
    packets resent inside the burst after 2.2 s, the first at 2415856348 ns: a PTO fired") and lc1. l4-send-no-restart
    (a reliable send no longer re-arms the timer): FAILS it at :417 ("2 stream packets ... the first at 2396452694
    ns"), also host test (path check "PTOs []"), lc2, lc3, lc6, lc9.
  - 10:21Z N1, bites build, n1-acks-never-lost-bites (r2-plant2.py; `let lost = false` in send_ack under the bites):
    tests::bites::bite_no_pto_doubling_fails_lc3 FAILS at tests.rs:1657 ("failed on another check than L-c3's \"doubles
    at each expiry\": L-c3: link 0: no probe timeout fired") while bite_acks_in_bursts_fails_lc3 passes (its own
    check) and the other 6 bite tests pass; also lc3, the two-thread test and the L2 test fail (expected). Lib 23/26,
    tests/l1.rs 20/21.
- 10:21:46Z Step 7: sha256 -c of all 158 spike files vs sha-before.txt: ALL MATCH (byte-identical to the start; only
  mtimes moved by the touch loop). git status = the start's (lib.rs M, tests/l1.rs M, src/tests.rs ??, fixer notes);
  HEAD 097f897; no target/ in the copy. Nothing committed.

## Judgement
- Closed (each by round 1's own gap plant now failing the new test by name, plus a new plant of mine): M1 (and rust
  round-1 M, the same gap), M2, L1, L2, L3, L4 (comment reworded to claim no packet-threshold path; the PTO claim now
  asserted), N1. F1: the doc line (lib.rs:54-57) is accurate; the Join-deadline question it raises stays netcode's /
  the main session's ruling, not closed by a doc line, not reopened here. Not claimed, stay follow-ups: L1-L4's
  siblings in rust/netcode round 1 (F2-F7, rust Lows), O1 (npt-net flake, unnamed), O2 (check.mjs tail hides names).
- Tests that cannot fail: none found among the new ones (each failed under at least two plants). Timing dependence:
  none (no wall clock; fixed seeds; the path checks in the host test and the trial counts in the time-threshold test
  make a silent pass on an unexercised path fail instead).
- New, of the fix: R2-1 Low (= rust Low A): tests.rs:1242-1243 doc says "±20 ms of jitter (50 ± 10 ms each way)"; the
  profile (tests.rs jitter_ns 40 ms) is ±40 ms round trip, 50 ± 20 ms each way (lib.rs:149-150, :156). Doc only.
  R2-2 Low (= rust Low B): drive() (tests.rs:873-877) steps 1 ns per call while next_event_ns names a past instant, up
  to the horizon (5e9 / 1e10 steps): a model stuck on a past instant hangs the host test and L-c5 (check.mjs has no
  step timeout) instead of failing on the stuck check. Fix: break after a small stuck count (e.g. 1,000) and return.
  R2-3 Low (process): the fix notes end (:346) on "a third run follows", and the fixer's report says its watch is armed
  on run 3's log; r2-check-3.log is 15 B, unchanged since 2026-11-10 21:04:46Z, with no process running: that run
  never ran. This round's counted run (10:04:57-10:07:34Z, 289 / 316, test step free of foreign cargo) stands in its
  place; the report should say so rather than wait for that watch.
- Verdict: APPROVE WITH FINDINGS (all claimed fixes closed; 3 Lows, follow-ups; no Critical/High, so no round 3).
DONE
