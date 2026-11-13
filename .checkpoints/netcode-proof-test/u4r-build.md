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
