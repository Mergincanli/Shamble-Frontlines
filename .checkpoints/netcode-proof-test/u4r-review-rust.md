# U4r review, rust-reviewer seat (batch 2), round 1

Snapshot: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (verified by sha256sum at start).
Packet: u4r-packet.md cbd82485..., tests file d12555c8... (verified).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch).

## Plan
1. Read the panel section 5 + "What every seat is told" (done).
2. Read the packet and tests file; read the patch whole.
3. Per crate: lab/src (scenario, logs, mirror, precheck, replay, contract, hash, lib, main), bots (lib, tests, clippy.toml), server (pump, schedule, tests/schedule), log/src/lib.rs.
   Checks: panics on input, overflow, determinism (HashMap order, cfg(bites) gating), memory on large runs (no-log path, judge reading logs whole).
4. npt-server edits: site in pump, clippy allow on site alone, field rule.
5. npt-log edits: dup-key and control-char refusals, cost, nothing else changed.
6. Own earlier findings: carry 82 (RR-L5), 83 (d) (RR2-L3), 85 (RR2-L1, RR2-L2, RR2-L4 (a)), 3.3 carries 6 and 14 (rust 1, rust 3).
7. scratch helper scenario::tests::scratch.
8. Runs only if a finding needs one.
   Note: u4r-run.sh accepts only a worktree under .claude/worktrees/ and a log under .gate/u4r/, so neither the read
   copy nor a .gate/ worktree nor .gate/u4r-review-rust/ logs can go through it; free disk 4.7 GB at start. So: no
   cargo run; findings by reading, labelled so.

## Status: COMPLETE (all blocks done). Verdict: REQUEST CHANGES (F1 before merge; the rest Lows).

## Findings

- F1 (Medium) scenario.rs:588-594 `record()` builds argv by format! + split_whitespace; the runner entry's
  `profile` (hash.rs:273-282, any JSON string) is not checked, so whitespace in it injects scenario flags: `--lead 3`,
  `--every-step 1`, `--behaviour steady`, `--bytes <dir>` or `--record <file>` (file writes at a manifest-chosen
  path, carry 82's class), `--role twin --bite <id>` in a bites build. parse's refusal of --lead/--every-step with a
  recording (:1377-1384) keys on `record.is_some()`, which record() never sets (record_mem is set after parse,
  :596-597), so ruling 1 point 9 (a)'s refusal does not hold on `npt-lab hash`'s regeneration path (det-u4.mjs).
  c-u4r-runs.mjs passes argv as an array, so it is not affected. Fix: refuse a profile other than P0..P4|B1 in
  manifest(); build record()'s argv as a Vec<String> of separate items; apply the :1377 refusal when record_mem too.
  By reading; not run.
- F2 (Low) hash.rs:271-292, :312-317: for Kind::Runner, `ticks` does not drive generation; warmup_s/measured_s do,
  unbounded and not checked against ticks before generating (parse `* hz` unchecked, scenario.rs:1322-1324), so
  carry 82's "bound before generating" does not hold for runner entries (a typo measured_s 2900 runs 10x before
  check() fails). Also `bytes` itself is unbounded, so `ticks <= bytes/12` bounds nothing when bytes is large (my own
  RR-L5 fix's limit). Fix: for a runner entry require (warmup_s + measured_s) * TICK_HZ == ticks by checked ops and
  span_ms == ROOM_HISTORY_MS; cap bytes (e.g. 64 MiB).
- F3 (Low) the no-log path: (a) logs.rs `filed()` inserts a Vec<Res> per tick (:285-288, :325) and only
  write_client_tick removes it, which flush never reaches without room_tick (phases empty, :437-441): ~7 MB per
  9,000-tick regeneration at 48 clients, bounded. (b) events_stamped is counted only inside `if s.logs`
  (scenario.rs:928-932), so carry 75's admission bound (:849-854) is never checked on a --logs 0 / record() run.
  Fix: skip `filed`'s map when no folder (or flush by the room's tick); count events from dump_events in both paths.
- F4 (Low) scenario.rs parse `* hz` (:1322-1324), burst_starts (:1263), run sums (:821, :1038): unchecked u64
  arithmetic on operator text; debug panics, release wraps (npt-lab has no release overflow-checks). Fix: checked
  ops, usage error.
