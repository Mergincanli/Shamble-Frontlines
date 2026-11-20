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
