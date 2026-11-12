# SERIES.md: every series and figure name the marks read (netcode proof test)

The naming contract for U2-U7: the code that writes the results log uses these names, and `marks.json` reads them.
Drafted by test-engineer, 2026-11-06, from `marks.json`'s draft by its registry script (every name a mark reads has a
row; the script fails otherwise). Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-67, `0006-n` = PM-0006-n.

## Rules

- A name follows README §1: lowercase with underscores, ending in its unit (`_ns`, `_bytes`, `_bits`, `_permille`, `_count`) or a 0/1 flag with no suffix. Event field names follow the same rule (the draft's choice: README §1 states it for `v` and `s`).
- Two more units, now in README §1: `_um` (micrometres, integer) for lengths and `_microdeg` (millionths of a degree) for angles.
- **Counters and flags are written on every record, 0 included** (the advisor's C2 ruling): a tick or second record carries every one of its names each time, so an empty tick or second series is a gap in the record, never a 0. Only an event that never happened counts 0.
- **`second.s.inputs_count` lists the playing clients only** (C2, C3): one value per client the load drives, in slot order, never a flood, hostile or probe session.
- 'signed' marks a value that may go below 0 (an excess over a bound, a headroom).
- 'Written by' is the crate that measures the value; npt-log's writer thread builds the record. `web` is the TypeScript page and Worker (not a crate). The probe and check tools named as `npt-lab` or `npt-bots (probe)` are placements to confirm at the build.
- One quantity, one name: a name read by several marks is listed once, with all of them.

## Series (235)

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `tick.v.ack_starved_headroom_count` | count, signed | npt-room | event-queue cap minus queue length of the never-acking client (negative = over the cap) | -22 |
| `tick.v.big_vehicles_count` | count | npt-room | big vehicles live (stand-ins included) | -2 |
| `tick.v.block_extrap_over_interval` | flag | npt-client | client: 1 when that extrapolation reached beyond one snapshot interval | -45 |
| `tick.v.block_extrapolated` | flag | npt-client | client: 1 when the block built this tick used extrapolation | -45 |
| `tick.v.cmd_doubled_count` | count | npt-room | commands run twice found this tick | -58 |
| `tick.v.cmd_lost_count` | count | npt-room | commands lost (a sequence that never ran) found this tick | -58 |
| `tick.v.cmd_refused_count` | count | npt-room | commands refused this tick | -58 |
| `tick.v.corrections_count` | count | npt-client | client: reconciliation corrections above 1 mm this tick | -41 |
| `tick.v.datagram_too_large_count` | count | npt-server | snapshot sends refused as above the peer's maximum datagram size (Link's SendResult TooLarge) | -14 |
| `tick.v.dropped_ns` | ns | npt-server | time the loop dropped at this tick's wake (0 on most ticks) | -4 |
| `tick.v.effects_per_soldier_max_count` | count | npt-room | the most active effects any soldier has this tick | -2 |
| `tick.v.lagcomp_ns` | ns | npt-server | the tick's lag-compensation queries (timed inside the step, counted server-only) | -5 |
| `tick.v.lagswitch_cmds_count` | count | npt-room | commands run this tick for the lag-switch client | -21 |
| `tick.v.lagswitch_excess_move_um` | µm (proposed, Q3), signed | npt-lab | the lag-switch client's movement this tick minus CMD_CATCHUP ticks of top-speed movement (0 or less passes) | -21 |
| `tick.v.overrun` | flag | npt-server | 1 when this tick overran its period (the loop's overrun counter's increment) | -4 |
| `tick.v.own_event_misplaced_count` | count | npt-client | client: own, predicted or owner events not in the block written at the tick that emitted or received them | -61 |
| `tick.v.padded_step_ns` | ns | npt-server | the padded sim step's time (PM-0006-14's busy-wait); the `padding` rule compares it with params.padding_target_ns | 0006-14 |
| `tick.v.pair_count` | count | npt-room | enemy pairs tested this tick (768 at 16v16v16) | -9 |
| `tick.v.pair_test_per_pair_ns` | ns | npt-server | the tick's enclosure pair-test time divided by its pair count, rounded up to whole ns (key 1500-1501) | -9 |
| `tick.v.press_count` | count | npt-room | scripted press edges seen this tick | -20 |
| `tick.v.press_doubled_count` | count | npt-room | press edges counted twice this tick | -58, -20 |
| `tick.v.press_lost_count` | count | npt-room | press edges lost this tick (against the press counters) | -58, -20 |
| `tick.v.projectiles_count` | count | npt-room | live projectiles (trajectory records) | -2 |
| `tick.v.rank_pack_ns` | ns | npt-server | ranking and packing for all clients this tick | -8 |
| `tick.v.reflood` | flag | npt-room | 1 when this tick re-flooded the areas after a portal change | -9 |
| `tick.v.region_query_ns` | ns | npt-server | the region query for all clients this tick (interest part a) | -8 |
| `tick.v.remote_event_misplaced_count` | count | npt-client | client: remote spatial events this tick not in the first block whose delayed time reaches their tick | -61 |
| `tick.v.replay_depth_count` | count | npt-client | client: ticks replayed at this tick's reconciliation | -44 |
| `tick.v.server_only_ns` | ns | npt-server | server-only work: commands, lag-compensation queries, interest, encoding and the tick's sends | -5 |
| `tick.v.sim_ns` | ns | npt-server | sim step time: the step's elapsed time minus its lag-compensation queries | -4, -5 |
| `tick.v.smallest_team_count` | count | npt-room | soldiers in the smallest team this tick | -2 |
| `tick.v.snapshot_clients_count` | count | npt-room | soldiers with a connected, snapshot-receiving client | -2 |
| `tick.v.soldiers_count` | count | npt-room | soldiers in the sim this tick | -2 |
| `tick.v.start_jitter_ns` | ns | npt-server | actual minus scheduled start of the tick (S17), from the loop's monotonic clock | 0004-67 |
| `tick.v.tick_allocs_count` | count | npt-server | allocations on the tick thread this tick (counting GlobalAlloc) | 0006-20 |
| `tick.v.unsent_snaps_max_count` | count | npt-server | the most snapshots any client has waiting above the socket (not yet written to it, held by the room or its WebSocket library), sampled each tick | -31 |
| `tick.v.untraced_corrections_count` | count | npt-client | client: corrections above 1 mm this tick NOT traced to a late or lost command (PM-0001-41's count; read by -58 too) | -58, -41 |
| `tick.v.very_large_units_count` | count | npt-room | very large units of at least 20 parts live | -2 |
| `tick.v.wake_ticks_count` | count | npt-server | ticks run in the wake this tick ran in (at most LOOP_CATCHUP = 7 at 30 Hz) | 0004-67 |
| `tick.v.work_ns` | ns | npt-server | server tick time: the tick thread's work from its start to its end (sim step, commands, lag-compensation queries, interest, encoding, sends), measured directly. Also read by V-S's and clean-ref's p99s | -4, -24, -25, -27, 0006-13, 0006-19 |
| `tick.s.cmd_on_time` | flag | npt-room | one per command due this tick: 1 when it arrived before its tick ran | -20 |
| `tick.s.encode_ns` | ns | npt-server | one per client per snapshot: its encode time | -6 |
| `tick.s.input_missing` | flag | npt-room | one per player this tick: 1 when no command for this tick had arrived | -20 |
| `tick.s.nearest8_missing_count` | count | npt-room | one per client per snapshot: of its 8 nearest enemies, how many the snapshot lacks | -8 |
| `tick.s.own_state_sent` | flag | npt-room | one per snapshot sent: 1 when it carries the local player's own state | -23 |
| `tick.s.pitch_err_microdeg` | µdeg (proposed, Q3) | npt-lab | L1: one per decoded pitch: error against the server's value before quantising | -57 |
| `tick.s.pos_err_um` | µm (proposed, Q3) | npt-lab | L1: one per decoded entity position: error against the server's value before quantising | -57 |
| `tick.s.relevant_starved_count` | count | npt-room | one per client per snapshot: relevant entities sent in none of this and the previous 3 snapshots | -8 |
| `tick.s.rewind_ns` | ns | npt-server | one per rewound shot: its query time against the candidates | -7 |
| `tick.s.snap_datagram_bytes` | bytes | npt-server | one per snapshot datagram sent: its payload as handed to send_datagram (the cap rule) | -12, -14 |
| `tick.s.snap_payload_bytes` | bytes | npt-room | one per client per snapshot: the encoded payload | -6 |
| `tick.s.snap_wire_bytes` | bytes | npt-server | one per client per snapshot interval: UDP payload + 28 B per datagram | -8 |
| `tick.s.yaw_err_microdeg` | µdeg (proposed, Q3) | npt-lab | L1: one per decoded yaw: error against the server's value before quantising | -57 |
| `second.v.console_errors_count` | count | web | browser: console errors this second | -32 |
| `second.v.delay_step_abs_ns` | ns | npt-client | client: |change of the interpolation delay| over this second | -46 |
| `second.v.disconnects_count` | count | npt-server | connections that dropped this second | -27 |
| `second.v.early_feedback_count` | count | web | browser: damage numbers or kills shown this second before the Worker had the server's event | -52 |
| `second.v.feedback_shown_count` | count | web | browser: damage numbers and kills shown this second | -52 |
| `second.v.handshake_headroom_count` | count, signed | npt-server | the handshake cap minus handshakes in progress | -25 |
| `second.v.inputs_received_count` | count | npt-server | input datagrams the room received this second (all clients); not V-S's per-client `inputs_count` | -11 |
| `second.v.io_cpu_permille` | permille of one vCPU | npt-server | the transport runtime's threads' CPU time this second | -11 |
| `second.v.one_shots_sent_count` | count | npt-bots | one-shot actions LG's clients sent this second | -38 |
| `second.v.own_cosmetic_sent_count` | count | npt-room | cosmetic events sent to a shooter for its own shots this second | -62 |
| `second.v.page_errors_count` | count | web | browser: page errors this second | -32 |
| `second.v.person_rtt_ns` | ns | npt-server | the server-measured RTT of the person's slot (header params.person_slot) | -3 |
| `second.v.pong_cheat_gap_ns` | ns | npt-lab | the pong delayer's transport RTT minus its ping RTT, sampled each second | -50 |
| `second.v.portal_changes_count` | count | npt-room | portal changes (doors) this second | -2 |
| `second.v.proc_cpu_permille` | permille of one vCPU | npt-server | a room process's whole CPU time this second (density rows); not LG's `cpu_permille` | -27, 0006-13 |
| `second.v.projectile_spawns_count` | count | npt-room | projectiles spawned this second | -2 |
| `second.v.recv_over_max_count` | count | web | browser: datagrams received this second above its current maxDatagramSize | -14 |
| `second.v.reliable_events_count` | count | npt-room | reliable gameplay events emitted this second | -2 |
| `second.v.rewinds_count` | count | npt-room | rewound shots this second | -7 |
| `second.v.rss_bytes` | bytes | npt-server | the room process's resident memory, read each second | -15, -22, -25 |
| `second.v.shots_count` | count | npt-room | hitscan shots fired this second | -2 |
| `second.v.snap_datagrams_sent_count` | count | npt-server | snapshot datagrams sent this second (all clients) | -11 |
| `second.v.stand_in_changes_count` | count | npt-room | stand-in entity changes this second | -2 |
| `second.v.stand_in_spawns_count` | count | npt-room | stand-in entity spawns this second | -2 |
| `second.v.steal_permille` | permille | npt-server | RS: steal time of the measured vCPU(s) this second (V-S reads it from RS's logs) | -1, -27, 0006-13 |
| `second.v.ws_stall_permille` | permille | npt-server | share of this second the WebSocket streams spent stalled | -31 |
| `second.v.ws_stalls_count` | count | npt-server | stalls the WebSocket delay wrapper produced this second | -31 |
| `second.s.buffer_depth_count` | count | npt-room | one per client: its input buffer depth in ticks, sampled each second | -20 |
| `second.s.cmd_sent_bytes` | bytes | npt-proto | one per command: its size as sent, delta-coded (U2) | -13 |
| `second.s.cmd_whole_bytes` | bytes | npt-proto | one per command: its size whole (U1) | -13 |
| `second.s.down_bits` | bits | npt-server | one per client: bits sent to it this second, UDP payload + 28 B per datagram | -12 |
| `second.s.frame_correction_um` | µm (proposed, Q3) | web | one per rendered frame below the 2 m snap: the drawn correction's movement | -42 |
| `second.s.frame_extrap_over_interval` | flag | web | one per rendered frame: 1 when that extrapolation reached beyond one interval | -45 |
| `second.s.frame_extrapolated` | flag | web | one per rendered frame: 1 when drawn from a block built by extrapolation | -45 |
| `second.s.input_cmds_count` | count | npt-server | one per input datagram: commands it carried | -13 |
| `second.s.input_datagram_bytes` | bytes | npt-server | one per input datagram received this second | -13 |
| `second.s.interp_delay_ns` | ns | npt-client | one per client tick: the interpolation delay in use | -46 |
| `second.s.probe_burst_down_lost` | flag | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per probe packet sent down inside the burst: 1 when lost | -3 |
| `second.s.probe_burst_up_lost` | flag | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per probe packet sent up inside B1's 1 s burst: 1 when lost | -3 |
| `second.s.probe_down_delay_ns` | ns | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per delivered probe packet, server to client: its one-way delay | -3 |
| `second.s.probe_down_lost` | flag | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per probe packet sent down, outside the B1 burst: 1 when lost | -3 |
| `second.s.probe_rtt_ns` | ns | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per probe round trip (echo minus hold time, filed under the echoing down packet's second): its time | -3 |
| `second.s.probe_up_delay_ns` | ns | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per delivered probe packet, client to server: its one-way delay | -3 |
| `second.s.probe_up_lost` | flag | npt-lab (L1 probe), npt-server's probe mode (L2 probe, from npt-net's `ImpairSocket` stamps: M15) | one per probe packet sent up, outside the B1 burst: 1 when lost | -3 |
| `second.s.replay_forced_ns` | ns | web | one per replay forced at P3's or P4's measured maximum depth | -44 |
| `second.s.replay_ns` | ns | web | one per forced replay: reset and replay time in the Worker | -44 |
| `second.s.send_call_ns` | ns | npt-server | one per send_datagram call: its time | -11 |
| `second.s.snap_age_ns` | ns | npt-bots | one per client sample: age of the newest snapshot it holds (server send stamp, clock estimate) | -24 |
| `second.s.snap_corr_over_5cm` | flag | npt-client, web | one per snapshot applied: 1 when its correction of the local player exceeds 5 cm (-42, -55) | -42, -55 |
| `second.s.snap_late_after_send` | flag | npt-bots | one per expected snapshot: 1 when lost or later than 66.67 ms after sending (R2's reading) | -37 |
| `second.s.snap_lost_or_late` | flag | npt-bots | one per expected snapshot per client: 1 when lost or decoded more than 66.67 ms after its expected arrival | -37 |
| `second.s.throttle_missing_snaps_count` | count | npt-bots | one per throttled client per throttled second: snapshots missing | -23 |
| `second.s.up_bits` | bits | npt-server | one per client: bits received from it this second, UDP payload + 28 B per datagram | -13 |
| `second.s.ws_extra_delay_ns` | ns | npt-bots, web | one per snapshot decoded over WebSocket: its age at decode minus the profile's one-way delay (50 ms at P2) | -31 |
| `event.ack_starved.dropped` | flag | npt-room | when the server resolves the never-acking client: 1 dropped, 0 resynced | -22 |
| `event.affinity.alone` | flag | npt-server | README §1: the tick thread alone on its vCPU (`when`: start, end) | -1 |
| `event.affinity.sibling_free` | flag | npt-server | README §1: the tick thread's SMT sibling free of the transport runtime (1 also on a 1-thread core) | -1, 0006-11 |
| `event.aim_encoded.size_bits` | bits | npt-proto | per sampled command: the aim field's size before delta coding | -51 |
| `event.attempt.ok` | flag | web | per connection attempt: 1 when the session is ready and its exchange worked (PM-0001-28: datagrams + one bidirectional stream; PM-0006-15 and PM-0006-25: one echo; PM-0006-27: 0 = rejected) | -28, 0006-15, 0006-25, 0006-27 |
| `event.bulk_transfer.size_bytes` | bytes | npt-bots | per bulk transfer on its own stream: its size | -38 |
| `event.burst_heal.after_link_ns` | ns | npt-bots (L2), npt-lab (L1) | per client per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant (a fresh delta; V29, fixes.rs:786-787); fields `slot` and `burst` (below) | -18 |
| `event.canary_grep.address_matches_count` | count | npt-lab | matches for client addresses | 0006-21 |
| `event.canary_grep.canaries_count` | count | npt-lab | canary values it searched for | 0006-21 |
| `event.canary_grep.ip_pattern_matches_count` | count | npt-lab | IPv4/IPv6 pattern matches other than the VMs' own | 0006-21 |
| `event.canary_grep.lines_count` | count | npt-lab | shipped log lines the grep searched | 0006-21 |
| `event.canary_grep.secret_matches_count` | count | npt-lab | matches for guest secrets | 0006-21 |
| `event.canary_grep.seed_matches_count` | count | npt-lab | matches for match seeds | 0006-21 |
| `event.canary_grep.token_matches_count` | count | npt-lab | matches for join tokens | 0006-21 |
| `event.capture.fragments_count` | count | npt-lab | once per run: fragmented IP packets in RS's packet capture (summary of the capture) | -14 |
| `event.cargo_tree.sim_transport_deps_count` | count | npt-lab | transport crates in npt-sim's `cargo tree` | -35 |
| `event.clamp_kind.counted_count` | count | npt-lab | per weapon kind: of those, refused and counted, from the sim's refusal counters only: a clamp report adds nothing to it (a clamped point is neither refused nor counted; the advisor's condition 1 on U4) | -21 |
| `event.clamp_kind.sent_count` | count | npt-lab | per weapon kind: aim points beyond reach sent | -21 |
| `event.conn_panic.closed` | flag | npt-server | per injected connection panic, a minute after: 1 when that connection closed | -26 |
| `event.conn_panic.others_dropped_count` | count | npt-server | the other connections that dropped in that minute | -26 |
| `event.conn_panic.overruns_next_min_count` | count | npt-server | overruns in that minute | -26 |
| `event.connect_clock.offset_err_ns` | ns | npt-lab | per connect: |clock estimate - true clock| after 2 s | -47 |
| `event.counter_wrap.tick_count` | count (a tick number) | npt-lab | per counter that wrapped (`counter` names it): the tick of its wrap | -58 |
| `event.cpu_read.changed` | flag | npt-server | at each session start: 1 when /proc/cpuinfo's model differs from the run's first read | 0006-11 |
| `event.delay_settle.after_change_ns` | ns | npt-client | per profile change: until the delay settled (within +- half an interval of its final value) | -46 |
| `event.edge_cheat_shot.gain_ns` | ns, signed | npt-lab | per tolerance-edge cheat shot: its rewind minus its honest twin shot's | -50 |
| `event.fallback.playing_ns` | ns | web | per fallback attempt: from the page starting to connect to a playing WebSocket session | -29 |
| `event.fast_clock.excess_cmds_count` | count, signed | npt-lab | per 60 s fast-clock episode: commands run minus (server ticks + lead bound) | -21 |
| `event.fast_clock.excess_move_um` | µm (proposed, Q3), signed | npt-lab | per episode: movement minus (honest movement + lead-bound ticks of movement) | -21 |
| `event.feature_detect.webgpu` | flag | web | once per session: 1 when the feature-detection path found WebGPU | -32 |
| `event.forged_kind.sent_count` | count | npt-lab | per forged kind: forged commands sent | -21 |
| `event.forged_kind.unrejected_count` | count | npt-lab | per forged kind: sent minus rejected-and-counted | -21 |
| `event.fuzz_end.corpus_count` | count | fuzz | corpus entries at the end | -34 |
| `event.fuzz_end.inputs_count` | count | fuzz | per fuzz run (one message type): inputs tried | -34 |
| `event.fuzz_end.over_limit_allocs_count` | count | fuzz | allocations above the size limits (counting allocator) | -34 |
| `event.fuzz_end.panics_count` | count | fuzz | panics found | -34 |
| `event.fw_probe.tcp_timed_out` | flag | npt-bots (probe) | TCP connect to the closed port: 1 when it timed out (not refused) | 0006-15 |
| `event.fw_probe.udp_no_reply` | flag | npt-bots (probe) | UDP probe to the closed port: 1 when no reply came | 0006-15 |
| `event.hit_history.size_bytes` | bytes | npt-room | every minute: the hit history's bytes from its own capacity counters | -7 |
| `event.input_latency.to_screen_ns` | ns | web | per scripted input: input event's time stamp to the first frame drawing its effect (`params.kind`: move, flash) | -43 |
| `event.join.from_connect_ns` | ns | npt-bots, web | per join: from the page's first connect call to a complete world | -19 |
| `event.join.world_ns` | ns | npt-bots, web | per join: Join sent to a complete world (client-held hash equal to the server's) | -19 |
| `event.lag_burst.held_ns` | ns | npt-lab | per lag-switch burst: how long its inputs were held | -21 |
| `event.mds_read.size_bytes` | bytes | web | per read (start, then every 10 s): the browser's maxDatagramSize | -14, -36 |
| `event.metric.present` | flag | npt-load | per metric of part 9: 1 when present on MV | 0006-16, 0006-22 |
| `event.metric.stuck_triggered` | flag | npt-load | per metric: 1 when the soak triggered it and it did not move | 0006-22 |
| `event.metrics_probe.answered` | flag | npt-bots (probe) | per probe from ML to the exporter's port: 1 when it answered | 0006-16 |
| `event.no_join_close.age_ns` | ns | npt-server | per session closed for no Join: age from TCP accept or first QUIC packet | -25 |
| `event.over_cap.refused` | flag | npt-server | per over-cap connection from one address: 1 when refused | -25 |
| `event.over_cap_v6.refused` | flag | npt-server | per over-cap connection from the /64 cycler: 1 when refused | -25 |
| `event.own_change.blind_ns` | ns | npt-client | time the client predicted without the change | -60 |
| `event.own_change.corrections_count` | count | npt-client | per scripted own-state change: corrections while it lasted | -60 |
| `event.own_change.first_size_um` | µm (proposed, Q3) | npt-client | the first correction's size | -60 |
| `event.own_change.later_over_1mm_count` | count | npt-client | corrections above 1 mm after that snapshot, every command in time | -60 |
| `event.own_change.misplaced_count` | count | npt-client | corrections not at the first snapshot carrying the change | -60 |
| `event.own_cue.plays_count` | count | npt-client (L1), web (ML) | per own-shot cue key and kind: times it played | -62 |
| `event.ping.lost_count` | count | web (record form) | echoes lost of 100 | 0006-12 |
| `event.ping.max_ns` | ns | web (record form) | max round trip | 0006-12 |
| `event.ping.mean_ns` | ns | web (record form) | the pasted `ping -n 100` output: mean round trip | 0006-12 |
| `event.pong_cheat_shot.gain_ns` | ns, signed | npt-lab | per pong-delayer shot: the same gain | -50 |
| `event.record_item.has_value` | flag | npt-lab | per PM-0001-56 item (`item` names it): 1 when it has a number or value | -56 |
| `event.records_check.crash_report_kept` | flag | npt-super | 1 when the match record and the crash report are kept | -59 |
| `event.records_check.error_log` | flag | npt-super | 1 when the report reached the error log | -59 |
| `event.records_check.match_crashed` | flag | npt-super | after the last injection: 1 when the match record exists and is marked crashed | -59 |
| `event.reflood.encoded` | flag | npt-server | one per re-flood: 1 when it fell on an encoding tick | -9 |
| `event.reflood.time_ns` | ns | npt-server | one per re-flood: its time | -9 |
| `event.refusal.http_403` | flag | web | per ws:// or wrong-Origin probe: 1 when refused with 403 | -25 |
| `event.reload.ok` | flag | web | per reload after a certificate rotation: 1 when it connects with the new hash | 0006-26 |
| `event.remote_event.lead_ns` | ns, signed | web | per remote spatial event: first frame drawing its entity at or past its tick minus the first frame playing it | -61 |
| `event.respawn.slide_frames_count` | count | web | per respawn or teleport: frames drawing the entity more than 0.1 m from both ends | -46 |
| `event.room_exit.after_panic_ns` | ns | npt-super | per room exit: exit time minus the crash record's panic stamp (same machine's monotonic clock) | -59 |
| `event.room_exit.nonzero` | flag | npt-super | per room exit: 1 when the exit code is not 0 | -59 |
| `event.room_start.spawn_to_open_ns` | ns | npt-super | one per room start: process spawn to Open | -16 |
| `event.rotation.dropped_count` | count | npt-server | per certificate rotation: live sessions dropped across it | 0006-26 |
| `event.rule_list.diff_count` | count | npt-lab | per VM role: rules in the pasted list that differ from part 5's list | 0006-15 |
| `event.scrape.loop_counters_present` | flag | npt-load | 1 when the scrape holds the overrun and dropped-time counters | 0004-67 |
| `event.series_count.foreign_labels_count` | count | npt-load | label names outside {VM, room slot, mode id, transport} and Prometheus' target labels | 0006-24 |
| `event.series_count.total_count` | count | npt-load | series from RS's exporter at the soak's end | 0006-24 |
| `event.shot.blend_ns` | ns | npt-room | its part, the blend fraction as time | -49 |
| `event.shot.buffer_wait_ns` | ns | npt-room | its part, the buffer wait | -49 |
| `event.shot.cheat` | flag | npt-room | per shot: 1 when a cheat client fired it | -49 |
| `event.shot.clipped` | flag | npt-room | per shot: 1 when the cap clipped its rewind | -48, -49 |
| `event.shot.delay_ns` | ns | npt-room | its part, the announced delay | -49 |
| `event.shot.frame_wait_ns` | ns | npt-room | its part, the drawn frame to the tick that sent the command | -49 |
| `event.shot.honest_rewind_ns` | ns | npt-room | per honest shot: how far behind the server's present the client's view was | -49 |
| `event.shot.over_cap` | flag | npt-room | per shot: 1 when it was resolved with a rewind above the cap | -49 |
| `event.shot.rewind_ns` | ns | npt-room | per shot: the rewind used | -49 |
| `event.shot.rtt_ns` | ns | npt-room | per shot: its part, the RTT | -49 |
| `event.shot.view_time_replaced` | flag | npt-room | per shot: 1 when the server replaced the view time by its estimate | -48 |
| `event.slow_loris_close.age_ns` | ns | npt-server | per slow-loris client closed: age at close | -25 |
| `event.snap_refused.tick_count` | count (a tick number) | npt-lab (from npt-room's per-client refusal counters) | per snapshot the room's encoder refused for a client (carry 54: that client's snapshot for that tick is skipped; `slot` names the client, `kind` the refusal: `out_of_range`, `too_many`, `too_large` or `not_canonical`): the tick it skipped | -17, -18 (reported) |
| `event.soak_trigger.bad_token_refusals_count` | count | npt-load | bad-token joins refused in the soak | 0006-22 |
| `event.soak_trigger.rooms_killed_count` | count | npt-load | room processes killed in the soak | 0006-22 |
| `event.stall_1s.dropped_ns` | ns | npt-server | per 1 s stall: the dropped-time counter's increase | 0004-67 |
| `event.stall_200ms.recover_ns` | ns | npt-server | per 200 ms stall: until no tick is owed and every later start is within 2 ms | 0004-67 |
| `event.stall_scrape.dropped_err_ns` | ns | npt-load | per stall: |dropped-time increase - (stall - 7 tick periods)| | 0006-23 |
| `event.stall_scrape.overrun_delta_count` | count | npt-load | per stall: the overrun counter's increase at the first scrape | 0006-23 |
| `event.stall_scrape.overruns_before_count` | count | npt-load | overruns before the first stall (the control) | 0006-23 |
| `event.static_shot.err_ground_um` | µm (proposed, Q3) | npt-room | per shot at the ground target: server hit point to the client-picked point | -51 |
| `event.static_shot.err_h2_5m_um` | µm (proposed, Q3) | npt-room | the same at 2.5 m | -51 |
| `event.static_shot.err_h4m_um` | µm (proposed, Q3) | npt-room | the same at 4 m | -51 |
| `event.swap_diff.files_outside_transport_count` | count | npt-lab | files changed outside the transport module by the crate swap's diff | -35 |
| `event.throttle_end.full_rate_ns` | ns | npt-bots | per throttle: from its end to the full snapshot rate back | -23 |
| `event.throw_kind.throws_count` | count | npt-lab, npt-bots | per projectile kind: throws compared | -54 |
| `event.tick_panic.after_warmup_ticks_count` | count | npt-server | the injected tick panic's tick, counted from warm-up's end | -59 |
| `event.worker.cross_origin_isolated` | flag | web | once per session: crossOriginIsolated as the Worker reads it | -43, -44 |
| `event.wss_cert.accepted` | flag | web | per wss session: 1 when the certificate was accepted with no warning | 0006-15 |
| `header.params.aim_bits` | bits | npt-proto | the aim's field widths from the codec | -51 |
| `header.params.area_count` | count | npt-room | the map's area count | -9 |
| `header.params.base_rtt_ns` | ns | npt-net, web | the path's base RTT: median of 10 s of pings at session start | -3 |
| `header.params.bottleneck_queue_datagrams_count` | count | npt-net | the bottleneck's queue depth in full datagrams | -24 |
| `header.params.bottleneck_queue_ns` | ns | npt-net | the bottleneck's queue depth as time at the bottleneck rate | -24 |
| `header.params.buffer_depth_ticks_count` | count | npt-room | -30's input buffer depth set for P0 | -30 |
| `header.params.cert_validity_ns` | ns | npt-server | the WebTransport certificate's validity period | 0006-25, 0006-27 |
| `header.params.history_ticks_count` | count | npt-room | the hit history's depth in ticks | -7 |
| `header.params.lg_ipv6` | bool | npt-load | LG reaches RS over IPv6 | -25 |
| `header.params.quinn_proto_version` | version code | npt-lab | major x 1,000,000 + minor x 1,000 + patch, from the lockfile | -35 |
| `header.params.reorder_permille` | permille | npt-lab (L1, from npt-l1's fate log), npt-net (L2) | the run's measured reorder share, the lower of its two directions (RFC 4737's reordered datagrams per 1000 delivered); `reorder_up_permille` and `reorder_down_permille` beside it (the `reorder` rule reads it, or the `reorder` event's `permille`) | -3, -58, -20 |
| `header.params.rewind_cap_ticks_count` | count | npt-room | the rewind cap in ticks (test cap or rule cap) | -48, -49 |
| `header.params.ring_version` | version code | npt-lab | the same for ring | -35 |
| `header.params.rustls_version` | version code | npt-lab | the same for rustls | -35 |
| `header.params.scrape_interval_ns` | ns | npt-load | Prometheus' scrape interval | 0006-23 |
| `header.params.throttle_queue_bytes` | bytes | npt-net | the drop-tail queue depth for the 128 kbps throttle | -23 |
| `header.params.throttle_queue_datagrams_count` | count | npt-net | the same in full datagrams | -23 |
| `header.params.tick_start` | count (a tick number) | npt-server | the run's first tick (63,736 for an S30) | -17 |
| `header.params.ticks_below_wrap` | count | npt-lab | ticks the run starts below a multiple of 65,536 (-58: 30) | -58 |
| `header.params.tokio_version` | version code | npt-lab | the same for tokio | -35 |
| `header.params.ws_bots_count` | count | npt-load | bots on WebSocket in the soak | 0006-22 |
| `header.params.wt_dangerous_config` | flag | npt-lab | wtransport's dangerous-configuration feature on | -35 |

## Figures (36): computations npt-judge carries in code

| Name | Unit | Written by (crate) | Meaning: formula and inputs | Draft words it implements | Marks |
|---|---|---|---|---|---|
| `alloc_twin_diff_count` | count | npt-judge | count of measured tick numbers where tick.v.tick_allocs_count differs between the run and its params.twin (role logging-off) | PM-0006-20 'on every tick after the warm-up ... equals the count with logging off', 'compared tick by tick' | 0006-20 |
| `bulk_control_p99_rise_ns` | ns | npt-judge | p99(second.s.control_delay_bulk_ns) - p99(second.s.control_delay_idle_ns) | PM-0001-38 'raises the control stream's p99 message delay by at most 5 ms (p99 with the transfer minus p99 without)' | -38 |
| `crash_close_max_ns` | ns | npt-judge | max over the run's clients of (its RoomEnded or close, event.room_end.server_est_ns on LG's server-clock estimate) minus the room's event.tick_panic.server_ns; a client with no room_end fails | PM-0001-59 'every client gets RoomEnded with a crash reason or has its connection closed within that second (0 left open)' | -59 |
| `cross_arch_mismatch_count` | count | npt-judge | the hash kind (built at U4j): count of (client, tick) whose LG aarch64 client's `hs.decoded_hash` differs from `hs.server_hash` while `hs.replay_hash`, the x86-64 re-decode (process `replay`), equals the server's entry; over every tick record of the run, any phase. A run with no `hs.decoded_hash` or no `hs.server_hash` is a gap, never 0; an LG aarch64 mismatch (the room on x86_64) with no replay log in the run is NOT-RUN. The shot view test's and the throw landing's kinds join with the figures that read them (U4h: view_hit_registered_share, false_hit_share, throw_desync_count) | the Arm rule: 'reported as a cross-architecture finding and decides nothing' (-17, -19, -48, -54) | -17, -19, -48, -54 |
| `decoded_hash_compared_count` | count | npt-judge | the client hashes decoded_hash_mismatch_count looked at: every non-null `hs.decoded_hash` entry, paired with the server's or not (0 is a real count here; the mismatch figure fails on it) | PM-0001-17, reported | -17 |
| `decoded_hash_mismatch_count` | count | npt-judge | over every tick record of the run, any phase (-58's wraps fall in its first seconds), per client slot and tick: a non-null `hs.decoded_hash` (each client's hash of the state it holds, under the tick it rebuilt; the sentinel 0 for a tick with a refused group) counts when `hs.server_hash` at that (slot, tick) differs or is absent (a client hash at a tick the server never logged: a wrong tick label); equal hashes count nothing; a server hash with no client hash is loss, not compared (`tick.s.snap_incomplete` counts the ticks a client could not complete). On a client log written on LG (`machine.role` LG) on aarch64, in a run whose room log is on x86_64, a mismatch against a server hash counts only when `hs.replay_hash` (a `replay` log on x86_64) at that (slot, tick) differs from the server's too (the Arm rule; else cross_arch_mismatch_count); a replay log with no entry there is a gap. A client hash with no server hash counts on every machine. Fails, never passes, on 0 compared (the check comes out `empty`: README §2.3). Gaps: no log with `hs.server_hash` or `hs.decoded_hash`, lists of different slot counts, two logs giving one (slot, tick) a hash, a server hash of 0, a name in a log of another process (`server_hash` outside `room`, `decoded_hash` outside `client` and `bots`, `replay_hash` outside `replay` on x86_64); the coverage rule (README §1's `null` rule, checked): every `hs` list and `tick.s.snap_incomplete` list holds the run's `clients` entries; a (slot, tick) is resolved by a client hash (compared) or by `tick.s.snap_incomplete` 1 (lost, not compared), never both; per slot, every tick from its first resolved tick to its last at which the room logged that slot's hash is resolved, and that span covers the room's measured ticks for the slot; a mismatch still fails the run, and 0 compared still comes out `empty`. Inputs: `hs` (README §1; Q30) | PM-0001-17 '100 % of decoded snapshots rebuild the server's quantised state'; -58; -19's join hash; u4-build-plan.md §4.6 (the advisor's Q11 ruling) | -17, -58, -19 |
| `down_client_mean_max_bits` | bits | npt-judge | max over client slots of the mean of second.s.down_bits[slot] over the measured seconds | PM-0001-12 'mean at most 160 kbps ... counted per client' | -12 |
| `down_mean_ipv6_bits` | bits | npt-judge | mean over client-seconds of second.s.down_bits + 160 x second.s.down_datagrams_count (20 B more header per datagram) | PM-0001-12 'if LG reaches RS over IPv6, the IPv6 figure' | -12 |
| `egress_interface_bytes` | bytes | npt-judge | event.iface.tx_bytes at the end minus at the start | PM-0006-17 'the interface's transmitted-byte counter read at the start and end' | 0006-17 |
| `egress_metric_vs_interface_ratio` | fraction (exact) | npt-judge | (event.room_metric.sent_bytes at the end - at the start) / egress_interface_bytes | PM-0006-17 'the room's own bytes-sent metric agrees with the interface counter within 5 %' | 0006-17 |
| `egress_vs_r2_ratio` | fraction (exact) | npt-judge | egress_interface_bytes / r2_egress_formula_bytes | PM-0006-17 'within +-20 % of R2's E3 estimate at the counts that ran' | 0006-17 |
| `everyone_in_view_min_share` | fraction (exact) | npt-judge | min over client slots of the share of that slot's snapshots with tick.s.everyone_in_view = 1. Inputs: tick.s.everyone_in_view (index = slot) | PM-0001-2 'every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots' | -2 |
| `false_hit_share` | fraction (exact) | npt-judge | among view misses (event.view_test.hit = 0), the share registered as hits; Arm rule on L2 as above | PM-0001-48 'false hits (a miss in the view counted as a hit) below 0.5 %' | -48 |
| `fate_log_twin_equal` | 0/1 | npt-judge | 1 if header.params.fate_log_sha256 equals that of the run named in header.params.twin, 0 if they differ. The twin must be in the same set, with role twin and the same layer, profile, seed and bite (a bite run's twin runs the same bite); a twin not in the set yet leaves the check NOT-RUN; a run naming no twin, a twin that differs in those fields or is not role twin, and a hash that is not 64 lowercase hex digits are gaps (built in U2); so is a twin that breaks a rule every run holds (README §4: complete, roster, capability, and C4's build rule, added at U4j) | PM-0001-3 'on L1, the same seed and profile give an identical packet-fate log on two runs' | -3 |
| `input_latency_p50_rise_ns` | ns | npt-judge | the same with medians | PM-0001-43 'and the same for the medians' | -43 |
| `input_latency_p99_rise_ns` | ns | npt-judge | max over kinds (params.kind: move, flash) of p99(event.input_latency.to_screen_ns at P4) - p99(at P0), over the part's runs | PM-0001-43 'p99 at P4 minus p99 at P0 at most 33.33 ms' | -43 |
| `kill_p99_ratio` | fraction (exact) | npt-judge | p99 of tick.v.work_ns over the ticks after the set's `kill` event (ph = after) / p99 over the measured ticks | PM-0001-27 'killing one room's process (SIGKILL) leaves the others' p99 within 5 %' | -27 |
| `logging_cost_ratio` | fraction (exact) | npt-judge | p99 over the pooled measured ticks of the part's 3 runs / p99 over the pooled ticks of the set's 3 role logging-off runs (tick.v.work_ns) | PM-0006-19 'the pooled p99 of the 3 on runs (54,000 ticks) with that of the 3 off runs', 'at most 1.02 x' | 0006-19 |
| `machine_named` | 0/1 | npt-judge | 1 if every log's header `machine` has role, name, plan, arch, cpu, location and kernel not empty, vcpus, mem_mb, threads_per_core and clock_res_ns above 0, and an SMT map (`smt`) whose every sibling list is filled, and a VM (RS, LG, MV) its day address (`address` not empty); else 0 (built at U4j). Inputs: headers | PM-0001-1 'RS and LG named with plan, CPU model, vCPU count, memory, location, kernel and the day's address'; PM-0006-11's list | -1, 0006-11 |
| `one_build_hash` | 0/1 | npt-judge | 1 if every run of the mark has the same client build (build.binary_sha256 of the web bundle) | PM-0001-32 'the same build (one hash)' | -32 |
| `one_shot_dropped_count` | count | npt-judge | sum of LG's second.v.one_shots_sent_count minus the room's second.v.one_shots_received_count over the run | PM-0001-38 '0 of 10,000 scripted one-shot actions dropped' | -38 |
| `queue_growth_count` | count | npt-judge | per client slot, for second.s.out_queue_count and second.s.event_queue_count: count of (slot, queue) whose max over the last 10 measured minutes exceeds its max over the first 10 | PM-0001-15 'its max over the last 10 minutes at most its max over the first 10 minutes' | -15 |
| `r2_down_formula_bits` | bits | npt-judge | R2's worked downstream formula (ADR 0001 lines 1754-1779; r2 lines 1764-1766) at the run's own counts: mean entities per client set (tick.s.relevant_count), shots, spawns and events per second (second.v.shots_count, projectile_spawns_count, reliable_events_count); the arithmetic printed | PM-0001-12 'its mean reported beside R2's formula evaluated at the run's own counts' | -12 |
| `r2_egress_formula_bytes` | bytes | npt-judge | R2's formula (as r2_down_formula_bits) x clients x 3,600 / 8 (at R2's counts 146.1 kbps x 48 x 3,600 / 8 = 3.156 GB) | PM-0006-17 'R2's formula ... evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown' | 0006-17 |
| `rooms_per_vm_count` | count | npt-judge | per N: per session, the largest k (params.k) for which every room run meets p99 <= 16,670,000 ns, max < 33,330,000 ns (whole run), 0 overruns and p99 <= 1.10 x the bracket mean; the figure is the lowest over the 3 sessions | PM-0006-14 'the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2'; 'the figure is the lowest count of the 3 sessions' | 0006-14 |
| `rss_slope_bytes_per_10min` | bytes per 10 min, signed | npt-judge | least-squares slope of second.v.rss_bytes over the measured seconds, x 600 (proposed method) | PM-0001-15 'slope within 1 MB per 10 minutes after warm-up' | -15 |
| `rtt_agreement_p99_ns` | ns | npt-judge | p99 over the measured seconds of |second.v.app_rtt_ns (browser) - second.v.quinn_rtt_ns (room, that session)|, joined by second on the client's server-clock estimate | PM-0001-33 'p99 of the absolute difference between app-level RTT and quinn's rtt(), sampled every second' | -33 |
| `rule_cap_set_by_rule` | 0/1 | npt-judge | 1 if header.params.rewind_cap_ticks_count = ceil(p99 of event.shot.honest_rewind_ns in the picked option's P2 test-cap runs / (10^9 / tick_hz)) | PM-0001-49 'the honest rewind's measured p99 at P2, rounded up to a whole number of ticks' | -49 |
| `session_memory_growth_count` | count | npt-judge | the same rule on halves of the measured window, over second.s.session_buffered_bytes, out_queue_count and event_queue_count | PM-0001-24 'memory per session flat over 10 minutes (PM-0001-15's rule)' | -24 |
| `sync_cost_ratio` | fraction (exact) | npt-judge | U = mean(tick.v.server_only_ns) / (header.clients x mean(tick.s.relevant_count)); per N = 48 run: max over N in {24, 36} of U(run) / mean of U over that N's part. Inputs: the mark's three parts | PM-0001-10 'at N = 48 at most 1.1 x the same figure at N = 24 and at N = 36' | -10 |
| `sync_unit_cost` | fraction (exact) | npt-judge | U per run, as above (ns per client-entity, a decimal) | PM-0001-10, reported | -10 |
| `throw_desync_count` | count | npt-judge | throws with event.throw.desync = 1; on L2 only those whose x86-64 replay (process replay) also desyncs | PM-0001-54 '0 desyncs of client-side flight against the server over 1,000 throws, per projectile kind' | -54 |
| `times_of_day_rule` | 0/1 | npt-judge | 1 if the mark's sessions (sets) start (header.params.started_utc) pairwise at least 6 h apart and at least one starts between 18:00 and 23:00 Europe/Amsterdam | ADR 0006's key: 'three sessions at least 6 hours apart, one of them between 18:00 and 23:00 Amsterdam time'; 'npt-judge checks the 6-hour and 18:00-23:00 rule from the results' UTC times' | -27, 0006-13, 0006-14 |
| `twin_hash_mismatch_count` | count | npt-judge | count of tick numbers, over every tick record of both runs (any phase), whose `h` differs between the run and the run named in header.params.twin (a tick hashed in one only counts); fails, never passes, on 0 ticks hashed in either (`empty`). The twin: in the same set, the same profile, seed and bite, role `twin` (-21's twins; -30's form, each other's measured runs differing in `params.transport`, comes with U6: C-U6-9), not the run itself, and holding the four rules every run holds (README §4, the build rule among them: C4); a twin of another role, a twin that breaks one, a run naming no twin, a malformed `h` and two logs giving one tick two hashes are gaps; a twin not in the set yet is NOT-RUN (built at U4j) | PM-0001-21 'the state hash of a run with the forged set equals that of a run with only their valid parts'; -30 'identical server state hashes on every tick' | -21, -30 |
| `up_client_mean_max_bits` | bits | npt-judge | the same over second.s.up_bits | PM-0001-13 'mean at most 48 kbps per client' | -13 |
| `view_hit_registered_share` | fraction (exact) | npt-judge | among shots whose shooter's view test hit (event.view_test.hit = 1), the share the server registered (event.shot.registered = 1), joined by shot id; on L2 a non-registered view hit counts only if the x86-64 replay's view test also hits | PM-0001-48 'of the shots whose ray hits the target's volume in the shooter's view ... at least 99 % register' | -48, -49 |

### Names added with the advisor's U1 rulings (2026-11-06)

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `event.worker_trap.tick` | tick number | web | the harness logs each Worker trap it injects, with the server tick it was injected at (Q12) | -19 |
| `event.pong_delayer.hold_ns` | ns | npt-bots | the 150 ms pong delayer's cheat client logs each pong it held, with the hold (Q21) | -49 |
| `untimed_trap_count` | count | npt-judge (figure, U7) | Worker traps with no timed rejoin (`event.join.world_ns`) after them and before the next trap (Q12) | -19 |
| `everyone_in_view_min_share` | fraction (exact) | npt-judge (figure, built in U1) | as below; built in U1 because PM-0001-2 is the precondition of the stress-scene marks (B8) | -2 |
| `header.params.started_utc` | ISO 8601 UTC | the run script | the run's UTC start, with seconds and `Z`; the times-of-day rule's input (B14) | -18, -27, 0006-13, 0006-14 |
| `header.params.cc` | `newreno` or `cubic` | the room unit's configuration | the congestion controller, a start-up setting never changed in a run (A2) | -24 |
| `header.params.bite_size_ns`, `header.params.bite_pad_bytes` | ns, bytes | the bites build | a bite's B-S or B-N size, printed beside the bite, never judged (B10) | every B-S or B-N bite |

### Names added with U4j (u4-build-plan.md rev 2 and the advisor's U4 rulings; PM-0001-18's split, advisor-ws-docs-r2.md:374-379; 2026-11-10)

`hs` lists (README §1) are read by the decoded-hash figures only; identifiers (`slot`, `burst`, `seq`) name an event's subject and are not measured values. -18's reports also read `event.burst_return.outstanding_bytes` and `srtt_ns` (DelayStream's rows, below; U6's writer adds what is held above DelayStream to `outstanding_bytes`, NC-U3c-4). The practice run is a header `role`, `practice` (README §1).

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `hs.server_hash` | 16 hex digits or null, per client slot | npt-room (the encoder's mirror, §4.6) | at tick t: the server's hash (`npt_proto::client_hash`) of what it meant that client to hold after decoding t; never 0; written only in the room's log (process `room`); null where it sent that client nothing for t | -17, -19, -58 (figures) |
| `hs.decoded_hash` | 16 hex digits or null, per client slot | npt-client (L1, through npt-lab's client log), npt-bots (L2), web (browser) | only in the clients' logs, process `client` on L1 and `bots` on L2, at the tick t the client rebuilt: its hash of the state it holds for t (its held floats re-gridded); the sentinel 0 for a tick with a received group it could not decode; null for none (a group lost, none refused) | -17, -19, -58 (figures) |
| `hs.replay_hash` | 16 hex digits or null, per client slot | npt-lab (`replay`, process replay: the x86-64 re-decode of an L2 client's logged bytes, R-8) | only in a `replay` log on x86_64: the same as `decoded_hash`, decoded again natively; read for the Arm rule | -17, -19 (figures) |
| `tick.s.snap_incomplete` | flag | npt-client (L1, through npt-lab's client log), npt-bots (L2) | one per client slot (the run's `clients` entries, each 0 or 1), at the tick it resolves: 1 when a group of that tick was lost and none refused, or the client dropped one as stale (README §1's `decoded_hash`), so the client logs no hash for it (§4.6), a tick none of whose groups arrived included (known from a later header's tick); with `hs.decoded_hash` it decides README §1's coverage rule (`decoded_hash_mismatch_count`) | -17, -58, -19 (the coverage rule; -17 reports it) |
| `event.burst_eviction.evicted` | flag | npt-room | per client per burst (fields `slot`, `burst`): 1 when the client's last acknowledged baseline had left the AckRing (ages 0 to 31) at the first tick after the burst, else 0; shows whether -18's no-fallback bite could bite (§4.5) | -18 (recorded) |
| `event.clamp_report.x_um`, `.y_um`, `.z_um` | µm | npt-room (from npt-sim's `reports()`) | per clamp report (fields `slot`, `seq`): the aim point the sim clamped to the weapon's reach, its grid cm x 10,000; read from `reports()`, never from the hashed event list; the clamp twin sends exactly these points (§4.10, the advisor's condition 1) | -21 (the clamp twin's input) |
| `event.burst_heal.burst` | identifier (0 to 9) | npt-bots (L2), npt-lab (L1) | the burst's index in the run, beside `slot` on each heal record | -18 |
| `event.burst_return.offset_ns` | ns | npt-server | per burst: its start's random offset of 0 to one tick period, drawn from the run's seed | -18 (recorded) |
| `event.burst_return.tick_phase_ns` | ns | npt-server | per burst: the return minus the room's last tick start | -18 (recorded) |
| `event.burst_return.budget_bytes` | bytes | npt-server | per WebSocket client per burst: the send budget at the return | -18 (reported) |
| `event.burst_return.snap_bytes` | bytes | npt-server | per WebSocket client per burst: the last written snapshot's size | -18 (recorded) |
| `tick.v.ws_skipped_snaps_count` | count | npt-server | snapshots the WebSocket send budget skipped this tick, all clients | -18 (recorded) |
| `event.ws_practice.class`, `.m_p_ns`, `.bound`, `.bound_file`, `.bound_sha256`, `.x_moved_ns`, `.y_moved_ns`, `.z_moved_ns`, `.k_count`, `.chance_ruling`, `.second_ruling` | README §2.5 | the run script (ops/runs, R3) | the WebSocket practice run's outcome record, its fields and their units as README §2.5 fixes them; read by the `ws-practice` rule | -18 (the rule) |
| `header.params.burst_spacing_ns` | ns | npt-lab (L1), the run script (L2) | B1's scheduled interval between burst starts, before each burst's offset; a practice run and its counted run share it | -18 (the rule) |
| `header.params.config_hz` | Hz | the run script (L2) | RS's kernel CONFIG_HZ (the ruling's `machine.config_hz`, as a header param: README §2.5) | -18 (the practice record's header checks) |

## Series read only by figures (18)

| Name | Unit | Written by (crate) | Meaning | Figures |
|---|---|---|---|---|
| `tick.s.everyone_in_view` | flag | npt-room | one per client per snapshot: 1 when its set holds all 47 other soldiers (ES: and all 15 vehicles) | everyone_in_view_min_share |
| `tick.s.relevant_count` | count | npt-room | one per client: entities in its relevant set this tick | sync_cost_ratio, sync_unit_cost, r2_down_formula_bits |
| `second.v.app_rtt_ns` | ns | web | browser: the app-level (ping) RTT, each second | rtt_agreement_p99_ns |
| `second.v.one_shots_received_count` | count | npt-room | one-shot actions the room received this second | one_shot_dropped_count |
| `second.v.quinn_rtt_ns` | ns | npt-server | room: quinn's rtt() for the session, each second | rtt_agreement_p99_ns |
| `second.s.control_delay_bulk_ns` | ns | npt-bots | one per control-stream message while a bulk transfer is in flight: its delay | bulk_control_p99_rise_ns |
| `second.s.control_delay_idle_ns` | ns | npt-bots | the same with no transfer in flight | bulk_control_p99_rise_ns |
| `second.s.down_datagrams_count` | count | npt-server | one per client: snapshot datagrams sent to it this second | down_mean_ipv6_bits |
| `second.s.event_queue_count` | count | npt-room | one per client: unacknowledged events queued, each second | queue_growth_count, session_memory_growth_count |
| `second.s.out_queue_count` | count | npt-server | one per client: outbound queue length, each second | queue_growth_count, session_memory_growth_count |
| `second.s.session_buffered_bytes` | bytes | npt-server | one per session: bytes buffered for it, each second | session_memory_growth_count |
| `event.iface.tx_bytes` | bytes | npt-server | RS's interface transmitted-byte counter (read at start and end) | egress_interface_bytes |
| `event.room_end.server_est_ns` | ns | npt-bots | per client: its RoomEnded or close, on its server-clock estimate | crash_close_max_ns |
| `event.room_metric.sent_bytes` | bytes | npt-server | the room's bytes-sent metric (read at start and end) | egress_metric_vs_interface_ratio |
| `event.shot.registered` | flag | npt-room | per shot (`shot` id): 1 when the server registered a hit | view_hit_registered_share, false_hit_share |
| `event.throw.desync` | flag | npt-bots, npt-lab (replay) | per throw: 1 when the client's landing differs from the server's | throw_desync_count |
| `event.tick_panic.server_ns` | ns | npt-server | the injected tick panic's time on the room's clock | crash_close_max_ns |
| `event.view_test.hit` | flag | npt-bots, npt-lab (replay) | per shot (`shot` id): 1 when the ray hits the target's volume in the shooter's view | view_hit_registered_share, false_hit_share |

## Names the judge's validity rules read (README §4; the coordinator's list, 2026-11-06)

| Name | Unit | Written by (crate) | Rule | In this draft |
|---|---|---|---|---|
| `tick.v.work_ns` | ns | npt-server | V-S's and clean-ref's p99s; PM-0006-13's bracket | the same name as every tick-time check: one name |
| `second.v.steal_permille` | permille | npt-server | V-S's steal (RS's logs) | the same name in -1 and the density reports |
| `second.s.inputs_count` | count | npt-bots | V-S: one per LG client, inputs sent this second, against tick_hz | not read by any check; the room's own count is second.v.inputs_received_count (a different side, all clients) |
| `second.v.cpu_permille` | permille of the machine | npt-bots, npt-load | V-S, lg-80, PM-0006-13: each LG machine's CPU (< 800) | the room process's share is second.v.proc_cpu_permille (a different quantity) |
| `tick.v.padded_step_ns` | ns | npt-server | padding: against header.params.padding_target_ns | the same name in PM-0006-14's report |
| `tick.v.late_count` | count | npt-room | late-zero: sums to 0 | not read by any check (-30 names it in its notes) |
| `header.params.reorder_permille / event reorder (`permille`)` | permille | npt-lab (L1), npt-net (L2) | reorder: above 0 | the same name in the -3, -20, -58 reports |

## Header params the parts select on and the notes name (README §1's `params`)

| Param | Values | Used by |
|---|---|---|
| `soak` | S30, S60 or none | -4 to -16, -17, PM-0004-67 (selects) |
| `sim` | stress or stand-in | -4's STAND-IN |
| `stand_ins` | list of names | -2 (reported in the detail: Q16) |
| `cmd_timing` | A or B (OQ-0001-6) | -20, -21, -42, -48, -49 |
| `suite` | the run's test suite (wrap, joins, forged, clamp, fast-clock, lag-switch, never-ack, throttle, bottleneck, flood, origin, conn-panic, tick-panic, wt-attempts, fallback, fallback-443, both-transports, ws-stall, one-build, rtt, rtt-fixed-100, fuzz, deps, mds, datagram-timing, one-shots, brawl, own-change, input-latency, replay, extrapolation, delay-adapt, events, clock, rewind-cheat, aim-height, feedback, cues, checklist, throws, beside-vehicle, record, stalls, ping, firewall, fw-probe, rule-list, metrics-probe, egress, bill, logging-cost, logging-alloc, soak-8, cert-14d, cert-15d, rotation; and precheck-30, the -30 pre-check's recorded npt-lab test, whose headers list no mark, so no part selects it) | selects throughout |
| `browser` | chrome, edge, firefox, safari, brave | browser parts |
| `backend` | webgpu or webgl2 | -32 |
| `transport` | wt or ws | -18, -25, -50, PM-0006-15 |
| `crate` | wtransport or web-transport-quinn | -28 (and -11, -25, -35 for -56's item 1) |
| `path` | loopback, eu-path, internet | -28, -36 |
| `cc` | newreno or cubic | -24 |
| `addr_cap` | raised or on | -25, -29 (recorded) |
| `lg_ipv6` | bool | -25 |
| `density` | 8-rooms or count | -27, PM-0006-13, -14 |
| `session` | D1, D2, D3 | -27, PM-0006-13 |
| `k` | rooms in this step of the search | PM-0006-14 |
| `killed` | bool: the run kills this room on purpose (exempt from `complete`) | -27, -59, PM-0006-22 |
| `twin` | the run name of its twin | -3, -21, -30, PM-0006-20 |
| `cap` | test or rule | -48, -49 |
| `msg` | message type | -34 |
| `replay_depth` | 6 or 7 | -44 |
| `interest` | on or off | PM-0006-17 |
| `person_slot` | the person's slot | -3 |
| `kind` | move or flash | -43 |
| `collision_rule` | string | -42 (recorded) |
| `fate_log_sha256` | hex string | -3 |
| `reorder_up_permille`, `reorder_down_permille` | each direction's measured reorder share: RFC 4737's reordered datagrams per 1000 delivered (`reorder_permille` is the lower of the two) | -3 (recorded), the `reorder` rule |
| `reorder_step_permille` | the reorder step's draw share (part 13's 10, on in every run), recorded beside the profile | -3 (recorded) |
| `probe_rate_pps` | the probe's datagrams per second per direction (10,000) | -3 (recorded) |
| `probe_sent_count` | the probe's datagrams sent per direction, bursts included (1,000,000, or 1,010,000 at B1) | -3 (recorded) |
| `burst_starts_ns` | B1's burst starts on the run's clock (a list; each burst lasts 1 s) | -3 (recorded) |
| `padding_target_ns` | ns | padding rule |
| `event_queue_cap_count` | count | -22 (reported) |
| `started_utc` | ISO 8601 UTC time the caller passes (proposed, Q25) | times_of_day_rule |

Machine names in selects: `machine.name` `RS`, `ML` (the named laptop), `ML*` (its stand-in), `MAC-r` (the runner), `MAC-b` (a borrowed Mac); `machine.role` `RS`, `LG`, `MP`, `MAC`, `ML`, `C`, and `RD` (not in README §1's list: Q6). `process`: `room`, `bots`, `browser`, `super`, `load`, `probe`, `replay`.

## L2's impairment layer (U3b, `ImpairSocket`; the advisor's M15): names no mark reads yet

What `ImpairSocket` measures, under the names its own code reports (npt-net's `l2::names`, checked against README §1's
rule by a unit test). Up is inbound (client to server: the socket sits on RS), down outbound.

**The `l2_summary` event** is written once at the run's end. Its fields: every per-second name below except
`l2_release_cpu_permille`, over the whole run (the counts as totals, the lateness quantiles over every sample, and the
levels `l2_peers_count`, `l2_pool_free_count`, `l2_up_queued_bytes` and `l2_down_queued_bytes` as at the run's end);
then `l2_release_cpu_ns` (the release thread's CPU over the run, in place of the per-second permille) and the measured
reorder shares under the existing names `reorder_up_permille`, `reorder_down_permille` and `reorder_permille`. A unit
test checks every field it writes against this file. The event's lateness fields cover the socket's whole life; R1's
bar reads the probe's own computation (U6). `base_rtt_ns` and `reorder_step_permille` (rows above) are written by
`ImpairSocket` too.

**Done at U3c (M15):** the TCP model's rows (the model's name and version, and S12's stall and repair counts) are in
the DelayStream section below, which DelayStream writes.

**The L2 probe's writing binary (PM-0001-3's `second.s.probe_*` rows above; proposed at U3b, built at U6):**
npt-server's binary in a probe mode (process `probe`), on a free slot port of RS with its own `ImpairSocket`, writes
every probe series from the socket's stamps on RS's clock (the drain stamp and the release of each datagram, both
directions, and each round trip as the echo's release minus the original send); its pacer is a dedicated thread with
1 ns slack, never tokio (M13). LG's side, npt-bots (probe), sends up, echoes, and writes no probe series.

### Per second (`second.v`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `second.v.l2_up_delivered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams released this second: sent on the real socket (down) or handed to quinn by `poll_recv` (up) |
| `second.v.l2_up_lost_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams the profile's loss draw dropped |
| `second.v.l2_up_burst_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams dropped inside one of the profile's B1 bursts |
| `second.v.l2_up_blocked_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams dropped by PM-0001-29's UDP block |
| `second.v.l2_up_shaper_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: tail drops at the shaper's drop-tail queue (PM-0001-23's throttle, -24's bottleneck; M19) |
| `second.v.l2_up_queue_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: tail drops at the byte bounds (per peer and in total, 1,500 B charged per datagram; M10), counted apart from the profile's loss |
| `second.v.l2_up_pool_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: drops for an empty buffer pool (M11) |
| `second.v.l2_up_oversize_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams above the size limit, dropped before they were queued (M10) |
| `second.v.l2_up_overtaken_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams the reorder step held and their successor overtook (npt-l1's step as built: M3) |
| `second.v.l2_up_reordered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: delivered datagrams released after a later-indexed datagram of their flow (RFC 4737), the impairment layer's reorder count |
| `second.v.l2_late_up_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): release-lateness samples this second |
| `second.v.l2_late_up_p50_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): the release lateness's median (L0 for this second) |
| `second.v.l2_late_up_p99_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its 99th percentile |
| `second.v.l2_late_up_p999_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its 99.9th percentile |
| `second.v.l2_late_up_max_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its maximum |
| `second.v.l2_late_up_excess_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+], per second, reported (L0 = that second's median); the judged bar is E with L0 = 0, ≤ 10,000 ns per direction (U3D-BAR, U3D-L0), over the probe's P1 measured records at R1 (plan §12.1 U8), never per second |
| `second.v.l2_late_up_over_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): samples with L − L0 above 0.25 ms |
| `second.v.l2_down_delivered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams released this second: sent on the real socket (down) or handed to quinn by `poll_recv` (up) |
| `second.v.l2_down_lost_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams the profile's loss draw dropped |
| `second.v.l2_down_burst_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams dropped inside one of the profile's B1 bursts |
| `second.v.l2_down_blocked_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams dropped by PM-0001-29's UDP block |
| `second.v.l2_down_shaper_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: tail drops at the shaper's drop-tail queue (PM-0001-23's throttle, -24's bottleneck; M19) |
| `second.v.l2_down_queue_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: tail drops at the byte bounds (per peer and in total, 1,500 B charged per datagram; M10), counted apart from the profile's loss |
| `second.v.l2_down_pool_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: drops for an empty buffer pool (M11) |
| `second.v.l2_down_oversize_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams above the size limit, dropped before they were queued (M10) |
| `second.v.l2_down_send_err_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: sends the kernel refused with an error other than EAGAIN (quinn-udp's `try_send`, never `send`: M5); the errno is in the fate log |
| `second.v.l2_down_would_block_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: sends refused with EAGAIN (the socket's send buffer full): a host drop, never retried |
| `second.v.l2_down_err_msgsize_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class EMSGSIZE |
| `second.v.l2_down_err_nobufs_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class ENOBUFS |
| `second.v.l2_down_err_perm_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class EPERM or EACCES (a firewall rule) |
| `second.v.l2_down_err_unreach_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class ENETUNREACH, EHOSTUNREACH or EADDRNOTAVAIL |
| `second.v.l2_down_err_refused_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class ECONNREFUSED |
| `second.v.l2_down_err_other_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: the send errors of class any other errno |
| `second.v.l2_down_overtaken_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams the reorder step held and their successor overtook (npt-l1's step as built: M3) |
| `second.v.l2_down_reordered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: delivered datagrams released after a later-indexed datagram of their flow (RFC 4737), the impairment layer's reorder count |
| `second.v.l2_late_down_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): release-lateness samples this second |
| `second.v.l2_late_down_p50_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): the release lateness's median (L0 for this second) |
| `second.v.l2_late_down_p99_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): its 99th percentile |
| `second.v.l2_late_down_p999_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): its 99.9th percentile |
| `second.v.l2_late_down_max_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): its maximum |
| `second.v.l2_late_down_excess_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+], per second, reported (L0 = that second's median); the judged bar is E with L0 = 0, ≤ 10,000 ns per direction (U3D-BAR, U3D-L0), over the probe's P1 measured records at R1 (plan §12.1 U8), never per second |
| `second.v.l2_late_down_over_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): samples with L − L0 above 0.25 ms |
| `second.v.l2_up_queued_bytes` | bytes | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound: buffer bytes queued at the second's end, 1,500 B per datagram whatever its payload (each holds one pool buffer: M10's charge), the datagrams in their delay, held by the reorder step, or due and waiting for quinn's `poll_recv` |
| `second.v.l2_down_queued_bytes` | bytes | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: buffer bytes queued at the second's end, 1,500 B per datagram whatever its payload (M10's charge), the datagrams in their shaper wait and delay, held by the reorder step, or in the release thread's batch being sent |
| `second.v.l2_peers_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | peers in the table at the second's end (fixed capacity: M10) |
| `second.v.l2_peer_evicted_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | peers evicted this second (the least recently seen, never one set by the control plane); only a datagram that passed the host checks (size, byte bounds, pool) takes a slot, so host drops evict nobody |
| `second.v.l2_peer_overflow_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | datagrams from new addresses with every slot pinned, carried by the shared overflow flow |
| `second.v.l2_fate_log_full_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | fate-log entries not kept: the log was at its fixed capacity |
| `second.v.l2_recv_err_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | errors draining the kernel socket (an ICMP error reported on it, ...): counted, never fatal |
| `second.v.l2_pool_free_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | free pool buffers at the second's end (M11) |
| `second.v.l2_release_cpu_permille` | permille of one core | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | the release thread's CPU this second, from its schedstat (M13); per second only (the `l2_summary` event carries `l2_release_cpu_ns` instead) |

### Header params (`header.params`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `header.params.l2_mode` | string | npt-net (ImpairSocket) | `impaired`, or `pass-through` (every impairment off, through the same socket, release thread and slack: M12) |
| `header.params.l2_sched_policy` | string | npt-net (ImpairSocket) | the release thread's scheduling policy and real-time priority read from /proc when the header is built (`SCHED_OTHER 0`; `SCHED_FIFO 50` when the recorded switch was applied from outside with `chrt`: M14) |
| `header.params.l2_timer_slack_ns` | ns | npt-net (ImpairSocket) | the release thread's timer slack as the thread itself reads its own `/proc/<tid>/timerslack_ns` when the header is built, beside `l2_sched_policy` read then (1; 0 under a real-time policy since kernel commit ed4fb6d7ef68, mainline 6.12 and 6.6.84; 50,000 after a return to SCHED_OTHER; u64::MAX unreadable or unanswered within 1 s: M14) |
| `header.params.l2_timer_slack_start_ns` | ns | npt-net (ImpairSocket) | the same slack as the release thread read it back at its start, right after writing it (1 written; u64::MAX unreadable: M14) |
| `header.params.l2_timer_slack_set` | flag | npt-net (ImpairSocket) | 1 when that write succeeded |
| `header.params.l2_timer_slack_error` | string | npt-net (ImpairSocket) | the write's error, present only when it failed |
| `header.params.l2_migration` | flag | npt-net (ImpairSocket) | QUIC migration in both servers' configurations: 0, off (M9; Mustafa's U3-Q6 (a)) |
| `header.params.l2_gso` | flag | npt-net (ImpairSocket) | GSO/GRO as quinn sees ImpairSocket: 0, off (one datagram per Transmit: M11) |
| `header.params.l2_max_datagram_bytes` | bytes | npt-net (ImpairSocket) | the size limit applied before any queueing (1,472 B by default: M10) |
| `header.params.l2_pool_buffers_count` | count | npt-net (ImpairSocket) | the fixed pool of 1,500 B buffers, shared by every peer and both directions (M11; 8,192 by default: both directions' totals) |
| `header.params.l2_peer_queue_bytes` | bytes | npt-net (ImpairSocket) | one peer's bound per direction in buffer bytes, every queued datagram charged its 1,500 B buffer whatever its payload (M10; 3,072,000 B = 2,048 buffers by default) |
| `header.params.l2_total_queue_bytes` | bytes | npt-net (ImpairSocket) | all peers' bound per direction, charged the same way (M10; 6,144,000 B = 4,096 buffers by default) |
| `header.params.l2_peer_capacity_count` | count | npt-net (ImpairSocket) | the per-peer table's fixed capacity (M10) |
| `header.params.l2_fate_log_capacity_count` | count | npt-net (ImpairSocket) | the fate log's fixed capacity in entries (0: none kept) |
| `header.params.l2_one_way_ns` | ns | npt-net (ImpairSocket) | the default peer's injected one-way delay: (profile round trip − base RTT) ÷ 2 (PM-0001-3's reading) |
| `header.params.l2_half_jitter_ns` | ns | npt-net (ImpairSocket) | the default peer's jitter half-width each way (± half the profile's jitter) |
| `header.params.l2_shaper_rate_bits` | bits per second | npt-net (ImpairSocket) | the default peer's shaper rate (0: none); -23's throttle 128,000 (M19) |
| `header.params.l2_shaper_queue_bytes` | bytes | npt-net (ImpairSocket) | its drop-tail queue counting payload + 28 B per datagram: -23's 5,920 B (4 full datagrams of 1,480 B), -24's max(200 ms at the rate, 5,920 B) (the advisor's ruling, M19). The run writes it again as `throttle_queue_bytes` or the `bottleneck_*` rows above |
| `header.params.l2_shaper_queue_ns` | ns | npt-net (ImpairSocket) | the same queue as time at the shaper's rate (`bottleneck_queue_ns` on -24's runs) |
| `header.params.l2_udp_block` | flag | npt-net (ImpairSocket) | 1 when the default peer starts blocked (PM-0001-29) |

## DelayStream (U3c, the WebSocket fallback's wrapper; the advisor's M15, u3c-plan.md 1.7): names no mark reads yet

What `DelayStream` measures, under the names its own code reports (npt-net's `l2::delay_names`, checked against README
§1's rule and against this file by unit tests). Directions are the TCP model's: `down` is what RS sends (model end 0),
`up` what the client sends (end 1). Every stamp is on DelayStream's clock (`DelayStream::now_ns`). The rows are per
connection; npt-server's writer merges its WebSocket connections into the run's records (counts summed, maxima and
levels the largest, lateness histograms merged: U6). The one exception is the lateness's per-release split, which is
the run's, pooled over every connection in the process (the `ws_split` event, below).

**The `ws_summary` event** is written once per connection at its end. Its fields: every per-second name below over the
connection's life (the counts as totals, the maxima as they stand, `ws_<dir>_held_bytes` as at the end with
`ws_<dir>_held_max_bytes` the connection's largest, the lateness over every sample), then the four rows of its own
table.

**The per-release split** (the advisor's "condition 10 again", 2026-11-08): each release's lateness L = (3) − M (M the
record's model delivery stamp, (3) where L is taken: S's poll_write return down, the hand-over up) splits on stamps
DelayStream already takes: W, the release clock thread's `now` when it fired the connection's entry (the first stamp
after its Condvar wait returns, and after each batch of wakes); P, the stamp of the pump pass's opening step; T, the
stamp of the step that released the record. The **woken** class (released at an opening step, with the clock's latest
untaken fire for the connection's entry due at D and stamped W: D <= M <= W <= P) has clock = W − M, hop = P − W and
work = (3) − P; the **other** class (every other release: no clock part) has wait = T − M and work = (3) − T, counted by its
step (open, mid, app). Each part is the difference of two stamps; the parts of every release sum to its L exactly
(npt-net's identity test), and each release is checked as it is recorded, a mismatch counted
(`ws_late_<dir>_split_violations_count`). Each part's statistics are the lateness's own (p50, p99, p99.9, max, E, over),
over the run: one histogram per part per direction shared by every connection in the process (`stats::Pooled`: relaxed
per-bucket atomics, so a release takes no lock and allocates nothing; the advisor's ruling (b) on the split's memory,
2026-11-08), written once at the run's end as the `ws_split` event (U6: npt-server's writer), never per second.

**The `burst_return` event**: DelayStream's part of advisor-ws-docs-r2.md:379 (3) per WebSocket client per burst
(u3c-plan.md C8), recorded as the model passes each burst's end; the room's fields (offset_ns, tick_phase_ns,
budget_bytes, snap_bytes) are U6's.

**Carried to U6:** the per-second merge across connections; whether the plain path (no wrapper) writes `ws_wrapper` 0
and which of these names it can write from TCP_INFO and SIOCOUTQ.

### Per second (`second.v`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `second.v.ws_down_segments_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): data segments formed |
| `second.v.ws_down_transmissions_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): data transmissions, resends included |
| `second.v.ws_down_lost_profile_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): data transmissions the profile's loss draw dropped |
| `second.v.ws_down_lost_burst_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): data transmissions sent inside a burst (the profile's or one added after the wrap), all lost |
| `second.v.ws_down_first_lost_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): segments whose first transmission was lost |
| `second.v.ws_down_pure_acks_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): pure acknowledgements this end sent (for the other direction's data) |
| `second.v.ws_down_pure_acks_lost_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): of those, lost |
| `second.v.ws_down_repairs_rack_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): resends by RACK (S5) |
| `second.v.ws_down_repairs_probe_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): resends by the tail-loss probe (S6) |
| `second.v.ws_down_repairs_timeout_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): resends by the retransmission timeout (S7) |
| `second.v.ws_down_backoff_max_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): the RTO's largest backoff (doublings) so far on this connection: a maximum, written as it stands |
| `second.v.ws_down_stalls_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): segments first received later than their first transmission's drawn arrival (S12's stalls) |
| `second.v.ws_down_stall_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): the longest stall so far on this connection: a maximum, written as it stands |
| `second.v.ws_down_stall_sum_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): the stalls' total length |
| `second.v.ws_down_ooo_held_max_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): the most out-of-order bytes the receiver held at once so far on this connection: a maximum, written as it stands |
| `second.v.ws_down_discards_old_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): old duplicates the receiver discarded unread (tcp_validate_incoming) |
| `second.v.ws_down_dsacks_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): D-SACK blocks the sender accepted |
| `second.v.ws_down_undos_loss_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): losses undone in the Loss state by timestamps or D-SACKs |
| `second.v.ws_down_undos_frto_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): timeouts undone by F-RTO |
| `second.v.ws_down_undos_recovery_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): losses undone in Recovery or Disorder |
| `second.v.ws_down_held_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): bytes DelayStream holds at the second's end (down: plaintext not yet taken by S, at most the 64 KB budget plus the acknowledged prefix of the front record not yet released, so below 64 KB + 16,406 B (a loss inside a record lets the acknowledgement of its first segments free that prefix in the budget while the record waits whole); up: records in the model or released and not yet read, plus the scanner's buffer: at most 64 KB plus the acknowledged prefix of the front unreleased record (below 549 B, or the request's 8,214 B) plus one frame (527 B) or the 8,192 B request cap) |
| `second.v.ws_down_held_max_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0): the most bytes DelayStream held at once this second |
| `second.v.ws_late_down_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): release-lateness samples this second (one per record) |
| `second.v.ws_late_down_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): the release lateness's median (L0 for this second) |
| `second.v.ws_late_down_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): its 99th percentile |
| `second.v.ws_late_down_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): its 99.9th percentile (condition 10's trigger: above 1 ms, or a max above 10 ms, goes to the advisor) |
| `second.v.ws_late_down_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): its maximum |
| `second.v.ws_late_down_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+] over the second, L0 its median |
| `second.v.ws_late_down_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), lateness L = release − the record's model delivery stamp, at the return of S's poll_write that takes its last byte (M1): samples with L − L0 above 0.25 ms |
| `second.v.ws_up_segments_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): data segments formed |
| `second.v.ws_up_transmissions_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): data transmissions, resends included |
| `second.v.ws_up_lost_profile_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): data transmissions the profile's loss draw dropped |
| `second.v.ws_up_lost_burst_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): data transmissions sent inside a burst (the profile's or one added after the wrap), all lost |
| `second.v.ws_up_first_lost_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): segments whose first transmission was lost |
| `second.v.ws_up_pure_acks_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): pure acknowledgements this end sent (for the other direction's data) |
| `second.v.ws_up_pure_acks_lost_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): of those, lost |
| `second.v.ws_up_repairs_rack_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): resends by RACK (S5) |
| `second.v.ws_up_repairs_probe_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): resends by the tail-loss probe (S6) |
| `second.v.ws_up_repairs_timeout_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): resends by the retransmission timeout (S7) |
| `second.v.ws_up_backoff_max_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): the RTO's largest backoff (doublings) so far on this connection: a maximum, written as it stands |
| `second.v.ws_up_stalls_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): segments first received later than their first transmission's drawn arrival (S12's stalls) |
| `second.v.ws_up_stall_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): the longest stall so far on this connection: a maximum, written as it stands |
| `second.v.ws_up_stall_sum_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): the stalls' total length |
| `second.v.ws_up_ooo_held_max_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): the most out-of-order bytes the receiver held at once so far on this connection: a maximum, written as it stands |
| `second.v.ws_up_discards_old_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): old duplicates the receiver discarded unread (tcp_validate_incoming) |
| `second.v.ws_up_dsacks_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): D-SACK blocks the sender accepted |
| `second.v.ws_up_undos_loss_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): losses undone in the Loss state by timestamps or D-SACKs |
| `second.v.ws_up_undos_frto_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): timeouts undone by F-RTO |
| `second.v.ws_up_undos_recovery_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): losses undone in Recovery or Disorder |
| `second.v.ws_up_held_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): bytes DelayStream holds at the second's end (down: plaintext not yet taken by S, at most the 64 KB budget plus the acknowledged prefix of the front record not yet released, so below 64 KB + 16,406 B (a loss inside a record lets the acknowledgement of its first segments free that prefix in the budget while the record waits whole); up: records in the model or released and not yet read, plus the scanner's buffer: at most 64 KB plus the acknowledged prefix of the front unreleased record (below 549 B, or the request's 8,214 B) plus one frame (527 B) or the 8,192 B request cap) |
| `second.v.ws_up_held_max_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1): the most bytes DelayStream held at once this second |
| `second.v.ws_late_up_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): release-lateness samples this second (one per record) |
| `second.v.ws_late_up_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): the release lateness's median (L0 for this second) |
| `second.v.ws_late_up_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): its 99th percentile |
| `second.v.ws_late_up_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): its 99.9th percentile (condition 10's trigger: above 1 ms, or a max above 10 ms, goes to the advisor) |
| `second.v.ws_late_up_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): its maximum |
| `second.v.ws_late_up_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+] over the second, L0 its median |
| `second.v.ws_late_up_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), lateness L = release − the record's model delivery stamp, at the return of poll_read that hands its last byte to tungstenite (M1): samples with L − L0 above 0.25 ms |
| `second.v.ws_close_request_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because the upgrade request reached ws_max_request_bytes (8,192 B) without its blank line, before any of its payload entered the model (plan §4.4) |
| `second.v.ws_close_frame_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because a client frame's payload above ws_max_message_bytes (513 B), before any of its payload entered the model (plan §4.4) |
| `second.v.ws_close_message_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because a fragmented message's running payload above ws_max_message_bytes, before any of its payload entered the model (plan §4.4) |
| `second.v.ws_close_unmasked_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because an unmasked client frame (RFC 6455 §5.1), before any of its payload entered the model (plan §4.4) |
| `second.v.ws_close_control_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because a control frame above 125 B, or fragmented (§5.5), before any of its payload entered the model (plan §4.4) |
| `second.v.ws_close_length_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | connections closed because a 64-bit length with its top bit set (§5.2), before any of its payload entered the model (plan §4.4) |
| `second.v.ws_shutdown_timeout_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | shutdowns that gave up at their 10 s deadline (delivery, S's taking the bytes, or S's own shutdown unfinished) and dropped S |
| `second.v.ws_split_writes_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | released down records S took in more than one write call (so more than one real TLS record) |
| `second.v.ws_returns_dropped_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | per-return records not kept: the connection held its 1,024 already |