- F5 (Low) npt-lab's own bites are compiled into the measured build: logs.rs:160-163, :316-331 (whole-as-lost
  writer) and precheck.rs:165-167, :193-215, :317-327 (late-command hold) are not cfg(feature = "bites"); they are
  only unreachable (Bites::of refuses every id in a measured build, scenario.rs:245-250). plan.md:630-631: "compiled
  only into the bites feature build ... The measured builds never contain them"; npt-server gates its site
  (pump.rs:451-452). Fix: cfg-gate the fields and branches.
- F6 (Low) /tmp/npt-lab-tests holds 124 folders now: the 93 counted by ruling 2 (mtime before 00:28) plus 31 made at
  00:29:45-00:34:28 (pids 9579, 15700: the builder's c3-reg.log test run at 00:29:40-00:31:17 and the next, before the
  helper changed). None of the 93 deleted. The report must say 124 and why.
- F7 (Low, speed, not measured) log/src/lib.rs:334-403 + :578: every line is parsed twice (the NoDuplicateKey pass
  allocates a String per key and a BTreeSet per object, then from_str to Value). Estimated +30-50% of read_log's time
  on -57's 24 KB lab lines; memory unchanged. Fix if the judge's timing (ruling 1 point 3 (4)) shows it matters:
  borrowed keys, or one pass that builds the Value and refuses duplicates.
- F8 (Low) replay.rs:265-279: n = header.clients (u32) sizes `vec![None; n]` per tick; the u8 check at :272 fails
  only at slot 256, after slot 0's vec![None; n]; a header with clients 4e9 aborts on allocation. Fix:
  u8::try_from(h.clients) before the loop.
- F9 (Low) scenario.rs:214: Bites::of sets `schedule` from npt_server::BITES, which now holds the pump's id
  (schedule.rs:252-257), so `schedule` is true for PM-0001-30/wallclock though Schedule::with_bite returns None for
  it; harmless (Bites::schedule falls back, :311-322) but the flag misnames. Fix: compute it as `pump` is,
  `Schedule::with_bite(TICK_HZ, LOOP_CATCHUP, id).is_some()`.

## Met
- npt-server (U4R-Q6 (a)): site pump.rs:449-453 on that instance's decoded Inputs; allow on wallclock_aims alone
  (:232-246); field rule (aim[0] only; never tick, seq, slot, buttons, presses, view time, ack); measured build
  unchanged (all cfg(bites)); BITES 4 with its pin server/tests/schedule.rs:347-368.
- npt-log (U4R-Q13 (a)): refusals at :307-403, :410-416, :578; depth bounded by serde_json's recursion limit; only
  crates/log/src/lib.rs changed; Writer::new runs check_header (:455).
- bots: no clock, no hash map, splitmix64 seeded per slot; clippy.toml bans fire (b4-plant-*.log, builder evidence).
- lab determinism: BTreeMap/Vec only; the one HashMap is r1's test-only bite (scenario.rs:684-689).
- RR-L5 / carry 82: hash.rs:235-237, :266-271, :312-317; dispatch :271-292, :332-361 (residual F2).
- RR2-L3 / carry 83 (d): hash.rs:1655-1749; plant c4-plant-83d.log fails it (builder evidence, pre-commit).
- RR2-L1 / carry 85 (a): check.mjs:1436-1438, :1470, :1487-1488; loader.mjs:13-16.
- RR2-L2 / carry 85 (d): check.mjs:1790-1799. RR2-L4 (a) / carry 85 (e): check.mjs:1576.
- carry 6 (rust 1): scenario.rs:317, :321; precheck.rs:303.
- carry 14 (rust 3) U4r half: scenario.rs:56, :815; pinned by p5 scenario.rs:2064-2115; check() hash.rs:365-396.
- scratch (ruling 2 point 32): scenario.rs:1544-1579, rooted at target/<profile>/npt-lab-tests; removed on pass;
  kept on failure (the planted 83 (d) failure's .gate/shared-target/debug/npt-lab-tests/hash-83d-3588 kept); no /tmp
  use in lab/src.

## Could not check
- No cargo run (runner constraints above; disk 4.7 GB). F1, F2, F3, F8 by reading. F7's cost estimated, not measured.
