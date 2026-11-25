# netcode-architect, review round 2 of change-set 3b (head 6380df45; round 1 bbbf09e4; base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/netcode-architect-cs3b2 (detached at 6380df45).
Round 1's notes: netcode-architect-review1.md (F1 Medium = NA-1; F2..F7 Lows = NA-2..NA-7).

## Plan
1. Diff bbbf09e4..6380df45 -- crates packages (stat, then file by file for queue.rs, lib.rs, endpoint.rs, press.rs,
   codec.rs, decoder_inputs.rs, local_match.rs, any_bytes.rs); also 0dfca749..6380df45 alone (the queue commit).
2. NA-1: Readout.late vs refused in endpoint.rs on_datagram; write_block(refused_elsewhere); where late goes.
   Re-run my round-1 toy-link probe (nc_probe_link) ported to the new API: honest link -> refused 0, late > 0.
3. NA-2/NA-3: queue insert window before the first take; build_input pushes newest first, repeats filtered.
   Re-run my nc_probe_queue (0, 40000, 20000, 0) and nc_probe_endpoint (32,768 commands unconfirmed) ported.
4. NA-4 press.rs doc; NA-5 codec Join version before token + decoder generator; NA-6 step(_now_ms).
5. Point 3 (the queue's window): read queue.rs newest_run/take_due/ran and lib.rs step/admit; prove or break the
   half-circle invariant for waiting commands under any take/refuse sequence; cost of re-offering a refused command;
   honest-client claim (admit never refuses an honest command); Inserted::Stale doc + admit doc truth; compare
   with the dropped alternative (stale-in-admit). Probes as untracked test files in my copy.
6. Point 4: any_bytes.rs content: client model, rules per step / per match; propose cheap missing checks.
7. Point 5: M1 deferral (ADR 0001 OQ-0001-6 ~:1156, PM-0001-21, part 5 :231-239; DECISIONS 2026-11-24); the bound's
   form under (a) run-on-arrival and (b) fixed-delay; PM-0001-21 long-hold row follow-up.
8. Point 6: RR-3 for 3c: corruption as loss vs plausibility bound; which bound.
9. Runs: cargo test room, client-endpoint, protocol, sim-wasm (+ any_bytes --nocapture), clippy workspace.
10. Answer <= 30 lines via hand-off; DONE.

## Findings (one line each, as found)
- RUNS (my copy, 6380df45, debug): room 15, client-endpoint 26, protocol 19 + decoder 8, sim-wasm 13 + any_bytes 2 +
  panic_hook 1 + step_alloc 1: all pass. any_bytes --nocapture: room 81,106 run (8,737 late), 217,671 presses granted,
  [missing 404,840, queue_full 213, rejections 8,245, decode 64,087, ack 2,110, ahead 685, not_newer 13,210, presses
  11,382, duplicates 50,455, stale 145,976]; endpoint 84,744 inputs, 19 rejected, [received 16,708, late 78,324,
  refused 26,425, lost 77,132, doubled 144,424]. (vs security-auditor's 0dfca749 run, an older version of the test:
  not_newer 1,174 -> 13,210, stale 208,217 -> 145,976, run 67,572 -> 81,106: refused copies now re-queued and refused
  again instead of stale; honest commands behind a refused one now run.)
- NA-1 read: endpoint.rs:69-73 refused = no snapshot; :71-73 late = well formed, not newer or before the join;
  on_datagram :291-300; write_block(refused_elsewhere) :175-178 adds; local_match.rs:158 passes room.refused() +
  unsent; late in no block (layout untouched for it). Fixed as asked.
- NA-3 read: build_input :249-274 pushes `ahead = newest` first unconditionally, then repeats while unconfirmed and
  1..=255 ticks before the one ahead; encode failure counted in Readout.unsent (:278-285). Fixed.
- NA-4 read: press.rs:7-9 states MAX_BACKLOG 255, dropped and counted, shown as lost. Fixed.
- NA-6 read: local_match.rs:112 step(_now_ms) stores nothing. Fell away, confirmed.
- NA-5 read: codec.rs:9 table, :126 encode version first, :160 decode version first, byte test :520, cap tests
  :636-638; decoder_inputs.rs:161-163 generator writes 2 version bytes then len then len+32; plant reads len at [3..5]
  (:511). Fixed. (The room still decodes the whole Join before its version check, bounded by 1,024: unchanged, fine.)
- NA-2 read: queue.rs:72 get_or_insert(first offered - 1); test :188-196. My round-1 case (0, 40000, 20000, 0) now
  runs 0, 20000 only. Fixed.
- POINT 3 PROOF (read): insert admits only seq_newer(new, base) (queue.rs:72-75); within (base, base+32767] seq_newer
  is the linear order of offsets, so the array stays sorted; take_due takes waiting[0] = smallest offset
  (queue.rs:101-108); base moves only by ran(next.sequence) right after admit passes (lib.rs:258-270), and every
  other waiting command has a larger offset, so it stays inside the new window; a refused take leaves base. Hence:
  every waiting command within half the circle of base whatever is refused; every run newer than the last run;
  seq_ahead in lib.rs:261-264 always Some, so missing keeps the distance.
- PROBE crates/room/tests/nc2_probe_window.rs (my copy, untracked, release): 40 seeds x 250,000 random ops against a
  shadow model (inserts near a head and 1 in 4 anywhere, takes, refusals 0/8..8/8 of takes per seed, ticks around
  running): 893,122 runs, 726,413 refusals (a streak of 38,096), answers queued 1,619,789 / duplicate 439,608 /
  stale 2,682,034 / full 259,404; every insert answer and every take equal the model; no waiting command outside the
  window; 0 runs not newer than the last. BITES: the same ops with round 1's window (ran() also on refused takes):
  24,393 runs not newer than the last run, 249,445 near-head commands stale though newer than the last run; new: 0, 0.
- PROBE crates/sim-wasm/tests/nc2_probe_link.rs (my copy, untracked, release; round 1's toy link ported, plus a heavy
  reorder profile delay 4+-4 / dup 5 % and a 3,000-tick outage; start tick 131,069, first sequence 65,500): 9 honest
  runs, room refused 0 and admit refused 0 (not_newer 0, presses 0) in every run, so ran() is called on every take and
  the new window evolves exactly as round 1's `taken` did (by construction: nothing reads the window between take_due
  and ran, lib.rs:256-270); endpoint refused 0, late 712 / 720 / 787 / 1,976 / 3,619 / 754 / 763 / 427 (NA-1 split
  right); received + late + refused == datagrams delivered to the client, every run (candidate identity for
  any_bytes); outages 45 / 100 / 3,000 ticks: missing 39 / 93 / 2,993 (outage - 7: the repeat window).
