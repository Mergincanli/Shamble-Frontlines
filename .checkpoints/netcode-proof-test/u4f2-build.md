# U4f2's build: -21's forged-command suite and the code for all of -21's runs (builder 1)

## Step 0 (2026-11-20)
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807 started at f5030a3 (origin/main);
  `git merge --ff-only 0376c76` fast-forwarded it: HEAD 0376c7697440bd2c78c6ce7c4d10adbcbe5ded25.
- `git rev-parse HEAD:spikes/netcode-proof` = 2b7647ffc3c1927e0bb8d6269bbf240217b8f6db (the brief's 2b7647ffc3c1927e…).
- `git diff --quiet a289def HEAD -- spikes/netcode-proof`: exit 0 (spikes/ equals `<base>`'s).
- Slip, said at the top as the rules ask (u4f-brief.md section 4, Commands): my first two Bash calls (step 0 itself)
  used `cd <worktree> && ...` with `;` and `|` in one call, against P17's one-command form. Read-only git commands and
  a `merge --ff-only`; no file written by them. Every later call is one command, from the worktree.
- `df -h /home/user` at start: 9.9G free.

## Plan (blocks in u4f2-brief.md section 1's order; each at most about 400 added lines, compiling with its tests
## passing before the next; SIZE-CHECK `git diff --stat a289def -- spikes/` after each)
1. f28's clamp and lag-switch halves under option B (one test per suite and option).
2. npt-bots: forgers of kinds 3, 7, 8, 9, 10 and 11, f15's six cases.
3. npt-lab: `--suite forged` (11 forger slots, 37 honest open-loop), `--forge-every` with header param and SERIES.md
   row; C-U4F2-1's parser half (below 2 a usage error).
4. npt-lab attribution: C-U4F2-3 first (room `Refusal` enum -> `CommandRefusal`, records `{ slot, seq, kind }`), then
   `event.forged_kind.sent_count` / `.unrejected_count`; C-U4F2-SEC-1 (`refusals_dropped` logged).
5. npt-lab twins: skip through `ClientEndpoint::skip_command`, kind 10 and kind 3 twin rows, clamp twins'
   `--twin-logs`; C-U4F2-2's `skip_command` unit test, `twin_left_out` doc and row.
6. Tests f16 (24), f22 (2), f23, f33 (13) + U4F2-Q4's case + C-U4F2-10's case; C-U4F2-1's proof.
7. C-U4F2-5 (Steady's move by Q1 (b)/(c), sim-only test with option-A pattern and the plant), C-U4F2-7 with T-6's
   five cases, C-U4F2-8, ADV-U4FP-3.
8. Scripts: c-u4f-r2.mjs p1, p7, p14, p17 (C-U4F2-4, F5, p19's expect); c-u4f-runs.mjs -21's set.
9. End checks (u4f-brief.md section 6 items 1-5).

## Log (one line per block, plant, finding)
- Finding (time budget, 14:2x UTC): 1faf894's counted check (.gate/u4f/f1-b5-check2.log) has the test step at 931.9 s
  against the 1,207 s stop line: about 275 s of room. c62hb_p2_short alone 13.59 s (w4-times.log); a 16-client 2+4 s
  run about 8-9 s in debug (f19 8.64 s). 22 separate f16 pairs (one forger, 15 honest: 2.10's scale text) would add
  about 190 s of wall to the step; so f16 shares one forged pair per option (11 forgers + 5 honest at 16 clients) by
  line 6's sharing rule, and the clamp pair per option is shared by f18, f28 and f16 (a departure from 2.10's "one
  forger, 15 honest", named for the report).
- Block 1 DONE (f28's clamp and lag-switch B halves): `pair(suite, timing)` (OnceLock, the clamp run and its twin),
  f18 rewritten on pair("clamp","A") (+ every tick's world hash), f28_option_b_the_clamp_suite_..., `lag_short`, f20 on
  it, f28_option_b_the_lag_switch_suite_..., `unnamed`. u4f2-b1-t.log: 6 passed (f18, f20, f28 B x4) in 34.39 s.
  SIZE-CHECK: +93 -36.
- Block 2 DONE (npt-bots: the forgers of kinds 3, 7, 8, 9, 10, 11 and every twin, f15's six): `forger` takes kinds
  1-11; `twin()`, `catchup_under_a()`, `take_copies()`, `forge_unencodable()`, `reliable()` (kind 11's control-stream
  message) are new public items; `Forge::outgoing`/`forge_input`/`restamp`; `forge_step`, `catchup_step`. Constructions:
  kind 3 the previous command's copy (move turned 90 degrees, or aim x +100 cm, in turn), in place or added; kind 7 an
  honest reload edge on the command before a due one, then fire and reload edges in turn inside it (window
  `ticks_for_ms(reload_ms - RELOAD_MARGIN_MS)` = 54 at 30 Hz / 108 at 60 Hz; next reload after
  `ticks_for_ms(reload_ms + RELOAD_MARGIN_MS)` = 66 / 132; RELOAD_MARGIN_MS 200 ms, carry 39's form: a ms constant
  through npt-sim's ticks_for_ms, site bots/src/lib.rs Forge::reload); kind 8 ack tick + 1,000; kind 9 a trailing 0 byte
  or byte 27's top pad bit in turn; kind 10 restamped contiguous from its last accepted stamp + 1, episodes every
  every x 8 (A) / x 4 (B) commands: A 6 hold steps + 8 (held) + 8 (refused by the cap), B 8 + 4 (refused, buffer full)
  then 6 hold steps (CATCHUP_HOLD = REDUNDANCY_CAP - 2: a step count, keeps the lead); kind 11 a control-framed `Ping`
  (WrongChannel). Twins: 1, 2, 4, 5, 6 `skip_command` at the due step; 10's skips at the next step; 3 the honest copy; 7
  without the edge; 8, 9 header only with the previous ack; 11 `ClientSettings { delay: 0 }`. The old in-loop forger
  code moved into `Forge::outgoing` (due set at the step by the given count, = the old `seen` at one command a step).
  u4f2-b2-t3.log, u4f2-b2-fmt.log: f15 12 passed; whole npt-bots lib 18 + the new passed in u4f2-b2-t.log but kind 10
  (fixed: last hold run, sequence set); clippy both builds bots+lab exit 0 (u4f2-b2-clippy.log), fmt clean.
  SIZE-CHECK: +861 -68 (bots +796: above the block's ~400 and the brief's ~250 for bots; the twins' bots half, block 5's
  in the brief, is in it). Finding: the estimate for bots ran about 3 times; the rest must stay lean.
- Block 3 DONE (npt-lab's forged suite): `--suite forged` with `--forge-every <n>` (C-U4F2-1's parser half: below 2 or
  not a number a usage error; the two go together; at least 11 clients), `FORGERS` = 11 (slot s < 11 forges kind s + 1
  on a fixed-lead endpoint, steady script; `catchup_under_a()` under A; `twin()` under role twin), the honest clients
  on the run's behaviour under the controller; `header.params.forge_every_count` + its SERIES.md row (new U4f2
  subsection); `Behaviour::reliable` wired into the client step; USAGE names the option; `pair("forged", t)` at the
  judged interval 12. Tests: c_u4f2_1_a_forge_interval_below_2_is_a_usage_error (+1 extra), f22 A and B (2, neg), f23
  (1, neg). u4f2-b3-t.log: 4 passed, 24.88 s (4 short runs: A buffer full 8 = one episode; B 12 = three; twins 0; press
  refused 14 forged, 0 twin). SIZE-CHECK: +1000 -69.
- Block 4 DONE (C-U4F2-3 first, then the attribution and SEC-1): npt-room's enum `Refusal` renamed `CommandRefusal`
  (input.rs, room.rs, lib.rs), its records the struct `RefusalRecord { slot, seq, kind }` (room.rs; exported), the
  one consumer room/tests/commands.rs:82 updated; npt-sim's `Refusal` untouched (crates/sim shut). New lab/src/forged.rs
  (`Attribution`: per forger, items by seq with kind 3's copy, own-kind room records (kinds 1-6, 10), sim
  Cooldown/Reload (kind 7), runs per seq and copy runs, kinds 8/9/11 by count at the cutoff). Wired in `run`: items
  taken at every client step, counted before the cutoff = measured end - the room's lead bound (12 ticks); each tick
  the room's records, the sim's and the TickInput; kinds 8/9/11's counters (ack_refused; trailing + unknown_field;
  wrong_channel) at k == cutoff; at the end one `forged_kind` record per kind (not on a twin) with `slot`, `kind`,
  `sent_count`, `measured_sent_count`, `unrejected_count`, `refusals_dropped_count`; client_counts gains
  `twin_left_out_count` and `forge_unencodable_count` on the forged suite. Two new SERIES.md rows. f33's 13 + Q4 + C-U4F2-10
  in forged.rs. u4f2-b4-t.log: npt-room all passed (24+1+27+19), lab 18 passed (15 forged:: + f22 x2 + f23).
  SIZE-CHECK: +1100 -110 tracked, + forged.rs 320 new = 1,420 added.
- Block 5 DONE (the twins in the lab, C-U4F2-2): the twins themselves are block 2's bots half (role twin -> `twin()`,
  kind 10's and kind 3's rows) and block 3's wiring; the clamp twins' `--twin-logs` stands (U4f's). New:
  client/tests/endpoint.rs `c_u4f2_2_skip_command_uses_the_index_and_stamp_and_sends_nothing` (+1 extra): before the
  join nothing, joined index used and nothing sent, an unacknowledged copy left out and counted. `twin_left_out`'s doc
  already names both causes (FP1); its row (both causes) is block 4's. u4f2-b5-t.log: 1 passed.
- Block 6 DONE (f16, with f22, f23, f33 from blocks 3-4): `Summary::fates` (test builds only: each fate entry's first
  32 canonical bytes), f16(suite, kind, timing) + its 24 tests on the shared pairs: fates entry by entry, snapshot
  datagrams, room-log tick hashes, every tick's state hash, no command left out, no forged `Input` unencodable, the
  kind's forged_kind (measured >= 1; kind 10 >= 8 A / 4 B), unrejected 0, refusals dropped 0, the forger's link alike;
  clamp: clamp_kind sent >= 1, counted 0. C-U4F2-1's proof: f16 kinds 2 and 5 at the judged interval 12, steady, P0,
  Room<World>: 13 items each, unrejected 0. u4f2-b6-t.log: 24 passed in 66.18 s (8 runs). Per-kind items at 2+4 s:
  13 (kinds 1-6, 8, 9, 11; 9 measured), kind 7 11 (7), kind 10 8 A / 12 B (8).
  SIZE-CHECK: +1339 -110 tracked + forged.rs 320 = 1,659 added.
- Block 7 DONE (the routes):
  - C-U4F2-5 by U4F2-Q1: BRAWL-1's sim-only model takes the script as its parameter (`sim_script`, `printed_script`)
    and gains `Pattern::OptionA` (P1: each slot's client step at a seeded phase, one-way 20-30 ms uniform per command,
    run at the first tick at or after arrival, lead `ticks_for_ms(60) + 1` = 3; no re-run on an empty tick). Today's
    walk (the plant, the tree as at 25d316e for Steady): u4f2-b7-walk-today.log, every pattern enters: seed 1 EveryTick
    11,355 / Skips 53,211 / OptionA 2,828 / Bursts 21,658 soldier-ticks; seed 2 0 / 82,088 / 10,245 / 18,705; seed 3 9 /
    57,207 / 0 / 15,278. (b) tried, a 1/32-length circle: still enters (u4f2-b7-walk-fix.log: seed 1 EveryTick 4,362,
    OptionA 2,282; seed 2 Skips 2,220): (b) not shown. My reading (not verified by a position log): a walking soldier
    that meets a passing vehicle part is set at the part's face (world.rs collide, :1260-1268), whatever its speed, so
    any open-loop walk is carried. (c) built: `Kind::Steady => [0, 0]` (its fire, aim, presses kept; MOVES, TURN_EVERY,
    Brawl, NoFire, Presses, Clamp and Lane arms unchanged). u4f2-b7-stand.log: all 12 (seed, pattern) 0 in a building
    (largest distance 1,414 cm = the spawn grid's corner), the short copy passes; whole bots lib 20 passed.
    FINDING F-A (for the advisor, Medium): the lag-switch suite's lag client (slot 0) wraps the run's `steady`
    behaviour, so it now stands: `tick.v.lagswitch_excess_move_um` = 0 - 8 x full speed on every tick, and -21's lag
    check "no tick moves further than the catch-up cap allows" can no longer fail by movement (TE-7 already had it not
    failing at 200 ms). An option outside my lane's rule (point 1 keeps the arms, not the lab's wiring): the lag client on
    the Lane walk (full speed, collision-free, closed loop allowed since that suite has no twin). Not built.
    FINDING F-B (Medium, for -21's set): the sim-only steady runs show 5,300-6,200 dead soldier-ticks per 610 s (fixed
    blind aims cross standing soldiers); a kind-7 item run while its forger is dead gets no sim record (run_command
    returns at `!s.alive`, world.rs:1353) and counts unrejected; a respawn refills the magazine, so a reload edge just
    after it is refused as an honest one. The short f16 runs show none; the 48-client 10-minute set may. To measure on
    a hand run (block 8/9).
  - kind 3's move change is now `move_xy[0] + 1` (wrapping) in place of a 90-degree turn (the standing walk is [0, 0]).
  - C-U4F2-7 (Q3 (a)): contract.rs's band keyed on `params.suite` = `clock`, line "not applied (suite clock: its
    clients never fire)"; tests c_u4f2_7_the_shot_band_is_not_applied_to_the_clock_suite and
    ..._still_fails_another_suite_with_no_shots (+2 extras). T-6's five: t6_a_missing_carry_60_header_param_...,
    t6_a_missing_class_audit_event_..., t6_a_duplicate_admit_..., t6_a_log_the_reader_refuses_is_exit_1,
    t6_a_usage_error_is_exit_2 (+5). r2's short run now a OnceLock shared by r2 and these.
  - C-U4F2-8: `params.cmd_timing` written by `run` from `server.room.cmd_timing()`, no longer by `params()`; f6 pins it.
  - ADV-U4FP-3: client lib.rs `with_clock`'s long line and `plan()`'s short line rewrapped (docs only).
  u4f2-b7-t.log: whole lab bin 131 passed, 1 ignored, 246.73 s (debug, measured); u4f2-b7-lint.log: fmt clean, clippy
  both builds exit 0, contract:: 10, client endpoint 12, bots f15 12 passed.
  SIZE-CHECK: +1658 -151 tracked + forged.rs 362 new = 2,020 added.
- Block 8 DONE (the scripts): c-u4f-r2.mjs gains p1, p7, p14, p17 (FORGED = P21 + `--suite forged --forge-every
  12`), C-U4F2-4's reworded FP8 line (header comment and r2.txt's head: "-21's 5.3 record only from U4f2's merge commit
  or a later one"), F5 (a) p4 "5 bursts in 30 s", F5 (b) p12 "89.5 % on U4f's early read", F5 (c) p12 "'0 presses lost'
  fails too, reached by the bite", p19's expect reworded to section 5's reading (no fixed counts); 21 rows.
  c-u4f-runs.mjs gains set 21 (PARTS_21, NO_STAR; `plan` takes parts, a twin right after its measured run, each naming
  the other in `--twin`, the clamp twin's `--twin-logs @run` placed by main; 24 measured + 12 twins in the measured call,
  13 bites under A in the second; usage 47|22|20|21; STOP_NOTE names the twins). Node check: plans 47 5, 22 4, 20 7
  (unchanged), 21 49 (36 + 13).
- Section 6 item 5, by hand (release, `-x`, the worktree's tree, commit 0376c76 named): u4f2-hand-a.log: the 8
  measured lines and 4 twin lines of set 21 at seed 1 (10 s + 5 s) and r2's p1, p14, p17 (10 s + 20 s), all exit 0;
  u4f2-hand-b.log: the 13 bite lines and p7, all exit 0, and the two judge calls in 5.4's form (`--only
  PM-0001-2,PM-0001-21`, measured+twins, then bites), both exit 0. The measured call: every PM-0001-2 check passes on
  every run (shots 700.8 on the clamp runs); -21 fails only by length (1,000 a kind; 1,000 points; 10 runs of 60 s;
  100 bursts), the fast clock's two checks a gap (no episode in 5 s); the twin checks of forged and clamp PASS on
  both options. Bites: 13 shown; each forged no-* bite's own kind unrejected (36, kind 10's 32).
- FINDING F-D (fixed, my construction, High had it gone to the set): the kind-7 forger's fire edges lay inside its own
  reload, where `PM-0001-21/no-cooldown` (world.rs:1449-1455: relaxes only outside a reload) cannot reach them: on the
  bite run kind 7 read unrejected 0, so the bite would not fail its check at the judged length. Rebuilt kind 7
  (bots `RELOAD_ITEMS` 40, `FIRE_REST` 3): fire items (fire released 3 commands, an honest press that fires the rested
  weapon, the forged press in its cooldown) and every 40th item (index 2) a reload item (honest reload edge, forged
  reload edge inside it). f15's kind-7 test rewritten (`f15_the_kind_7_forger_presses_where_the_sim_refuses`); f15 12,
  f16 + forged:: 39 passed (u4f2-b8-k7.log). No-cooldown bite run (u4f2-hand2): kind 7 31 of 36 unrejected: reached.
- FINDING F-B measured (open, for the advisor before -21's set; High for that set): a judged-length forged run (10 s +
  600 s, seed 1, option A, release): every kind sent at least 1,000 (1,523; kind 10 1,656) and unrejected 0 but kind 7.
  Kind 7: first shape 41 of 1,270 (u4f2-hand-long.log), new shape 18 of 1,523 (u4f2-hand-long2.log). By my reading
  (not verified: no death series is logged), a forged edge while the forger is dead gets no sim record
  (world.rs:1353), and a respawn resets the weapon so an item next to it can fire or start a reload. -21's check
  "forged: every forged command rejected and counted" would then fail. Shots per measured second 677 and 685 (band
  665-735): F-C, a thin margin on seed 1.
  SIZE-CHECK: +1718 -175 tracked + forged.rs 362 = 2,080 added.
- Plants (each Edit, one `-x -k` runner call, restored by Edit, sha256 checked; clean sha256s: forged.rs 7884e8d8…
  then 7919200c… after the C-U4F2-10 case change below, scenario.rs edb4b289…, contract.rs a39f39b9…, bots lib.rs
  0c8c6d21…, client lib.rs 76b507ad…, room input.rs efadaeb2…):
  1. forged.rs `tick`, any kind matches (C-U4F2-10's plant): u4f2-plant1.log fails only f33_a_forged_command_refused_
     under_another_kind; c_u4f2_10 PASSED under it (its items ran, and a run item counts unrejected whatever the
     records): FINDING, fixed: the case now runs item 1 only, item 2's sim Cooldown record alone (forged.rs, clean sha
     7919200c…); u4f2-plant1b.log: c_u4f2_10_kind_5_items_with_only_sim_cooldown_records_count_unrejected and f33_a_
     forged_command_refused_under_another_kind_counts_unrejected FAIL by name. Restored, 7919200c… equal.
  2. forged.rs kind 3's copy rule off: u4f2-plant2.log fails f33_a_planted_copy_that_ran_counts_unrejected_1 and
     f33_kind_3_counts_its_copy_rejected_with_the_honest_original_run. Restored, equal.
  3. forged.rs a record of an unforged sequence made an item (Q4's plant): u4f2-plant3.log fails
     q4_an_honest_kind_10_refusal_changes_neither_count. Restored (slip: plant 4 was put in before re-checking the
     sha256; checked after 4's restore: 7919200c… equal, so both restores held).
  4. forged.rs kinds 8, 9, 11's counts ignored: u4f2-plant4.log fails f33_kind_8_…, f33_kind_9_…, f33_kind_11_….
     Restored, equal.
  5. scenario.rs `--forge-every` 1 taken: u4f2-plant5.log fails c_u4f2_1_a_forge_interval_below_2_is_a_usage_error.
     Restored, edb4b289… equal.
  6. scenario.rs the header's option fixed at B, not the live room's (C-U4F2-8): u4f2-plant6.log fails
     f6_option_a_takes_the_one_controller_setting_…. Restored, equal.
  7. contract.rs the band applied to the clock suite (C-U4F2-7's plant): u4f2-plant7.log fails
     c_u4f2_7_the_shot_band_is_not_applied_to_the_clock_suite (the other case passes). Restored, a39f39b9… equal.
  8. bots `Kind::Steady => walk` (today's walk, C-U4F2-5's plant) on the debug short copy: u4f2-plant8.log fails
     c_u4f2_5_sim_short_no_steady_soldier_enters_a_building (Skips 8,433 soldier-ticks). Restored, 0c8c6d21… equal.
  9. bots: the room-whole kinds' twin makes the refused command's honest content instead of skipping it:
     u4f2-plant9.log, f16_option_a_* 12 PASS under it. FINDING F-E (Medium, for the advisor): with the standing steady
     script (C-U4F2-5 (c)) every honest command repeats the last one's effect, so f16 (and -21's twin figure) cannot
     tell "never made" from "made honest" for kinds 1, 2, 4, 5, 6; the claim it tests (the forged content had no
     effect) still holds and bites (plant 10). Restored, 0c8c6d21… equal.
  10. room input.rs kind 4's check removed: u4f2-plant10.log fails f16_option_a_kind_4_… and f16_option_b_kind_4_…
      by name ("the ticks hashed and their hashes"; the filter also ran f16 A kinds 1, 10, 11, which share the pair
      and failed with it). Restored, efadaeb2… equal.
  11. client `skip_command` not using the index: u4f2-plant11.log fails c_u4f2_2_skip_command_uses_the_index_and_….
      Restored, 76b507ad… equal.
- Each new test alone (TE31-10; `-x`, debug, measured build): u4f2-times-f16.log: c62hb_p2_short 16.73 s (the
  yardstick in the same call; limit 33.46 s); f16's 24 from 19.42 to 21.47 s. u4f2-times-rest.log: f28 B clamp 20.25,
  f28 B lag 12.61, c_u4f2_1 0.00, f22 A 19.83, B 19.84, f23 19.36, c_u4f2_7 x2 29.87 / 29.24, t6 carry-60 30.15,
  class-audit 29.99, duplicate-admit 31.94 (the closest: U4r's shared 48-client short run), refused-log 0.00, usage
  0.00, forged:: 15 in 0.00, f15 x6 0.01-0.03, c_u4f2_5_sim_short 23.61, c_u4f2_2 0.00 s. None past the limit.
- New tests, 60 (each in both builds; none bites-only): bots 7 (f15 kinds 3, 7, 8, 9, 10, 11; c_u4f2_5_sim_short;
  plus c_u4f2_5_sim_full, ignored, release by name), client 1 (c_u4f2_2), lab scenario 30 (f28 B x2, c_u4f2_1, f22 x2,
  f23, f16 x24), lab forged 15 (f33 x13, q4, c_u4f2_10), lab contract 7 (c_u4f2_7 x2, t6 x5). Expected counts:
  642 + 60 = 702 measured, 739 + 60 = 799 bites. Share: 2.10's 51 (f15 6, f16 24, f22 2, f23 1, f33 13, T-6 5) + 9
  extras; negatives 22 (f15 6, f22 2, f23 1, f33 13) + Q4 and C-U4F2-10 beyond.
- End check A (u4f2-end-a.log, 16:03 UTC, `-x`, df 7.6 G before): provenance builds `cargo test --locked --workspace
  --no-run` exit 0, `--all-features` exit 0; `node check.mjs`: fmt PASS, clippy PASS, clippy-wasm32 PASS, **doc FAIL**
  ("public documentation for `forger` links to private item `RELOAD_ITEMS`"; `CATCHUP_HOLD` the same). Stopped by me
  during the test step (the run could not pass; no cargo, rustc or node left: pgrep empty). Fix: `forger`'s doc names
  the two constants as plain text ("every 40th item, the crate's `RELOAD_ITEMS`"; "6 steps ... `CATCHUP_HOLD`"),
  rewrapped. NOT re-verified: the rustdoc call (u4f2-doc.log, 16:14 UTC) was refused by the runner.
- **STOP (disk), 16:14 UTC:** the runner refused: "free 3834 MiB ... STOP: under 4 GB free" (u4f2-doc.log); `df -h
  /home/user`: 3.8G avail. Nothing cleaned or deleted. What grew since the start (9.9G): the shared target's release
  builds of npt-lab (measured and bites, twice) and npt-judge for the hand runs, the debug test builds, and the hand
  runs' logs under .gate/u4f/u4f2-hand*/ (two 610 s 48-client runs, about 53 MiB each, and 33 short ones). Left to do
  on resume: rustdoc both builds (the doc fix), fmt check, then section 6's end checks: check.mjs twice (`-x`, df
  first), with the provenance builds; npt-server's tests are untouched (item 4 not triggered).
