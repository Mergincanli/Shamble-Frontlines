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
| `tick.v.unsent_snaps_max_count` | count | npt-server | the most unsent snapshots any client has queued (WebSocket newest-only), sampled each tick | -31 |
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
| `event.burst_heal.after_link_ns` | ns | npt-bots (L2), npt-lab (L1) | per client per burst: link back to first delta decoded | -18 |
| `event.canary_grep.address_matches_count` | count | npt-lab | matches for client addresses | 0006-21 |
| `event.canary_grep.canaries_count` | count | npt-lab | canary values it searched for | 0006-21 |
| `event.canary_grep.ip_pattern_matches_count` | count | npt-lab | IPv4/IPv6 pattern matches other than the VMs' own | 0006-21 |
| `event.canary_grep.lines_count` | count | npt-lab | shipped log lines the grep searched | 0006-21 |
| `event.canary_grep.secret_matches_count` | count | npt-lab | matches for guest secrets | 0006-21 |
| `event.canary_grep.seed_matches_count` | count | npt-lab | matches for match seeds | 0006-21 |
| `event.canary_grep.token_matches_count` | count | npt-lab | matches for join tokens | 0006-21 |
| `event.capture.fragments_count` | count | npt-lab | once per run: fragmented IP packets in RS's packet capture (summary of the capture) | -14 |
| `event.cargo_tree.sim_transport_deps_count` | count | npt-lab | transport crates in npt-sim's `cargo tree` | -35 |
| `event.clamp_kind.counted_count` | count | npt-lab | per weapon kind: of those, refused or counted as forged | -21 |
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
| `cross_arch_mismatch_count` | count | npt-judge | L2 mismatches (hash, shot view test, throw landing) that the x86-64 replay does not reproduce | the Arm rule: 'reported as a cross-architecture finding and decides nothing' (-17, -19, -48, -54) | -17, -19, -48, -54 |
| `decoded_hash_compared_count` | count | npt-judge | the (client, tick) pairs decoded_hash_mismatch_count compared | PM-0001-17, reported | -17 |
| `decoded_hash_mismatch_count` | count | npt-judge | count of (client, tick) whose decoded hash differs from the server's logged quantised hash for that client and tick; with arm_rule on L2, only those the x86-64 re-decode (process `replay`) also misses; fails on 0 compared. Inputs: per-client hashes per tick (format: Q30) | PM-0001-17 '100 % of decoded snapshots rebuild the server's quantised state'; -58; -19's join hash | -17, -58, -19 |
| `down_client_mean_max_bits` | bits | npt-judge | max over client slots of the mean of second.s.down_bits[slot] over the measured seconds | PM-0001-12 'mean at most 160 kbps ... counted per client' | -12 |
| `down_mean_ipv6_bits` | bits | npt-judge | mean over client-seconds of second.s.down_bits + 160 x second.s.down_datagrams_count (20 B more header per datagram) | PM-0001-12 'if LG reaches RS over IPv6, the IPv6 figure' | -12 |
| `egress_interface_bytes` | bytes | npt-judge | event.iface.tx_bytes at the end minus at the start | PM-0006-17 'the interface's transmitted-byte counter read at the start and end' | 0006-17 |
| `egress_metric_vs_interface_ratio` | fraction (exact) | npt-judge | (event.room_metric.sent_bytes at the end - at the start) / egress_interface_bytes | PM-0006-17 'the room's own bytes-sent metric agrees with the interface counter within 5 %' | 0006-17 |
| `egress_vs_r2_ratio` | fraction (exact) | npt-judge | egress_interface_bytes / r2_egress_formula_bytes | PM-0006-17 'within +-20 % of R2's E3 estimate at the counts that ran' | 0006-17 |
| `everyone_in_view_min_share` | fraction (exact) | npt-judge | min over client slots of the share of that slot's snapshots with tick.s.everyone_in_view = 1. Inputs: tick.s.everyone_in_view (index = slot) | PM-0001-2 'every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots' | -2 |
| `false_hit_share` | fraction (exact) | npt-judge | among view misses (event.view_test.hit = 0), the share registered as hits; Arm rule on L2 as above | PM-0001-48 'false hits (a miss in the view counted as a hit) below 0.5 %' | -48 |
| `fate_log_twin_equal` | 0/1 | npt-judge | 1 if header.params.fate_log_sha256 equals that of the run named in header.params.twin, 0 if they differ. The twin must be in the same set, with role twin and the same layer, profile, seed and bite (a bite run's twin runs the same bite); a twin not in the set yet leaves the check NOT-RUN; a run naming no twin, a twin that differs in those fields or is not role twin, and a hash that is not 64 lowercase hex digits are gaps (built in U2) | PM-0001-3 'on L1, the same seed and profile give an identical packet-fate log on two runs' | -3 |
| `input_latency_p50_rise_ns` | ns | npt-judge | the same with medians | PM-0001-43 'and the same for the medians' | -43 |
| `input_latency_p99_rise_ns` | ns | npt-judge | max over kinds (params.kind: move, flash) of p99(event.input_latency.to_screen_ns at P4) - p99(at P0), over the part's runs | PM-0001-43 'p99 at P4 minus p99 at P0 at most 33.33 ms' | -43 |
| `kill_p99_ratio` | fraction (exact) | npt-judge | p99 of tick.v.work_ns over the ticks after the set's `kill` event (ph = after) / p99 over the measured ticks | PM-0001-27 'killing one room's process (SIGKILL) leaves the others' p99 within 5 %' | -27 |
| `logging_cost_ratio` | fraction (exact) | npt-judge | p99 over the pooled measured ticks of the part's 3 runs / p99 over the pooled ticks of the set's 3 role logging-off runs (tick.v.work_ns) | PM-0006-19 'the pooled p99 of the 3 on runs (54,000 ticks) with that of the 3 off runs', 'at most 1.02 x' | 0006-19 |
| `machine_named` | 0/1 | npt-judge | 1 if every log's header `machine` has role, name, plan, arch, cpu, vcpus, mem_mb, location, kernel, threads_per_core, smt and clock_res_ns, and a VM (RS, LG, MV) its day address; else 0. Inputs: headers | PM-0001-1 'RS and LG named with plan, CPU model, vCPU count, memory, location, kernel and the day's address'; PM-0006-11's list | -1, 0006-11 |
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
| `twin_hash_mismatch_count` | count | npt-judge | count of tick numbers whose `h` differs between the run and the run named in header.params.twin (a tick in one only counts); fails on 0 compared | PM-0001-21 'the state hash of a run with the forged set equals that of a run with only their valid parts'; -30 'identical server state hashes on every tick' | -21, -30 |
| `up_client_mean_max_bits` | bits | npt-judge | the same over second.s.up_bits | PM-0001-13 'mean at most 48 kbps per client' | -13 |
| `view_hit_registered_share` | fraction (exact) | npt-judge | among shots whose shooter's view test hit (event.view_test.hit = 1), the share the server registered (event.shot.registered = 1), joined by shot id; on L2 a non-registered view hit counts only if the x86-64 replay's view test also hits | PM-0001-48 'of the shots whose ray hits the target's volume in the shooter's view ... at least 99 % register' | -48, -49 |

### Names added with the advisor's U1 rulings (2026-11-06)

| Name | Unit | Written by (crate) | Meaning | Marks |
|---|---|---|---|---|
| `event.worker_trap.tick` | tick number | web | the harness logs each Worker trap it injects, with the server tick it was injected at (Q12) | -19 |
| `event.pong_delayer.hold_ns` | ns | npt-bots | the 150 ms pong delayer's cheat client logs each pong it held, with the hold (Q21) | -49 |
| `untimed_trap_count` | count | npt-judge (figure, U7) | Worker traps with no timed rejoin (`event.join.world_ns`) after them and before the next trap (Q12) | -19 |
| `everyone_in_view_min_share` | fraction (exact) | npt-judge (figure, built in U1) | as below; built in U1 because PM-0001-2 is the precondition of the stress-scene marks (B8) | -2 |
| `header.params.started_utc` | ISO 8601 UTC | the run script | the run's UTC start, with seconds and `Z`; the times-of-day rule's input (B14) | -27, 0006-13, 0006-14 |
| `header.params.cc` | `newreno` or `cubic` | the room unit's configuration | the congestion controller, a start-up setting never changed in a run (A2) | -24 |
| `header.params.bite_size_ns`, `header.params.bite_pad_bytes` | ns, bytes | the bites build | a bite's B-S or B-N size, printed beside the bite, never judged (B10) | every B-S or B-N bite |

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
| `suite` | the run's test suite (wrap, joins, forged, clamp, fast-clock, lag-switch, never-ack, throttle, bottleneck, flood, origin, conn-panic, tick-panic, wt-attempts, fallback, fallback-443, both-transports, ws-stall, one-build, rtt, rtt-fixed-100, fuzz, deps, mds, datagram-timing, one-shots, brawl, own-change, input-latency, replay, extrapolation, delay-adapt, events, clock, rewind-cheat, aim-height, feedback, cues, checklist, throws, beside-vehicle, record, stalls, ping, firewall, fw-probe, rule-list, metrics-probe, egress, bill, logging-cost, logging-alloc, soak-8, cert-14d, cert-15d, rotation) | selects throughout |
| `browser` | chrome, edge, firefox, safari, brave | browser parts |
| `backend` | webgpu or webgl2 | -32 |
| `transport` | wt or ws | -25, -50, PM-0006-15 |
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
rule by a unit test). Up is inbound (client to server: the socket sits on RS), down outbound. Each name below is also
a field of the `l2_summary` event, written once at the run's end with the run's totals and quantiles, plus
`l2_release_cpu_ns` (the release thread's CPU over the run) and the measured reorder shares under the existing names
`reorder_up_permille`, `reorder_down_permille` and `reorder_permille`. `base_rtt_ns` and `reorder_step_permille` (rows
above) are written by `ImpairSocket` too.

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
| `second.v.l2_up_queue_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: tail drops at the byte bounds (per peer and in total; M10), counted apart from the profile's loss |
| `second.v.l2_up_pool_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: drops for an empty buffer pool (M11) |
| `second.v.l2_up_oversize_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams above the size limit, dropped before they were queued (M10) |
| `second.v.l2_up_overtaken_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: datagrams the reorder step held and their successor overtook (npt-l1's step as built: M3) |
| `second.v.l2_up_reordered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server: delivered datagrams released after a later-indexed datagram of their flow (RFC 4737), the impairment layer's reorder count |
| `second.v.l2_late_up_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): release-lateness samples this second |
| `second.v.l2_late_up_p50_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): the release lateness's median (L0 for this second) |
| `second.v.l2_late_up_p99_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its 99th percentile |
| `second.v.l2_late_up_p999_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its 99.9th percentile |
| `second.v.l2_late_up_max_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): its maximum |
| `second.v.l2_late_up_excess_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+] over the second, L0 its median (Mustafa's bar: about 10 µs or less at P1, judged on RS at R1) |
| `second.v.l2_late_up_over_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound, client to server, lateness L = release − due at `poll_recv`'s return, where quinn receives it (M1): samples with L − L0 above 0.25 ms |
| `second.v.l2_down_delivered_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams released this second: sent on the real socket (down) or handed to quinn by `poll_recv` (up) |
| `second.v.l2_down_lost_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams the profile's loss draw dropped |
| `second.v.l2_down_burst_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams dropped inside one of the profile's B1 bursts |
| `second.v.l2_down_blocked_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: datagrams dropped by PM-0001-29's UDP block |
| `second.v.l2_down_shaper_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: tail drops at the shaper's drop-tail queue (PM-0001-23's throttle, -24's bottleneck; M19) |
| `second.v.l2_down_queue_drop_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client: tail drops at the byte bounds (per peer and in total; M10), counted apart from the profile's loss |
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
| `second.v.l2_late_down_excess_ns` | ns | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): U3-Q1's statistic E[(L − L0 − 0.25 ms)+] over the second, L0 its median (Mustafa's bar: about 10 µs or less at P1, judged on RS at R1) |
| `second.v.l2_late_down_over_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound, server to client, lateness L = release − due at `try_send`'s return on the real socket (M1): samples with L − L0 above 0.25 ms |
| `second.v.l2_up_queued_bytes` | bytes | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | inbound: bytes queued at the second's end (in delay, held, or waiting for quinn) |
| `second.v.l2_down_queued_bytes` | bytes | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | outbound: bytes queued at the second's end (in delay, held, or waiting for quinn) |
| `second.v.l2_peers_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | peers in the table at the second's end (fixed capacity: M10) |
| `second.v.l2_peer_evicted_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | peers evicted this second (the least recently seen, never one set by the control plane) |
| `second.v.l2_peer_overflow_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | datagrams from new addresses with every slot pinned, carried by the shared overflow flow |
| `second.v.l2_fate_log_full_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | fate-log entries not kept: the log was at its fixed capacity |
| `second.v.l2_recv_err_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | errors draining the kernel socket (an ICMP error reported on it, ...): counted, never fatal |
| `second.v.l2_pool_free_count` | count | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | free pool buffers at the second's end (M11) |
| `second.v.l2_release_cpu_permille` | permille of one core | npt-net (ImpairSocket; the owning process's writer: npt-server on RS) | the release thread's CPU this second, from its schedstat (M13) |

### Header params (`header.params`)

| Name | Unit | Written by (crate) | Meaning |
|---|---|---|---|
| `header.params.l2_mode` | string | npt-net (ImpairSocket) | `impaired`, or `pass-through` (every impairment off, through the same socket, release thread and slack: M12) |
| `header.params.l2_sched_policy` | string | npt-net (ImpairSocket) | the release thread's scheduling policy and real-time priority read from /proc when the header is built (`SCHED_OTHER 0`; `SCHED_FIFO 50` when the recorded switch was applied from outside with `chrt`: M14) |
| `header.params.l2_timer_slack_ns` | ns | npt-net (ImpairSocket) | the release thread's timer slack as it read it back after writing its own `/proc/<tid>/timerslack_ns` (1; 0 under a real-time policy; u64::MAX unreadable: M14) |
| `header.params.l2_timer_slack_set` | flag | npt-net (ImpairSocket) | 1 when that write succeeded |
| `header.params.l2_timer_slack_error` | string | npt-net (ImpairSocket) | the write's error, present only when it failed |
| `header.params.l2_migration` | flag | npt-net (ImpairSocket) | QUIC migration in both servers' configurations: 0, off (M9; Mustafa's U3-Q6 (a)) |
| `header.params.l2_gso` | flag | npt-net (ImpairSocket) | GSO/GRO as quinn sees ImpairSocket: 0, off (one datagram per Transmit: M11) |
| `header.params.l2_max_datagram_bytes` | bytes | npt-net (ImpairSocket) | the size limit applied before any queueing (1,472 B by default: M10) |
| `header.params.l2_pool_buffers_count` | count | npt-net (ImpairSocket) | the fixed pool of 1,500 B buffers (M11) |
| `header.params.l2_peer_queue_bytes` | bytes | npt-net (ImpairSocket) | one peer's byte bound per direction (M10) |
| `header.params.l2_total_queue_bytes` | bytes | npt-net (ImpairSocket) | all peers' byte bound per direction (M10) |
| `header.params.l2_peer_capacity_count` | count | npt-net (ImpairSocket) | the per-peer table's fixed capacity (M10) |
| `header.params.l2_fate_log_capacity_count` | count | npt-net (ImpairSocket) | the fate log's fixed capacity in entries (0: none kept) |
| `header.params.l2_one_way_ns` | ns | npt-net (ImpairSocket) | the default peer's injected one-way delay: (profile round trip − base RTT) ÷ 2 (PM-0001-3's reading) |
| `header.params.l2_half_jitter_ns` | ns | npt-net (ImpairSocket) | the default peer's jitter half-width each way (± half the profile's jitter) |
| `header.params.l2_shaper_rate_bits` | bits per second | npt-net (ImpairSocket) | the default peer's shaper rate (0: none); -23's throttle 128,000 (M19) |
| `header.params.l2_shaper_queue_bytes` | bytes | npt-net (ImpairSocket) | its drop-tail queue counting payload + 28 B per datagram: -23's 5,920 B (4 full datagrams of 1,480 B), -24's max(200 ms at the rate, 5,920 B) (the advisor's ruling, M19). The run writes it again as `throttle_queue_bytes` or the `bottleneck_*` rows above |
| `header.params.l2_shaper_queue_ns` | ns | npt-net (ImpairSocket) | the same queue as time at the shaper's rate (`bottleneck_queue_ns` on -24's runs) |
| `header.params.l2_udp_block` | flag | npt-net (ImpairSocket) | 1 when the default peer starts blocked (PM-0001-29) |

## Open naming questions

- Q3: no length or angle unit (`_um`, `_microdeg` proposed).
- Q30: per-client quantised hashes per tick (48 per tick) have no home: `h` holds one hash per tick record, and `s` holds integers (a 64-bit hash exceeds JSON's safe 2^53). Proposal: an `hs` map (name -> list of 16-hex strings) on tick records, or the hash as two 32-bit halves in `s`.
- Q16: string values (the negotiated draft, browser versions, the stand-ins, the certificate presented) need the verdict detail to print named header params.
- Q25: `params.started_utc` for the times-of-day rule.