- OBSERVED (known follow-up, not new): 3,000 ticks dark -> 1,024 presses per button lost, exactly 4 x 256 (8-bit
  counters across a gap > 255 commands; round 1's F5 trap = the card's "press check past 255 ticks" follow-up). Make
  sure the follow-up says presses are LOST past 255 ticks of outage, not only that the check is vacuous.
- OBSERVED (3b by design): every honest command ran late (late == run) and ~1/3 of ticks ran two commands under jitter
  (2,196-2,825 two-command ticks per ~6,780 ticks): run on arrival with no input buffer and no client lead; change-set
  7's scheduler and clock. Any bank bound must leave this honest pairing alone (a budget refilled +1 per tick does).
- PROBE crates/client-endpoint/tests/nc2_probe_endpoint.rs (untracked, release): (1) NA-3: one confirmation then
  40,000 commands with none: carried 1,2,...,8, then from command 32,769 on 1 (the newest alone; round 1: 0, a stall).
  FIXED. (2) RR-3 on this head: one snapshot with bit 20 of its tick flipped (111 -> 1,048,687) is taken; the next
  1,000 honest snapshots: taken 0, late 1,000; every Input then acks 1,048,687, a tick the room never sent (the room
  drops each whole, refused_ack). (3) 1,000,000 random/mutated snapshots: handed 1,000,000 = received 61 + late
  471,144 + refused 528,795: every datagram counted exactly once.
