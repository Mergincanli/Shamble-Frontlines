# U4r build notes: the L1 runner and its judged runs (builder, from u4r-brief.md rev 3.2)

## Block 0: start (done)

- Launch state found: the worktree was created at 4d21c1a (origin/main, PR #48), not at the spike head 2fbec77, and held
  no `.checkpoints/` and no `spikes/netcode-proof`. The first message's checks failed there (diff spikes exit 1; both
  ancestor checks exit 1). I did not move the worktree on my own.
- The main session's correction (message during the run): `git merge --ff-only 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef`,
  not a stop at `<base>` 614823c (whose brief is the pre-fill one, sha256 362e771e...; I confirmed that hash with
  `git show 614823c:...u4r-brief.md | sha256sum`). Before that merge I had run no `git merge --ff-only 614823c` and no
  `git switch -c`. The merge: "Updating 4d21c1a..2fbec77 / Fast-forward".
- Where the prompt and the brief differ: the brief's section 1 item 1 says `git merge --ff-only <base>` and then
  `git log -1` shows `<base>`; the prompt and the correction say HEAD is 2fbec77 (later than `<base>` by .checkpoints/
  commits only: `git log --oneline 614823c..2fbec77` is 1 commit and `git diff --name-only 614823c 2fbec77` outside
  .checkpoints/ is empty). I followed the prompt and the correction, as U4d2's builder did (u4d2-build.md, Block 0).
- Worktree path: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf
- Branch: worktree-agent-a11e2354169f39acf
- `git log -1 --format=%H`: 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef
- Brief: .checkpoints/netcode-proof-test/u4r-brief.md, sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046,
  1,909 lines (matches the prompt).
- Runner: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh, sha256
  349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831 (matches), `bash -n` exit 0.
- `git diff --quiet 614823c HEAD -- spikes/`: exit 0
- `git merge-base --is-ancestor 614823c HEAD`: exit 0
- `git merge-base --is-ancestor 4e91830 HEAD`: exit 0
- `df -h /home/user`: /dev/vda 252G size, 32G used, 6.8G avail, 83% (above the 4 GB floor).
- Next: read the brief whole; write the plan as blocks here.

## Block 1: the brief read whole (done)

- Read u4r-brief.md lines 1-1909 in full (sections 0-12). Next: read the merged code section 1 item 6 names, then
  write the block plan below.

## Block 2: first build, the shared target checked (done)

- df before: /dev/vda 6.8G avail (83 %); uptime 8:36.
- `u4r-run.sh <wt> .gate/u4r/b0-warm.log cargo test --locked --workspace --no-run` (shared lock): exit 0, 19:37:16 to
  19:37:42 UTC; provenance lists all 17 npt-* crates "Compiling ... (<wt>/spikes/netcode-proof/crates/...)".
- Section 4 rule 1 checked after this first build: the test executables are under
  /home/user/Shamble-Frontlines/.gate/shared-target/debug/deps/ (the log's "Executable unittests ... (.gate/shared-target/...)"
  lines), and `ls <wt>/spikes/netcode-proof/target` gives "No such file or directory".

## The block plan (each block written done here when it is)

Order: the core first, since every test, run and script rests on it. Each block is one unit of section 1 item 3.
- B3 carry 63: crates/log/src/lib.rs, `read_log`'s duplicate-key refusal and `check_header`'s control-character
  refusal, each test shown failing first (U4R-Q13 (a)).
- B4 npt-bots: crates/bots/clippy.toml (A13) with its two plants; `Behaviour` (brawl, open-loop scripts), 2.2's step
  and press rules, its tests.
- B5 npt-lab core: `SimClock`, `L1Scenario` in RQ8's order on `L1Net` (and on Loopback for the pre-check), the room,
  one `Decoder` per session, 48 `ClientEndpoint`s with the lead and starts, session handling (carry 76), tokens, the
  `--bite` dispatch, scripted placements and carry 49's refusal.
- B6 npt-lab logs: room, client and lab logs (header params, `hs` writers under carry 62, events: admit, burst_heal,
  burst_eviction, counter_wrap, snap_refused), the room-side series PM-0001-2 reads.
