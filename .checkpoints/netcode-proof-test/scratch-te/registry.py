"""Build ../series-draft.md (the draft of marks/SERIES.md) from names.json and the entries below.
Fails if the draft reads a name that has no entry. Run: python3 -I -B registry.py
"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
names = json.load(open(os.path.join(HERE, "names.json")))
OUT = os.path.join(HERE, "..", "series-draft.md")

SRV, ROOMC, LAB, BOTS, WEB, CLI, SUP, LOAD, NET, L1, FUZZ, PROTO = (
    "npt-server", "npt-room", "npt-lab", "npt-bots", "web", "npt-client", "npt-super", "npt-load", "npt-net", "npt-l1", "fuzz", "npt-proto")
UM, UDEG = "µm (proposed, Q3)", "µdeg (proposed, Q3)"

S = {  # name: (unit, written by, meaning)
 # ---- tick.v (one value per tick record) ----
 "tick.v.work_ns": ("ns", SRV, "server tick time: the tick thread's work from its start to its end (sim step, commands, lag-compensation queries, interest, encoding, sends), measured directly. Also read by V-S's and clean-ref's p99s"),
 "tick.v.sim_ns": ("ns", SRV, "sim step time: the step's elapsed time minus its lag-compensation queries"),
 "tick.v.server_only_ns": ("ns", SRV, "server-only work: commands, lag-compensation queries, interest, encoding and the tick's sends"),
 "tick.v.lagcomp_ns": ("ns", SRV, "the tick's lag-compensation queries (timed inside the step, counted server-only)"),
 "tick.v.overrun": ("flag", SRV, "1 when this tick overran its period (the loop's overrun counter's increment)"),
 "tick.v.dropped_ns": ("ns", SRV, "time the loop dropped at this tick's wake (0 on most ticks)"),
 "tick.v.start_jitter_ns": ("ns", SRV, "actual minus scheduled start of the tick (S17), from the loop's monotonic clock"),
 "tick.v.wake_ticks_count": ("count", SRV, "ticks run in the wake this tick ran in (at most LOOP_CATCHUP = 7 at 30 Hz)"),
 "tick.v.region_query_ns": ("ns", SRV, "the region query for all clients this tick (interest part a)"),
 "tick.v.rank_pack_ns": ("ns", SRV, "ranking and packing for all clients this tick"),
 "tick.v.pair_test_per_pair_ns": ("ns", SRV, "the tick's enclosure pair-test time divided by its pair count, rounded up to whole ns (key 1500-1501)"),
 "tick.v.pair_count": ("count", ROOMC, "enemy pairs tested this tick (768 at 16v16v16)"),
 "tick.v.reflood": ("flag", ROOMC, "1 when this tick re-flooded the areas after a portal change"),
 "tick.v.soldiers_count": ("count", ROOMC, "soldiers in the sim this tick"),
 "tick.v.smallest_team_count": ("count", ROOMC, "soldiers in the smallest team this tick"),
 "tick.v.snapshot_clients_count": ("count", ROOMC, "soldiers with a connected, snapshot-receiving client"),
 "tick.v.big_vehicles_count": ("count", ROOMC, "big vehicles live (stand-ins included)"),
 "tick.v.very_large_units_count": ("count", ROOMC, "very large units of at least 20 parts live"),
 "tick.v.projectiles_count": ("count", ROOMC, "live projectiles (trajectory records)"),
 "tick.v.effects_per_soldier_max_count": ("count", ROOMC, "the most active effects any soldier has this tick"),
 "tick.v.cmd_refused_count": ("count", ROOMC, "commands refused this tick"),
 "tick.v.cmd_lost_count": ("count", ROOMC, "commands lost (a sequence that never ran) found this tick"),
 "tick.v.cmd_doubled_count": ("count", ROOMC, "commands run twice found this tick"),
 "tick.v.press_count": ("count", ROOMC, "scripted press edges seen this tick"),
 "tick.v.press_lost_count": ("count", ROOMC, "press edges lost this tick (against the press counters)"),
 "tick.v.press_doubled_count": ("count", ROOMC, "press edges counted twice this tick"),
 "tick.v.corrections_count": ("count", CLI, "client: reconciliation corrections above 1 mm this tick"),
 "tick.v.untraced_corrections_count": ("count", CLI, "client: corrections above 1 mm this tick NOT traced to a late or lost command (PM-0001-41's count; read by -58 too)"),
 "tick.v.replay_depth_count": ("count", CLI, "client: ticks replayed at this tick's reconciliation"),
 "tick.v.block_extrapolated": ("flag", CLI, "client: 1 when the block built this tick used extrapolation"),
 "tick.v.block_extrap_over_interval": ("flag", CLI, "client: 1 when that extrapolation reached beyond one snapshot interval"),
 "tick.v.remote_event_misplaced_count": ("count", CLI, "client: remote spatial events this tick not in the first block whose delayed time reaches their tick"),
 "tick.v.own_event_misplaced_count": ("count", CLI, "client: own, predicted or owner events not in the block written at the tick that emitted or received them"),
 "tick.v.datagram_too_large_count": ("count", SRV, "snapshot sends refused as above the peer's maximum datagram size (Link's SendResult TooLarge)"),
 "tick.v.unsent_snaps_max_count": ("count", SRV, "the most unsent snapshots any client has queued (WebSocket newest-only), sampled each tick"),
 "tick.v.ack_starved_headroom_count": ("count, signed", ROOMC, "event-queue cap minus queue length of the never-acking client (negative = over the cap)"),
 "tick.v.lagswitch_cmds_count": ("count", ROOMC, "commands run this tick for the lag-switch client"),
 "tick.v.lagswitch_excess_move_um": (UM + ", signed", LAB, "the lag-switch client's movement this tick minus CMD_CATCHUP ticks of top-speed movement (0 or less passes)"),
 "tick.v.padded_step_ns": ("ns", SRV, "the padded sim step's time (PM-0006-14's busy-wait); the `padding` rule compares it with params.padding_target_ns"),
 "tick.v.tick_allocs_count": ("count", SRV, "allocations on the tick thread this tick (counting GlobalAlloc)"),
 # ---- tick.s (samples per tick) ----
 "tick.s.encode_ns": ("ns", SRV, "one per client per snapshot: its encode time"),
 "tick.s.snap_payload_bytes": ("bytes", ROOMC, "one per client per snapshot: the encoded payload"),
 "tick.s.snap_wire_bytes": ("bytes", SRV, "one per client per snapshot interval: UDP payload + 28 B per datagram"),
 "tick.s.snap_datagram_bytes": ("bytes", SRV, "one per snapshot datagram sent: its payload as handed to send_datagram (the cap rule)"),
 "tick.s.rewind_ns": ("ns", SRV, "one per rewound shot: its query time against the candidates"),
 "tick.s.nearest8_missing_count": ("count", ROOMC, "one per client per snapshot: of its 8 nearest enemies, how many the snapshot lacks"),
 "tick.s.relevant_starved_count": ("count", ROOMC, "one per client per snapshot: relevant entities sent in none of this and the previous 3 snapshots"),
 "tick.s.own_state_sent": ("flag", ROOMC, "one per snapshot sent: 1 when it carries the local player's own state"),
 "tick.s.cmd_on_time": ("flag", ROOMC, "one per command due this tick: 1 when it arrived before its tick ran"),
 "tick.s.input_missing": ("flag", ROOMC, "one per player this tick: 1 when no command for this tick had arrived"),
 "tick.s.pos_err_um": (UM, LAB, "L1: one per decoded entity position: error against the server's value before quantising"),
 "tick.s.yaw_err_microdeg": (UDEG, LAB, "L1: one per decoded yaw: error against the server's value before quantising"),
 "tick.s.pitch_err_microdeg": (UDEG, LAB, "L1: one per decoded pitch: error against the server's value before quantising"),
 # ---- second.v ----
 "second.v.steal_permille": ("permille", SRV, "RS: steal time of the measured vCPU(s) this second (V-S reads it from RS's logs)"),
 "second.v.io_cpu_permille": ("permille of one vCPU", SRV, "the transport runtime's threads' CPU time this second"),
 "second.v.proc_cpu_permille": ("permille of one vCPU", SRV, "a room process's whole CPU time this second (density rows); not LG's `cpu_permille`"),
 "second.v.rss_bytes": ("bytes", SRV, "the room process's resident memory, read each second"),
 "second.v.shots_count": ("count", ROOMC, "hitscan shots fired this second"),
 "second.v.portal_changes_count": ("count", ROOMC, "portal changes (doors) this second"),
 "second.v.reliable_events_count": ("count", ROOMC, "reliable gameplay events emitted this second"),
 "second.v.projectile_spawns_count": ("count", ROOMC, "projectiles spawned this second"),
 "second.v.stand_in_changes_count": ("count", ROOMC, "stand-in entity changes this second"),
 "second.v.stand_in_spawns_count": ("count", ROOMC, "stand-in entity spawns this second"),
 "second.v.rewinds_count": ("count", ROOMC, "rewound shots this second"),
 "second.v.inputs_received_count": ("count", SRV, "input datagrams the room received this second (all clients); not V-S's per-client `inputs_count`"),
 "second.v.snap_datagrams_sent_count": ("count", SRV, "snapshot datagrams sent this second (all clients)"),
 "second.v.disconnects_count": ("count", SRV, "connections that dropped this second"),
 "second.v.handshake_headroom_count": ("count, signed", SRV, "the handshake cap minus handshakes in progress"),
 "second.v.person_rtt_ns": ("ns", SRV, "the server-measured RTT of the person's slot (header params.person_slot)"),
 "second.v.own_cosmetic_sent_count": ("count", ROOMC, "cosmetic events sent to a shooter for its own shots this second"),
 "second.v.ws_stalls_count": ("count", SRV, "stalls the WebSocket delay wrapper produced this second"),
 "second.v.ws_stall_permille": ("permille", SRV, "share of this second the WebSocket streams spent stalled"),
 "second.v.pong_cheat_gap_ns": ("ns", LAB, "the pong delayer's transport RTT minus its ping RTT, sampled each second"),
 "second.v.one_shots_sent_count": ("count", BOTS, "one-shot actions LG's clients sent this second"),
 "second.v.recv_over_max_count": ("count", WEB, "browser: datagrams received this second above its current maxDatagramSize"),
 "second.v.console_errors_count": ("count", WEB, "browser: console errors this second"),
 "second.v.page_errors_count": ("count", WEB, "browser: page errors this second"),
 "second.v.early_feedback_count": ("count", WEB, "browser: damage numbers or kills shown this second before the Worker had the server's event"),
 "second.v.feedback_shown_count": ("count", WEB, "browser: damage numbers and kills shown this second"),
 "second.v.delay_step_abs_ns": ("ns", CLI, "client: |change of the interpolation delay| over this second"),
 # ---- second.s ----
 "second.s.down_bits": ("bits", SRV, "one per client: bits sent to it this second, UDP payload + 28 B per datagram"),
 "second.s.up_bits": ("bits", SRV, "one per client: bits received from it this second, UDP payload + 28 B per datagram"),
 "second.s.input_datagram_bytes": ("bytes", SRV, "one per input datagram received this second"),
 "second.s.input_cmds_count": ("count", SRV, "one per input datagram: commands it carried"),
 "second.s.cmd_whole_bytes": ("bytes", PROTO, "one per command: its size whole (U1)"),
 "second.s.cmd_sent_bytes": ("bytes", PROTO, "one per command: its size as sent, delta-coded (U2)"),
 "second.s.send_call_ns": ("ns", SRV, "one per send_datagram call: its time"),
 "second.s.buffer_depth_count": ("count", ROOMC, "one per client: its input buffer depth in ticks, sampled each second"),
 "second.s.probe_up_delay_ns": ("ns", f"{L1} (L1), {NET} (L2 probe)", "one per delivered probe packet, client to server: its one-way delay"),
 "second.s.probe_down_delay_ns": ("ns", f"{L1} (L1), {NET} (L2 probe)", "one per delivered probe packet, server to client: its one-way delay"),
 "second.s.probe_up_lost": ("flag", f"{L1}, {NET}", "one per probe packet sent up, outside the B1 burst: 1 when lost"),
 "second.s.probe_down_lost": ("flag", f"{L1}, {NET}", "one per probe packet sent down, outside the B1 burst: 1 when lost"),
 "second.s.probe_burst_up_lost": ("flag", f"{L1}, {NET}", "one per probe packet sent up inside B1's 1 s burst: 1 when lost"),
 "second.s.probe_burst_down_lost": ("flag", f"{L1}, {NET}", "one per probe packet sent down inside the burst: 1 when lost"),
 "second.s.probe_rtt_ns": ("ns", f"{L1}, {NET}", "one per probe round trip: its time"),
 "second.s.snap_age_ns": ("ns", BOTS, "one per client sample: age of the newest snapshot it holds (server send stamp, clock estimate)"),
 "second.s.ws_extra_delay_ns": ("ns", f"{BOTS}, {WEB}", "one per snapshot decoded over WebSocket: its age at decode minus the profile's one-way delay (50 ms at P2)"),
 "second.s.throttle_missing_snaps_count": ("count", BOTS, "one per throttled client per throttled second: snapshots missing"),
 "second.s.snap_lost_or_late": ("flag", BOTS, "one per expected snapshot per client: 1 when lost or decoded more than 66.67 ms after its expected arrival"),
 "second.s.snap_late_after_send": ("flag", BOTS, "one per expected snapshot: 1 when lost or later than 66.67 ms after sending (R2's reading)"),
 "second.s.snap_corr_over_5cm": ("flag", f"{CLI}, {WEB}", "one per snapshot applied: 1 when its correction of the local player exceeds 5 cm (-42, -55)"),
 "second.s.frame_correction_um": (UM, WEB, "one per rendered frame below the 2 m snap: the drawn correction's movement"),
 "second.s.frame_extrapolated": ("flag", WEB, "one per rendered frame: 1 when drawn from a block built by extrapolation"),
 "second.s.frame_extrap_over_interval": ("flag", WEB, "one per rendered frame: 1 when that extrapolation reached beyond one interval"),
 "second.s.interp_delay_ns": ("ns", CLI, "one per client tick: the interpolation delay in use"),
 "second.s.replay_ns": ("ns", WEB, "one per forced replay: reset and replay time in the Worker"),
 "second.s.replay_forced_ns": ("ns", WEB, "one per replay forced at P3's or P4's measured maximum depth"),
 # ---- event fields ----
 "event.affinity.alone": ("flag", SRV, "README §1: the tick thread alone on its vCPU (`when`: start, end)"),
 "event.affinity.sibling_free": ("flag", SRV, "README §1: the tick thread's SMT sibling free of the transport runtime (1 also on a 1-thread core)"),
 "event.cpu_read.changed": ("flag", SRV, "at each session start: 1 when /proc/cpuinfo's model differs from the run's first read"),
 "event.hit_history.size_bytes": ("bytes", ROOMC, "every minute: the hit history's bytes from its own capacity counters"),
 "event.reflood.time_ns": ("ns", SRV, "one per re-flood: its time"),
 "event.reflood.encoded": ("flag", SRV, "one per re-flood: 1 when it fell on an encoding tick"),
 "event.room_start.spawn_to_open_ns": ("ns", SUP, "one per room start: process spawn to Open"),
 "event.capture.fragments_count": ("count", LAB, "once per run: fragmented IP packets in RS's packet capture (summary of the capture)"),
 "event.burst_heal.after_link_ns": ("ns", f"{BOTS} (L2), {LAB} (L1)", "per client per burst: link back to first delta decoded"),
 "event.join.world_ns": ("ns", f"{BOTS}, {WEB}", "per join: Join sent to a complete world (client-held hash equal to the server's)"),
 "event.join.from_connect_ns": ("ns", f"{BOTS}, {WEB}", "per join: from the page's first connect call to a complete world"),
 "event.forged_kind.sent_count": ("count", LAB, "per forged kind: forged commands sent"),
 "event.forged_kind.unrejected_count": ("count", LAB, "per forged kind: sent minus rejected-and-counted"),
 "event.clamp_kind.sent_count": ("count", LAB, "per weapon kind: aim points beyond reach sent"),
 "event.clamp_kind.counted_count": ("count", LAB, "per weapon kind: of those, refused or counted as forged"),
 "event.fast_clock.excess_cmds_count": ("count, signed", LAB, "per 60 s fast-clock episode: commands run minus (server ticks + lead bound)"),
 "event.fast_clock.excess_move_um": (UM + ", signed", LAB, "per episode: movement minus (honest movement + lead-bound ticks of movement)"),
 "event.lag_burst.held_ns": ("ns", LAB, "per lag-switch burst: how long its inputs were held"),
 "event.ack_starved.dropped": ("flag", ROOMC, "when the server resolves the never-acking client: 1 dropped, 0 resynced"),
 "event.throttle_end.full_rate_ns": ("ns", BOTS, "per throttle: from its end to the full snapshot rate back"),
 "event.no_join_close.age_ns": ("ns", SRV, "per session closed for no Join: age from TCP accept or first QUIC packet"),
 "event.slow_loris_close.age_ns": ("ns", SRV, "per slow-loris client closed: age at close"),
 "event.over_cap.refused": ("flag", SRV, "per over-cap connection from one address: 1 when refused"),
 "event.over_cap_v6.refused": ("flag", SRV, "per over-cap connection from the /64 cycler: 1 when refused"),
 "event.refusal.http_403": ("flag", WEB, "per ws:// or wrong-Origin probe: 1 when refused with 403"),
 "event.conn_panic.closed": ("flag", SRV, "per injected connection panic, a minute after: 1 when that connection closed"),
 "event.conn_panic.others_dropped_count": ("count", SRV, "the other connections that dropped in that minute"),
 "event.conn_panic.overruns_next_min_count": ("count", SRV, "overruns in that minute"),
 "event.room_exit.nonzero": ("flag", SUP, "per room exit: 1 when the exit code is not 0"),
 "event.room_exit.after_panic_ns": ("ns", SUP, "per room exit: exit time minus the crash record's panic stamp (same machine's monotonic clock)"),
 "event.records_check.match_crashed": ("flag", SUP, "after the last injection: 1 when the match record exists and is marked crashed"),
 "event.records_check.crash_report_kept": ("flag", SUP, "1 when the match record and the crash report are kept"),
 "event.records_check.error_log": ("flag", SUP, "1 when the report reached the error log"),
 "event.tick_panic.after_warmup_ticks_count": ("count", SRV, "the injected tick panic's tick, counted from warm-up's end"),
 "event.attempt.ok": ("flag", WEB, "per connection attempt: 1 when the session is ready and its exchange worked (PM-0001-28: datagrams + one bidirectional stream; PM-0006-15 and PM-0006-25: one echo; PM-0006-27: 0 = rejected)"),
 "event.reload.ok": ("flag", WEB, "per reload after a certificate rotation: 1 when it connects with the new hash"),
 "event.fallback.playing_ns": ("ns", WEB, "per fallback attempt: from the page starting to connect to a playing WebSocket session"),
 "event.feature_detect.webgpu": ("flag", WEB, "once per session: 1 when the feature-detection path found WebGPU"),
 "event.fuzz_end.inputs_count": ("count", FUZZ, "per fuzz run (one message type): inputs tried"),
 "event.fuzz_end.panics_count": ("count", FUZZ, "panics found"),
 "event.fuzz_end.over_limit_allocs_count": ("count", FUZZ, "allocations above the size limits (counting allocator)"),
 "event.fuzz_end.corpus_count": ("count", FUZZ, "corpus entries at the end"),
 "event.swap_diff.files_outside_transport_count": ("count", LAB, "files changed outside the transport module by the crate swap's diff"),
 "event.cargo_tree.sim_transport_deps_count": ("count", LAB, "transport crates in npt-sim's `cargo tree`"),
 "event.mds_read.size_bytes": ("bytes", WEB, "per read (start, then every 10 s): the browser's maxDatagramSize"),
 "event.bulk_transfer.size_bytes": ("bytes", BOTS, "per bulk transfer on its own stream: its size"),
 "event.input_latency.to_screen_ns": ("ns", WEB, "per scripted input: input event's time stamp to the first frame drawing its effect (`params.kind`: move, flash)"),
 "event.worker.cross_origin_isolated": ("flag", WEB, "once per session: crossOriginIsolated as the Worker reads it"),
 "event.delay_settle.after_change_ns": ("ns", CLI, "per profile change: until the delay settled (within +- half an interval of its final value)"),
 "event.respawn.slide_frames_count": ("count", WEB, "per respawn or teleport: frames drawing the entity more than 0.1 m from both ends"),
 "event.remote_event.lead_ns": ("ns, signed", WEB, "per remote spatial event: first frame drawing its entity at or past its tick minus the first frame playing it"),
 "event.connect_clock.offset_err_ns": ("ns", LAB, "per connect: |clock estimate - true clock| after 2 s"),
 "event.own_change.corrections_count": ("count", CLI, "per scripted own-state change: corrections while it lasted"),
 "event.own_change.misplaced_count": ("count", CLI, "corrections not at the first snapshot carrying the change"),
 "event.own_change.later_over_1mm_count": ("count", CLI, "corrections above 1 mm after that snapshot, every command in time"),
 "event.own_change.first_size_um": (UM, CLI, "the first correction's size"),
 "event.own_change.blind_ns": ("ns", CLI, "time the client predicted without the change"),
 "event.shot.view_time_replaced": ("flag", ROOMC, "per shot: 1 when the server replaced the view time by its estimate"),
 "event.shot.clipped": ("flag", ROOMC, "per shot: 1 when the cap clipped its rewind"),
 "event.shot.over_cap": ("flag", ROOMC, "per shot: 1 when it was resolved with a rewind above the cap"),
 "event.shot.cheat": ("flag", ROOMC, "per shot: 1 when a cheat client fired it"),
 "event.shot.rewind_ns": ("ns", ROOMC, "per shot: the rewind used"),
 "event.shot.honest_rewind_ns": ("ns", ROOMC, "per honest shot: how far behind the server's present the client's view was"),
 "event.shot.rtt_ns": ("ns", ROOMC, "per shot: its part, the RTT"),
 "event.shot.delay_ns": ("ns", ROOMC, "its part, the announced delay"),
 "event.shot.buffer_wait_ns": ("ns", ROOMC, "its part, the buffer wait"),
 "event.shot.blend_ns": ("ns", ROOMC, "its part, the blend fraction as time"),
 "event.shot.frame_wait_ns": ("ns", ROOMC, "its part, the drawn frame to the tick that sent the command"),
 "event.edge_cheat_shot.gain_ns": ("ns, signed", LAB, "per tolerance-edge cheat shot: its rewind minus its honest twin shot's"),
 "event.pong_cheat_shot.gain_ns": ("ns, signed", LAB, "per pong-delayer shot: the same gain"),
 "event.static_shot.err_ground_um": (UM, ROOMC, "per shot at the ground target: server hit point to the client-picked point"),
 "event.static_shot.err_h2_5m_um": (UM, ROOMC, "the same at 2.5 m"),
 "event.static_shot.err_h4m_um": (UM, ROOMC, "the same at 4 m"),
 "event.aim_encoded.size_bits": ("bits", PROTO, "per sampled command: the aim field's size before delta coding"),
 "event.own_cue.plays_count": ("count", f"{CLI} (L1), {WEB} (ML)", "per own-shot cue key and kind: times it played"),
 "event.throw_kind.throws_count": ("count", f"{LAB}, {BOTS}", "per projectile kind: throws compared"),
 "event.record_item.has_value": ("flag", LAB, "per PM-0001-56 item (`item` names it): 1 when it has a number or value"),
 "event.counter_wrap.tick_count": ("count (a tick number)", LAB, "per counter that wrapped (`counter` names it): the tick of its wrap"),
 "event.stall_200ms.recover_ns": ("ns", SRV, "per 200 ms stall: until no tick is owed and every later start is within 2 ms"),
 "event.stall_1s.dropped_ns": ("ns", SRV, "per 1 s stall: the dropped-time counter's increase"),
 "event.scrape.loop_counters_present": ("flag", LOAD, "1 when the scrape holds the overrun and dropped-time counters"),
 "event.ping.mean_ns": ("ns", WEB + " (record form)", "the pasted `ping -n 100` output: mean round trip"),
 "event.ping.max_ns": ("ns", WEB + " (record form)", "max round trip"),
 "event.ping.lost_count": ("count", WEB + " (record form)", "echoes lost of 100"),
 "event.fw_probe.tcp_timed_out": ("flag", BOTS + " (probe)", "TCP connect to the closed port: 1 when it timed out (not refused)"),
 "event.fw_probe.udp_no_reply": ("flag", BOTS + " (probe)", "UDP probe to the closed port: 1 when no reply came"),
 "event.wss_cert.accepted": ("flag", WEB, "per wss session: 1 when the certificate was accepted with no warning"),
 "event.rule_list.diff_count": ("count", LAB, "per VM role: rules in the pasted list that differ from part 5's list"),
 "event.metric.present": ("flag", LOAD, "per metric of part 9: 1 when present on MV"),
 "event.metric.stuck_triggered": ("flag", LOAD, "per metric: 1 when the soak triggered it and it did not move"),
 "event.metrics_probe.answered": ("flag", BOTS + " (probe)", "per probe from ML to the exporter's port: 1 when it answered"),
 "event.soak_trigger.bad_token_refusals_count": ("count", LOAD, "bad-token joins refused in the soak"),
 "event.soak_trigger.rooms_killed_count": ("count", LOAD, "room processes killed in the soak"),
 "event.stall_scrape.overruns_before_count": ("count", LOAD, "overruns before the first stall (the control)"),
 "event.stall_scrape.overrun_delta_count": ("count", LOAD, "per stall: the overrun counter's increase at the first scrape"),
 "event.stall_scrape.dropped_err_ns": ("ns", LOAD, "per stall: |dropped-time increase - (stall - 7 tick periods)|"),
 "event.series_count.total_count": ("count", LOAD, "series from RS's exporter at the soak's end"),
 "event.series_count.foreign_labels_count": ("count", LOAD, "label names outside {VM, room slot, mode id, transport} and Prometheus' target labels"),
 "event.canary_grep.lines_count": ("count", LAB, "shipped log lines the grep searched"),
 "event.canary_grep.canaries_count": ("count", LAB, "canary values it searched for"),
 "event.canary_grep.token_matches_count": ("count", LAB, "matches for join tokens"),
 "event.canary_grep.secret_matches_count": ("count", LAB, "matches for guest secrets"),
 "event.canary_grep.address_matches_count": ("count", LAB, "matches for client addresses"),
 "event.canary_grep.seed_matches_count": ("count", LAB, "matches for match seeds"),
 "event.canary_grep.ip_pattern_matches_count": ("count", LAB, "IPv4/IPv6 pattern matches other than the VMs' own"),
 "event.rotation.dropped_count": ("count", SRV, "per certificate rotation: live sessions dropped across it"),
 # ---- header.params (one value) ----
 "header.params.reorder_permille": ("permille", f"{L1}, {NET}", "the run's measured reorder share (the `reorder` rule reads it, or the `reorder` event's `permille`)"),
 "header.params.base_rtt_ns": ("ns", f"{NET}, {WEB}", "the path's base RTT: median of 10 s of pings at session start"),
 "header.params.area_count": ("count", ROOMC, "the map's area count"),
 "header.params.history_ticks_count": ("count", ROOMC, "the hit history's depth in ticks"),
 "header.params.tick_start": ("count (a tick number)", SRV, "the run's first tick (63,736 for an S30)"),
 "header.params.ticks_below_wrap": ("count", LAB, "ticks the run starts below a multiple of 65,536 (-58: 30)"),
 "header.params.throttle_queue_bytes": ("bytes", NET, "the drop-tail queue depth for the 128 kbps throttle"),
 "header.params.throttle_queue_datagrams_count": ("count", NET, "the same in full datagrams"),
 "header.params.bottleneck_queue_datagrams_count": ("count", NET, "the bottleneck's queue depth in full datagrams"),
 "header.params.bottleneck_queue_ns": ("ns", NET, "the bottleneck's queue depth as time at the bottleneck rate"),
 "header.params.buffer_depth_ticks_count": ("count", ROOMC, "-30's input buffer depth set for P0"),
 "header.params.rewind_cap_ticks_count": ("count", ROOMC, "the rewind cap in ticks (test cap or rule cap)"),
 "header.params.aim_bits": ("bits", PROTO, "the aim's field widths from the codec"),
 "header.params.lg_ipv6": ("bool", LOAD, "LG reaches RS over IPv6"),
 "header.params.quinn_proto_version": ("version code", LAB, "major x 1,000,000 + minor x 1,000 + patch, from the lockfile"),
 "header.params.rustls_version": ("version code", LAB, "the same for rustls"),
 "header.params.ring_version": ("version code", LAB, "the same for ring"),
 "header.params.tokio_version": ("version code", LAB, "the same for tokio"),
 "header.params.wt_dangerous_config": ("flag", LAB, "wtransport's dangerous-configuration feature on"),
 "header.params.scrape_interval_ns": ("ns", LOAD, "Prometheus' scrape interval"),
 "header.params.ws_bots_count": ("count", LOAD, "bots on WebSocket in the soak"),
 "header.params.cert_validity_ns": ("ns", SRV, "the WebTransport certificate's validity period"),
}

FIG = {  # figure: (formula and inputs, the draft words it implements)
 "machine_named": ("1 if every log's header `machine` has role, name, plan, arch, cpu, vcpus, mem_mb, location, kernel, threads_per_core, smt and clock_res_ns, and a VM (RS, LG, MV) its day address; else 0. Inputs: headers", "PM-0001-1 'RS and LG named with plan, CPU model, vCPU count, memory, location, kernel and the day's address'; PM-0006-11's list"),
 "everyone_in_view_min_share": ("min over client slots of the share of that slot's snapshots with tick.s.everyone_in_view = 1. Inputs: tick.s.everyone_in_view (index = slot)", "PM-0001-2 'every client's relevant set holds all 47 other soldiers on at least 99 % of its snapshots'"),
 "fate_log_twin_equal": ("1 if header.params.fate_log_sha256 equals that of the run named in header.params.twin (role twin)", "PM-0001-3 'on L1, the same seed and profile give an identical packet-fate log on two runs'"),
 "sync_cost_ratio": ("U = mean(tick.v.server_only_ns) / (header.clients x mean(tick.s.relevant_count)); per N = 48 run: max over N in {24, 36} of U(run) / mean of U over that N's part. Inputs: the mark's three parts", "PM-0001-10 'at N = 48 at most 1.1 x the same figure at N = 24 and at N = 36'"),
 "sync_unit_cost": ("U per run, as above (ns per client-entity, a decimal)", "PM-0001-10, reported"),
 "down_client_mean_max_bits": ("max over client slots of the mean of second.s.down_bits[slot] over the measured seconds", "PM-0001-12 'mean at most 160 kbps ... counted per client'"),
 "up_client_mean_max_bits": ("the same over second.s.up_bits", "PM-0001-13 'mean at most 48 kbps per client'"),
 "r2_down_formula_bits": ("R2's worked downstream formula (ADR 0001 lines 1754-1779; r2 lines 1764-1766) at the run's own counts: mean entities per client set (tick.s.relevant_count), shots, spawns and events per second (second.v.shots_count, projectile_spawns_count, reliable_events_count); the arithmetic printed", "PM-0001-12 'its mean reported beside R2's formula evaluated at the run's own counts'"),
 "down_mean_ipv6_bits": ("mean over client-seconds of second.s.down_bits + 160 x second.s.down_datagrams_count (20 B more header per datagram)", "PM-0001-12 'if LG reaches RS over IPv6, the IPv6 figure'"),
 "rss_slope_bytes_per_10min": ("least-squares slope of second.v.rss_bytes over the measured seconds, x 600 (proposed method)", "PM-0001-15 'slope within 1 MB per 10 minutes after warm-up'"),
 "queue_growth_count": ("per client slot, for second.s.out_queue_count and second.s.event_queue_count: count of (slot, queue) whose max over the last 10 measured minutes exceeds its max over the first 10", "PM-0001-15 'its max over the last 10 minutes at most its max over the first 10 minutes'"),
 "session_memory_growth_count": ("the same rule on halves of the measured window, over second.s.session_buffered_bytes, out_queue_count and event_queue_count", "PM-0001-24 'memory per session flat over 10 minutes (PM-0001-15's rule)'"),
 "decoded_hash_mismatch_count": ("count of (client, tick) whose decoded hash differs from the server's logged quantised hash for that client and tick; with arm_rule on L2, only those the x86-64 re-decode (process `replay`) also misses; fails on 0 compared. Inputs: per-client hashes per tick (format: Q30)", "PM-0001-17 '100 % of decoded snapshots rebuild the server's quantised state'; -58; -19's join hash"),
 "decoded_hash_compared_count": ("the (client, tick) pairs decoded_hash_mismatch_count compared", "PM-0001-17, reported"),
 "cross_arch_mismatch_count": ("L2 mismatches (hash, shot view test, throw landing) that the x86-64 replay does not reproduce", "the Arm rule: 'reported as a cross-architecture finding and decides nothing' (-17, -19, -48, -54)"),
 "twin_hash_mismatch_count": ("count of tick numbers whose `h` differs between the run and the run named in header.params.twin (a tick in one only counts); fails on 0 compared", "PM-0001-21 'the state hash of a run with the forged set equals that of a run with only their valid parts'; -30 'identical server state hashes on every tick'"),
 "crash_close_max_ns": ("max over the run's clients of (its RoomEnded or close, event.room_end.server_est_ns on LG's server-clock estimate) minus the room's event.tick_panic.server_ns; a client with no room_end fails", "PM-0001-59 'every client gets RoomEnded with a crash reason or has its connection closed within that second (0 left open)'"),
 "kill_p99_ratio": ("p99 of tick.v.work_ns over the ticks after the set's `kill` event (ph = after) / p99 over the measured ticks", "PM-0001-27 'killing one room's process (SIGKILL) leaves the others' p99 within 5 %'"),
 "times_of_day_rule": ("1 if the mark's sessions (sets) start (header.params.started_utc) pairwise at least 6 h apart and at least one starts between 18:00 and 23:00 Europe/Amsterdam", "ADR 0006's key: 'three sessions at least 6 hours apart, one of them between 18:00 and 23:00 Amsterdam time'; 'npt-judge checks the 6-hour and 18:00-23:00 rule from the results' UTC times'"),
 "one_build_hash": ("1 if every run of the mark has the same client build (build.binary_sha256 of the web bundle)", "PM-0001-32 'the same build (one hash)'"),
 "rtt_agreement_p99_ns": ("p99 over the measured seconds of |second.v.app_rtt_ns (browser) - second.v.quinn_rtt_ns (room, that session)|, joined by second on the client's server-clock estimate", "PM-0001-33 'p99 of the absolute difference between app-level RTT and quinn's rtt(), sampled every second'"),
 "one_shot_dropped_count": ("sum of LG's second.v.one_shots_sent_count minus the room's second.v.one_shots_received_count over the run", "PM-0001-38 '0 of 10,000 scripted one-shot actions dropped'"),
 "bulk_control_p99_rise_ns": ("p99(second.s.control_delay_bulk_ns) - p99(second.s.control_delay_idle_ns)", "PM-0001-38 'raises the control stream's p99 message delay by at most 5 ms (p99 with the transfer minus p99 without)'"),
 "input_latency_p99_rise_ns": ("max over kinds (params.kind: move, flash) of p99(event.input_latency.to_screen_ns at P4) - p99(at P0), over the part's runs", "PM-0001-43 'p99 at P4 minus p99 at P0 at most 33.33 ms'"),
 "input_latency_p50_rise_ns": ("the same with medians", "PM-0001-43 'and the same for the medians'"),
 "view_hit_registered_share": ("among shots whose shooter's view test hit (event.view_test.hit = 1), the share the server registered (event.shot.registered = 1), joined by shot id; on L2 a non-registered view hit counts only if the x86-64 replay's view test also hits", "PM-0001-48 'of the shots whose ray hits the target's volume in the shooter's view ... at least 99 % register'"),
 "false_hit_share": ("among view misses (event.view_test.hit = 0), the share registered as hits; Arm rule on L2 as above", "PM-0001-48 'false hits (a miss in the view counted as a hit) below 0.5 %'"),
 "rule_cap_set_by_rule": ("1 if header.params.rewind_cap_ticks_count = ceil(p99 of event.shot.honest_rewind_ns in the picked option's P2 test-cap runs / (10^9 / tick_hz))", "PM-0001-49 'the honest rewind's measured p99 at P2, rounded up to a whole number of ticks'"),
 "throw_desync_count": ("throws with event.throw.desync = 1; on L2 only those whose x86-64 replay (process replay) also desyncs", "PM-0001-54 '0 desyncs of client-side flight against the server over 1,000 throws, per projectile kind'"),
 "rooms_per_vm_count": ("per N: per session, the largest k (params.k) for which every room run meets p99 <= 16,670,000 ns, max < 33,330,000 ns (whole run), 0 overruns and p99 <= 1.10 x the bracket mean; the figure is the lowest over the 3 sessions", "PM-0006-14 'the most rooms k for which every room meets R2-ROOM-REC2 marks 1 and 2'; 'the figure is the lowest count of the 3 sessions'"),
 "egress_interface_bytes": ("event.iface.tx_bytes at the end minus at the start", "PM-0006-17 'the interface's transmitted-byte counter read at the start and end'"),
 "r2_egress_formula_bytes": ("R2's formula (as r2_down_formula_bits) x clients x 3,600 / 8 (at R2's counts 146.1 kbps x 48 x 3,600 / 8 = 3.156 GB)", "PM-0006-17 'R2's formula ... evaluated in the results with the run's P, V, shots and projectile spawns, the arithmetic shown'"),
 "egress_vs_r2_ratio": ("egress_interface_bytes / r2_egress_formula_bytes", "PM-0006-17 'within +-20 % of R2's E3 estimate at the counts that ran'"),
 "egress_metric_vs_interface_ratio": ("(event.room_metric.sent_bytes at the end - at the start) / egress_interface_bytes", "PM-0006-17 'the room's own bytes-sent metric agrees with the interface counter within 5 %'"),
 "logging_cost_ratio": ("p99 over the pooled measured ticks of the part's 3 runs / p99 over the pooled ticks of the set's 3 role logging-off runs (tick.v.work_ns)", "PM-0006-19 'the pooled p99 of the 3 on runs (54,000 ticks) with that of the 3 off runs', 'at most 1.02 x'"),
 "alloc_twin_diff_count": ("count of measured tick numbers where tick.v.tick_allocs_count differs between the run and its params.twin (role logging-off)", "PM-0006-20 'on every tick after the warm-up ... equals the count with logging off', 'compared tick by tick'"),
}

# series read only by figures (inputs), and names the judge's validity rules read
FIG_INPUTS = {
 "tick.s.relevant_count": ("count", ROOMC, "one per client: entities in its relevant set this tick", "sync_cost_ratio, sync_unit_cost, r2_down_formula_bits"),
 "tick.s.everyone_in_view": ("flag", ROOMC, "one per client per snapshot: 1 when its set holds all 47 other soldiers (ES: and all 15 vehicles)", "everyone_in_view_min_share"),
 "second.s.down_datagrams_count": ("count", SRV, "one per client: snapshot datagrams sent to it this second", "down_mean_ipv6_bits"),
 "second.s.out_queue_count": ("count", SRV, "one per client: outbound queue length, each second", "queue_growth_count, session_memory_growth_count"),
 "second.s.event_queue_count": ("count", ROOMC, "one per client: unacknowledged events queued, each second", "queue_growth_count, session_memory_growth_count"),
 "second.s.session_buffered_bytes": ("bytes", SRV, "one per session: bytes buffered for it, each second", "session_memory_growth_count"),
 "second.v.app_rtt_ns": ("ns", WEB, "browser: the app-level (ping) RTT, each second", "rtt_agreement_p99_ns"),
 "second.v.quinn_rtt_ns": ("ns", SRV, "room: quinn's rtt() for the session, each second", "rtt_agreement_p99_ns"),
 "second.v.one_shots_received_count": ("count", ROOMC, "one-shot actions the room received this second", "one_shot_dropped_count"),
 "second.s.control_delay_bulk_ns": ("ns", BOTS, "one per control-stream message while a bulk transfer is in flight: its delay", "bulk_control_p99_rise_ns"),
 "second.s.control_delay_idle_ns": ("ns", BOTS, "the same with no transfer in flight", "bulk_control_p99_rise_ns"),
 "event.tick_panic.server_ns": ("ns", SRV, "the injected tick panic's time on the room's clock", "crash_close_max_ns"),
 "event.room_end.server_est_ns": ("ns", BOTS, "per client: its RoomEnded or close, on its server-clock estimate", "crash_close_max_ns"),
 "event.view_test.hit": ("flag", f"{BOTS}, {LAB} (replay)", "per shot (`shot` id): 1 when the ray hits the target's volume in the shooter's view", "view_hit_registered_share, false_hit_share"),
 "event.shot.registered": ("flag", ROOMC, "per shot (`shot` id): 1 when the server registered a hit", "view_hit_registered_share, false_hit_share"),
 "event.throw.desync": ("flag", f"{BOTS}, {LAB} (replay)", "per throw: 1 when the client's landing differs from the server's", "throw_desync_count"),
 "event.iface.tx_bytes": ("bytes", SRV, "RS's interface transmitted-byte counter (read at start and end)", "egress_interface_bytes"),
 "event.room_metric.sent_bytes": ("bytes", SRV, "the room's bytes-sent metric (read at start and end)", "egress_metric_vs_interface_ratio"),
}
VALIDITY = [
 ("tick.v.work_ns", "ns", SRV, "V-S's and clean-ref's p99s; PM-0006-13's bracket", "the same name as every tick-time check: one name"),
 ("second.v.steal_permille", "permille", SRV, "V-S's steal (RS's logs)", "the same name in -1 and the density reports"),
 ("second.s.inputs_count", "count", BOTS, "V-S: one per LG client, inputs sent this second, against tick_hz", "not read by any check; the room's own count is second.v.inputs_received_count (a different side, all clients)"),
 ("second.v.cpu_permille", "permille of the machine", f"{BOTS}, {LOAD}", "V-S, lg-80, PM-0006-13: each LG machine's CPU (< 800)", "the room process's share is second.v.proc_cpu_permille (a different quantity)"),
 ("tick.v.padded_step_ns", "ns", SRV, "padding: against header.params.padding_target_ns", "the same name in PM-0006-14's report"),
 ("tick.v.late_count", "count", ROOMC, "late-zero: sums to 0", "not read by any check (-30 names it in its notes)"),
 ("header.params.reorder_permille / event reorder (`permille`)", "permille", f"{L1}, {NET}", "reorder: above 0", "the same name in the -3, -20, -58 reports"),
]
PARAMS = [
 ("soak", "S30, S60 or none", "-4 to -16, -17, PM-0004-67 (selects)"), ("sim", "stress or stand-in", "-4's STAND-IN"),
 ("stand_ins", "list of names", "-2 (reported in the detail: Q16)"), ("cmd_timing", "A or B (OQ-0001-6)", "-20, -21, -42, -48, -49"),
 ("suite", "the run's test suite (wrap, joins, forged, clamp, fast-clock, lag-switch, never-ack, throttle, bottleneck, flood, origin, conn-panic, tick-panic, wt-attempts, fallback, fallback-443, both-transports, ws-stall, one-build, rtt, rtt-fixed-100, fuzz, deps, mds, datagram-timing, one-shots, brawl, own-change, input-latency, replay, extrapolation, delay-adapt, events, clock, rewind-cheat, aim-height, feedback, cues, checklist, throws, beside-vehicle, record, stalls, ping, firewall, fw-probe, rule-list, metrics-probe, egress, bill, logging-cost, logging-alloc, soak-8, cert-14d, cert-15d, rotation)", "selects throughout"),
 ("browser", "chrome, edge, firefox, safari, brave", "browser parts"), ("backend", "webgpu or webgl2", "-32"),
 ("transport", "wt or ws", "-25, -50, PM-0006-15"), ("crate", "wtransport or web-transport-quinn", "-28 (and -11, -25, -35 for -56's item 1)"),
 ("path", "loopback, eu-path, internet", "-28, -36"), ("cc", "newreno or cubic", "-24"), ("addr_cap", "raised or on", "-25, -29 (recorded)"),
 ("lg_ipv6", "bool", "-25"), ("density", "8-rooms or count", "-27, PM-0006-13, -14"), ("session", "D1, D2, D3", "-27, PM-0006-13"),
 ("k", "rooms in this step of the search", "PM-0006-14"), ("killed", "bool: the run kills this room on purpose (exempt from `complete`)", "-27, -59, PM-0006-22"),
 ("twin", "the run name of its twin", "-3, -21, -30, PM-0006-20"), ("cap", "test or rule", "-48, -49"), ("msg", "message type", "-34"),
 ("replay_depth", "6 or 7", "-44"), ("interest", "on or off", "PM-0006-17"), ("person_slot", "the person's slot", "-3"),
 ("kind", "move or flash", "-43"), ("collision_rule", "string", "-42 (recorded)"), ("fate_log_sha256", "hex string", "-3"),
 ("padding_target_ns", "ns", "padding rule"), ("event_queue_cap_count", "count", "-22 (reported)"),
 ("started_utc", "ISO 8601 UTC time the caller passes (proposed, Q25)", "times_of_day_rule"),
]

missing = []
rows_s, rows_f = [], []
for k, v in names.items():
    marks = ", ".join(x.replace("PM-0001-", "-").replace("PM-0004-", "0004-").replace("PM-0006-", "0006-") for x in v["marks"])
    if v["kind"] == "figure":
        f = k[7:]
        if f not in FIG: missing.append(k); continue
        rows_f.append((f, FIG[f], marks))
    else:
        if k not in S: missing.append(k); continue
        rows_s.append((k, S[k], marks))
extra = sorted(set(S) - {k for k in names}) + sorted(set(FIG) - {k[7:] for k in names if k.startswith("figure:")})
if missing or extra:
    print("MISSING", missing); print("EXTRA", extra); sys.exit(1)

order = {"tick.v": 0, "tick.s": 1, "second.v": 2, "second.s": 3, "event": 4, "header": 5}
def key(r):
    n = r[0]; p = ".".join(n.split(".")[:2]) if not n.startswith(("event", "header")) else n.split(".")[0]
    return (order[p], n)
rows_s.sort(key=key)
rows_f.sort()
md = []
md.append("# SERIES.md draft: every series and figure name the marks read (netcode proof test)\n")
md.append("Draft by test-engineer, 2026-11-06, for `spikes/netcode-proof/marks/SERIES.md`: the naming contract for U2-U7. "
          "Built from `marks-draft.jsonl` by `scratch-te/registry.py` (every name a mark reads has a row; the script fails otherwise). "
          "Marks: `-n` = PM-0001-n, `0004-67` = PM-0004-67, `0006-n` = PM-0006-n.\n")
md.append("## Rules\n")
md.append("- A name follows README §1: lowercase with underscores, ending in its unit (`_ns`, `_bytes`, `_bits`, `_permille`, `_count`) or a 0/1 flag with no suffix. "
          "Event field names follow the same rule (the draft's choice: README §1 states it for `v` and `s`).\n"
          "- Two units are missing and proposed (question Q3): `_um` (micrometres, integer) for lengths and `_microdeg` (millionths of a degree) for angles.\n"
          "- 'signed' marks a value that may go below 0 (an excess over a bound, a headroom).\n"
          "- 'Written by' is the crate that measures the value; npt-log's writer thread builds the record. `web` is the TypeScript page and Worker (not a crate). "
          "The probe and check tools named as `npt-lab` or `npt-bots (probe)` are placements to confirm at the build.\n"
          "- One quantity, one name: a name read by several marks is listed once, with all of them.\n")
md.append(f"## Series ({len(rows_s)})\n")
md.append("| Name | Unit | Written by (crate) | Meaning | Marks |\n|---|---|---|---|---|")
for n, (u, w, m), mk in rows_s:
    md.append(f"| `{n}` | {u} | {w} | {m} | {mk} |")
md.append(f"\n## Figures ({len(rows_f)}): computations npt-judge carries in code\n")
md.append("| Name | Unit | Written by (crate) | Meaning: formula and inputs | Draft words it implements | Marks |\n|---|---|---|---|---|---|")
for f, (form, words), mk in rows_f:
    unit = "ns" if f.endswith("_ns") else "bits" if f.endswith("_bits") else "bytes" if f.endswith("_bytes") else "count" if f.endswith("_count") else \
        "0/1" if f in ("machine_named", "fate_log_twin_equal", "times_of_day_rule", "one_build_hash", "rule_cap_set_by_rule") else "fraction (exact)"
    if f == "rss_slope_bytes_per_10min": unit = "bytes per 10 min, signed"
    md.append(f"| `{f}` | {unit} | npt-judge | {form} | {words} | {mk} |")
md.append(f"\n## Series read only by figures ({len(FIG_INPUTS)})\n")
md.append("| Name | Unit | Written by (crate) | Meaning | Figures |\n|---|---|---|---|---|")
for n, (u, w, m, f) in sorted(FIG_INPUTS.items(), key=lambda kv: key((kv[0],))):
    md.append(f"| `{n}` | {u} | {w} | {m} | {f} |")
md.append("\n## Names the judge's validity rules read (README §4; the coordinator's list, 2026-11-06)\n")
md.append("| Name | Unit | Written by (crate) | Rule | In this draft |\n|---|---|---|---|---|")
for n, u, w, r, note in VALIDITY:
    md.append(f"| `{n}` | {u} | {w} | {r} | {note} |")
md.append("\n## Header params the parts select on and the notes name (README §1's `params`)\n")
md.append("| Param | Values | Used by |\n|---|---|---|")
for n, vals, used in PARAMS:
    md.append(f"| `{n}` | {vals} | {used} |")
md.append("\nMachine names in selects: `machine.name` `RS`, `ML` (the named laptop), `ML*` (its stand-in), `MAC-r` (the runner), `MAC-b` (a borrowed Mac); "
          "`machine.role` `RS`, `LG`, `MP`, `MAC`, `ML`, `C`, and `RD` (not in README §1's list: Q6). `process`: `room`, `bots`, `browser`, `super`, `load`, `probe`, `replay`.\n")
md.append("## Open naming questions\n")
md.append("- Q3: no length or angle unit (`_um`, `_microdeg` proposed).\n"
          "- Q30: per-client quantised hashes per tick (48 per tick) have no home: `h` holds one hash per tick record, and `s` holds integers (a 64-bit hash exceeds JSON's safe 2^53). Proposal: an `hs` map (name -> list of 16-hex strings) on tick records, or the hash as two 32-bit halves in `s`.\n"
          "- Q16: string values (the negotiated draft, browser versions, the stand-ins, the certificate presented) need the verdict detail to print named header params.\n"
          "- Q25: `params.started_utc` for the times-of-day rule.\n")
open(OUT, "w").write("\n".join(md) + "\n")
print("wrote", OUT, len(rows_s), "series,", len(rows_f), "figures,", len(FIG_INPUTS), "figure inputs")