- PROBE crates/sim-wasm/tests/nc2_any_bytes_plus.rs + nc2_any_bytes_echo.rs (untracked copies of any_bytes.rs with
  extra checks; same seeds, same totals as the tracked test): ROOM per-step identity "last_processed moved by exactly
  the commands run + the missing counted in this tick (wrapping u16)" held on 74,336 steps (exact for 1 or 2 runs: each
  <= 32,767, sum <= 65,534); it would replace the weak 2-command rule `new != old` (any_bytes.rs:235-241) and imply the
  per-match identity (:264-270). ENDPOINT: every datagram handed = received + late + refused, per match (100/100);
  each Input's ack == the newest snapshot taken, none before (84,744 inputs); each repeat == the command first sent
  with its sequence (384,169 repeats). All hold; all cheap.
- NIT any_bytes.rs:376 vs :380: the "rules-keeping" room names sequence-1 as run but echoes the newest command's
  presses; with a consistent echo (presses of the command named) doubled drops 144,424 -> 101,082 (lost 77,132 ->
  77,480): ~30 % of the doubled count is the model, the rest the lies. Wording/model nit for when the test moves.
- PROBE crates/room/tests/nc2_probe_cost.rs (untracked, release, 4 cores, load ~2): 1,000,000 datagrams + steps, x3:
  honest one command per Input 76 / 83 / 83 ns; eight copies refused every time (ticks not later than the last run,
  sequences newer, so queued, taken out and refused again each Input: 8,000,000 refused_not_newer) 380 / 384 / 409 ns,
  i.e. ~40 ns per refused copy, <= 8 per datagram, <= 16 takes per step, no allocation, counted only on that client's
  own counters. Not a cost that matters; the per-connection message rate (Slice 3, lib.rs:173) bounds it in time.
- POINT 3 DOCS: admit's doc (lib.rs:228) true: newest_run == last_run.sequence after any run (set together,
  lib.rs:270-272), queue holds only newer. ran's doc (queue.rs:110-112) true. Inserted::Stale (queue.rs:26) "not newer
  than the last command that ran: an honest late repeat": true for the honest case only; before any run the base is
  the first offered - 1 (field doc :37 says so), and a hostile sequence 32,768+ ahead also lands there (the queue's
  own test :211-212); RoomCounters doc lib.rs:59 same. Wording nit.
- POINT 3 API (Low): queue.rs:113-115 `ran(sequence)` trusts its caller to pass the command take_due returned last;
  the half-circle invariant and admit's doc sentence hold only for that (a caller that runs a command picked by tick,
  as change-set 7's option (b) scheduler might, breaks them). Smallest fix: the queue records the sequence take_due
  returned and `ran` debug-asserts it (or takes no argument); or change-set 7's seam test pins it.
- SOURCE (read 2026-11-25, rfc-editor.org): RFC 9001 (TLS for QUIC) §6.6: "In comparison, QUIC ignores any packet
  that cannot be authenticated, allowing multiple forgery attempts." and "endpoints MUST count the number of received
  packets that fail authentication during the lifetime of a connection."; §5.3: "These cipher suites have a 16-byte
  authentication tag ...". So over WebTransport a changed datagram is a lost one. Confidence: high (spec text).
- SOURCE (read 2026-11-25, rfc-editor.org): RFC 8446 (TLS 1.3) §5.2: "If the decryption fails, the receiver MUST
  terminate the connection with a "bad_record_mac" alert." So over the WebSocket fallback (wss) a changed byte
  closes the connection; never a changed message. Confidence: high (spec text).
- ADR 0001 part 13 (:583-600): the profiles P0-P4, B1 hold latency, jitter, loss and a 1 % reorder step; no
  corruption (and per Mustafa 2026-11-24 item 3c the offered simulator has no duplication). L1 = the link simulator
  on simulated time, logic marks only (:1466-1467).