### The per-release split over the run (`event.ws_split`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `event.ws_split.ws_late_down_woken_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): releases in the woken class over the run, pooled over every connection (released at a pump pass's opening step after a release-clock wake that came when the record was due: D <= M <= W <= P); each one's clock, hop and work parts sum to its L exactly |
| `event.ws_split.ws_late_down_clock_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its median (x0 over the run) |
| `event.ws_split.ws_late_down_clock_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its 99th percentile |
| `event.ws_split.ws_late_down_clock_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its 99.9th percentile |
| `event.ws_split.ws_late_down_clock_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its maximum |
| `event.ws_split.ws_late_down_clock_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_down_clock_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_down_hop_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its median (x0 over the run) |
| `event.ws_split.ws_late_down_hop_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its 99th percentile |
| `event.ws_split.ws_late_down_hop_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its 99.9th percentile |
| `event.ws_split.ws_late_down_hop_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its maximum |
| `event.ws_split.ws_late_down_hop_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_down_hop_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_down_work_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): its median (x0 over the run) |
| `event.ws_split.ws_late_down_work_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): its 99th percentile |
| `event.ws_split.ws_late_down_work_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): its 99.9th percentile |
| `event.ws_split.ws_late_down_work_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): its maximum |
| `event.ws_split.ws_late_down_work_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_down_work_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the return of S's poll_write that takes the record's last byte (the pump's work, earlier records, rustls and the write syscall; a later pass when S is full): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_down_other_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): releases in the other class over the run, pooled over every connection (no clock wake covered them); each one's wait and work parts sum to its L exactly, with no clock part |
| `event.ws_split.ws_late_down_other_wait_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its median (x0 over the run) |
| `event.ws_split.ws_late_down_other_wait_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its 99th percentile |
| `event.ws_split.ws_late_down_other_wait_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its 99.9th percentile |
| `event.ws_split.ws_late_down_other_wait_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its maximum |
| `event.ws_split.ws_late_down_other_wait_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_down_other_wait_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_down_other_work_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): its median (x0 over the run) |
| `event.ws_split.ws_late_down_other_work_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): its 99th percentile |
| `event.ws_split.ws_late_down_other_work_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): its 99.9th percentile |
| `event.ws_split.ws_late_down_other_work_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): its maximum |
| `event.ws_split.ws_late_down_other_work_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_down_other_work_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the return of S's poll_write that takes the record's last byte (the pump's wake when another task or its own up loop released the record, then its write): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_down_other_open_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): of the other class, released at a pump pass's opening step that no clock wake covered (the pass woken by S's I/O, an application task or a shutdown, or by a clock wake before the record was due) |
| `event.ws_split.ws_late_down_other_mid_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): of the other class, released at the pump's step inside its pass (its up loop, after the pass's down writes; a down record released there is written by the pump's next pass, which the pump wakes for at once (R1)) |
| `event.ws_split.ws_late_down_other_app_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): of the other class, released by an application task's step (DelayStream's poll_read, poll_write, an accessor or add_burst stepping the model) |
| `event.ws_split.ws_late_down_split_violations_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | server to client (RS's TCP sends; model end 0), the release lateness's per-release split (**The per-release split** above): releases whose recorded parts did not sum to their L (checked per release and counted; 0 while the stamps chain, as npt-net's identity test shows) |
| `event.ws_split.ws_late_up_woken_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): releases in the woken class over the run, pooled over every connection (released at a pump pass's opening step after a release-clock wake that came when the record was due: D <= M <= W <= P); each one's clock, hop and work parts sum to its L exactly |
| `event.ws_split.ws_late_up_clock_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its median (x0 over the run) |
| `event.ws_split.ws_late_up_clock_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its 99th percentile |
| `event.ws_split.ws_late_up_clock_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its 99.9th percentile |
| `event.ws_split.ws_late_up_clock_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): its maximum |
| `event.ws_split.ws_late_up_clock_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_up_clock_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's clock part, W − M: from the record's model delivery stamp to the release clock thread's wake that covered it (its own wake lateness): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_up_hop_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its median (x0 over the run) |
| `event.ws_split.ws_late_up_hop_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its 99th percentile |
| `event.ws_split.ws_late_up_hop_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its 99.9th percentile |
| `event.ws_split.ws_late_up_hop_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): its maximum |
| `event.ws_split.ws_late_up_hop_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_up_hop_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's hop part, P − W: from the clock thread's wake to the stamp of the pump pass's opening step (the thread's wake calls, tokio's scheduling of the pump task, the pump's wait for the connection's lock): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_up_work_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): its median (x0 over the run) |
| `event.ws_split.ws_late_up_work_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): its 99th percentile |
| `event.ws_split.ws_late_up_work_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): its 99.9th percentile |
| `event.ws_split.ws_late_up_work_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): its maximum |
| `event.ws_split.ws_late_up_work_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_up_work_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the woken class's work part, (3) − P: from the pass's opening stamp to the poll_read that hands the record's last byte to tungstenite (the rest of the pump's pass, the reader's wake and turn, one record per read): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_up_other_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): releases in the other class over the run, pooled over every connection (no clock wake covered them); each one's wait and work parts sum to its L exactly, with no clock part |
| `event.ws_split.ws_late_up_other_wait_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its median (x0 over the run) |
| `event.ws_split.ws_late_up_other_wait_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its 99th percentile |
| `event.ws_split.ws_late_up_other_wait_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its 99.9th percentile |
| `event.ws_split.ws_late_up_other_wait_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: its maximum |
| `event.ws_split.ws_late_up_other_wait_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_up_other_wait_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's wait part, T − M: from the record's model delivery stamp to the stamp of the step that released it: samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_up_other_work_p50_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): its median (x0 over the run) |
| `event.ws_split.ws_late_up_other_work_p99_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): its 99th percentile |
| `event.ws_split.ws_late_up_other_work_p999_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): its 99.9th percentile |
| `event.ws_split.ws_late_up_other_work_max_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): its maximum |
| `event.ws_split.ws_late_up_other_work_excess_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): E[(x − x0 − 0.25 ms)+] over the run, x0 its median (U3-Q1's statistic, as for L) |
| `event.ws_split.ws_late_up_other_work_over_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above), the other class's work part, (3) − T: from the releasing step to the poll_read that hands the record's last byte to tungstenite (the reader's wake and turn): samples with x − x0 above 0.25 ms |
| `event.ws_split.ws_late_up_other_open_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): of the other class, released at a pump pass's opening step that no clock wake covered (the pass woken by S's I/O, an application task or a shutdown, or by a clock wake before the record was due) |
| `event.ws_split.ws_late_up_other_mid_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): of the other class, released at the pump's step inside its pass (its up loop, after the pass's down writes; the reader is woken at the pass's end) |
| `event.ws_split.ws_late_up_other_app_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): of the other class, released by an application task's step (DelayStream's poll_read, poll_write, an accessor or add_burst stepping the model) |
| `event.ws_split.ws_late_up_split_violations_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | client to server (the client's TCP; model end 1), the release lateness's per-release split (**The per-release split** above): releases whose recorded parts did not sum to their L (checked per release and counted; 0 while the stamps chain, as npt-net's identity test shows) |

### The `ws_summary` event's own fields (`event.ws_summary`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `event.ws_summary.ws_conns_before_count` | count | npt-net (DelayStream; the owning process's writer: npt-server on RS) | the connection's index: connections the accept path accepted before it (first = 0); its fates' key (S10) |
| `event.ws_summary.ws_accepted_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | the TCP accept stamp passed to `wrap` (npt-server's Join deadline runs from it, never from the end of a delay: plan §4.4) |
| `event.ws_summary.ws_down_written_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | plaintext tungstenite wrote down through DelayStream |
| `event.ws_summary.ws_up_written_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | plaintext of the client's records that entered the model up (the request and every whole frame) |

### The `burst_return` event's DelayStream fields (`event.burst_return`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `event.burst_return.ws_return_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | the burst's end (the link's return) on DelayStream's clock |
| `event.burst_return.outstanding_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | the down direction's unsent plus unacknowledged bytes at the return (the model's write_seq − snd_una: SIOCOUTQ, tcp.c:630), in the model's bytes (+ 22 B per TLS record); mechanism test (c) checks it against N x the snapshot, N from the srtt at the writer's last write before the return (NC-U3c-1). Narrower than advisor-ws-docs-r2.md:379 (3)'s outstanding_bytes, which adds what is held above the socket: that is this plus `ws_owed_bytes` plus what WsServer holds itself (U6 writes the sum; NC-U3c-4) |
| `event.burst_return.srtt_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | the down direction's smoothed RTT at the return (tcpi_rtt, tcp.c:3842); 0 before a sample |
| `event.burst_return.srtt_sampled` | flag | npt-net (DelayStream; the owning process's writer: npt-server on RS) | 1 when `srtt_ns` is a sample (else fix 1 uses 100 ms) |
| `event.burst_return.ws_untaken_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | down bytes (model bytes) released by the model and not yet taken by S at the return |
| `event.burst_return.ws_owed_bytes` | bytes | npt-net (DelayStream; the owning process's writer: npt-server on RS) | bytes tungstenite held above DelayStream at the return (offered in its latest write and not taken); r2:379 (3)'s outstanding_bytes = `outstanding_bytes` + this + what WsServer holds (NC-U3c-4) |
| `event.burst_return.ws_down_first_byte_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | from the return to the first in-order byte the model delivers down (its receiver's rcv_nxt advances; S12), which may sit inside a record not yet released whole; u64::MAX when none came before the connection ended |
| `event.burst_return.ws_down_first_message_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | from the return to the first whole data message released down (S12); u64::MAX: none |
| `event.burst_return.ws_up_first_byte_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | from the return to the first in-order byte the model delivers up (rcv_nxt advances); u64::MAX: none |
| `event.burst_return.ws_up_first_message_ns` | ns | npt-net (DelayStream; the owning process's writer: npt-server on RS) | from the return to the first whole data message released up; u64::MAX: none |

### Header params (`header.params`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `header.params.ws_wrapper` | flag | npt-net (DelayStream) | 1: the WebSocket path runs through DelayStream (impaired runs); clean runs go without it (R20), and their CPU differs (C16) |
| `header.params.ws_one_way_ns` | ns | npt-net (DelayStream) | each direction's injected mean delay: (profile round trip − base RTT) ÷ 2, ImpairSocket's helper (C5) |
| `header.params.ws_half_jitter_ns` | ns | npt-net (DelayStream) | each direction's jitter half-width (± half the profile's jitter), drawn per transmission |
| `header.params.ws_tcp_write_budget_bytes` | bytes | npt-net (DelayStream) | the model's write budget: unsent plus unacknowledged bytes, 65,536 B (S9) |
| `header.params.ws_tcp_mss_bytes` | bytes | npt-net (DelayStream) | the model's segment size: 1,448 B with timestamps (S1) |
| `header.params.ws_tcp_rto_min_ns` | ns | npt-net (DelayStream) | the model's minimum RTO (TCP_RTO_MIN, 200 ms) |
| `header.params.ws_tcp_thin_linear_timeouts` | flag | npt-net (DelayStream) | thin-stream linear timeouts (0: Linux's default) |
| `header.params.ws_tcp_timestamps` | flag | npt-net (DelayStream) | the timestamp option (1: Linux's default) |
| `header.params.ws_tcp_syn_rto_ns` | ns | npt-net (DelayStream) | the model's first SYN timeout (1 s; for its reconnect path) |
| `header.params.ws_tcp_syn_linear_timeouts_count` | count | npt-net (DelayStream) | SYN retries before the timeout doubles (Linux: 4) |
| `header.params.ws_clock_timer_slack_ns` | ns | npt-net (DelayStream) | the release clock thread's timer slack as the thread itself reads it when the header is built (1; u64::MAX unreadable: M14) |
| `header.params.ws_clock_timer_slack_start_ns` | ns | npt-net (DelayStream) | the same slack as the thread read it back at its start, right after writing it |
| `header.params.ws_clock_timer_slack_set` | flag | npt-net (DelayStream) | 1 when that write succeeded |
| `header.params.ws_max_message_bytes` | bytes | npt-net (DelayStream) | the client's frame and message limit applied before any delay buffering (513 B; tungstenite's max_frame_size and max_message_size take the same value) |
| `header.params.ws_max_request_bytes` | bytes | npt-net (DelayStream) | the client's upgrade request cap on DelayStream's path (8,192 B; the plain path keeps tungstenite's 64 KiB) |
| `header.params.ws_record_max_bytes` | bytes | npt-net (DelayStream) | the largest down record per poll_write: one TLS 1.3 record's plaintext, 16,384 B (C1) |
| `header.params.ws_record_overhead_bytes` | bytes | npt-net (DelayStream) | each record's overhead in the model: 22 B (5 B header, 1 B content type, 16 B tag; S0) |
| `header.params.ws_up_read_floor_bytes` | bytes | npt-net (DelayStream) | S is read only while the up room is at least this: one largest client record, 513 + 14 + 22 = 549 B (C2) |
| `header.params.ws_shutdown_wait_ns` | ns | npt-net (DelayStream) | how long a shutdown waits for delivery and for S to take the bytes before dropping S (10 s; C3) |
| `header.params.ws_bursts_max_count` | count | npt-net (DelayStream) | the most bursts a connection holds still to come (64; C4) |
| `header.params.ws_returns_max_count` | count | npt-net (DelayStream) | the per-return records a connection keeps (1,024) |
| `header.params.ws_conn_fixed_bytes` | bytes | npt-net (DelayStream) | the heap a connection allocates at wrap, before any WebSocket byte (F7): one lateness histogram per direction with its window's sparse mark (142,240 B each), the scanner's buffer (8,192 B at the defaults), the down staging buffer (16,384 B) and the pump's read buffer (527 B): 309,583 B at the defaults (the per-release split's histograms are the process's: `ws_split_fixed_bytes`). On top: the held bytes (below about 82 KB each way) and, once a second's samples touch more than 256 of a histogram's buckets, a dense mark of 135,040 B for that histogram. U6 sizes WsServer's accept cap from it (and from tungstenite's own buffers) |
| `header.params.ws_split_fixed_bytes` | bytes | npt-net (DelayStream) | the heap the process allocates once, at its first wrap, for the per-release split over the run, shared by every connection (the advisor's ruling (b), 2026-11-08): per direction five histograms (`stats::Pooled`, 135,040 B each), 1,350,400 B whatever the number of connections |
| `header.params.ws_tcp_model` | string | npt-net (DelayStream) | the TCP model's name and version (`linux-6.8-tcp v3`, tcp_model.rs MODEL) |
| `header.params.ws_tcp_not_modelled` | string | npt-net (DelayStream) | what the model leaves out (S11), as one sentence |
| `header.params.ws_clock_sched_policy` | string | npt-net (DelayStream) | the release clock thread's scheduling policy and priority read from /proc when the header is built (`SCHED_OTHER 0`) |
| `header.params.ws_conn_index_rule` | string | npt-net (DelayStream) | the connection index rule: the accept path's order, the first accepted 0; fates from (seed, index, direction) |
| `header.params.ws_records` | string | npt-net (DelayStream) | the record boundaries DelayStream uses and that real TLS records may differ (recorded, u3c-plan.md §4's risk) |
| `header.params.ws_handshake` | string | npt-net (DelayStream) | that the TCP and TLS handshakes run undelayed below the wrapper and the model starts without an RTT sample (RTO 1 s and fix 1's 100 ms srtt until the first data acknowledgement, where Linux has the handshake's sample; NC-U3c-11) |
| `header.params.ws_clock_timer_slack_error` | string | npt-net (DelayStream) | the slack write's error, present only when it failed |

## Open naming questions

- Q3: no length or angle unit (`_um`, `_microdeg` proposed).
- Q30 (answered at U4j): per-client hashes per tick live in `hs` on tick records (name -> one 16-hex string or null per client slot; README §1), under the names `server_hash`, `decoded_hash` and `replay_hash` (the U4j section above).
- Q16: string values (the negotiated draft, browser versions, the stand-ins, the certificate presented) need the verdict detail to print named header params.
- Q25: `params.started_utc` for the times-of-day rule.