- B7 the room-side frame mirror per client (decoded from the room's datagrams): everyone_in_view, carry 49's two
  figures, carry 59's ES share, -57's pairing, r4's class audit.
- B8 `scenario` subcommand (with `--record`), the recordings manifest (carry 82), precheck-recordings.json.
- B9 `replay` (`ReplayX86`) and r3.
- B10 the -30 pre-check (both legs, replay clients, pacing, comparer, RTT rule; late-command bite; the wallclock bite's
  npt-server items, U4R-Q6 (a)); r6, r7.
- B11 tests r1 (twin), r2 (contract, with U4R-Q11's negatives), r8, r9; r10 with check.mjs's carries 83-85.
- B12 run scripts ops/runs/c-u4r-*.mjs; SERIES.md's new rows.
- B13 D13's five runs, r5, the three recordings, the `wasm` step's time; the end checks (section 5).

## B3: carry 63 in crates/log/src/lib.rs (done)

- Tests first: `fu3_a_json_object_with_a_duplicate_key_is_refused_on_read` and
  `fu7_a_header_text_field_with_a_control_character_is_refused` (log/src/lib.rs, tests module). Run before the
  refusals (`-x`, .gate/u4r/b3-log-before.log): both FAILED; fu3 panicked at crates/log/src/lib.rs:874:46 "a duplicate
  key in a tick's v: Log { ... v: {"work_ns": 9000002} ... }" (the log read, the first value lost); fu7 panicked at
  :908:13 "case 0: the writer refuses the header".
- The refusals: `no_duplicate_key` (a serde visitor over the line, refusing any object holding a key twice, at any
  depth) called by `read_log` on every line; `control_char` over the header as a JSON value (every string and key) in
  `check_header`, which the writer and the reader both call. Nothing else in crates/log changed.
- After (`-x -k`, .gate/u4r/b3-log-after.log): npt-log 13 passed (11 + 2); clippy -p npt-log --all-targets -D warnings
  clean; npt-judge's tests all pass (14, 2, 43, 19: its fixtures included).
- fmt: rustfmt's diff (two `replacen` calls in fu3) applied by Edit.

## B4: npt-bots' `Behaviour` and crates/bots/clippy.toml (done)

- crates/bots/clippy.toml (new, A13): lab/clippy.toml's two type bans and clock ban, crate-wide.
- crates/bots/src/lib.rs: `Behaviour` (pub) with `brawl(scene, slot, seed)`, `script(scene, slot, seed, name)`
  (`presses`, `clamp`, `steady`; else `None`), `every_step(self)` (D13's test-only setting), `given()`, `skipped()`,
  `step(endpoint, received, now_ns, view) -> bool`. 2.2's rules in `step`: `input(None, received, now)` first; no
  command before `newest_tick()` and `slot()` are `Some`; none at a step whose newest tick did not move (counted in
  `skipped`); else `input(Some(c), &[], now)`. Fire held on every command (the weapon interval sets the shot rate);
  press counters move by at most one edge per command (`presses` moves PRESS_FIRE on every second command); view =
  newest tick, fraction 0; moves from a 16-entry integer table, each inside 127²; the brawl aims at a seeded enemy
  from the host's `view` (slot, position cm) +100 cm, else a seeded blind point; seeded by splitmix64 per slot. No
  float, no clock, no hash map.
- Finding (plan silent): the brawl needs to know where enemies are, and npt-client exposes no decoded record (R16),
  so `step` takes a `view` from its host: in npt-lab, the room-side frame of that client's newest tick (B7). The
  open-loop scripts never read it.
- Tests, crates/bots/tests/behaviour.rs, 5 (b1-b5): b1 the step rule (no command before a snapshot; one per moved
  newest tick; stamped newest + lead; view newest, fraction 0; a skipped step counted); b2 every_step; b3 at most one
  press edge per counter per command, every script and the brawl; b4 seeded determinism and the 127² bound; b5 the
  brawl aims at an enemy in view, never a teammate; clamp aims beyond reach. All pass; clippy -D warnings clean
  (.gate/u4r/b4-bots.log).
- A13's two plants (`-x`, each one call, restored by Edit, sha256 of the clean file a3051aff... before and after each):
  (1) a scratch `std::collections::HashMap` in crates/bots/src/lib.rs: clippy "error: use of a disallowed type
  `std::collections::HashMap`" at crates/bots/src/lib.rs:104:48 and :104:12 (.gate/u4r/b4-plant-hashmap.log); restored,
  sha256 a3051affb3fba12a8e3c44dfba5abfd06889595572fcfe3150a318bf5736e47a; (2) a scratch `std::time::Instant::now()`:
  "error: use of a disallowed method `std::time::Instant::now`" at :104:5 and "disallowed type `std::time::Instant`"
  (.gate/u4r/b4-plant-clock.log); restored, the same sha256.
- fmt: rustfmt's diffs applied by Edit after the plants (lib.rs now 3a0ec91e...); fmt check exit 0, 5 tests pass
  (.gate/u4r/b4-fmt2.log).

## B5-B7: the scenario, its logs and the mirror (in progress)

- Design choices (each for the report): `L1Scenario` and its modules live in npt-lab's binary (scenario.rs, logs.rs,
  mirror.rs), as U4d2's `hash` does; the library keeps `HashCompare` and `TickHash` (fewer pub items; 2.7 allows but
  does not require them pub). RQ8's step (4), the 60 Hz frames, has no work until U4g's `Interp`, so the runner adds no
  instants for it. Client links are polled at every instant into a per-client inbox (the socket's buffer), read by
  the client at its step (5), so `next_event_ns` names internal events again; arrivals keep their stamps (host clock =
  SimClock + the client's seeded offset, rate 1).
- Written, not compiled yet: crates/lab/src/mirror.rs (the room-side frame per client: sets, -57's errors, r4's audit,
  the bots' view), crates/lab/src/logs.rs (room, client, lab logs; carry 62's client writer and the stop's decision).
- Findings so far (for the report): (F1) npt-room exposes neither its interest radius nor its flood
  (`RADIUS_CM` private, `AreaFlood`'s methods pub(crate)), so r4's spatial rule checks the radius, a copy of 6,400 cm
  (mirror.rs `INTEREST_RADIUS_CM`), not the flood. (F2) shots and gameplay events per second are read from the World's
  dump (`events` section), since no item gives a tick's events. (F3) -57's series hold one value per decoded entity
  position (SERIES.md:66-73): on SS that is about 60 positions per client per tick, about 50 M values per 10-min run;
  the log size is measured once the runner runs (B13) and goes to the main session. (F4) carry 62 (h)'s bite as worded
  ("a client that writes null and incomplete 1 for a refused group") has nothing to act on in a correct P0 run (no
  group is refused there), so on its own it cannot fail the P0 setup check: built as worded (`refused_as_lost` in
  logs.rs), reported to the main session, whose check it is (U4R-Q2 (c)).
- Built and compiling (both builds, clippy -D warnings clean: .gate/u4r/b5-r1b.log): scenario.rs (`SimClock`,
  `Server` with carry 76's handling, `Settings`/`parse` for `npt-lab scenario`, `Bites::of` dispatch by each crate's
  list, `run` in RQ8's order, `params`, `burst_starts`, `lead_for`, `stream_of` (exactly one stream channel decodes a
  reliable message, else an error), `check_placements` (carry 49)), wired in main.rs (`scenario` subcommand, exit 0/1/2).
- Smoke run (debug, .gate/u4r/b5-smoke1.log, SS P0 seed 1, 48 clients, 10 s warm-up + 5 s measured): 450 ticks in 38 s
  wall; late 0; shots per measured second 700; 0 audit breaches; client hashes 21,552, 0 sentinels, 0 incomplete.
  F3 measured: lab.jsonl 10,961,471 B for 450 ticks (-57's values: ~24 KB a tick), so ~440 MB per 10-min run (18,300
  ticks); room.jsonl 881,985 B, client.jsonl 492,198 B.
- Unit tests (scenario.rs tests): s1 carry 49's refusal (above 3,955 cm refused by name, at it accepted); s2 the leads
  (P0 2, P1 3, P2 5, B1 5, P3 7); s3 -18's burst starts within [0, 1 tick) of their boundaries, seeded; s4 the channel
  rule. All pass (.gate/u4r/b5-clippy1.log).
- r1 (scenario.rs `r1_two_processes_of_one_seed_give_equal_fate_logs_and_per_tick_hashes`, with `r1_leg` re-run as two
  processes through `current_exe`, D-2's form): 16 clients, P2, 6 s + 2 s: 543 tick records (room and client logs)
  equal, fate logs equal, connect-order digests equal (de1838107fbd6d84). Its bite (bites build,
  `r1_bite_a_hash_map_client_order_makes_the_two_processes_differ`; site `client_order_bitten`, scenario.rs, under
  `#[cfg(all(test, feature = "bites"))]` with `#[allow(clippy::disallowed_types, clippy::iter_over_hash_type)]`, id
  `npt-lab/client-order`, chosen by the test only): order digests 8e18633ced50ba5a and ab0a9e84dfb44cb2; caught in the
  fate logs and in the per-tick hashes ("room: tick 2 (and 2) differs") (.gate/u4r/b5-r1b.log). Why connect order:
  L1 draws each link's fates by its connect index, so a client order taken from a hash map moves every client's fates.
- Next: B8 (carry 82's manifest schema and the runner entries' regeneration), then B9, B10, B11, B12, B13.
- Release smoke (.gate/u4r/b5-rel-p2.log, SS P2 seed 1, 48 clients, 10 s + 20 s): 933 ticks (33 after); lead 5;
  commands 38,311, steps without one 6,194, duplicate stamps 0; late 112 (0.29 %); press refused 0; rewinds refused 0;
  stale 0; measured incomplete 585; shots 700 a measured second; 0 audit breaches. df then 6.0G.

## B8: carry 82, the recordings manifest (done, schema; the runner entries join at B13)

- hash.rs: `Kind` (`Synthetic`; `Runner { profile, clients, warmup_s, measured_s }`) on each `Entry`, read from a
  required `kind` key; `manifest` refuses a name outside `[A-Za-z0-9-]+` (`name_ok`) and `ticks` above `bytes / 12`
  (`TICK_HEADER_BYTES`), both before anything is generated; `regenerate_all` checks each entry's generator against its
  kind (synthetic: `Recording::GENERATOR`; runner: 0) before generating; `generate` dispatches: synthetic as merged,
  runner through `scenario::record` (the scenario re-run whole, logs to sinks, bytes in memory).
- recordings.json: `"kind": "synthetic"` on the three entries, its `about` text rewritten for carry 82 (no hash
  changed). precheck-recordings.json: made at B13 with SS P0 seed 4.
- scenario.rs: `Settings.logs`, `Settings.record_mem`, `record(...)`; a recording run stops at the measured phase's end
  (it holds exactly the warm-up and measured ticks); logs.rs: `Out` is `Writer<Box<dyn Write>>`, a sink with no folder.
- Tests: hash.rs `r10_carry_82_the_manifest_refuses_a_bad_name_and_ticks_above_bytes_over_12`,
  `r10_carry_82_a_runner_entry_reads_its_settings_and_its_generator_is_0`; h5's committed-manifest test also asserts
  every kind is synthetic; scenario.rs `s5_a_runner_recording_regenerates_byte_for_byte_and_replays_to_the_room_s_hashes`
  (8 clients, P0, 6 s: regenerated twice equal, 180 ticks, generator 0, span 2,000 ms, the logged run's bytes equal,
  and npt-sim's `Replayer` gives the room's World hash at every tick). All pass (.gate/u4r/b8-test.log, b8-test2.log);
  clippy --all-features -D warnings clean.
- Finding F5: npt-room's `HISTORY_MS` is private, so the recording's `span_ms` is a copy (scenario.rs
  `ROOM_HISTORY_MS` = 2,000, room.rs:33).

## D13: the five short runs (done, before any `scenario --record`)

- One `-x -k` runner call (.gate/u4r/d13.log, 20:13:08-20:15:42 UTC): `cargo build --release --locked -p npt-lab`, then
  that binary five times: `scenario --set d13 --scene SS --seed 1 --warmup-s 10 --measured-s 50 --suite brawl` (48
  clients, the brawl; one length for all: 1,800 ticks measured plus warm-up and after). Added for it: `--lead <1..8>`
  (test-only, refused on a run with marks, like `--every-step`). df before: 6.0G.

| Run | Commands | Late (share) | Press refused | Buffer full | Duplicate stamps | Steps without one | Rewinds refused |
|---|---|---|---|---|---|---|---|
| (1) P2 lead 5 | 75,419 | 213 (0.28 %) | 0 | 0 | 0 | 12,290 | 0 |
| (2) P2 lead 5, every step | 87,709 | 12,489 (14.24 %) | 0 | 0 | 12,290 (14.01 %) | 0 | 0 |
| (3) P2 lead 3 | 75,419 | 65,870 (87.34 %) | 0 | 0 | 0 | 12,290 | 2,873 |
| (4) P3 lead 7 | 69,652 | 129 (0.19 %) | 0 | 0 | 0 | 17,889 | 0 |
| (5) P0 lead 2 | 86,352 | 0 (0 %) | 0 | 0 | 0 | 0 | 0 |

- As expected: (1), (4), (5) at most 1 % late with 0 press refusals; (3) most commands late (and 479 shots a second,
  below PM-0001-2's band); (2) against U4R-Q14 (b)'s arithmetic of about 10 % at P2: measured 14.0 % duplicate stamps
  and 14.2 % late, above it (a fact for the advisor; (b1) is in force, so no judged run has duplicates). No stop:
  the recordings may proceed.
- Each run took about 30 s wall for about 1,833 ticks: R about 61 ticks a second on the release build (r5 measures it
  properly). At that R a 9,000-tick runner entry's regeneration inside `npt-lab hash` takes about 148 s, two about 296 s,
  on top of the `wasm` step's 33.1 s: over the 300 s budget (section 5 item 4: U4r's det-u4.mjs question, RR-L4
  weighed first). To be measured, and the runner entries held out of recordings.json until the advisor rules if so.

## B13 (part): the three runner recordings and the determinism route (done; route to the advisor)

- `-x -k` runner call (.gate/u4r/rec.log, 20:17-20:25 UTC; df before 5.8G): release build, then `bash -c "time
  <release npt-lab> scenario ... --warmup-s 10 --measured-s 290 --logs 0 --suite record --record <file>"` (48 clients,
  the brawl, 9,000 ticks each, generator 0, span_ms 2,000). `--logs 0` added (only with `--record`): s5 shows the
  recording does not depend on the logs.

| Recording | Settings | Bytes | SHA-256 | `scenario --record` wall | Late / commands | Rewinds refused |
|---|---|---|---|---|---|---|
| SS-P2-s1 | SS P2 seed 1, first tick 0 | 16,937,652 | 5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3 | 200.1 s | 991 / 371,880 | 0 |
| ES-P3-s2 | ES P3 seed 2, first tick 1,000,000 | 16,929,501 | fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b | 104.2 s | 548 / 355,244 | 0 |
| SS-P0-s4 | SS P0 seed 4, first tick 0 | 16,950,444 | 6933b56a619197c719439ff0c16eed3b8927645c3a0bb66910568c6ccf83bab3 | 167.2 s | 0 / 431,904 | 0 |

  (Their runs kept no log, so their -57 candidate errors piled up in memory, which slowed them: fixed after, a
  no-log run now drops them.) SS-P0-s4's `rewinds_refused` is 0: 7.1's order lets the pre-check proceed.
- crates/lab/precheck-recordings.json (new): SS-P0-s4 as a runner entry. `npt-lab hash --manifest
  crates/lab/precheck-recordings.json` (.gate/u4r/regen-precheck.log): "regenerated SS-P0-s4: 16950444 bytes, sha256
  6933b56a..., as the manifest's"; native legs 9,000 ticks, 0 mismatching; 110.1 s wall (regeneration + legs). So carry
  82's runner path holds end to end, byte for byte with the file the logged-off run wrote.
- With both runner entries appended to recordings.json, `npt-lab hash --manifest crates/lab/recordings.json`
  (.gate/u4r/regen-all.log): all five regenerated as the manifest's, native legs 45,000 ticks equal, **201.0 s for that
  one call**. The `wasm` step makes three such legs calls (measured; hash-map and std-sin in the bites build: check.mjs
  `wasmStep` lines "npt-lab hash", "hash-map native legs", "std-sin native legs"), and each regenerates every entry
  (hash.rs `legs`: `regenerate_all(&entries, ...)` over the whole manifest, `--only` selecting the replay only), so the
  step would take about 33 s + 3 x ~170 s: well over its 300 s budget. Per section 5 item 4 this is **U4r's
  det-u4.mjs question to the advisor**, with RR-L4's remedy (crates/hash-wasm, not my lane) and a cheaper one to weigh
  first: regenerate only the entries a call replays (`--only`'s), which would make the bites calls cheap (their
  `--only` is the first, synthetic, recording) and the step about 33 + 170 = ~205 s plus the two runner entries' Wasm
  replays. That edit is hash.rs `legs`, U4d2's behaviour: not made without a ruling.
  **So recordings.json keeps the three synthetic entries (with `kind`)**; the two runner entries, exact, for whichever
  route is ruled: `{"name": "SS-P2-s1", "kind": "runner", "profile": "P2", "clients": 48, "warmup_s": 10,
  "measured_s": 290, "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes":
  16937652, "sha256": "5b79abb2a2cba55b06a8916b235aea1766960867f09a038aed146805a29a32e3"}` and `{"name": "ES-P3-s2",
  "kind": "runner", "profile": "P3", "clients": 48, "warmup_s": 10, "measured_s": 290, "scene": 2, "seed": 2,
  "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 0, "bytes": 16929501, "sha256":
  "fad8f67ded2a220e42d7e4d42a3d3f2f76e980d4c55abb4fbcd01c550045953b"}`. hash.rs h5 asserts the three and the
  pre-check file's runner entry.

## B11 (part): r4, r8, r9 (done)

- scenario.rs tests (.gate/u4r/b11-r4r8r9.log): `r4_every_group_is_one_its_class_lets_that_client_see_on_three_seeds`
  (SS, 48 clients, P2, 6 s + 2 s, seeds 1-3: 273/274/273 ticks; owner 33,135/33,265/33,111 own-state records, spatial
  374,694/371,513/347,972 entity records, public 12,761/12,825/12,767 headers, events 857,615/874,057/888,067; 0
  breaches; team and public sections: not exercised); its bite `r4_bite_owner_to_all_fails_the_audit_by_name` (bites
  build, `--role bite --bite catalogue-5/owner-to-all`, no mark): 16 breaches, the first "tick 2: client 1's own-state
  section holds Soldier 2 (owner: only its own slot's)". `r8_a_replaced_session_s_inputs_and_leave_reach_no_room_call`
  and `r9_a_kicked_session_s_later_datagrams_reach_no_room_call` (a `Server` on Loopback pairs) pass.
- Plants of the lab's glue (`-x`, each one call, restored by Edit, sha256 checked): r8's (the `drop_slot(...,
  "replaced", ...)` call replaced by `let _ = (slot, keep);`): r8 FAILED at scenario.rs:1741 "the first was told and
  dropped" (.gate/u4r/b11-plant-r8.log); restored 021511de.... r9's (`if kicked && false`): first run failed at the kick
  detection, because the test read the kick from the glue (`server.ended`); the test now reads it from the room
  (`room.client(0).kicked()`), clean file e528f98084a4482601fed6bdeebc56b0d048901e62b8f47042838b6c4854ea0d, r8 and r9
  pass (.gate/u4r/b11-r9-clean.log); re-planted, r9 FAILED at scenario.rs:1799 "the lab dropped the kicked session"
  (.gate/u4r/b11-plant-r9b.log); restored, sha256 e528f980... again.

## State at this copy's end, and what is next (a fresh copy resumes here)

- `git status` holds: crates/log/src/lib.rs (carry 63), crates/bots/{src/lib.rs, tests/behaviour.rs, clippy.toml},
  crates/lab/src/{main.rs, hash.rs, scenario.rs, logs.rs, mirror.rs}, crates/lab/recordings.json (kind key only),
  crates/lab/precheck-recordings.json (new). Everything compiles in both builds; clippy -D warnings was clean at
  .gate/u4r/b5-r1b.log and b8-test.log (re-run it after fmt).
- **Next, in order:**
  1. fmt: `cargo fmt --all --check` (.gate/u4r/b11-fmt.log) shows 53 hunks: hash.rs 3, logs.rs 8, mirror.rs 6,
     scenario.rs 36. Apply each by Edit (never `cargo fmt` writing), then clippy both builds `-D warnings`.
  2. B9: `replay` (`ReplayX86`): read each client's `--bytes` files (scenario.rs `write_step`'s format: host ns u64,
     count u32, then per message channel u8, arrival u64, length u32, bytes), feed a fresh `ClientEndpoint` (same join,
     lead, starts) step by step, write a `replay` log (process `replay`, `hs.replay_hash`); r3 on a short SS P2 run;
     its bite: the re-decoder's endpoint `with_bite("PM-0001-17/client-rounding")` (npt-client's id, no new one).
  3. B10: the -30 pre-check: the Loopback leg (`Server` over `npt_link::Loopback` pairs, `RttSource::Link`) and the L1
     P0 leg; replay clients encoding the SS-P0-s4 recording's commands with npt-proto (7.2's pacing: send T at the
     instant tick T - L is due), scripts by `Room::script` through `check_placements` (then drop its dead-code
     allows); `Session.first_sample_ns`/`first_input_ns` already logged by `Server::io`; the comparer through
     `HashCompare::of` and TickHash's doc (carry 83 (a)); r6, r7; late-command bite (npt-lab) and the wallclock bite
     (npt-server: `Decoder::with_bite`, BITES to four, tests/schedule.rs:347 pin, U4R-Q6 (a)).
  4. r2, the contract test (U4R-Q4 (a) scope, U4R-Q11's four negative cases), and run the real judge by hand on its
     short-run logs (`npt-judge --marks marks/marks.json --logs <dir> --only ...`), headers with marks.
  5. r10's check.mjs half: carries 83 (b)-(d), 84, 85 with check.test.mjs cases; loader.mjs's header sentence.
  6. B12: ops/runs/c-u4r-*.mjs (c-u2's model, binaries' paths from cargo's JSON messages); SERIES.md's new rows for
     every name U4r writes that has none (lead_ticks_count, event_start_count, cmd_start_count, the weapon and health
     params, event_queue_cap_provisional, warmup/measured_ticks_count, behaviour, every_step, stand_in_burn_ns,
     stand_in_top_up_ns, input_payload_bytes, client_counts', burst_heal's full_state and queue_max_count,
     burst_eviction's baseline_age, overflow_resync, fate_log, run_lengths, recording, class_audit, admit's slot).
  7. r5 (seeds 1-3 per scene and profile, the slowest kept), then section 5's end checks twice.
- **Stops and findings for the report:** F1 (r4's spatial rule checks the radius only, a copy of RADIUS_CM); F2
  (events read from the World's dump); F3 (-57's series, ~440 MB per 10-min run: disk); F4 (carry 62 (h)'s bite as
  worded has nothing to act on at P0); F5 (HISTORY_MS copied); the determinism route (the runner entries held out of
  recordings.json: U4r's det-u4.mjs question, with the cheaper `--only` regeneration to weigh); D13 (2)'s 14 % against
  the arithmetic's 10 %.

## Copy 2

(Copy 2's state and next steps: the section "Copy 2: state and what is next" at this file's end, which replaces the
first copy's state section above for items 1 to 3.)

### Start checks (done)

- Started 2026-11-13 about 20:55 UTC, from the copy-2 prompt (the main folder's u4r-copy2-prompt.md).
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf, branch
  worktree-agent-a11e2354169f39acf; `git rev-parse HEAD`: 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef (holds).
- Brief sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046, 1,909 lines (holds).
- Runner sha256 349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831 (holds).
- `df -h /home/user`: /dev/vda 252G size, 33G used, 5.6G avail, 86 %.
- Read before any change: these notes whole (282 lines); `git status --short` (5 modified, 6 untracked under spikes/
  and this file, as the state section lists); `git diff` of main.rs, recordings.json, hash.rs, log/src/lib.rs,
  bots/src/lib.rs; and each untracked file whole: bots/clippy.toml, bots/tests/behaviour.rs,
  lab/precheck-recordings.json, lab/src/mirror.rs, lab/src/logs.rs, lab/src/scenario.rs (1,833 lines).
- Re-read of the brief for the next blocks: sections 1, 4, 5 and 6 (2.1, 2.4, 2.5 and 7.2 as each block needs them).

### Item 1: fmt (done)

- `cargo fmt --all --check` (.gate/u4r/c2-fmt1.log, exit 1): 53 hunks (hash.rs 3, logs.rs 8, mirror.rs 6, scenario.rs
  36), as the first copy counted. Each applied by Edit (the log's added lines carry a stray 0x0f byte; read with `tr -d
  '\017'`). No content change: line breaks only.
- After (.gate/u4r/c2-fmt2.log, one `-k` call, 20:55:38-20:55:42 UTC, df 5.6G): fmt check exit 0; `cargo clippy --locked
  -p npt-lab -p npt-bots -p npt-log --all-targets -- -D warnings` exit 0 and the same with `--all-features` exit 0;
  provenance "Checking npt-lab/npt-bots/npt-log ... (<wt>/spikes/netcode-proof/crates/...)" twice each.
- Next: item 2, B9 `replay` and r3.

### Item 2: B9, `replay` (`ReplayX86`) and r3 (done)

- crates/lab/src/replay.rs (new): `npt-lab replay --logs <run folder> --bytes <folder> --out <folder> [--bite <id>]`
  (main.rs dispatch, exit 0/1/2 as `scenario`). `ReplayX86::of(header, bite)` reads the run's client log header (seed,
  params lead_ticks_count, cmd_start_count, event_start_count, tick_start); `decode(slot, steps)` feeds a fresh
  `ClientEndpoint` (join_of(seed, slot), the lead, the starts; the run's bite on its client, or `--bite`'s) each logged
  step as `input(None, bytes, host)` then `step()`, the filed hashes by wide tick (two different: the sentinel 0, as
  logs.rs `filed`). Why no command is needed: `ClientEndpoint::input` decodes before it stamps (client/src/lib.rs:379-410).
  `read_steps` reads scenario.rs `write_step`'s format (made pub(crate)). The log: `replay.jsonl`, the run's client
  header but process `replay`, this machine (`container()`), this binary's build, the replay's bite if given; one
  tick record per tick with a replay hash, `hs.replay_hash` of `clients` entries, phase from the client log; footer
  clean. `compare` both ways: every client-log hash against the replay's, and every replay hash with no client-log
  hash counted as a mismatch.
- r3's bite: no new id, npt-client's PM-0001-17/client-rounding given to `--bite` (its grid floored:
  `npt_client::grid_with_bite`, client/src/lib.rs:37-40); npt-lab's sources never spell it (the test does).
- Tests (replay.rs tests): `r3_the_bytes_file_reads_back_step_by_step` (round trip, cut short, channel code 9 refused);
  `r3_the_re_decode_gives_every_client_s_logged_hash` (SS P2 seed 1, 16 clients, 6 s + 2 s, `--bytes` into the system
  temp folder: 16 clients, 4,368 steps, 4,127 replay hashes; **4,127 (client, tick) pairs compared, 0 mismatches**; the
  log is process `replay`, every list 16 entries, no decoded_hash); bites build
  `r3_bite_a_floored_re_decoder_grid_gives_a_mismatch`: **4,127 compared, 4,127 mismatches**, the first "client 1 tick
  2: logged 53d6faf8b267eb6b, replay Some(6f5719629d5cadee)". Clean 2 passed, bites 3 passed
  (.gate/u4r/c2-b9-1.log; numbers .gate/u4r/c2-b9-2.log, `--nocapture`); clippy -D warnings both builds exit 0.
  The bite test follows the first copy's r1 and r4 form (a test asserting the check catches the bite).
- Note for the report: the test's bytes go to the system temp folder (never committed), not .gate/; the run scripts
  (B12) point `--bytes` under .gate/ (plan :762).
- fmt: rustfmt's 6 hunks in replay.rs applied by Edit (.gate/u4r/c2-b9-fmt.log); re-checked with B10's first build.
- Next: item 3, B10 (the -30 pre-check).

### Item 3, B10 plan (sub-blocks, each saved when done)

- B10a the wallclock bite's npt-server items (U4R-Q6 (a)); B10b the RTT rule and r7; B10c the two legs, the replay
  clients, the comparer and the records, with r6; B10d the late-command bite on one leg; B10e the wallclock pair shown.

### B10a: PM-0001-30/wallclock's npt-server items (done)

- crates/server/src/pump.rs: `Decoder.wallclock` (bites only); `WALLCLOCK_BITE` = "PM-0001-30/wallclock" (the id's one
  home); `Decoder::with_bite(id) -> Option<Decoder>` (pub, bites only: that id, else `None`); the site
  `wallclock_moves` in the pump's `(Joined, Input)` arm, after the bucket takes it: reads `SystemTime::now()` (allow
  `clippy::disallowed_methods`, `disallowed_types`, with its reason) and sets every passed-on command's `move_xy` to
  `[1 + (ns % 100), 0]`: inside 127², never a brawl move (127 long) nor `[0, 0]`, so it changes a sim input on every
  command of every run, whatever R (never the RTT, lateness or a bucket).
- crates/server/src/schedule.rs: `BITES` widened to four (the three schedule ids and `crate::pump::WALLCLOCK_BITE`),
  doc updated. crates/server/tests/schedule.rs: the pin (:347) now lists four (const `WALLCLOCK`), and the
  `Schedule::with_bite(..) is none` loop also takes WALLCLOCK (the schedule never plants it). Test name kept.
- npt-lab: `Bites.pump` (true when `Decoder::with_bite(id)` takes the id: no id spelled), `Server.decoder_bite`
  (set from `Bites` in `run`), `Server::accept` plants it on each new session's decoder (bites only).
  `LAB_BITES` is now `["PM-0001-17/refused-as-lost", "PM-0001-30/late-command"]`; `refused_as_lost` matches index 0
  only, `late_command` index 1 (B10d uses it).
- Runs (.gate/u4r/c2-b10a-1.log, -2.log; df 5.6G): clippy -D warnings npt-server + npt-lab both builds exit 0; `cargo
  test -p npt-server` (clean) all pass (3, 5, 16, 11 tests by target); `cargo test --workspace --all-features --test
  schedule --test loopback`: schedule 17 passed, loopback 7 passed. Finding (not a failure of mine, not check.mjs's
  measure): `cargo test -p npt-server --all-features` does not compile tests/loopback.rs (E0425 `npt_client::BITES`):
  `-p` with `--all-features` enables npt-server's features only, not npt-client's; check.mjs's `--workspace
  --all-features` unifies them. Recorded, not TE2-5 (a cargo selection artefact, before any test ran).
- Next: B10b.

### B10b-e: crates/lab/src/precheck.rs, r6, r7 and the two bites (built; depth re-run pending)

- precheck.rs (new; main.rs `precheck` dispatch, exit 0/1/2): `npt-lab precheck --manifest <precheck-recordings.json>
  --recording <file> [--lead 1..7] [--bite <id> --bite-leg loopback|L1]`: checks the file against the manifest's one
  entry (byte length and SHA-256) before anything runs, then `pair` runs both legs and prints the result lines (the run
  script writes them to runs.txt). Its module doc states 7.2's items: tick base, encoder route, pacing, admission,
  JoinAccepted first, scripts, comparer, setup condition.
- Route stated (R16): each replay client runs a `ClientEndpoint` only to decode and to send its Join and Pongs; its own
  ack-only `Input` is decoded back for `ack_tick`/`ack_groups` and replaced by the replay client's own `Input` (npt-proto
  encode): that ack plus the recorded commands, newest first, consecutive sequences, at most REDUNDANCY_CAP: every
  sent command whose tick has not run. No npt-client change. Pacing: the command stamped T is sent at step 5 of the
  instant tick T - L ran (default L = lead_for(P0) = 2). `commands_of` refuses a recording command that ran off its
  stamp. Both legs: `World::new(scene, rec.seed, rec.first_tick)`; `run_leg` refuses a room tick that is not the
  recording's; `Server` with `RttSource::Link` on both legs; Loopback pairs accepted at instant 0; L1 P0 links through
  the listener. Admission: no join enters `TickInput` (room.rs:833 region: only scripted placements), and s5 shows the
  World's hashes follow from the `TickInput`s alone, so admit ticks may differ between legs (they did not: 1..1).
- `rtt_rule(leg, records)` and `verdict` (VOID: late, rewinds refused, off-stamp, RTT rule, or Empty; else PASS on
  Equal, FAIL on Mismatch). Late-command bite (npt-lab id LAB_BITES[1]): slot 0 holds its first command stamped >= first
  tick + 60 and every later one until that tick ran, then sends T first with the later ones (one Input of L + 1).
  Wallclock bite: `Server.decoder_bite` on the bitten leg (B10a's site).
- Tests (.gate/u4r/c2-b10c-3.log; clippy -D warnings exit 0 on npt-lab clean and the workspace bites build):
  r7 (four tests, each case by name: first sample after the run VOID; equal and earlier not VOID; no first sample on
  L1 VOID; Loopback never VOID) pass; **r6** (`r6_a_clean_replay_runs_every_command_at_its_stamp_on_both_legs`: the
  SS P0 seed 4 runner settings with a 4 s warm-up and no measured phase = the recording's first 120 ticks, 48 clients):
  both legs late_count 0, rewinds_refused 0, off-stamp 0, 48 admitted (admit ticks 1..1), no session ended, RTT rule
  holds (L1) / Some(0) from the start (Loopback), scripted 0 (the recording holds no script), comparer Equal { entries:
  120 }, **PASS**; bites build: `precheck_bite_late_command_makes_the_pair_void_by_late_count_alone`: L1 late_count 1,
  delayed command tick 60, rewinds 0, RTT holds, comparer Mismatch at index 60 (1 of 120 differ), **result VOID (L1
  late_count 1)**; `precheck_bite_wallclock_gives_a_valid_pair_whose_comparer_names_a_tick`: both legs valid, **FAIL,
  first differing index 3, 117 of 120 differ**. Clean 5 passed, bites 7 passed.
- Finding (depth): reading `buffer_depth_ticks` before each tick gave "not 2" at all 5,568 reads on both legs: the room
  measures "at the last take" (input.rs:28), so its value is the newest stamp minus the tick taken; changed to a
  histogram read after each tick (re-run pending).
- Next: re-run r6 for the depth histogram; fmt; then item 4 (r2 and the real judge).

### B10: done (the depth re-run, fmt, the regression run)

- Depth (.gate/u4r/c2-b10c-4.log, r6 re-run): the room's measured depth at its take, both legs alike, **{0: 48, 1:
  5,568}**: 1 = L - 1 at every take after each client's first command, 0 at each client's first take. Why: the take of
  tick j holds commands sent up to the instant tick j - 1 ran, so stamped at most t_j + L - 1 (input.rs:28 "at the
  last take"). The header value 7.2 asks for (`buffer_depth_ticks_count` = the pacing lead L = 2) and the room's
  measure (L - 1) thus differ by 1 by definition at every tick: **for the advisor** (7.2's reading; plan silent).
  Logged per leg as a histogram in the result lines; no header is written by the pre-check (see below).
- fmt: 22 hunks (precheck.rs 21, scenario.rs 1) applied by Edit; check exit 0.
- Regression (.gate/u4r/c2-b10-reg.log, 21:20-21:27 UTC, df 5.3G): fmt exit 0; clippy -D warnings exit 0 (npt-lab,
  npt-bots, npt-server, npt-log clean; workspace `--all-features`); `cargo test -p npt-lab -p npt-bots`: npt-lab bin
  **38 passed, 1 ignored**, lib 5, behaviour.rs 5; workspace bites build for the same: npt-lab bin **43 passed, 1
  ignored**, lib 5, bots 5, and every other crate's tests that run in it pass.
- Not built in B10 (for the report): the pre-check writes no room log per leg (its hashes are `SimCore::state_hash`,
  the value logs.rs writes as `h`, printed through `HashCompare`'s Debug in the result lines); `npt-lab precheck`
  itself not yet run by hand on the full SS-P0-s4 recording (its release run is section 5 item 5's, with B12's script).

## Rulings (advisor-u4r-build1)

Source: the main folder's .checkpoints/netcode-proof-test/advisor-u4r-build1-full.md, sha256 476121ab..., 169 lines,
section "The rulings" read whole (21:31 UTC); each point's "Builder:" line is mine. Lanes opened: crates/lab/
det-recordings.json (point 1), check.mjs `wasmStep`'s two options and one check.test.mjs case (point 1 (4)),
ops/runs/det-u4.mjs (point 1 (5)). Order: points 3, 4, 9 before r2's runs; point 1 with r10's check.mjs half;
det-u4.mjs's run needs df >= 4 GB, and its non-cargo calls summed above 240 s stop and report.

| Point | What | State |
|---|---|---|
| 1 | route (c): det-recordings.json (2 runner entries), h5 asserts it; `legs` as merged; wasmStep options + 1 test; det-u4.mjs; one -x run | (1)-(3) done; (4)-(7) open |
| 2 | radius copy pinned by a placement test + plant 6,300; r4 counts checked/unchecked apart; full breach counter; report words | done (pinned by test) |
| 3 | -57's three series written only when the header's marks list PM-0001-57; per-profile bytes per tick in the report | writer done + test; bytes per tick for the report open (needs a B1 short run; P0/P2/P3 from d13 logs) |
| 4 | bite PM-0001-17/whole-as-lost (first whole measured tick per client filed lost); unit proof; kept logs .gate/u4r/whole-as-lost/ | done |
| 5 | ring pin test (HitHistory::for_ms capacity) + plant 2,100, or unpinned; hash.rs `check` refuses header first_tick/span_ms/ticks mismatch + 1 case | done (pinned by test) |
| 6 | dump reader test against SimCore::step's events + plant (kind one word off); report words | done |
| 7 | brawl's view: no change; report words | report only |
| 8 | D13 facts; correct the run-length line (10 s + 50 s = 300 + 1,500 ticks, plus 0-35 after) | report only (note corrected here: D13's "1,800 ticks measured" was wrong) |
| 9 | (a) --lead/--every-step refused with --record + 2 parse cases; (b) --logs 0 summary "not counted (--logs 0)"; (c) wall-time labels; (d) carry 75 statement | (a), (b) done; (c), (d) report only |
| 10 | data: R6 cites l1 :746-751 and :753-766; lab/Cargo.toml:32, bots/Cargo.toml:22 and :25 | data, noted |

- Also (U4R-Q6 (a)'s field rule, re-read at 21:30): the wallclock site now XORs each command's aim x with `1 |
  (ns & 0xfe)` (never 0: never the received value; at most 255 cm), in place of B10a's move replacement, which could
  equal a received move. Re-run (.gate/u4r/c2-p349-1.log): FAIL, first differing index 3, 117 of 120 differ.

### Points 9 (a)-(b), 3, 4 and point 2's counters (done; .gate/u4r/c2-p349-1.log, 21:31-21:33 UTC, df 5.4G)

- Point 9 (a): `parse` refuses `--lead` and `--every-step` with `--record` ("a recording run (--record) takes
  neither"); test `p9_lead_and_every_step_are_refused_with_record` (one case each). (b): `summary_line` prints "not
  counted (--logs 0)" for client hashes, sentinels, measured incomplete, decided lost at the stop and shots per
  measured second on a `--logs 0` run (the -57 and PM-0001-2 figures are not on the line).
- Point 3: `run` keeps the mirror's -57 errors (`logs.errors`) only when `s.meta.marks` holds "PM-0001-57"; the mirror
  and the bots' view unchanged. Test `p3_p4_a_short_p0_run_writes_57_s_series_only_when_listed_and_files_no_tick_lost`
  (SS P0 seed 2, 16 clients, 6 s + 2 s): listing -17 and -57: measured incomplete 0, -57 series present; listing -17
  only: 0, none.
- Point 4: id PM-0001-17/whole-as-lost (LAB_BITES[0], in place of refused-as-lost; `Bites.whole_as_lost`); logs.rs
  `filed`: on each slot, the first non-sentinel hash of a measured tick (and every later filing of that tick) goes in
  as `Res::Lost` (`bitten_at` per slot). Bites test `p4_bite_whole_as_lost_makes_the_measured_incomplete_sum_the_client_count`:
  **16 of 16**; the clean half is the p3_p4 test's 0.
- Point 2's counters: `Audit.spatial` now counts only records checked against the radius, `spatial_unchecked` the
  rest (doors, the wall, records the tick's room records lack, and owner-only records in the entity section),
  `breaches_count` every breach (16 texts kept). `class_audit` gains `spatial_unchecked_count`; its `breaches_count` is
  the full count. r4 (clean): seeds 1-3: 273/274/273 ticks; owner 33,135/33,265/33,111; **spatial checked
  371,660/368,494/345,239, not checked 3,034/3,019/2,733**; public 12,761/12,825/12,767; events 857,615/874,057/888,067;
  breaches 0. r4's bite: **33,020 breaches** (not the 16 kept), the first "tick 2: client 1's own-state section holds
  Soldier 2 (owner: only its own slot's)". Point 2's placement pin test: not yet.
- clippy -D warnings exit 0 (npt-lab, npt-server clean; workspace all-features).
- Next: the kept bitten run under .gate/u4r/whole-as-lost/; then points 2 (pin), 5, 6 tests; point 1.
- **Point 4's kept logs (done):** /home/user/Shamble-Frontlines/.gate/u4r/whole-as-lost/ (room.jsonl 1,176,443 B,
  client.jsonl 657,158 B, lab.jsonl 16,794 B), runner log .gate/u4r/whole-as-lost.log (-x, 21:33:39-21:34:35 UTC):
  `cargo build --locked -p npt-lab --features bites` then the debug binary's `scenario --run u4r-whole-as-lost-1 --set
  u4r-short --date 2026-11-13 --commit 2fbec77... --scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10
  --measured-s 10 --role bite --bite PM-0001-17/whole-as-lost --marks PM-0001-17 --suite whole-as-lost` (48 clients):
  600 ticks, commands 28,752, late 0, client hashes 28,704, sentinels 0, **measured incomplete 48** (= 48 clients),
  rewinds refused 0. The header's commit is the worktree's HEAD (the code is uncommitted).

### Point 6: the dump reader pinned (done)

- logs.rs: the reader moved into `dump_events(world, &mut at) -> (events, shots, hits + kills)`, which `room_second`
  calls (no change of what it counts). Test `logs::tests::p6_the_dump_reader_counts_the_shots_hits_and_kills_simcore_step_returns`:
  a World stepped directly (SS, seed 3, 600 ticks, every soldier firing at one soldier of another team, Rewind None,
  an empty `HitHistory::for_ms(2_000)`): at every tick the reader's (shots, hits + kills) equal `SimCore::step`'s
  returned events; totals 13,990 shots, 8,830 hits, 2 kills (.gate/u4r/c2-p6-2.log; a first spread-fire version had
  0 kills, so the fire was concentrated). Plant (kind read one word off, `a + 2 + 4 * i`; one -x call,
  .gate/u4r/c2-p6-plant.log): FAILED by name at logs.rs:772 "tick 0: the reader's shots and hits plus kills, left
  (1, 2), right (48, 29)"; restored by Edit, sha256 77ad357da672c10db48ac76157456c73ca17592106b59c86ec4f85e0610c28fe
  before and after. (One runner call before it stopped at the runner's argument check, exit 2, before the lock: I had
  left out the worktree argument; nothing ran.)
- Report words due: these figures are counted only in logged runs (`--logs 0` counts none); SERIES.md:91
  (reliable_events_count) and :94 (shots_count) name npt-room as writer while npt-lab computes them from the World's
  dump into the room process's log; no row changed.

### Point 1 (1)-(3) and point 5's hash.rs half (done; .gate/u4r/c2-p15-1.log, -2.log)

- Point 1 (1): crates/lab/recordings.json unchanged from the first copy (three synthetic entries, each `"kind":
  "synthetic"`). (2): crates/lab/det-recordings.json (new): the two runner entries exactly as the notes give them,
  SS-P2-s1 then ES-P3-s2, `about` naming the ruling; h5 (`h5_the_committed_manifest_reads_and_names_three_generator_2_recordings`)
  asserts it: names [SS-P2-s1, ES-P3-s2], both kind runner, generator 0. (3): hash.rs `legs` untouched.
- Point 5, hash.rs `check`: after the length and SHA-256, `Recording::decode` of the bytes, and a refusal by name
  ("<name>: the recording's <field> is <x>, the manifest's <y>") for first_tick, span_ms or ticks; one test case each
  in `h5_a_regenerated_entry_matches_its_recorded_sha256_and_one_changed_byte_fails_naming_it`. hash tests 5 passed.
- fmt: 9 hunks (hash.rs 2, logs.rs 2, scenario.rs 4, pump.rs 1) applied by Edit; fmt exit 0; clippy -D warnings exit 0
  in both builds (21:38 UTC).

### Point 5: the ring pinned by behaviour (done)

- Test `scenario::tests::p5_the_room_s_ring_holds_room_history_ms_s_capacity_and_no_more` (a lab `Server` over a
  Loopback pair, slot 0 joined, the ring filled with C + 10 ticks, then two firing commands encoded with npt-proto,
  each stamped the next tick T, view ticks T - C and T - C - 1, C = `HitHistory::for_ms(ROOM_HISTORY_MS).capacity()`):
  "capacity 60 ticks; (view age, Rewind given, refused) [(60, Some(true), 0), (61, Some(false), 1)]"
  (.gate/u4r/c2-p5-1.log). So ROOM_HISTORY_MS is **pinned by test**. Plant (2,100; one -x call,
  .gate/u4r/c2-p5-plant.log): FAILED by name at scenario.rs:2036, left [(63, Some(false), 1), (64, Some(false), 1)],
  right [(63, Some(true), 0), (64, Some(false), 1)]; restored by Edit, sha256
  d3a0c89865cf2f37aabbd8bda50c9bdfdd05212a878f3f2340664bde24586703 before and after.

### Point 2: the radius pinned by placement (done)

- Test `scenario::tests::p2_the_mirror_s_radius_copy_is_the_room_s_by_placement` (helper `radius_pin`): slot 0's
  soldier at [700, -300, 0] (area 0); slots 20 and 21 teleported by `Room::script` through `check_placements` to
  exactly INTEREST_RADIUS_CM and one cm beyond it (the first direction that lands both exactly in slot 0's area: +x
  for both); slot 0 joins; its join FullState of tick 2, decoded by the mirror: "holds slot 20 true, slot 21 false (47
  soldiers in view)" (.gate/u4r/c2-p2-1.log). So the copy is **pinned by test**. Plant (6,300; one -x call,
  .gate/u4r/c2-p2-plant.log): FAILED by name at scenario.rs:2118, left (true, true), right (true, false) (both 7,000
  and 7,001 cm lie inside the room's 6,400); restored by Edit, sha256
  4a1edc12796cd93c69c1e06e93d7fed7ed3d2524fb2267830466ed4f6027e822 before and after.
- r4's report words (point 2): "spatial: the room's radius only, through a copy pinned by test; the area flood, doors,
  the wall and the events' radius-and-area test not checked, since npt-room exposes neither its radius nor its flood
  (room.rs:29; interest.rs:86-147); events checked only for no own cosmetic shot." Never "spatial passed".

### Regression after the rulings' changes (done; .gate/u4r/c2-rul-reg.log, c2-rul-fmt.log, 21:41-21:50 UTC, df 5.3G)

- clippy -D warnings exit 0 (npt-lab, npt-bots, npt-server, npt-log clean; workspace `--all-features`). `cargo test
  -p npt-lab -p npt-bots -p npt-server`: all pass, npt-lab bin **43 passed, 1 ignored**; workspace bites build for
  npt-lab and npt-bots: npt-lab bin **49 passed, 1 ignored**; every other crate's tests in that run pass. fmt: 3 hunks
  (the p2 test) applied by Edit, then fmt exit 0, clippy exit 0, p2 passes.
- Point 3's raw data (D13's logs, written before point 3's gate, so with -57's series; runs had no marks): P0 d13-5
  client 1,971,258 B, lab 44,436,766 B, room 3,518,423 B (1,800 ticks: 300 warm-up, 1,500 measured, 0 after); P2 d13-1
  1,976,863 / 16,882,755 / 3,648,994 B (1,833 ticks); P3 d13-4 1,937,608 / 7,299,589 / 3,649,341 B (1,835 ticks). Per
  measured tick and per 10-minute run (18,300 ticks): to compute from the measured-phase records only (the fact check
  gives P0 27,810, P2 12,005, P3 6,868 B per measured tick across the three logs); B1 needs r2's short B1 run.

## Copy 2: state and what is next (a fresh copy resumes here)

- `git status` holds the first copy's files plus copy 2's: crates/lab/src/replay.rs and precheck.rs (new), main.rs
  (`replay`, `precheck` dispatch), scenario.rs (fmt; `write_step` pub(crate); `Bites.pump`, `Bites.late_command`,
  LAB_BITES of two; `Server.decoder_bite`; check_placements' dead-code allows dropped), and npt-server's three files
  (pump.rs, schedule.rs, tests/schedule.rs: U4R-Q6 (a)'s items only). No plant in place; nothing committed. Clean at
  .gate/u4r/c2-b10-reg.log: fmt, clippy both builds, npt-lab/npt-bots tests both builds.
- Done in copy 2: item 1 (fmt), item 2 (B9 `replay`, r3 and its bite), item 3 (B10: the wallclock bite's npt-server
  items, precheck.rs, r6, r7, the late-command and wallclock bites shown); the rulings' points 3, 4, 6, 9 (a)-(b),
  1 (1)-(3), 5's hash.rs half and 2's counters (the table above). New files since the first copy's list:
  crates/lab/det-recordings.json, crates/lab/src/replay.rs, crates/lab/src/precheck.rs. Clean at 21:38 UTC
  (.gate/u4r/c2-p15-2.log: fmt, clippy both builds); the full npt-lab test suite last ran green at c2-b10-reg.log,
  before the rulings' changes (re-run it at the next block).
- **Copy 2 stopped here (21:51 UTC) at a clean point, its context mostly used:** no plant in place (the last three
  restored with their sha256 checked), no half edit, no run going; fmt, clippy and the npt-lab/npt-bots/npt-server
  suites green in both builds (c2-rul-reg.log, c2-rul-fmt.log). Rulings left: point 1 (4)-(7) and point 3's
  per-profile bytes (the table above).
- **Next, in order:** (1) done (point 5's ring pin: pinned); (2) done (point 2's radius pin: pinned); (3) done (the
  suites in both builds); (4) item 4, r2 (the contract test: U4R-Q4 (a)'s scope, U4R-Q11's four negative cases; brief 2.4 r2's (a) to (e)
  and the carries' figures) and the real judge by hand on its short-run logs, plus point 3's per-profile bytes per
  measured tick (P0 d13-5, P2 d13-1, P3 d13-4, B1 from r2's short B1 run) and a 10-minute run's size (18,300 ticks);
  r2's (b) needs item 6's SERIES.md rows (add `spatial_unchecked_count` to the class_audit names); (5) point 1
  (4)-(7) with r10's check.mjs half (carries 83 (b)-(d), 84, 85): `wasmStep`'s two options, one check.test.mjs case,
  ops/runs/det-u4.mjs, its one -x run (df >= 4 GB; non-cargo calls above 240 s: stop and report); (6) B12's run
  scripts and SERIES.md rows; (7) r5 and section 5's end checks twice.
- Copy 2's findings for the report: the depth's L - 1 against the header's L (B10 above); the `-p npt-server
  --all-features` feature-selection artefact (B10a); r3's bytes in the system temp folder in the test (B9); the
  pre-check writes no per-leg room log (B10).

## Copy 3

### Start checks (done)

- Started 2026-11-13 21:54 UTC, from the copy-3 prompt (the main folder's u4r-copy3-prompt.md, 34 lines, read whole).
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf, branch
  worktree-agent-a11e2354169f39acf; `git rev-parse HEAD`: 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef (holds).
- Brief sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046 (holds).
- Runner sha256 349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831 (holds).
- `df -h /home/user`: /dev/vda 252G size, 34G used, 5.3G avail, 87 %.
- Held for the advisor (no ruling yet): precheck.rs's outputs and its depth reading; neither changed.
- Read before any change: these notes whole (569 lines); `git status --short` (8 modified, 10 untracked under spikes/
  and this file); `git diff` of all 8 modified files; and each untracked file whole: bots/clippy.toml,
  bots/tests/behaviour.rs (236), lab/det-recordings.json, lab/precheck-recordings.json, lab/src/mirror.rs (469),
  logs.rs (786), scenario.rs (2,224), replay.rs (427), precheck.rs (777). Brief re-read: 2.3, 2.4 (r1-r10), 2.5-2.7,
  4, 5, 6, 7.1. The advisor's rulings file (sha256 476121ab..., holds) read: points 1-10 and their Builder lines.
- Seen in the read (a nit, fixed with the next scenario.rs edit): scenario.rs:1421-1422 doubles the doc's first line
  "The summary line of a run.".

### Item 4, r2: the design (stated for the report)

- npt-lab does not depend on npt-judge, and crates/lab/Cargo.toml is not my lane, so r2 has two halves. (1) A
  checker, crates/lab/src/contract.rs (`npt-lab contract --logs <run folder> --series marks/SERIES.md`, a subcommand
  beyond 2.7's list, named as such), for r2 (b) (every written name has its SERIES.md row), (c) (one `admit` per
  client with `in_roster` true, a footer on every log, `tick.v.late_count` on every room tick record: stronger than
  "where late-zero applies", which is PM-0001-30's only and no U4r run's) and (e) (the shot band), plus the carries'
  figures it can read from the logs; its unit tests carry U4R-Q11's four negative cases on doctored copies of one
  short run and are in check.mjs's count. (2) The per-suite short runs (48 clients, the release build, by hand through
  the runner, -x: each output a result): `scenario`, then `contract`, then the real judge (`npt-judge --marks
  marks/marks.json --logs <run> --only <ids> --out <run>-verdict.json`) for (a) and (d). Why not one Rust test: ten
  48-client suites cannot go into check.mjs's test step (npt-lab's bin tests already take 90.7 s in debug:
  c2-rul-reg.log:185).
- scenario.rs: the summary line now prints carry 49's figures (measured client sets, the share holding every very
  large unit, projectiles per set; "not counted (--logs 0)" on a no-log run); the doubled doc line fixed.
- Short runs' length: 10 s warm-up (300 ticks) and 20 s measured (600 ticks), plus the after phase; B1 longer (below).

### r2 suite 1: -17 L1 P0 (done; .gate/u4r/c3-r2-p0.log, -x, 21:58:56-21:59:20 UTC, df 5.3G)

- Release build of npt-lab and npt-judge (provenance: every npt-* crate "Compiling ... (<wt>/...)"), then `scenario
  --run u4r-r2-p0-1 --set u4r-r2 --scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20
  --marks PM-0001-2,PM-0001-17,PM-0001-57 --suite r2-p0` (logs .gate/u4r/r2/p0-s1/): 900 ticks (0 after), 12 s wall;
  lead 2; commands 43,152, steps without one 0, duplicates 0; late 0; press refused 0; buffer full 0; rewinds refused
  0; stale 0; overflow resyncs 0; client hashes 43,152, sentinels 0; **measured incomplete 0** (carry 62 (h)); shots 700
  a measured second; carry 49: 28,800 measured sets, all holding every very large unit (1.0000), 298.98 projectiles
  per set; audit 0 breaches.
- The judge (`--only PM-0001-2,PM-0001-17,PM-0001-57`, verdict .gate/u4r/r2/p0-s1-verdict.json): each mark NOT-RUN
  "1 of 3 valid runs" (a part needs 3 seeds); the run valid; **every check resolves, none a gap**: PM-0001-2's 11 SS
  checks all pass (48, 16, 48, 15, 4, 300, shots 700.8 in [665, 735], portal 1 in [0.95, 1.05], everyone in view 1);
  -17 pass (0 mismatches; 43,152 compared; 0 left incomplete); -57 pass (position max 6,981 µm, yaw 2,750 µdeg, pitch
  88,044 µdeg; position p99 5,638).

### r2's checker and SERIES.md's rows (written; tests not yet run)

- crates/lab/src/contract.rs (new; main.rs `contract` dispatch, exit 0 PASS, 1 FAIL with every failure by name, 2
  usage): `rows` (SERIES.md's first-cell names, `.x` shorthands joined), `written`, `check` ((b), (c), (e), carry 60
  as a failure; carries 62 (h), 74, 77 (c), 58, rewinds refused and -18's heals as lines), `read`, `main`. Tests:
  `r2_the_checker_passes_a_clean_run_and_fails_each_doctored_copy_by_name` (one SS P0 run, 16 clients, 6 s + 2 s;
  U4R-Q11's four cases on in-memory copies of its read-back logs: an admit gone, a footer gone, the
  `tick.s.snap_incomplete` row gone, one room tick's late_count gone) and `r2_rows_reads_each_first_cell_s_names`.
- First `contract` on suite 1's logs (.gate/u4r/c3-r2-contract1.log, -x, 22:01:32-22:01:44 UTC; clippy -p npt-lab
  -D warnings exit 0 first): (c) footers 3, admits 48 of 48, late_count on 900 room ticks (sum 0); (e) 700.80 in
  [665, 735]; carry 62 (h) 0; carry 74 lead 2, commands 43,152, late 0, steps without one 0; 77 (c) stale 0; rewinds
  refused 0; events queued 783,024; **(b) 43 of 73 written names had no SERIES.md row: FAIL, as expected before item
  6's rows**.
- marks/SERIES.md (U4R-Q2 (c), new rows only): a subsection "Names added with U4r" after U4j's (21 rows, in its `.x`
  shorthand), every name U4r writes that had none, on every kind of run (P0's 43 names, plus B1's burst_heal
  full_state and queue_max_count, burst_eviction baseline_age, counter_wrap counter, snap_refused kind, every_step,
  the recording event's four fields). No existing row changed. Not added: `header.params.burst_spacing_ns` already has
  its row ("npt-lab (L1)"), which U4r's runner does not write (-18's ws-practice rule, L2 parts; a fact for the report);
  an `overflow_resync` event carries only its identifier `slot`, so it writes no name.
- **Cut by the spend limit at 22:02:54 UTC** right after the SERIES.md edit (the coordinator's message; resumed
  00:16 UTC, df 5.3G). Checked on resuming: SERIES.md's diff is 31 lines as planned (one cell then tidied: its first
  cell named `event_queue_cap_count`, which already has a row, beside `event_queue_cap_provisional`); no plant in
  place; no cargo or npt-lab process running.

### r2's checker: PASS on suite 1, unit tests green (done; .gate/u4r/c3-r2-contract2.log, -3.log, 00:16-00:19 UTC)

- `contract` on suite 1 with the rows: **(b) 73 written, 0 without a row; contract: PASS** (exit 0).
- A first unit-test run failed by design of the test, not of the code: its 16-client run fired 232 shots a second,
  outside SS's band, which is the 48 soldiers' (c2-contract2.log). Changed: (e) applies only to a 48-client run
  (`FULL_CLIENTS`; PM-0001-2's parts select 48), saying "not applied (N clients ...)" otherwise; the test's run is
  now 48 clients (SS P0 seed 2, 6 s + 2 s), with a fifth doctored case (every measured second's shots halved: the
  (e) failure alone).
- c3-r2-contract3.log: clippy -p npt-lab -D warnings exit 0; `cargo test -p npt-lab --bin npt-lab contract`: **2
  passed** (19.67 s): clean (b) 70 written, 0 without a row; (c) 3 footers, 48 admits of 48, late_count on 240 room
  ticks (sum 0); (e) 696.00 in [665, 735]; "the five doctored copies each fail by name". One fmt hunk left there,
  applied by Edit after.

### r2 suites 2-10, the measured build (done; .gate/u4r/c3-r2-suites.log, -x -k, 00:19:52-00:21:54 UTC, df 5.2G)

- One runner call: release build of npt-lab and npt-judge, then per suite `scenario` (48 clients, seed 1, 10 s + 20 s
  unless named; logs .gate/u4r/r2/<suite>/), `contract`, `npt-judge --only` (verdict .gate/u4r/r2/<suite>-verdict.json).
  Every command exit 0; **contract: PASS on all nine** ((b) 0 names without a row; (c) 3 footers, 48 admits of 48,
  late_count on every room tick). -21's option-B suites: open-loop scripts stand in (my mapping, U4R-Q4 (a)): forged
  by `presses`, clamp by `clamp`, fast-clock and lag-switch by `steady`; their headers list PM-0001-2 only.

| Suite | Settings | Commands / late (share) / steps without | Incomplete (62 (h)) | Shots/s | Judge |
|---|---|---|---|---|---|
| p2 (-17/-57 L1 P2) | P2, tick 63,736 | 38,311 / 112 (0.29 %) / 6,194 | 585 | 700.45 | -2 all 11 pass; -17 pass (0 of 43,488; 1,066 left incomplete); -57 pass (6,974; 2,750; 88,061) |
| p3 (-17/-57 L1 P3; -20 P3 option B; 62 (g)) | P3, tick 63,736 | 35,169 / 73 (0.21 %) / 9,139 | 1,485 | 700.35 | -2 pass; -17 pass (0 of 41,851; 2,565); -57 pass (6,952; 2,750; 88,062) |
| b1 (-17/-57/-18 L1 B1) | B1, tick 63,736, 30 s measured, 2 bursts (5 s in, 15 s apart) | 48,159 / 297 (0.62 %) / 10,707 | 3,697 | 699.80 | **-2 FAIL: everyone in view 0.273333 (needs >= 0.99)**; so -17 and -18 L1 B1 **INVALID** (pre:PM-0001-2); -57 pass (6,974; 2,750; 88,060) |
| forged (-21 option B) | P0, presses | 43,152 / 0 / 0 | 0 | 700.80 | -2 all pass |
| clamp | P0, clamp | 43,152 / 0 / 0 | 0 | 700.10 | -2 all pass |
| fast-clock | P0, steady | 43,152 / 0 / 0 | 0 | 700.80 | -2 all pass |
| lag-switch | P1, steady | 42,771 / 222 (0.52 %) / 1,811 | 300 | 700.80 | -2 all pass |
| es (carry 59) | ES, P0 | 43,152 / 0 / 0 | 0 | 72.00 | **-2 FAIL: ES reliable events 50.05 a second (needs <= 10.5)**; the other 11 pass (shots 72, projectiles 32.38, spawns 21.1, everyone in view 1) |
| evstart (carry 75) | P0, tick 63,736, event start 65,533 | 43,152 / 0 / 0 | 0 | 700.80 | -2 pass; -17 pass (0 of 43,152; 0) |

- Every suite: stale 0, rewinds refused 0, press refused 0, buffer full 0, overflow resyncs 0, audit breaches 0; no
  run above 1 % late (carry 74). Carry 49: SS suites hold every very large unit in 1.0000 of measured client sets
  (B1 0.2811), 296-299 projectiles per set (B1 189.87); ES has none (0.0000; 32.38 projectiles per set). B1's -18:
  96 heal records, all 96 within 500 ms (48 clients x 2 bursts, each pair once), no full state during a burst; queue
  high-water mark 631. Each mark's overall verdict is NOT-RUN "1 of 3 valid runs" (a part needs 3 seeds): r2 reads
  the per-run checks.
- **Two suites failed: for the advisor (condition 5; never re-run into a pass).** (1) **B1**: after each 1 s burst
  every client's `everyone_in_view` flag goes to 0 for about 5 s and recovers over about 10 s (per 30-tick window from
  tick 64,186: 47, 0, 0, 0, 0, 0, 4, 12, 23, 31, 40, 44, 45, 46 slots holding everyone). Every client's baseline was
  evicted (`burst_eviction` evicted 1, baseline_age 0, all 96), the room then sends one datagram of about 1,000 B a
  tick per client (snap_datagram_bytes 47.9 KB for 48 per tick before, during and after), so the set the room means
  the client to hold is rebuilt from nothing over many ticks; the mirror records 0 breaches. So the share is
  npt-room's from-scratch recovery under its datagram budget, not the mirror's; at 10 bursts in 600 s it would leave
  every judged L1 B1 run INVALID for -17 and -18. Not mine to change (npt-room). (2) **ES**: the brawl aims at its
  target's exact position (mirror.rs `view`; bots `brawl_aim`), so about 70 % of its 72 shots a second hit: 50.05
  reliable events a second against ES's 9.5-10.5. The brawl is mine (npt-bots), but tuning it to a band is a
  design choice (carry 59; U4d-F's 7.250 figure) that comes to the advisor first: not changed.

### r2's bite suites: tick16 (carry 73) and -18's no-fallback (done; c3-r2-bites.log, -tick16b.log, -tick16c.log)

- Each in one -x -k call: `cargo build --release --locked -p npt-lab --features bites`, the run(s), `contract`, the
  judge, then `cargo build --release --locked -p npt-lab` again (rule 8: the measured binary back before any claim).
- **-18 heal bite** (c3-r2-bites.log, 00:23-00:24 UTC; .gate/u4r/r2/b1-nofallback/): b1's settings, `--role bite
  --bite PM-0001-18/no-fallback`: 96 heal records, **25 within 500 ms**, median 1,197,359,484 ns, **max
  1,688,286,323 ns**; full state during 71 bursts; overflow resyncs 71; sentinels 2,451; queue high-water 1,024;
  evictions 96 of 96 (baseline ages 0, 33, 34). The clean b1: 96 of 96 within 500 ms. So the bite fails -18's max
  check (> 500 ms) and the clean run holds one heal per client and burst, but **"no client holds a heal within 500
  ms" (2.4 r2) does not hold: 25 of 96 do** (the brief's expectation; for the advisor). The judge: -18 INVALID by
  pre:PM-0001-2 (as the clean b1: everyone in view).
- **tick16 past the wrap, first try** (c3-r2-bites.log; .gate/u4r/r2/tick16/, kept as that failure's evidence): SS
  P0 seed 1, `--tick-start 65506` (30 below the wrap), `--role bite --bite PM-0001-17/tick16`: the scenario **exit 1**,
  "the measured ticks were not all resolved within the after phase's 900 ticks (carry 62 (e))": past the wrap the
  bitten clients file their rebuilds under ticks the room never ran, so the real ticks never resolve; logs without
  footers; contract FAIL (3 footers); the judge: bite run INVALID (cut logs). Fixed in scenario.rs: **a bite run
  that reaches the after phase's cap ends there** (`Summary.after_capped`; `run_lengths.after_capped`, its SERIES.md
  row widened; the summary line says so): the stop decides each client's unresolved ticks inside its span lost and
  the logs end clean. A measured run still fails at the cap (carry 62 (e)).
- **Second try** (c3-r2-tick16b.log; tick16b/, `--after-max-s 5`): 1,050 ticks (150 after, capped); commands 1,532,
  steps without one 48,772; rewinds refused 37,820; overflow resyncs 140; client hashes 50,304 (sentinels 48,772);
  contract PASS (3 footers). The judge: still INVALID, now by **pre:PM-0001-2: everyone in view 0**. The mirror had
  **5,828 breaches**, the first "tick 65536: client 0: its baseline frame (age 1) is not mirrored": it decoded the
  wire's 16-bit tick with the bitten Quant and looked the baseline up by it. Fixed in mirror.rs: frames kept and
  baselines looked up by the room's tick (the datagrams and full states the room took at `tick`; equal to the wire's
  in a clean run).
- **Third try** (c3-r2-tick16c.log, 00:27:41-00:28:20 UTC; tick16c/): the mirror **0 breaches**; every count as the
  second try; carry 49: very large units in 0.0378 of sets, 12.33 projectiles per set; the judge: **still INVALID by
  pre:PM-0001-2, everyone in view 0** (every client, every measured tick: from tick 65,746 in the warm-up on). So the
  room's own sets collapse past the wrap under tick16 (its clients' acks and commands stop matching), not the
  mirror's. **For the advisor: carry 73's "FAIL with at least one mismatch, never INVALID" cannot hold while -17's
  L1 parts carry pre:PM-0001-2: a judged tick16 bite run, which crosses the wrap 1,800 ticks in, would be INVALID
  the same way** (marks.json and the judge are not mine).
- Clean P0 re-run after the mirror change (same call; .gate/u4r/r2/p0-s1b/): the summary equal to suite 1's figure
  for figure (fate log be408a87..., audit 92,920 / 2,623,643 / 12,720, 0 breaches, carry 49 1.0000 and 298.98).

### Point 3's per-profile bytes (done; read-only Python over the logs, 00:29 UTC)

- Measured bytes per measured tick (every record of the measured phase over the measured ticks), room / client /
  lab, and a 10-minute run (18,300 ticks) at that rate:

| Profile (run) | Measured ticks | Room | Client | Lab | Sum | 10-minute run |
|---|---|---|---|---|---|---|
| P0 (d13-5) | 1,500 | 1,953 | 1,096 | 24,768 | 27,817 | 509.1 MB |
| P2 (d13-1) | 1,500 | 1,993 | 1,082 | 8,936 | 12,011 | 219.8 MB |
| P3 (d13-4) | 1,500 | 1,995 | 1,062 | 3,818 | 6,875 | 125.8 MB |
| B1 (r2/b1, -57 listed) | 900 | 2,006 | 1,039 | 7,371 | 10,416 | 190.6 MB |
| P0 (r2/p0-s1, -57 listed) | 600 | 1,954 | 1,097 | 24,630 | 27,681 | 506.6 MB |
| P2 (r2/p2) | 600 | 1,996 | 1,083 | 8,592 | 11,671 | 213.6 MB |
| P3 (r2/p3) | 600 | 1,997 | 1,062 | 3,610 | 6,669 | 122.0 MB |

- D13's logs predate point 3's gate (their headers list no mark, yet they hold -57's series). A run whose header does
  not list -57 now writes a lab log of events only: r2/forged (P0) lab.jsonl 16,877 B whole, r2/evstart 16,748 B; its
  room and client logs are as above (about 1,760,000 and 985,000 B for 900 ticks). So a run without -57 is about
  3,050 B per measured tick, about 56 MB per 10-minute run, at every profile.
- .gate/u4r/r2/ holds 119 MB; df 5.1G.

### Regression after r2's changes (done; .gate/u4r/c3-reg.log, -x -k, 00:29:17-00:37:28 UTC, df 5.1G)

- Before the second ruling's changes: fmt exit 0; clippy -D warnings exit 0 (npt-lab, npt-bots, npt-server, npt-log
  clean; workspace `--all-features`); `cargo test -p npt-lab -p npt-bots`: npt-lab bin **45 passed, 1 ignored** (43 +
  r2's 2), lib 5, behaviour.rs 5; `cargo test --workspace --all-features -p npt-lab -p npt-bots` (the `--workspace`
  ran every crate): npt-lab bin **51 passed, 1 ignored** (49 + 2), every other crate's tests pass, exit 0.

## Rulings (advisor-u4r-build2)

Source: the main folder's .checkpoints/netcode-proof-test/advisor-u4r-build2-full.md (sha256 f4f5ac92..., 62 lines;
holds), section "The rulings" read whole at 00:31 UTC; each point's "Builder:" line is mine; it releases precheck.rs.
A regression run (.gate/u4r/c3-reg.log) had started at 00:30, before the message: it tests the code before these
changes; point 32 comes before the next full run.

| Point | What | State |
|---|---|---|
| 1 | depth: keep L on the pair line; leg lines: the relation, histogram after first takes, deviations (first 64); client lines: first-take depth; r6 asserts L - 1; verdict unchanged | open |
| 2 | no per-leg log; `--commit` required (+ parse test, exit 2); build line; recording line; leg room config (layer, profile, cmd_timing, lead, tick base, seed, ring); refused Rewinds per command (total + first 64) | open |
| 31 | `cargo test -p npt-server --all-features` E0425: no change; report's "known, not fixed" | report only |
| 32 | `scratch` under the test binary's target/<profile>/npt-lab-tests/, removed on pass (kept on panic); never touch /tmp/npt-lab-tests; re-run both builds | open |
| 33 | label every pre-commit log ("tree: 2fbec77 plus the uncommitted U4r changes; header.build.commit names 2fbec77; the binary is header.build.binary_sha256 <value>"); whole-as-lost's SHA and command lines | open (labels at the report) |
| 4 | (a) p2's `None` fails by name; (b) late-command test: result line exact, L1 off_stamp 0, Loopback late/rewinds/off_stamp 0 and rtt_void None; re-run in bites | open |
| 5 | (a) TickHash struct doc (lab/src/lib.rs:8) to its field doc's words; (b) `Bites::of`'s `any` takes `b.pump`; fmt, clippy both builds | open |
| Low finding | client lines print the first command tick's due instant beside its run instant (or a report line why equal) | open |

- Edits made (00:38-00:46 UTC; run pending): **32** scenario.rs `tests::Scratch` (a guard: Deref to PathBuf; Drop
  removes the folder unless `std::thread::panicking()`) and `scratch` rooted at current_exe's
  target/<profile>/npt-lab-tests/<name>-<pid>; `two_legs` returns `[Scratch; 2]` (the env var takes `as_os_str`);
  replay.rs `r3_run` returns the guards, the tests pass `to_path_buf()` and keep the guards to the end. /tmp/npt-lab-tests
  untouched. **4 (a)** p2's `None` arm panics "no direction lands both placements in slot 0's area: the copy is no
  longer pinned". **5 (a)** lab/src/lib.rs:8 TickHash's doc: "the recording both legs replay (its manifest name), the
  tick and the state hash" (nothing else in lib.rs); **(b)** `Bites::of`'s `any` takes `b.pump`. **1, 2, Low finding,
  4 (b)** precheck.rs: `ClientRecord.first_run_due_ns` (due_ns of its index from the leg's first tick) and
  `first_take_depth`; `LegOutcome.depth` over the takes after each client's first only, `depth_deviations` and the
  first 64 (client, tick, measured), `refused_logged` and the first 64 (client, tick, sequence) from
  `ClientSlot::rewinds_refused()` after each tick; the pair line "lead L; buffer_depth_ticks_count L (the pacing lead,
  the value set), both legs"; per leg three new lines (room configuration: layer, profile, cmd_timing B, lead, tick
  base, seed, ring ROOM_HISTORY_MS ms = HitHistory::for_ms capacity ticks; the depth relation with the input.rs:28-29
  words, the histogram and the deviations; the refused Rewinds per command); client lines gain "its due instant" and
  "measured depth at its first take"; `main` requires `--commit` (usage error 2 otherwise, before any file is read),
  prints the build line (`machine::build`: commit, binary sha256, features, profile; `machine::container()`: arch, cpu,
  vCPUs, MB, kernel) and the recording line (name, bytes, sha256 "as the manifest's", generator, first_tick, span_ms,
  ticks) before the pair's lines; USAGE names `--commit`; `verdict` unchanged. Tests: r6 asserts the histogram is {L -
  1} with 0 deviations on both legs, every first run instant equal to its due instant, and 0 refused Rewinds, and
  prints the first-take depths; the late-command test asserts the result line exactly, L1 off_stamp 0, and the
  Loopback leg's late, rewinds_refused, off_stamp 0 and rtt_void None (one more Loopback leg run), and prints the
  bitten leg's deviations; new `precheck_without_commit_is_a_usage_error`. The module doc names the record; the
  wallclock test's doc says "aim" (the site's field since 21:30).
- First run (.gate/u4r/c3-rul2.log, -x -k, 00:40:49-00:44:46 UTC): fmt 3 hunks (precheck.rs; applied by Edit after);
  clippy **exit 101**: `map_entry` ("usage of `contains_key` followed by `insert` on a `BTreeMap`", my leg-loop edit;
  fixed with the entry API); tests clean **45 passed, 1 failed** and bites **51 passed, 1 failed, 1 ignored**: the one
  failure both times **r6** at precheck.rs:840 (my new point 1 (4) assertion). Every other test passed, the p2 pin
  (Some branch, "holds slot 20 true, slot 21 false") and the late-command test's new exact asserts included
  ("result: VOID (L1 late_count 1)"; L1 off_stamp 0; the Loopback leg clean).
- **Finding (for the advisor; point 1's premise):** the histogram after each client's first take is still {0: 48, 1:
  5,520} on both legs: the 48 zeros are **not the first takes** (copy 2's reading, B10 above) but the takes of the
  recording's **last tick, 119, one per client** (the late-command run's deviation list: (0, 59, 0), (0, 60, 0), the
  bite's around T = 60, then (0..47, 119, 0)). Why: the recording holds no command stamped after its last tick, so at
  its last L - 1 takes the newest held command is the last tick itself (depth last - j). So point 1 (4)'s "every
  reading after each client's first take is L - 1, with no deviation" cannot hold at a recording's end under 7.2's
  pacing. Done: the leg lines report every deviation as point 1 (2) says (48 at tick 119 on a clean pair); r6 asserts
  the relation wherever it can hold: L - 1 at every take but the recording's last L - 1 ticks, there exactly (last
  tick - j), one take per client and tick, and no other deviation. `verdict` unchanged.
- Point 32 shown: after that run .gate/shared-target/debug/npt-lab-tests/ exists and is **empty** (every test's
  folder removed on its pass); /tmp/npt-lab-tests untouched.
- Second run (.gate/u4r/c3-rul2b.log, -x -k, 00:45:38-00:49:25 UTC): fmt exit 0; clippy -D warnings exit 0 (npt-lab,
  npt-bots, npt-server, npt-log; workspace `--all-features`); `cargo test -p npt-lab -p npt-bots`: npt-lab bin **46
  passed, 1 ignored** (45 + `precheck_without_commit_is_a_usage_error`), lib 5, behaviour.rs 5; `--features
  npt-lab/bites`: npt-lab bin **52 passed, 1 ignored**, lib 5, bots 5. r6 on both legs: "the measured depth at each
  client's first take {Some(1): 48}; after it {0: 48, 1: 5520}; not L - 1 only at the recording's last tick 119 (48
  takes)"; each first run instant equal to its due instant; 0 refused Rewinds. The late-command pair: L1 deviations
  50 (slot 0's (0, 59, 0) and (0, 60, 0), the bite's around T = 60, and the 48 at tick 119). p2's pin holds (slot 20
  true, slot 21 false). The scratch folder empty again (0 entries).
- **Report line (point 1 (6)):** under 7.2's pacing the room's measured depth (input.rs:28-29; :142-145: the newest
  buffered command's tick minus the tick just taken) is L - 1 at every take but the recording's last L - 1 ticks: r6,
  L = 2, both legs: every client's first take 1, then {1: 5,520} and 48 takes of 0 at tick 119, the recording's last.
- **Report line (the Low finding):** each client line prints its first command tick's due instant beside its run
  instant; r6 asserts them equal on both legs (run_leg steps the instants to `schedule.next_due_ns()`, with no
  catch-up at P0).

### Point 33's labels (every log I made before the code is committed)

- The label, for each: "tree: 2fbec77 plus the uncommitted U4r changes; header.build.commit names 2fbec77; the binary
  is header.build.binary_sha256 <value>". Builder evidence only; no committed record is drawn from them.
- Values (each log folder's room.jsonl header; .gate/u4r/): d13-1 to d13-5 17fc0627221676ef6c54414993c9f278fd2df6469b29dc9039d2b84f46fd0dc4;
  smoke1 f9dd14a9a0a2309f9795a93ebd529a70ef93e20d3ea840a12faeb6d54a8ed32d; relp2
  8e950384595f056b9cd2d21dab23078f7d314df24d166d807fa8fe2faf651d7f; r2/p0-s1
  f9302a9679d264ace36b5391460b45a80229d5369d0c8a250283b62842c416df; r2/p2, p3, b1, forged, clamp, fast-clock,
  lag-switch, es, evstart dd100e98407b2b9ccc910491ef94ceec6bb08b37d621577a2e240543039f782f; r2/p0-s1b
  9f127b0c6ffaba4231e6182effb47b95f640d04d253a9a425a87b347c2918f4a; bites builds: r2/tick16 and r2/b1-nofallback
  4f759317a6e92b473392a361df24e42e660727ecc3498cb8cce7c3e0ea009e58, r2/tick16b
  f8403304a545fd98c500ed4a05b2c8414916c948d22760fd720b08b24d95ace4, r2/tick16c
  a95d331becf6a1cc4de2b3468dceca2cdd5da9191e75653669cce2d99a56f1f2; and r2's verdict files beside them (each judges
  the folder of its name). The three recordings' runs (.gate/u4r/rec/, rec.log) kept no log (`--logs 0`), so no header:
  their label is "tree: 2fbec77 plus the uncommitted U4r changes; no header (--logs 0)", and their bytes are pinned by
  SHA-256 (det-recordings.json, precheck-recordings.json).
- **whole-as-lost** (.gate/u4r/whole-as-lost/, kept as it is): header.build.binary_sha256
  225e43338cfe360ee4d71cd052e3fe459ca59e848a790d91e36386b67ce0091b (features bites; a debug build). Its command lines
  (whole-as-lost.log:3 and :18): `cargo build --locked -p npt-lab --features bites`, then
  `<shared-target>/debug/npt-lab scenario --run u4r-whole-as-lost-1 --set u4r-short --date 2026-11-13 --commit
  2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef --out /home/user/Shamble-Frontlines/.gate/u4r/whole-as-lost --scene SS
  --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 10 --role bite --bite PM-0001-17/whole-as-lost
  --marks PM-0001-17 --suite whole-as-lost` (the main session re-makes it from the committed code with its own
  `--commit` and a new `--out`).

### Rulings (advisor-u4r-build2): states at copy 3's end

| Point | State |
|---|---|
| 1 | done (pair line, leg depth lines, client first-take depth, r6, verdict unchanged), with the finding: the deviations at the recording's last L - 1 ticks (for the advisor) |
| 2 | done (`--commit` required + test; build line; recording line; leg room configuration; refused Rewinds per command; no per-leg log) |
| 31 | report only ("known, not fixed") |
| 32 | done (shown: the folder empty after passing runs; both builds' suites re-run green) |
| 33 | done in these notes (labels above); the report repeats them |
| 4 | done ((a) and (b); re-run in the bites build: c3-rul2b.log) |
| 5 | done ((a) lib.rs:8-9; (b) `any` takes `b.pump`; fmt and clippy both builds exit 0) |
| Low finding | done (due instant printed beside the run instant; r6 asserts them equal) |

## Copy 3: state and what is next (a fresh copy resumes here)

- `git status` holds copies 1 and 2's files plus copy 3's: crates/lab/src/contract.rs (new); main.rs (`contract`
  dispatch); scenario.rs (summary line's carry 49 and after-cap figures, `share`, the bite run's after-phase cap,
  `Summary.after_capped`, `run_lengths.after_capped`, `Bites::of`'s `any` with `b.pump`, `tests::Scratch` and
  `scratch`, p2's `None` a failure); mirror.rs (frames by the room's tick); replay.rs (tests: the scratch guards);
  precheck.rs (rulings 2's points 1, 2, 4 (b) and the Low finding); lab/src/lib.rs:8-9 (TickHash's doc, carry 83 (a));
  marks/SERIES.md (22 rows in "Names added with U4r"). No plant in place; nothing committed; no run going. Clean at
  .gate/u4r/c3-rul2b.log (fmt, clippy both builds, npt-lab and npt-bots suites both builds).
- Done in copy 3: item 4 (r2: the checker and its tests, ten measured suites and three bite runs with the real judge,
  SERIES.md's rows, point 3's per-profile bytes), the second ruling whole, carry 83 (a).
- **Next, in order** (copy 2's items 5 to 7):
  1. Ruling 1's point 1 (4)-(7) with r10's check.mjs half: `wasmStep`'s two options (manifest path, default
     RECORDINGS_MANIFEST; bites half on/off, default on) and one check.test.mjs case (fake runner: a non-default
     manifest reaches every `npt-lab hash` call's `--manifest` and the loader's recordings; bites half off starts no
     bites-build call); carries 83 (b) (fixture cases: the step's clean half requires exit 0 from `npt-lab hash` and
     `npt-lab hash compare`), 83 (c) (the std-sin verdict's re-show and its failing case), 83 (d) (the npt-lab bin test
     of `hash compare` with a bite and no `--on` over two recordings, only the first listed armed), 84 (one shared 300 s
     deadline from the first non-cargo call; a stopped call takes every process it started; two self-tests of 84 (c)),
     85 (the deps step's refusals, DEPS_CHECKED's limits, the text beside `buildInputProblems`, one self-test per
     refusal; loader.mjs's header sentence :13-14); then ops/runs/det-u4.mjs (imports `wasmStep`, det-recordings.json,
     bites half off, its own .gate/ folder) and its one -x run (df >= 4 GB; non-cargo calls above 240 s: stop and
     report). Read check.mjs's `wasmStep` (:2254 on) and check.test.mjs first.
  2. B12: ops/runs/c-u4r-*.mjs (c-u2's model; binaries' paths from cargo's `--message-format=json`; the pre-check
     script passes `--commit` and writes precheck's lines to results/u4r-precheck-30/<commit12>/runs.txt; `--bytes`
     under .gate/; each npt-lab process timed); each npt-lab line run by hand on a short setting.
  3. r5 (seeds 1-3 per scene and profile, the slowest kept; 7.3's estimate redone), then section 5's end checks twice.
- **For the advisor (copy 3's findings):** (1) r2's B1 suite fails PM-0001-2's everyone-in-view (0.273), so every L1
  B1 run is INVALID for -17 and -18: npt-room's from-scratch recovery after each 1 s burst. (2) r2's ES suite fails
  ES's reliable-events band (50.05 against 9.5-10.5): the brawl hits about 70 % of its shots. (3) carry 73's tick16
  run is INVALID by pre:PM-0001-2 (everyone in view 0 past the wrap), even after the run-cap and mirror fixes: the
  room's own sets collapse. (4) -18's no-fallback bite: 25 of 96 heals within 500 ms (max 1.69 s), not "no client".
  (5) Point 1 (4)'s "no deviation" meets the recording's last L - 1 ticks (48 takes of 0 at tick 119). (6) r2's design:
  `npt-lab contract`, a subcommand beyond 2.7's list (the second ruling's point 5 left it to copy 3; the advisor or
  the review rules it).

## Copy 4

### Start checks (done)

- Started 2026-11-14 00:54 UTC, from the copy-4 prompt (the main folder's u4r-copy4-prompt.md, 35 lines, read whole).
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf, branch
  worktree-agent-a11e2354169f39acf; `git rev-parse HEAD`: 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef (holds).
- Brief sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046 (holds).
- Runner sha256 349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831 (holds).
- Rulings files: advisor-u4r-build1-full.md 476121ab... and advisor-u4r-build2-full.md f4f5ac92... (both hold);
  wip/u4r.patch 987946cc... (holds).
- `df -h /home/user`: /dev/vda 252G size, 34G used, 4.9G avail, 88 %.
- No cargo or npt-* process running at start.
- Held for the advisor's third ruling (u4r-build3-questions.md): r5 and section 5's end checks, the brawl, the after
  phase's cap for bite runs, the mirror's tick keying, contract.rs, precheck.rs's depth assertion, the -18 and tick16
  bite runs. None changed until a ruling reaches me by message.
- Read before any change (00:55-01:05 UTC): these notes whole (906 lines); `git status --short` (10 modified, 11
  untracked under spikes/ and this file); `git diff` of all 10 modified files (recordings.json, hash.rs, lab/lib.rs,
  main.rs, pump.rs, schedule.rs, tests/schedule.rs, SERIES.md, log/lib.rs, bots/lib.rs); each untracked file whole:
  bots/clippy.toml, bots/tests/behaviour.rs (236), lab/det-recordings.json, lab/precheck-recordings.json,
  lab/src/contract.rs (478), mirror.rs (473), logs.rs (786), replay.rs (429), precheck.rs (958), scenario.rs (2,291).
- Re-read: the brief's 2.3, 2.4 (r1-r10), 2.5-2.7, carries 82-85 word for word (:818-878), 4, 5, 6; ruling 1 point 1's
  Builder line; u4r-build3-questions.md (points 1-8, the held items); check.mjs `run` (:1851), the deps step
  (:1984-2096) and `wasmStep` (:2098-2464).

### Item 1 plan (check.mjs half and det-u4.mjs; each sub-block saved when done)

- C4a ruling 1 point 1 (4): `wasmStep`'s two options (manifest path, default RECORDINGS_MANIFEST; bites half, default
  on) + one check.test.mjs case (fake runner).
- C4b carry 84: one shared 300 s deadline from the first non-cargo call (each call given what is left; once one is
  stopped the rest not started, each named skipped); `run` stopping a timed-out call's whole process group; its two
  self-tests (fake runner and clock; real processes, a child that starts a grandchild, ~200 ms).
- C4c carry 83 (b): the clean half requires exit 0 from `npt-lab hash` and `npt-lab hash compare` (one fixture case
  each); (c) the std-sin re-show (the bitten native leg's first difference from a clean bites-build native leg is the
  committed catch, or FAIL when the catch is not on the first recording before HASH_MAP_ARM) with its failing case; (d)
  the npt-lab bin test (hash.rs) of `hash compare` with a bite and no `--on` over two recordings.
- C4d carry 85 (a)-(e): the deps step's refusals with one self-test each, DEPS_CHECKED's limits, the text beside
  `buildInputProblems`, loader.mjs's header sentence (:13-14).
- C4e ops/runs/det-u4.mjs and its one -x run (df >= 4 GB; non-cargo calls above 240 s: stop and report).
- Order taken: C4a-C4c, then C4e (det-u4.mjs needs C4a and inherits C4b and 83 (b)), then C4d (85 touches only the
  deps step), then 83 (d)'s Rust test.

### C4a, C4b, C4c (b) and (c): built, tests green (.gate/u4r/c4-self1.log, 01:04 UTC, df 4.9G)

- A probe first (`node -e`, no file written): Node 22.23.4's `spawnSync` with a 300 ms timeout returns at 305 ms
  (ETIMEDOUT, the child SIGTERM'd) while a grandchild holding stdout keeps running (state S); with `detached: true`
  the grandchild is in the child's new group (pgid = the child's pid), and `process.kill(-pid, "SIGKILL")` leaves it a
  zombie (dead, unreaped: state Z). The probe left two such zombies (pids 27807, 27821), reaped by their new parent.
- check.mjs `run` (now exported, for 84 (c)'s test): a call with a timeout gets `detached: true`, and on ETIMEDOUT its
  whole group is SIGKILLed (carry 84 (b); its doc names the means and two limits: a process leaving the group by its
  own setsid, and a Ctrl-C of check.mjs not reaching the call's own session).
- `wasmStep(runner, io, now, dir, { manifest = RECORDINGS_MANIFEST, bitesHalf = true } = {})` (point 1 (4)): the
  manifest reaches `io.read`, every `--manifest` and so the loader's entries; `bitesHalf: false` skips the two bites
  builds and step 6, and the summary says "hash-map: not run: the bites half is off (wasmStep's bitesHalf option)" (and
  std-sin the same). With the defaults the calls are as before.
- Carry 84 (a): one deadline, WASM_CALL_LIMIT_MS from the first non-cargo call; each call given what is left; once one
  is stopped (or the time spent) every later non-cargo call is not started and named "<label>: not started (skipped):
  <label> was stopped at the step's deadline (300 s shared by every call but its cargo builds, from the first of
  them)"; a stopped call's line names the seconds it was left.
- Carry 83 (b): `exited`: `npt-lab hash` and the clean `compare` must exit 0 ("<what>: exit N, not 0 (the clean half
  requires exit 0 as well as its lines)").
- Carry 83 (c): after the std-sin native-Wasm verdict, the re-show: `npt-lab-bites hash compare --ours
  <hash-map>/native-1.lines --theirs <std-sin>/native-1.lines --manifest .. --dir .. --theirs-dump
  <std-sin>/native-1.dump --ours-name "clean native leg 1" --theirs-name "std-sin native leg 1"` (no bite: compare's
  own replay of "ours" is a clean bites-build replay, hash.rs:1186-1193), which must first differ at the scan's
  (recording, tick, field); a catch not on entries[0] before HASH_MAP_ARM fails the step by name. The caught line adds
  "; re-shown against a clean bites-build native leg".
- check.test.mjs: `wasmWorld` takes `recs`, `manifestPath`, `options` and a per-answer `ms`; std-sin's answers follow
  the scan's armed catch (new CATCH_SCAN_SS, SS-s1 tick 2 as the committed scan). Re-pinned (planned with the change,
  section 4's expected-failure rule): the FX-7 test (now "...share one 300 s deadline...", limits 300,000 - 1,500 i, the
  stopped text) and the first test's std-sin catch path (SS-s1 tick 2, four calls with the re-show, plus 83 (c)'s two
  failing cases: ES-s2's catch and a catch at tick 1,234; and a re-show first differing at tick 3). New tests: "carry
  84 (c), the shared deadline with a fake runner and clock ..." (legs takes the whole 300 s: nothing starts after it,
  Wasm SS-s1/ES-s2/TC-s3 and npt-lab bites each named skipped; a call ending at the deadline: the next "skipped: the
  step's deadline had passed"); "carry 84 (b) and (c): run() stops a timed call with every process it started ..."
  (real processes, limit 200 ms; both pids gone or zombies within 2 s); "carry 83 (b): ..." (one case each); "wasmStep's
  two options ..." (det-recordings.json's two entries reach both hash calls and the loader; no bites-build call).
- Result: `node --test check.test.mjs` through the runner (shared): **75 of 75 pass** (base 71 + 4), exit 0, 1.7 s.
  sha256 now: check.mjs 48dde7c1..., check.test.mjs 5985da0e....
- Not changed: the std-sin NOT-caught cases in "the wasm step judges PM-0004-21's two bites" still use CATCH_SCAN
  (ES-s2), so they now fail for two reasons (NOT caught, and 83 (c)'s not-first); their asserted lines still appear.

### C4a-C4c's plants (done; each one -x call, restored by Edit, check.mjs sha256 48dde7c1... after each)

| Plant (check.mjs) | Log (.gate/u4r/) | Fails by name |
|---|---|---|
| 84 (b): no group kill (`&& false` on the kill) | c4-plant-84b.log | 74 of 75: "not ok 71 - carry 84 (b) and (c): run() stops a timed call ..." (grandchild 30867 left running, ended by its own 30 s) |
| 84 (a): per-call 300 s and no skip | c4-plant-84a.log | 73 of 75: tests 69 (the shared deadline's limits) and 70 (carry 84 (c), skipped calls) |
| run()'s timeout wiring dropped | c4-plant-84run.log | 74 of 75: test 71, "run() reports the timeout: {...status 0, timedOut false}" |
| 83 (b): `exited` a no-op | c4-plant-83b.log | 74 of 75: "not ok 72 - carry 83 (b) ..." |
| 83 (c): both re-show refusals off | c4-plant-83c.log | 74 of 75: "not ok 67 - the wasm step's builds, legs and comparisons, in order ..." |
| point 1 (4): the hash calls given RECORDINGS_MANIFEST | c4-plant-p14.log | 74 of 75: "not ok 73 - wasmStep's two options ..." |

### C4e: ops/runs/det-u4.mjs and its one run (done; .gate/u4r/det-u4.log, -x, 01:07:59-01:11:41 UTC)

- ops/runs/det-u4.mjs (new, ruling 1 point 1 (5)): imports check.mjs's `wasmStep`, `main` and FAILED_STEP_DIR; runs
  `wasmStep(run, WASM_IO, Date.now, <ROOT>/.gate/netcode-proof-test/det-u4, { manifest:
  "crates/lab/det-recordings.json", bitesHalf: false })` as main's one step "det-u4" (so it prints as main prints a
  step, keeps a failed run's whole output in FAILED_STEP_DIR/det-u4.log, and exits 0 only on PASS). `node --check`:
  ok. No self-test of its own (the options' test covers what it passes).
- Before: uptime 56 min, load 0.11; df 4.9G (above the 4 GB floor). One -x call: `cargo build --locked --release -p
  npt-lab`, `cargo build --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm` (provenance: every npt-*
  crate "Compiling ... (<wt>/spikes/netcode-proof/crates/...)", npt-sim twice), then `node ops/runs/det-u4.mjs`. exit 0.
- **The line, word for word:** "PASS  det-u4        200.5 s  native leg 1 against native leg 2: 18000 ticks compared, 0
  mismatching, equal (SS-P2-s1 9000, ES-P3-s2 9000); native leg 1 against Wasm leg 1: 18000 ticks compared, 0
  mismatching, equal (SS-P2-s1 9000, ES-P3-s2 9000); h2 equal; hash-map: not run: the bites half is off (wasmStep's
  bitesHalf option); std-sin: not run: the bites half is off (wasmStep's bitesHalf option); seconds: build npt-lab 8.5,
  build npt-hash-wasm.wasm 1.0, npt-lab hash 187.8, Wasm SS-P2-s1 2.3, Wasm ES-P3-s2 0.8, compare 0.0, h2 vector-SS 0.0,
  h2 vector-ES 0.0, h2 vector-CO 0.0, h2 vector-TC 0.0, h2 vectors 0.0", then "check: all 1 steps pass".
- **Figures:** the calls other than the cargo builds sum to **190.9 s** (187.8 + 2.3 + 0.8 + 0.0 x 7, at the line's
  0.1 s precision): under 240 s, so no stop (ruling 1 point 1 (6)). **W** (the two Wasm legs) = 2.3 + 0.8 = **3.1 s**.
  The step 200.5 s, warm (uptime 56 min; its two builds 9.5 s, rebuilt after the runner's touch). `npt-lab hash`'s
  187.8 s holds both runner regenerations (each a whole `scenario --record` re-run: point 9 (c)'s time after the
  no-log memory fix) and the native legs; the split is not printed. df after 4.9G.

### C4d: carry 85 (a)-(e) (built; .gate/u4r/c4-self3.log: 76 of 76, 01:16 UTC)

- Checked first (names only, no value read): no environment variable ends in _LINKER or starts with CARGO_PROFILE_,
  CARGO_HOME unset, no ~/.cargo/config(.toml); in crates/tcpinfo and crates/hash-wasm every `include` is in a comment;
  no `#[$` / `#![$` anywhere in crates/.
- (a) `cargoConfigProblems`: an env var ending in _LINKER ("<k> in the environment sets a linker (<v>): its arguments can
  rewrite the target_features section the Wasm loader reads") and a `linker` key in any table or form of a config
  ("<path> has a key `linker` (<v>): ..."); its doc list names both. DEPS_CHECKED's limits bullet and loader.mjs's
  header sentence (:11-16 now; it was :13-14) say what the target_features read gives: a compile-time feature shows and
  is refused, but the section is the linker's record (a RUSTC_WRAPPER's link arguments can rewrite it:
  sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check.
- (b) an env var starting CARGO_PROFILE_ ("... sets a profile key (<v>): it can drop npt-sim's -C overflow-checks=on,
  which plan §4.5 asks for in every profile") and a `profile` header, dotted key, key or quoted key in a config
  ("<path> has a `profile` key or table: ...").
- (c) `unsafeProblems`: in the deny-level crates `include` only directly before `!` and `(`, `[` or `{` (regex
  `\binclude\b(?!![([{])`; the message re-worded, FX-6's pinned text re-pinned with it); in every crate an attribute
  whose text starts with a metavariable (`#\s*!?\s*\[\s*\$` on lexRust's mask).
- (d) `cargoConfigDirs` (new export, split out of `cargoConfigFiles`): CARGO_HOME empty reads as unset (~/.cargo), a
  relative one resolved against `start`, the folder cargo runs in (cargo's home crate's rule).
- (e) `buildInputProblems`' doc: "Cargo gives rustc one source and drops the others ...: CARGO_ENCODED_RUSTFLAGS if
  set, even empty; else RUSTFLAGS if set, even empty; else the matching target tables' flags, if any; else
  build.rustflags."
- DEPS_CHECKED (the deps step's summary): "... names `include` but directly before `!` and an opening bracket; no
  attribute's text starts with a metavariable; ... no other rustc or workspace wrapper, no linker, no profile, no config
  include and no test runner ...".
- Tests: new "carry 85 (a), (b) and (d): ..." (one case per refusal and form, a no-false-positive config, the real
  `process.env` with the spike's .cargo/config.toml passing, CARGO_HOME unset/empty/relative/absolute); FX-6's test
  gains (c)'s cases (`m!(include!)` and `include ! (...)` refused at their lines, `include!{..}` and `include![..]`
  pass, two metavariable attributes refused at :6 and :12 of a fixture). Re-pinned: FX-6's include message, and
  C2R-8's config pin (:1037), whose `[target.x86_64-unknown-linux-gnu] linker = "cc"` passed and is now refused
  (now `rustflags = []` there). First run (c4-self2.log) failed only on those two, as planned plus my guessed line
  numbers (10 and 15, actual 6 and 12), fixed.
- sha256 now: check.mjs 7297ca9d..., check.test.mjs 24563ff3..., loader.mjs a0507d1e....
- The real step (.gate/u4r/c4-deps.log, -x, 01:17 UTC, df 4.9G): `node check.mjs --only deps,self`: "PASS  deps  4.1 s
  held: ..." (the container's environment and the spike's own config pass the new rules), "PASS  self  1.7 s  76 of 76
  tests passed", "check: all 2 steps pass".
- Plants (each one -x call, restored by Edit, check.mjs sha256 7297ca9d... after each): (a) the env rule matching
  `_LINKER_PLANT$` (c4-plant-85a.log): 75 of 76, "not ok 38 - carry 85 (a), (b) and (d) ..."; (b) the config profile
  refusal off (c4-plant-85b.log): the same test 38 fails; (c) the old `(?!\s*!)` include form (c4-plant-85c1.log): "not
  ok 56 - npt-tcpinfo and npt-hash-wasm, the deny-level exceptions ..."; (c) the metavariable pattern unmatchable
  (c4-plant-85c2.log): test 56 fails; (d) `env.CARGO_HOME ?? ...` (c4-plant-85d.log): test 38 fails at "an empty
  CARGO_HOME is unset".

### The container restart (about 01:18 UTC) and the re-check (01:22 UTC)

- The coordinator's message: the container restarted at about 01:18 UTC and stopped me; its snapshot re-cut at 01:21
  (wip/u4r.patch, 24 files, sha256 ad3631d9...). My re-check: HEAD 2fbec77; check.mjs 7297ca9d..., check.test.mjs
  24563ff3..., loader.mjs a0507d1e..., det-u4.mjs dcffd94d... (the notes' values after the last restore); no plant marker
  in check.mjs; no npt-lab, cargo or rustc process; df 4.9G. The last block (carry 85's plants) is whole in these notes.
- Next: carry 83 (d)'s Rust test, then B12. The third ruling still holds item 3 and its points.

### Carry 83 (d): the npt-lab bin test (done; .gate/u4r/c4-83d-1.log, c4-plant-83d.log, c4-83d-reg.log, 01:23-01:26 UTC)

- hash.rs tests, bites build: `r10_carry_83_d_compare_with_a_bite_and_no_on_arms_only_the_first_recording_listed`.
  Two synthetic TC recordings (TC-s8 = `short()`, TC-s9 seed 9, both 60 ticks from u32::MAX - 20) written with their
  manifest and one clean list of both into a scratch folder (scenario.rs `tests::scratch`, removed on pass); dumps at
  first tick + 10: TC-s8's with PM-0004-21/hash-map armed at first tick + 5, TC-s9's clean. `run(compare ... --ours-dump
  <dump> --digest-field bite[0].hash_map_order --bite PM-0004-21/hash-map --arm <t>)` with no `--on`, once per dump.
  Why it can see the arming: the bite's field is in a dump only once the bite is armed and its tick reached
  (sim/src/world.rs:2018-2020 and :2091-2097), and `--digest-field` replays the given dump's recording as compare
  arms it (hash.rs:1153-1170). Output (clean): "ours digest da456c9a16818d15 (bite[0].hash_map_order at words [344,
  345] of its dump at TC-s8 tick 4294967285)" and "ours digest none (bite[0].hash_map_order at words [] of its dump at
  TC-s9 tick 4294967285)"; 1 passed.
- Plant (-x, one call; compare's `armed` closure `.or(arm.as_ref())`, i.e. every recording armed; restored by Edit,
  hash.rs sha256 e590aa2a... before and after): FAILED at hash.rs:1744, left "ours digest layouts differ (342 and 340
  words) (... at words [340, 341] of its dump at TC-s9 ...)", right "... none ...". (The armed digest's value differs
  between processes, 37a9a0ce... there: the bite folds a process-seeded HashMap order; the test asserts its form only.)
- After: `cargo fmt --all --check` exit 0; `cargo clippy --locked -p npt-lab --all-targets -- -D warnings` exit 0, and
  with `--all-features` exit 0.
- **Item 1 (copy 3's state section) is done:** ruling 1 point 1 (4)-(7) and r10's check.mjs half (carries 83 (b)-(d),
  84, 85). Point 1 (7)'s report words: check.mjs's wasm step on the three synthetic recordings (its time comes with
  section 5's end checks, held), det-u4.mjs's 200.5 s (non-cargo 190.9 s, W 3.1 s) beside it, ruling 1's route (c).

### B12: ops/runs/c-u4r-runs.mjs (written, `node --check` ok; its lines by hand next)

- One script for every set (2.3 leaves the choice): `c-u4r-runs.mjs precheck --date D` (set u4r-precheck-30: SS P0
  seed 4's recording by `scenario --record` at precheck-recordings.json's entry settings, read from the file; the set
  stops if its "rewinds refused" is not 0 (7.1); then three `precheck` pairs: clean (measured build), wallclock and
  late-command on the L1 leg (bites build), each line "## pair <p> (wall s): result X, expected PASS|FAIL|VOID" plus
  the pair's lines whole; results/u4r-precheck-30/<commit12>/runs.txt, no verdict) and `c-u4r-runs.mjs judged --profile
  P0|P2|P3|B1 --date D` (set u4r-<p>: 3 measured runs, seeds 1-3, SS 48 L1, 10 s + 600 s, tick_start 63,736, marks
  PM-0001-2,-17,-57 (+ -18 on B1); then one bites-build run per bite of those marks from marks.json at run time, seed
  1, on P0 for -17's and -57's and B1 for -18's (ws-budget-x2 skipped: L2's), each header PM-0001-2 and its mark; then
  `npt-judge --only` on the set's folder; results/u4r/<commit12>/<p>/{verdict.json, runs.txt}).
- c-u2's rules kept: `--date`, refuses uncommitted tracked changes, never overwrites (logs or results), raw logs
  .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run: npt-judge reads every *.jsonl under --logs,
  recursively: judge/src/lib.rs:18-27), runs.txt with the commit, the three binaries' SHA-256s, the raw-log path, the
  judge's line and one line per run (npt-lab's summary + wall seconds). Builds' paths from cargo's
  `--message-format=json` through check.mjs's exported `builtArtifact` (no copy), copied out before the next build.
  A 4 GB free-space floor before every run (statfsSync).
- Choices to report: one profile per judged call (the judge's memory and the disk: ruling 1 point 3); bite runs per
  mark, one each, since the judge matches a bite run to any part it fits by id (judge/src/eval.rs:1386-1396; marks.json
  lists bites per mark); no `--bytes` (no part these sets judge reads a replay log). Held by the third ruling and built
  as the brief says: the B1 runs (point 1), the tick16 run's length and cap (points 3, 7 (a)), -18's no-fallback run
  (point 4); whole-as-lost runs once the main session adds it to -17's bites in marks.json (ruling 1 point 4).

### B12: the script's lines run by hand on short settings (done; .gate/u4r/c4-b12-1.log, -2.log, -3.log, 01:30-01:33 UTC)

- The script itself cannot run here (it refuses uncommitted tracked changes); each line form it issues, through the
  runner (-x -k, df 4.9G), binaries from the shared target's release folder, logs under .gate/u4r/b12/, `--commit`
  2fbec77... (point 33's label: tree 2fbec77 plus the uncommitted U4r changes; the headers' build.binary_sha256 name
  the binary). Every command exit 0.
  1. judged, measured (P0): `npt-lab scenario --run l1-p0-s1 --set u4r-P0 --date 2026-11-14 --commit <c> --out <d> --seed 1
     --scene SS --profile P0 --tick-start 63736 --warmup-s 10 --measured-s 20 (the script: 600) --suite u4r-P0 --role
     measured --marks PM-0001-2,PM-0001-17,PM-0001-57`: 900 ticks, late 0, rewinds refused 0, measured incomplete 0,
     shots 700 a measured second.
  2. judged, measured (B1, -18 listed): the same with `--profile B1 --measured-s 30 --bursts 2 --burst-first-s 5
     --burst-every-s 15` (the script: 600 s and its defaults 10, 30, 60) `--marks PM-0001-2,PM-0001-17,PM-0001-57,PM-0001-18`:
     1,233 ticks (33 after), heals 96, late 297, measured incomplete 3,697 (as r2's b1 suite).
  3. precheck's recording: `npt-lab scenario --run record-SS-P0-s4 --set u4r-precheck-30 ... --scene SS --profile P0
     --seed 4 --clients 48 --tick-start 0 --warmup-s 4 --measured-s 0 (the script: the entry's 10 and 290) --logs 0
     --suite record --record <file>`: 120 ticks, rewinds refused 0 (the line the script reads); 220,524 B, sha256
     6decf613....
  4. the judge: `npt-judge --marks marks/marks.json --logs <l1-p0-s1> --only PM-0001-2,PM-0001-17,PM-0001-57 --out ..`:
     each NOT-RUN "1 of 3 valid runs" (one seed).
  5. judged, bite (bites build): `... --run bite-client-rounding ... --role bite --bite PM-0001-17/client-rounding
     --marks PM-0001-2,PM-0001-17`: 900 ticks. Its judge line: with `--only PM-0001-17` alone (my first by-hand call,
     not the script's form) INVALID "pre:PM-0001-2: not in marks.json" (the precondition's mark not loaded); with the
     script's form `--only PM-0001-2,PM-0001-17` (c4-b12-2.log): the bite **shown**, -17 NOT-RUN "0 of 3 valid runs".
  6. precheck pairs on a scratch manifest for the short recording (.gate/u4r/b12/precheck-short.json, written by Write,
     never committed; the script passes crates/lab/precheck-recordings.json), `npt-lab precheck --manifest <m>
     --recording <file> --commit <c>` [`--bite PM-0001-30/wallclock --bite-leg L1` | `--bite PM-0001-30/late-command
     --bite-leg L1`]: clean (measured build, binary a8f19f7c...) **PASS (120 ticks, every World hash equal)**; wallclock
     (bites build be6593d3...) **FAIL (first differing index 3 ...; 117 of 120 differ)**; late-command **VOID (L1
     late_count 1)**. The measured build rebuilt last (section 4 rule 8).
- Not run by hand (held by the third ruling): the tick16 bite line (point 3) and -18's no-fallback line (point 4); the
  -57 bite lines (pos-4cm, yaw-12bit, pitch-8bit) have the bite line's form of item 5.

## Copy 4: state and what is next (a fresh copy resumes here; updated at each block)

- `git status` (26 entries) holds copies 1-3's files plus copy 4's: check.mjs, check.test.mjs,
  crates/hash-wasm/loader.mjs (header sentence only), crates/lab/src/hash.rs (83 (d)'s test), ops/runs/det-u4.mjs and
  ops/runs/c-u4r-runs.mjs (new). No plant in place (last restore: hash.rs e590aa2a... at 01:25 UTC); nothing committed;
  no run going; the measured release build is the last one built.
- Clean at: .gate/u4r/c4-reg.log (01:33-01:37 UTC: `cargo test --locked -p npt-lab -p npt-bots` exit 0, npt-lab bin 46
  passed, 1 ignored, lib 5, npt-bots 5; the same with `--features npt-lab/bites` exit 0, npt-lab bin 53 passed, 1
  ignored: 52 + 83 (d)'s test); c4-83d-reg.log (fmt exit 0, npt-lab clippy both builds exit 0); c4-self3.log and
  c4-deps.log (check.test.mjs 76 of 76; deps PASS on this container).
- Point 32: .gate/shared-target/debug/npt-lab-tests/ holds one folder, hash-83d-3588, the 83 (d) plant run's (a test
  that panics keeps its folder by design); /tmp/npt-lab-tests untouched.
- Done in copy 4: item 1 whole (ruling 1 point 1 (4)-(7); carries 83 (b)-(d), 84, 85; det-u4.mjs's one run PASS) and
  item 2 (B12: c-u4r-runs.mjs and its lines by hand).
- **Next (held for the advisor's third ruling, u4r-build3-questions.md):** item 3, r5 (seeds 1-3 per scene and profile,
  the slowest kept; 7.3's estimate redone) and section 5's end checks twice (check.mjs whole, each step passing, with
  section 4 rule 3's provenance builds in the same -x call), and whatever the ruling's points 1-8 change (the brawl,
  the bite runs' after-phase cap, the mirror's tick keying, contract.rs, precheck.rs's depth assertion, the -18 and
  tick16 bite runs, and c-u4r-runs.mjs's B1, tick16 and -18 lines with them). Nothing else is open.
- check.mjs's names newly exported for its self-tests (2.7's count): `run`, `cargoConfigDirs`. The new scripts export
  nothing.

## Copy 5

### Start checks (done)

- Started 2026-11-14 01:41 UTC, from the copy-5 prompt (the main folder's u4r-copy5-prompt.md, 34 lines, read whole).
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf, branch
  worktree-agent-a11e2354169f39acf; `git rev-parse HEAD`: 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef (holds).
- Brief sha256 59d570b8a73fa6d9ab6b9a02e9f6d6cf6ec37a57d36d2c3ef3285bc8c5196046 (holds).
- Runner sha256 349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831 (holds).
- Rulings: advisor-u4r-build1-full.md 476121ab..., -build2-full.md f4f5ac92..., -build3-full.md
  78ebb5af7d48a035cc5260436cb5a92dc0636ad19624a8ca2647e5d4c0490862 (all hold); wip/u4r.patch e40449a7... (holds).
- `df -h /home/user`: /dev/vda 252G size, 34G used, 4.9G avail, 88 %.
- No cargo, rustc or npt-* process running at start. `git status --short`: 13 modified, 13 untracked (this file
  included), as copy 4's state section lists.
- Read before any change (01:42-01:58 UTC): these notes whole (1,171 lines); `git diff` of all 13 modified files
  (SERIES.md, recordings.json, lab/lib.rs, main.rs, pump.rs, schedule.rs, tests/schedule.rs, loader.mjs, check.mjs,
  check.test.mjs, bots/lib.rs, log/lib.rs, hash.rs); each untracked file whole: bots/clippy.toml,
  bots/tests/behaviour.rs (236), lab/det-recordings.json, lab/precheck-recordings.json, lab/src/mirror.rs (473),
  contract.rs (478), precheck.rs (958), scenario.rs (2,291), logs.rs (786), replay.rs (429), ops/runs/c-u4r-runs.mjs
  (197), ops/runs/det-u4.mjs (36). Also room.rs:205-245 and :1155-1175, ack.rs:95-213 (`stale()` filters on
  `Some(s.tick) == last_acked`: so pinning the mirror's frame at `last_acked()` is exactly the stale copy's frame).

## Rulings (advisor-u4r-build3)

Source: the main folder's .checkpoints/netcode-proof-test/advisor-u4r-build3-full.md (sha256 78ebb5af..., 235 lines),
section "The rulings" (:9-71) read whole at 01:42 UTC; each point's "Builder:" line is mine. Work order (the prompt):
(1) code: 8 (A), 8 (B), 8 (D), 6 (1)-(3), 5's two precheck.rs changes, then fmt, clippy, npt-lab tests both builds;
(2) runs once each: 3's tick16 bite and its clean partner, 8 (A)'s no-fallback bite and the clean P0 equality;
(3) 6 (4) c-u4r-r2.mjs, its npt-lab lines by hand; (4) report words 8 (C), (E), (F), points 1, 2, 4, 7; (5) r5, then
section 5's end checks twice.

| Point | What | State |
|---|---|---|
| 1 | B1: no code, no re-run; report words (FAIL under condition 5, three cause lines, window means, 0.919, U4g's) | report only |
| 2 | ES: no code, no re-run; report words (50.05 vs 9.5-10.5 beside 7.250; 68.5-69.5 % hits; U4h's) | report only |
| 3 | tick16 bite at --tick-start 64651 (bites build) and clean partner at 64936 (measured build), each once | both run once (c5-p3.log); clean partner as expected; the bite run as expected but `after_capped` false: stopped, to the advisor |
| 4 | -18 bite: no code; report words (25 fast heals counted apart: 14 burst 0 age 0, 11 burst 1 age 33/34; 71 slow after full state; 2,314 breaches; 0.48) | report only |
| 5 | precheck.rs: L from the leg's configured lead (not `let lead = 2u32`); whole deviation list or count <= 64 asserted | done (c5-3.log) |
| 6 | contract.rs (1) class_audit breaches fail (but owner-to-all) + 6th doctored case; (2) module doc; (3) unreadable input own exit code in USAGE; (4) c-u4r-r2.mjs | (1)-(3) done (c5-3.log); (4) script written, its lines run by hand (c5-r2lines.log; b1 and es not re-run, by points 1 and 2) |
| 7 | after cap and mirror tick keying accepted: no change; report words | report only |
| 8 | (A) mirror keeps the frame at last_acked() beyond FRAMES_KEPT + its bite run and clean P0 equality; (B) baseline_age row; (C) labels regen/, regen-all/; (D) burst_spacing_ns in every B1 header; (E), (F) report words | (A) done, both runs as expected (c5-p8a.log); (B), (D) done; (C), (E), (F) report words written (item (4) block) |

### Work item (1): the third ruling's code changes (written 01:44-01:49 UTC)

- **8 (A)** mirror.rs: `ClientMirror::keep(tick, frame, pin)` keeps the newest FRAMES_KEPT frames whose tick is not
  `pin`, plus the frame of `pin`; `Mirror::tick` gains `last_acked: Option<u32>` (the room's
  `ClientSlot::last_acked()` for that slot after the tick; `#[allow(clippy::too_many_arguments)]` with its reason);
  FRAMES_KEPT's doc says why (ack.rs `stale()` returns the stale copy only while its tick is `last_acked`, so the pin
  is exactly the stale copy's frame). scenario.rs `run` passes `server.room.client(slot).and_then(|c| c.last_acked())`;
  the p2 test's call passes the same for slot 0. The early return on a missing baseline is unchanged.
- **8 (B)** SERIES.md's `event.burst_eviction.baseline_age` row: "...at the first tick after the burst; 0: coded
  against the per-type defaults (no baseline); -1: no snapshot sent that tick".
- **8 (D)** scenario.rs: `Settings.burst_spacing_ns` (`--burst-every-s` x 10^9, saturating; 0 off B1), and `params`
  writes `burst_spacing_ns` beside `burst_starts_ns` on every run with bursts (so in all three logs' headers).
- **6 (1)** contract.rs `audit`: the lab log's `class_audit` `breaches_count` printed on every run ("class audit:
  breaches_count N (the first: ...)"); a failure when the event or the count is missing, or the count is above 0,
  unless the header's bite is npt-room's class-filter bite. **Choice (for the report):** the id is not spelled in
  npt-lab's sources (brief section 4: "npt-lab's sources never spell another crate's bite id as a string literal"):
  `room_class_filter_bite` takes it from `npt_room::BITES` at run time, the one id there outside the `PM-` form
  (room/src/lib.rs:32-46 documents it so); a measured build holds no list, so its contract never exempts. Test: a
  sixth doctored case (breaches_count 1 added: fails by its exact text), and in the bites build the exemption for
  `catalogue-5/owner-to-all` (the test module may spell ids) and a failure for another bite's breaches.
- **6 (2)** the module doc now says carry 60's missing params are a failure, names the class audit, and lists the
  exit codes.
- **6 (3)** `UNREADABLE` = 3: a `--series` file or a log that cannot be opened is exit 3, "cannot read ...", on
  stderr (main.rs), never "contract: FAIL"; a log that npt-log's reader refuses stays a FAIL (exit 1, "FAIL (c) <path>
  does not read back as a log"). USAGE names the four codes. Test
  `r2_an_input_that_cannot_be_read_is_exit_3_never_the_contract_s_fail` (empty folder; missing SERIES file).
- **5** precheck.rs: `LegOutcome.lead` (the leg's configured lead, set by `run_leg`); r6 passes P0's `lead_for` and
  asserts `o.lead` is it, takes L from `o.lead`, asserts `depth_deviations <= 64` and the kept list's length equals
  it before comparing, and builds the end set from the records' slots (not 0..48). The leg lines of `pair` print the
  leg's own `o.lead` (room configuration and depth lines), so the field is read outside the tests too.
- Runs: .gate/u4r/c5-1.log (-k, shared, 01:49:20-01:53:04 UTC, df 4.9G): fmt exit 1 (2 hunks, contract.rs), clippy
  exit 101 both builds ("field `lead` is never read": fixed as above); `cargo test --locked -p npt-lab -p npt-bots`
  exit 0, npt-lab bin **47 passed, 1 ignored** (46 + the exit-3 test), lib 5, behaviour.rs 5; with `--features
  npt-lab/bites` exit 0, npt-lab bin **54 passed, 1 ignored** (53 + 1). Then the 2 fmt hunks applied by Edit and the
  lead lines changed; c5-2.log (01:53:35-01:53:37): fmt exit 0, clippy `-p npt-lab -p npt-bots --all-targets -D
  warnings` exit 0 and with `--all-features` exit 0. Tests re-run on that final code: c5-3.log (01:53:40-01:57:27,
  runner exit 0): clean npt-lab bin **47 passed, 1 ignored**, lib 5, bots 5; bites npt-lab bin **54 passed, 1
  ignored**, lib 5, bots 5. **Item (1) done.**
- ops/runs/c-u4r-r2.mjs written (work item 3's file; `node --check` ok): its lines are run by hand after item (2).

### Work item (2), point 3's pair: run once each (.gate/u4r/c5-p3.log, -x -k, 01:57:39-01:58:27 UTC, df 4.9G, up 38 min)

- One call: `cargo build --release --locked -p npt-judge`, `... -p npt-lab --features bites`, then the bite run's three
  lines, then `cargo build --release --locked -p npt-lab` (the measured build, rule 8), then the clean partner's three
  lines. Every command exit 0. Label (point 33): tree 2fbec77 plus the uncommitted U4r changes; header.build.commit
  names 2fbec77; the bite run's binary dba6232ed87f59640eda5be82206d738e80954461a19098ba7f8bfbfb1c733f0 (features
  bites), the clean run's 76fd20903636bfea85298168f23934f4f6b8410943b7e996d53edf555e1115d2. Verdicts: tick16d-verdict.json
  76291cbc..., tick16-clean-verdict.json d774ff87....
- **(1) the tick16 bite run** (`--run u4r-r2-tick16-4 ... --out .gate/u4r/r2/tick16d --scene SS --profile P0 --seed 1
  --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16 --marks
  PM-0001-2,PM-0001-17 --suite r2-tick16`): header ticks_below_wrap **885**; measured ticks 64,951-65,550, the wrap at
  measured tick 585. Summary: 946 ticks (**46 after**, no cap note); commands 42,480; late 0; rewinds refused 48;
  overflow resyncs 48; client hashes 45,360 (sentinels 2,880); measured incomplete 720 (= 48 x the 15 post-wrap
  measured ticks 65,536-65,550); decided lost at the stop 1,488; shots 700 a measured second; carry 49 1.0000, 298.97;
  **audit 0 breaches**. contract: **PASS** ("class audit: breaches_count 0"; (e) 700.80 in). The judge: PM-0001-17's
  bite PM-0001-17/tick16 **shown**: run valid **true** (so pre:PM-0001-2 held), check "100 % of decoded snapshots
  rebuild..." **fail, value 2,880, needs == 0** (>= 48 as expected). Read-only from room.jsonl: every slot holds
  everyone on 600 of 600 measured ticks (share 1.0); the flag sum is 48 from 64,652 to 65,566, 0 from **65,567** (after
  phase) to 65,595, 48 again from 65,596; the 48 overflow_resync events all at tick **65,596**; run_lengths
  `after_capped` **false**, after_ticks_count 46.
- **Differs from the ruling's expectation in one item: `after_capped` is false, not true.** Everything else holds
  (PM-0001-2 valid, -17 FAIL with 2,880 >= 48, 0 mirror breaches, contract PASS). Why (from the logs): the room keeps the
  pre-wrap baselines until about wrap + 31 (65,567, as tick16c's 65,566 + 1), then at 65,596 (wrap + 60) sends each
  bitten client a full state by overflow resync, and that resolves the 15 post-wrap measured ticks as lost, so every
  measured tick is resolved 46 ticks into the after phase, before the 150-tick cap. The run ends by carry 62 (e)'s
  normal path, not by the bite run's cap. **Stopped here (condition 5; the prompt's rule): not re-run; to the main
  session and the advisor.** Open question for them: is the expectation "after_capped is true" to be read as not
  required at this setting (the cap path is not reached because the overflow resync resolves the measured ticks), and
  does c-u4r-r2.mjs's tick16 `expect` text (it says "after_capped true") change with it?
- **(2) the clean partner** (`--run u4r-r2-tick16-clean-1 ... --out .gate/u4r/r2/tick16-clean ... --tick-start 64936
  --warmup-s 10 --measured-s 20 --after-max-s 5 --marks PM-0001-2,PM-0001-17,PM-0001-57 --suite r2-tick16-clean`, no
  bite; the wrap at measured tick 300): 900 ticks (**0 after**: resolved without the cap); commands 43,152; late 0;
  rewinds refused 0; client hashes 43,152 (sentinels 0); measured incomplete 0; **0 breaches**; contract PASS. The
  judge, per run (valid true): PM-0001-2 all 11 SS checks pass (48, 16, 48, 15, 4, 300, 700.8 in [665, 735], portal 1
  in [0.95, 1.05], everyone in view 1 >= 0.99); -17 pass (0 == 0); -57 pass (6,981; 2,750; 88,044). **As expected on
  every item**, and it shows ruling 3's point 7 (b) (frames by the room's tick) across the wrap.

### The coordinator's message after the stop (about 02:02 UTC)

- "Your stop was right." P3-CAP and CR-ID are with the advisor (u4r-build4-questions.md); its ruling comes by
  message. Meanwhile, in order: (1) point 8 (A)'s two runs, once each; (2) two c-u4r-runs.mjs items read as ruling 3's
  own words: the judged tick16 bite's tick_start = 65,551 - (warm-up + measured ticks) from its own lengths (point 3,
  main session item 4), and no -18 no-fallback bite on B1 at U4r (point 1), said in the script's text; (3)
  c-u4r-r2.mjs's lines run by hand, its tick16 expected-outcome text left as it is until the ruling; (4) the report
  words 8 (C), (E), (F) and points 1, 2, 4, 7. **Hold r5 and section 5's end checks until the ruling** (CR-ID may
  change code). Snapshot re-cut 02:01 UTC: wip/u4r.patch, 26 files, sha256 707cc3df3c409d5fd3e21249567acc1b54a67f5833f2ee4df0abcb383a4bdaa5.

### Point 8 (A)'s two runs, once each (done; .gate/u4r/c5-p8a.log, -x -k, 02:02:22-02:03:08 UTC, df 4.8G)

- One call: bites build, the bitten run's three lines, measured build, the clean P0 run's three lines; every command
  exit 0. Labels (point 33): tree 2fbec77 plus the uncommitted U4r changes; header.build.commit names 2fbec77.
- **The no-fallback bite run** (`--run u4r-r2-b1-nofallback-1 --set u4r-r2 --date 2026-11-14 --commit 2fbec77... --out
  .gate/u4r/r2/b1-nofallback2 --scene SS --profile B1 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 30 --bursts
  2 --burst-first-s 5 --burst-every-s 15 --role bite --bite PM-0001-18/no-fallback --marks
  PM-0001-2,PM-0001-17,PM-0001-57,PM-0001-18 --suite r2-b1`): 1,233 ticks (33 after); commands 47,990; late 309
  (0.64 %); rewinds refused 0; overflow resyncs 71; heals 96; client hashes 54,500 (sentinels 2,631); measured
  incomplete 3,957; carry 49 0.8236, 274.41; **audit 0 breaches** (copy 3's r2/b1-nofallback: 2,314). contract
  **PASS** ("class audit: breaches_count 0"; "-18: 96 heal records (25 within 500 ms; 96 (client, burst) pairs of 48
  clients x 2 bursts; a full state during 71); carry 58: queue high-water mark 1023"). Heals (read-only, lab.jsonl):
  96, 25 within 500 ms, **max 1,688,286,323 ns** (> 500,000,000: -18's max check fails), median 1,376,834,974 ns. The
  judge: PM-0001-18 **INVALID**, "bite PM-0001-18/no-fallback: its run is invalid", invalid_why "pre:PM-0001-2:
  everyone in view on at least 99 % of each client's snapshots = **0.453333**, needs >= 0.99" (the worst slot 408 of
  900 measured ticks); an invalid run lists no checks, so the max FAIL is read from the heal records above. **As the
  ruling expects** (0 breaches; INVALID by pre:PM-0001-2 while the room is unfixed, reported): no stop. The header
  holds `burst_spacing_ns` **15,000,000,000** (8 (D) shown) beside burst_starts_ns [15,029,325,961, 30,011,005,135].
  Fact for the report: with the mirror following the stale baseline, the bots' view changes (their view is the
  mirror's frame), so the bitten run's commands differ from copy 3's (its median heal was 1,197,359,484); the max is
  the same 1,688,286,323. The PM-0001-2 value is now 0.453333, not copy 3's 0.48 (which the broken mirror gave).
- **The clean P0 run** (`--run u4r-r2-p0-1 --set u4r-r2 --date 2026-11-13 --commit 2fbec77... --out .gate/u4r/r2/p0-s1c
  --scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20 --marks PM-0001-2,PM-0001-17,PM-0001-57
  --suite r2-p0`; its run id and date as r2/p0-s1's): 900 ticks (0 after), 0 breaches, contract PASS, the judge 1 of 3
  valid runs each. **Equal record for record** (read-only Python, every line compared): against r2/p0-s1b every record
  line of all three logs equal (980 / 901 / 952 lines); against r2/p0-s1 every line equal but lab.jsonl:950, run_lengths'
  added `after_capped: false`. Headers: equal but build.binary_sha256 (the new binary), machine.kernel and
  machine.mem_mb (the container restart at about 01:18) and, against p0-s1b only, the run id (u4r-r2-p0-2 there).

### c-u4r-runs.mjs: ruling 3's two script items (done, 02:06 UTC; `node --check` ok)

- **Ruling 3, point 3 (main session item 4):** `TICK16_START = 65_536 + 15 - (WARMUP_S + MEASURED_S) * TICK_HZ` (47,251
  for 300 + 18,000: the wrap at measured tick 17,985 of 18,000, 15 before the end; `node -e` arithmetic); `one()` takes
  a tick start, the tick16 bite run gets TICK16_START, every other run (clean, client-rounding, -57's) TICK_START
  63,736. The header text and TICK_START's doc say so (U4R-Q7 (a) as ruling 3 reads it). The short form of this line
  (10 s + 20 s: 65,551 - 900 = 64,651) is point 3's run above, so the line form is run by hand.
- **Ruling 3, point 1:** `NOT_AT_U4R = ["PM-0001-18/no-fallback"]` filters the bite loop and `bitesOn` gives -18 no
  profile, so a B1 set runs no bite; the header text says -18's bitten judged run moves to U4g, the three B1 measured
  runs stay for -57's L1 B1 part, and their -17 and -18 verdicts are recorded INVALID, never counted.
- 7.3's table changes with it (one -18 bite run fewer, 600 s): redone with r5, held.

### Item (3): c-u4r-r2.mjs's npt-lab lines run by hand (done; .gate/u4r/c5-r2lines.log, -x -k, 02:07-02:09 UTC, df 4.8G)

- The script refuses uncommitted tracked changes, so its lines go through the runner as copy 4 did for
  c-u4r-runs.mjs: `cargo build --release --locked -p npt-lab -p npt-judge`, then per suite the script's scenario line
  (its run id, `--set u4r-r2 --date 2026-11-14 --commit 2fbec77...`, `--out .gate/u4r/r2c5/<key>`, its args, marks and
  suite), `npt-lab contract` on the folder (flags given as `--series marks/SERIES.md --logs <dir>`: the script's
  order is `--logs` first; the parser reads either order) and `npt-judge --marks marks/marks.json --logs <dir> --only
  <its marks> --out .gate/u4r/r2c5/<key>-verdict.json`. Every command exit 0.
- **Run here:** p2, p3, forged, clamp, fast-clock, lag-switch, evstart. Each summary equal to copy 3's figure for
  figure (p2 38,311 / late 112 / 6,194 / incomplete 585; p3 35,169 / 73 / 9,139 / 1,485; lag-switch 42,771 / 222 /
  1,811 / 300; the P0 four 43,152 / 0 / 0 / 0); every contract PASS with "class audit: breaches_count 0"; per run
  (read-only over the verdicts): PM-0001-2's 11 SS checks pass on all seven, -17 passes on p2, p3, evstart, -57's three
  pass on p2 and p3; every run valid.
- **Run before, with the script's own lines:** p0 (r2/p0-s1c, but its --date 2026-11-13, kept for the record-for-record
  comparison), tick16 (r2/tick16d), tick16-clean (r2/tick16-clean), b1-nofallback (r2/b1-nofallback2): the blocks above.
- **Not run, by ruling:** b1 (ruling 3, point 1: "do not re-run the b1 suite") and es (point 2: "do not re-run the es
  suite"). Their line forms are shown: b1's is b1-nofallback's without `--role bite --bite ...` (the same flags ran
  above in the bites build), es's is the P0 form with `--scene ES` (copy 3's c3-r2-suites.log:132). The main session's
  run from the merge commit makes both, expected to fail there as here (point 6's main-session item 2).
- Not run by hand: the script's three `cargo build --release --locked ... --message-format=json` lines with
  builtArtifact and the copy out (the same function and form as c-u4r-runs.mjs's, which copy 4's B12 used), and
  r2.txt's writing. Labels (point 33): tree 2fbec77 plus the uncommitted U4r changes; header.build.commit names 2fbec77.
  .gate/u4r/r2c5/ holds 31 MB.

### Item (4): the report words (done here; the report repeats them)

- **8 (C), labels:** .gate/u4r/regen/ and .gate/u4r/regen-all/ (each holds out/ and rec/: `npt-lab hash`'s
  regenerations, copy 1's regen-precheck.log and regen-all.log): "tree: 2fbec77 plus the uncommitted U4r changes; no
  header".
- **8 (E), corrections:** "Names added with U4r" holds **21** rows (counted at the head: awk over the subsection's
  table rows), not 22 as copy 3's state section said. tick16c's flags: the first collapse at **65,567**, every flag 0
  from **65,715** to the end (not 65,746). The bitten B1 run's PM-0001-2 value was **0.48** on copy 3's r2/b1-nofallback
  (not "as the clean b1", 0.273333), and that value is no figure (the mirror then had 2,314 breaches); with point 8
  (A)'s fix the re-made run (r2/b1-nofallback2) gives **0.453333** with 0 breaches. The P0 bytes sum per measured tick
  is **27,816** (27,817 is the sum of the rounded parts 1,953 + 1,096 + 24,768); the B1 window figures are means with
  one decimal: 47.9, 0.5, 0.0, 0.0, 0.0, 0.6, 4.1, 12.4, 23.1, 31.9, 40.1, 44.2, 45.8, 46.3 (slots holding everyone,
  per 30-tick window from tick 64,186).
- **8 (F):** npt-lab writes `tick.s.everyone_in_view` (the room log) from its room-side mirror of the room's datagrams
  and full states (logs.rs `room_tick`, from mirror.rs `SetStats::everyone`); SERIES.md:368 names npt-room as its
  writer (that row is the main session's to correct: U4r adds rows only).
- **Point 1, B1 (r2):** the b1 suite is a FAIL under condition 5: PM-0001-2's everyone in view 0.273333 (the worst slot
  246 of 900 measured ticks), so -17 and -18 L1 B1 are INVALID by pre:PM-0001-2, while -57 L1 B1's three checks pass
  (6,974; 2,750; 88,060). The cause, in three lines: (1) every client's baseline is evicted at each burst: 96
  burst_eviction records, evicted 1, baseline_age 0; (2) the event backlog is packed before any entity change
  (room.rs:1208-1235; queue high-water 631); (3) each client's set is rebuilt from the defaults under the 1,000 B cap
  (snap_datagram_bytes 996-1,000 B per client per tick before, during and after). The 30-tick window means are as in
  8 (E). If every burst costs what burst 0 did, the least share at 10 bursts in 600 s is at most 0.919. It is
  npt-room's, not U4r's; by the advisor's ruling 3, point 1, the room's fix and the passing evidence of -17's and -18's
  L1 B1 parts go to U4g. (Plain words for Mustafa: after each 1 s loss burst every client lost sight of the other
  soldiers for about 4-11 s; the room's, fixed in U4g, and -18's L1 evidence moves there.)
- **Point 2, ES (r2):** the es suite is a FAIL under condition 5, carry 59's pair side by side: reliable events 50.05 a
  second (1,001 in the 20 measured seconds) against the band 9.5-10.5, beside U4d-F's 7.250 (the synthetic recording
  at its old reckoning). Cause: the brawl aims at its target's exact position (bots/src/lib.rs:226-245), so between
  68.5 % and 69.5 % of its 1,440 measured shots hit (reliable events are Hit and Kill; at most 14 kills at damage 10 and
  health 1,000). The other 11 ES checks pass; no ES run counts at U4r; by ruling 3, point 2, the brawl's ES hit rule
  and its measurement are U4h's.
- **Point 4, -18's bite (r2):** in place of "no client holds a heal within 500 ms": the bitten run fails -18's max check
  (max 1,688,286,323 ns > 500,000,000; copy 3's run and copy 5's re-made run alike), and the clean run holds 96 of 96
  heals within 500 ms. The 25 fast heals counted apart: 14 in burst 0, each at burst_eviction baseline_age 0 (carry 77
  (b)'s case); 11 in burst 1, at baseline_age 33 or 34, a second case of the same cause: the stale copy is taken only
  when the ring slot is overwritten (ack.rs:109-114), after the base is chosen (room.rs:1163-1170); derived from
  timing, not shown client by client. All 71 slow heals followed a full state (71 overflow resyncs, full_state 1 on
  each); the heal record takes neither the sentinel nor a full state's tick (scenario.rs:994-999). Copy 3's run had
  2,314 mirror breaches (the first "tick 64218: client 23: its baseline frame (age 35) is not mirrored"), so its
  PM-0001-2 value 0.48 was no figure; after point 8 (A) the re-made run has 0 breaches and 0.453333 (INVALID by
  pre:PM-0001-2 while the room is unfixed). The re-made run (read-only over its logs) splits the same way: fast heals
  14 at (burst 0, baseline_age 0), 8 at (burst 1, 33) and 3 at (burst 1, 34); the 71 slow heals all with full_state 1,
  no fast one with it; evictions 96, ages 0 (14), 33 (67), 34 (15).
- **Point 7:** (a) carry 62 (e)'s reading for bite runs, in ruling 3's words: "a bite run that reaches the after
  phase's cap stops there; its unresolved ticks are decided lost at the stop (carry 62 (c)); the stop is recorded as
  after_capped in the summary and run_lengths". (b) the mirror's frames looked up by the room's tick: tick16b's 5,828
  breaches (the first "tick 65536: client 0: its baseline frame (age 1) is not mirrored") against tick16c's 0; the
  clean P0 re-run equal record for record but the added after_capped field; across the wrap, point 3's clean partner
  (r2/tick16-clean: 0 breaches, every check passing).

## Copy 5: state and what is next (a fresh copy resumes here)

- **Updated 02:08 UTC, after the coordinator's four items (all done):** point 8 (A)'s two runs (as expected), the two
  c-u4r-runs.mjs items (ruling 3, points 3 and 1), c-u4r-r2.mjs's lines by hand (b1 and es not re-run, by points 1 and
  2), and the report words (item (4) block). **Held until the advisor's ruling on P3-CAP and CR-ID** (by message):
  c-u4r-r2.mjs's tick16 `expect` text (it still says "after_capped true"), r5, and section 5's end checks twice (CR-ID
  may change contract.rs). No plant in place; no half edit; no run going; the measured release build is the last
  one built (c5-r2lines.log). df 4.8G; .gate/u4r/r2/ 145 MB plus r2/b1-nofallback2 and p0-s1c; .gate/u4r/r2c5/ 31 MB.
- `git status` holds copy 4's 26 entries plus copy 5's changes: mirror.rs, scenario.rs, contract.rs, main.rs,
  precheck.rs, marks/SERIES.md (one row's words), ops/runs/c-u4r-runs.mjs (two items), and ops/runs/c-u4r-r2.mjs (new).
  Nothing committed.
- Clean at: c5-2.log (fmt exit 0; clippy -p npt-lab -p npt-bots --all-targets -D warnings exit 0, and with
  --all-features exit 0) and c5-3.log (npt-lab bin 47 passed, 1 ignored; bites 54 passed, 1 ignored; lib 5; bots 5).
  No Rust file changed since; the two .mjs files pass `node --check`.
- **Next, once the ruling arrives:** apply it (P3-CAP: the tick16 `expect` text in c-u4r-r2.mjs; CR-ID: contract.rs's
  exception, and then fmt, clippy and npt-lab's tests in both builds again); then r5 (seeds 1-3 per scene and profile,
  the slowest kept; 7.3's estimate redone, with one -18 bite run fewer), then section 5's end checks twice, last.
- **The earlier plan (copy 5's first stop), kept for the record; items 1 to 3 are done above:**
  1. Point 8 (A)'s runs, each once, in one -x call: bites build, then `npt-lab scenario --run u4r-r2-b1-nofallback-1
     --set u4r-r2 --date 2026-11-14 --commit 2fbec77... --out .gate/u4r/r2/b1-nofallback2 --scene SS --profile B1
     --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 30 --bursts 2 --burst-first-s 5 --burst-every-s 15 --role
     bite --bite PM-0001-18/no-fallback --marks PM-0001-2,PM-0001-17,PM-0001-57,PM-0001-18 --suite r2-b1`, contract,
     `npt-judge --only PM-0001-2,PM-0001-17,PM-0001-57,PM-0001-18`; expected 0 breaches, -18's max FAIL, INVALID by
     pre:PM-0001-2 reported (anything else goes to the advisor). Then the measured build and the clean P0 equality:
     `--run u4r-r2-p0-1 ... --out .gate/u4r/r2/p0-s1c --scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10
     --measured-s 20 --marks PM-0001-2,PM-0001-17,PM-0001-57 --suite r2-p0`, compared record for record (read-only)
     with r2/p0-s1b (every line after the header equal; the header equal but build.binary_sha256) and with r2/p0-s1
     (equal but run_lengths' added after_capped). The B1 header now carries burst_spacing_ns (15,000,000,000): check it.
  2. Item (3): c-u4r-r2.mjs's npt-lab lines run by hand on this tree (short: they are the r2 suites themselves; the
     script refuses uncommitted changes, so each line is run through the runner as copy 4 did for c-u4r-runs.mjs),
     after the tick16 `expect` text follows the advisor's answer.
  3. Item (4): report words: 8 (C) labels (.gate/u4r/regen/ and regen-all/: "tree: 2fbec77 plus the uncommitted U4r
     changes; no header"); 8 (E) (21 rows, not 22; tick16's flags 0 from 65,715, first collapse 65,567; the bitten
     run's PM-0001-2 value 0.48; the P0 sum 27,816; window figures as means); 8 (F) (npt-lab writes
     tick.s.everyone_in_view from its mirror; SERIES.md:368 names npt-room); points 1, 2, 4, 7 by their Builder lines.
  4. Item (5): r5 (seeds 1-3 per scene and profile, the slowest kept; 7.3's estimate redone), then section 5's end
     checks twice, last.
- (Done since, on the coordinator's message: c-u4r-runs.mjs's tick16 start and -18's bite, the block "c-u4r-runs.mjs:
  ruling 3's two script items".)