- POINT 6 view: 3c's simulator models corruption as loss only (what both transports deliver, above), so RR-3 cannot
  arise on L1 and the 0-refused marks stand; changed-but-decodable bytes stay the job of any_bytes (hostile peer).
  The endpoint's plausibility bound is robustness against a buggy server (a hostile server is the authority anyway):
  build it with the client's server-time estimate (ADR 0001 part 5 "Time", change-set 7): a snapshot whose tick lies
  more than the estimate + a margin ahead (in 3c's lockstep: newest taken + the endpoint's own ticks since + 30) is
  refused and counted (refused, not late); a newest_command_received newer than the newest sequence sent likewise.
- SOURCE (read 2026-11-25, raw.githubusercontent.com, ValveSoftware/source-sdk-2013 master src/game/server/player.cpp,
  first 200,000 of 265,981 chars; confirms security-auditor's S1 independently): `static ConVar
  sv_maxusrcmdprocessticks( "sv_maxusrcmdprocessticks", "24", FCVAR_NOTIFY,` "Maximum number of client-issued usrcmd
  ticks that can be replayed in packet loss conditions, 0 to allow no restrictions"; PhysicsSimulate: "// Grant the
  client some time buffer to execute user commands" m_nMovementTicksForUserCmdProcessingRemaining++; clamp to nMaxTicks
  with "// still make sure to avoid speedhax"; DetermineSimulationTicks: simulation_ticks clamped to the remaining
  budget. The decrement itself was not in the part read (likely another file). Confidence: high for the bank and its
  cap; medium for exactly where it is spent.
- RUNS 2: cargo clippy --locked --workspace --all-targets: 1 warning, in my probe nc2_probe_endpoint.rs; shipped code
  0. sim 17 + 5 + 1 pass. world.rs (6380df45): PlayerCommand doc (255 presses after an outage; systems apply their
  own per-tick rule) and shared_hash zero-fills and truncates (hash value unchanged: same bytes): fine for 3c.
- PRESS-PACE (DECISIONS.md:660) asked about a tap faster than a tick ("the second tap lands 33 ms later, nothing is
  lost"); the outage loss mod 256 is another path, so no rule loosened; only the follow-up's wording (card :484).
- POINT 5 VIEW: agree with the deferral, the deadline (no room behind a network transport, Slice 3) and the place
  (change-set 7's scheduler seam, build plan :389-391, where PM-0001-20's L1 half runs under both ways, :486). Forms:
  (a) a per-client command budget +1 per room tick, banked to at most REDUNDANCY_CAP (8), one spent per command run,
  over-budget commands dropped and counted outside refused() (Source's bank, above); (b) a command whose stamped tick
  lies more than REDUNDANCY_CAP before the tick being run is dropped as late (late + missing, not refused), catch-up
  only within that window. One mark for both: idle 30 / 300 / 3,000 then a past-dated backlog -> at most 8 extra
  commands; honest outages -> refused 0, <= 7 two-command ticks; honest jitter pairs untouched. PM-0001-21: yes, a
  long-hold case (60 s) at the next ADR edit, with the bound-removed bite; part 5's lag-switch sentence gains the bank.

## FINDINGS (final, round 2)
- NA-1 fixed; NA-2 fixed; NA-3 fixed; NA-4 fixed; NA-5 fixed (follow-ups accepted); NA-6 fell away; NA-7 accepted.
- Queue's window (6380df45): right; invariant proven and probed; no honest change; cost bounded; docs true except
  Stale's wording. Prefer it to the dropped alternative; not both.
- L-1 Low any_bytes.rs:232-245, :351-368: cheap checks missing (per-step identity; endpoint accounting; ack; repeats);
  nit :376 vs :380 echo of another command's presses.
- L-2 Low queue.rs:113-115: ran(sequence) trusts its caller; record the last taken and debug-assert, or pin in cs7.
- L-3 Low queue.rs:26, lib.rs:59: Stale's doc "an honest late repeat" omits far-ahead and before-first cases.
- L-4 Low card :484 follow-up wording: presses are LOST past 255 ticks of outage (1,024 of 2,520 per button at 3,000).
VERDICT: PASS
DONE
