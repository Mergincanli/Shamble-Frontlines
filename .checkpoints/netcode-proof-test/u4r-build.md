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
