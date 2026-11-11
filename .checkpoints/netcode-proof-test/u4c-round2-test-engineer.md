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
scratch plant script written with Write (u4c-round2-test/r2-plant.py, loading round 1's u4c-test-plant.py by path):
the method plan §7.2 names ("the bites planted by a scratch script under §7.1") and round 1 used. Exact-match edits with
occurrence counts, in place, one at a time, original bytes restored, sha256 checked before and after each; backups in
the session scratchpad (u4c-r2-backup-*). Notes rewritten whole with Write.

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
