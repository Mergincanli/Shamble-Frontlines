# U4 build plan review (engine-architect), rev 1 of u4-build-plan.md

Memory: no agent-memory dir exists (checked); start fresh.
Branch claude/netcode-proof-test-plan-hcky5g; HEAD a99dd7f (autosaves on f985226, only .checkpoints changed).

## Plan
1. Read u4-build-plan.md (rev 1) in full; read plan.md's U4/U6/U7 sections and the marks.
2. Read the three scout notes; list claims the plan rests on.
3. Check against code: crate layout, Cargo.toml deps, sim seam, TICK_HZ, Link, public items.
4. Focus: §4.2 dep rules, §4.5 determinism native+Wasm, §4.7 public items, sim seam/TICK_HZ, Link use,
   no U6/U7 pull-forward, traced need per feature, sub-unit buildability.
5. Findings (severity, where, claim, evidence, fix), verdict.

## Findings log
- (read) plan rev 1 in full (646 lines); plan.md §4.1-§4.7 (:365-611), §12.1 U4-U7 (:1219-1226); arch scout F1-F48;
  notes M1-M25; net F90-F124; marks F10-F32; manifests; stub crates (verified :1-6 each); check.mjs LEAF :25,
  UNSAFE_EXCEPTIONS :135-137, WASM_CRATES :1110; link/src/lib.rs:63-94 (rtt_ns -> Option<u64>); sim/clippy.toml
  (mul_add explicitly allowed :3); room/clippy.toml clock ban only (:1-9); figures.rs:175-224 (planned U4 figures).
- A1 HIGH seam/recording: U4-HIT passes "two history entries and a fraction" into step (plan :89-92, §4.9 :196) and
  U4-REPLAY records "each tick's seam inputs" (:95) -> recordings carry rewound volume sets per firing command
  (~3 KB/tick-set: net F106; ~23 shots/tick at 700/s) = ~140 KB/tick undeduped, ~1.26 GB per 9,000-tick SS
  recording; or, if "entries" means indices, npt-sim alone cannot resolve them (HitHistory is room's: plan.md:577),
  and hash-wasm depends on npt-sim only (:359). Either reading leaves U4d's Rewind/Recording unspecified, and U4d is
  built in wave 1. Fix: Rewind = (tick_a, tick_b, frac, cover variant) integers; a fixed volume ring that the replay
  rebuilds from the sim's own hit_volumes() (in npt-sim, or the replay driver in npt-sim), state recording size.
- A2 MED seam: AreaFlood/portals read-out missing (arch F31 unanswered); §4.2 seam lists step/state_hash/
  entity_records/hit_volumes only (:125-127). Room generic over S cannot read World's portal state.
- A3 MED clamp is float geometry in room + needs fire origin & reach not on the seam (§4.10 :208-212; plan.md:503;
  room/clippy.toml has no float bans). Fix: npt-sim pure fn clamp_to_reach + read-outs; room float bans.
- A4 MED LOOP_CATCHUP in npt-server's Schedule (:115-116, :135) unreachable from client-wasm (server->net->tokio,
  server/Cargo.toml:24) yet the Worker needs the same cap (ADR 0002:336-337; plan.md:485). Also U4b "depends on
  nothing" but derives from npt_sim::TICK_HZ (U4d). Fix: const beside TICK_HZ in npt-sim; U4b parametric.
- A5 MED tick-counted values stated as fixed ticks (60-tick FullState ack bound :175; 64-tick ring :194; 30 ticks
  ClientSettings) vs plan.md:544-546 "every tick-counted timer derives from TICK_HZ" + :552 swap changes no other crate.
- A6 MED RTT source switch on params.transport inside room (§4.9 :200-202); server pings (ADR 0001:367-368); Link
  rtt_ns is Option (link :80-81). Fix: server's pump hands room one RTT per client.
- A7 MED U4-WINDOW budget shared by server pings/pongs and room encoder (§4.5 :160-162) but plan.md:505-506's
  server->room inputs lack "bytes sent since last tick"; ownership of the 256 kbps budget unstated.
- A8 MED RQ4 build flags (no +fma/target-cpu/simd128/relaxed-simd) have no check in any sub-unit; check.mjs refuses
  lint-lowering rustflags only (:903-942).
- A9 MED parallel lanes share files: U4a (a9) and U4b both edit check.mjs/check.test.mjs; U4d + U4d2 edit
  Cargo.toml; counts per brief from U3d baseline go stale after first merge (§5 :249-252, lanes :476-479).
- A10 MED U4e too large (L, ~7 crates + judge + marks + run scripts + 5 mark groups); judge/marks change has no
  dependency on room (npt-judge -> npt-log only, plan.md:426): split out to wave 1.
