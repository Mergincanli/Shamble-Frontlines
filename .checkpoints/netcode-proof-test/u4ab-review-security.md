# security-auditor: U4a / U4b round 1 review (netcode proof test)

Started 2026-11-10. Read-only on both worktrees. Focus: U4a decoders as attack surface. U4b: not reviewed.

## Plan
1. Read plan §5 U4a + §4.1/4.3/4.4-4.6/4.10/4.11/4.14, advisor rulings, carries 1-5 and 34, catalogue doc, netcode rules.
2. Read U4a builder notes (u4a-build.md).
3. `git status` / `git diff` in the U4a worktree.
4. Per decoder (bits, control, input, snapshot, wrap, msg, quant, hash, limits, error):
   size check before alloc/parse; length prefixes; integer overflow in sizes/counts; panic paths
   (index, slice, unwrap, arithmetic overflow, shifts >= width, as-casts); trailing bytes; unknown fields/
   reserved bits; per-type and per-channel limits.
5. Tests: wire.rs, bites.rs, catalogue.rs, common/mod.rs: do they prove a2 prefixes + random buffers, a3, a6,
   catalogue items 2, 3, 6.
6. check.mjs / check.test.mjs: what it checks (panic lint? unsafe?), clippy.toml.
7. Weigh skeptic S1 (layout shared by own-state and remote groups).
8. Findings with file:line, scenario, cheapest proof, before-merge flag.

## Progress
- 1 done: plan §4.1/4.3-4.6/4.10/4.11/4.14/§5 U4a read; u4a-build.md read; catalogue rows :61-101, :136-137 read.
- 2-4 done (read): bits.rs, error.rs, limits.rs, msg.rs, input.rs, quant.rs, snapshot.rs, control.rs, wrap.rs (part),
  hash.rs, lib.rs, clippy.toml; seam.rs:85-195 and grid.rs:40-100 for layout/Field bounds.
  Totality walk: every index in msg.rs decode (378-379, 392) is behind `bytes.get(frame)` at 369; row()/content_limit()
  only after the 1..=15 check (371); BitReader get/array/take all checked (bits.rs:69-128); layout <= FIELDS=12 <= 16
  by const asserts (seam.rs:184-195, snapshot.rs:73), Field bits 1..=32 (grid.rs:59-69 asserts min<max), so no shift
  overflow on decode; groups-1 in Snapshot::put guarded by group>=groups (snapshot.rs:460). overflow-checks=true in
  release for npt-proto (Cargo.toml:111-112) - any arithmetic overflow would be a panic; none found on decode paths.
  Allocation: Input with_capacity(<=8) after the count cap (input.rs:200-204); note <=480 after len check (control.rs
  228-241); peers with_capacity(<=16) (control.rs:389-392); Snapshot/FullState Vec::new + push bounded by the size
  check (each item >= 21 bits). No allocation before size checks.
- 5 done: wire.rs (a1, a2 prefix + random + flips, a3, a6, a8), catalogue.rs diff (items 1, 2, 3 size half, 6).
- 6 done: check.mjs diff (adds checks, weakens none; STEPS called with no args at :1644), check.test.mjs no side effects.
## Findings (one line each, as found)
- SEC-U4a-1 Medium (S1, security side), before merge: a ruling. snapshot.rs:173-194, :223-236, :429-437, :481-483
  send a kind's whole layout in every section; Soldier is in "own state" (Owner) and "remote soldiers" (Spatial).
  seam.rs:114-116 velocity today; U4d in flight (.claude/worktrees/wf_f54246ff-e0d-1 seam.rs:186-221) adds
  WEAPON_STATE (0 ready, 1 cooling down, 2 reloading, 3 empty) to SOLDIER. Catalogue :136 makes weapon state owner-only;
  :137 has no velocity or weapon state. Scenario: after U4a+U4d+U4e, a scripted client reads every nearby enemy's
  cooldown/reload/empty state from each snapshot. Proof: encode one Soldier Add in `entities`, assert its width is the
  remote row's (fails by velocity's 48 bits today). Fix (a) audience per layout entry in npt-sim + per-section
  selection in U4a (wire + public items change: before merge) or (b) owner-only fields in an owner-only kind (as
  Effect), U4a unchanged but one GROUPS line; binds U4d. Nothing sending to >1 client (U4e) merges before it.
- SEC-U4a-2 Low: GROUPS not enforced by the codec: Snapshot::put :481-483 / get :523-530 take any kind in either
  section; tests pin it (common/mod.rs:96-97, 238-240, 260-264; wire.rs:39-55 round-trip Effect in `entities`).
  Proof: encode entities=[Add(Effect)] -> Ok today. Fix: refuse kinds the section's GROUPS rows don't list.
- SEC-U4a-3 Low: Debug derives print secrets: msg.rs:63-74 (Join token, bind), control.rs:49-65 (SessionControl cap),
  msg.rs:155 Message. A `{:?}` in U4e's pump/room logs writes them in clear (plan E22; catalogue "never the data").
  Proof: format!("{:?}", Message::Join(token [0xab;16])) contains "171". Fix: hand-written Debug, "[16 B]".
- SEC-U4a-4 Low: no lint backstop against panics in decoders (lib.rs; overflow-checks=true in release,
  Cargo.toml:111-112). Every current site safe by invariant (msg.rs:214, 378-379, 382, 392; snapshot.rs:152, 175, 261,
  312; hash.rs:56; control.rs:563). Fix: cfg_attr(not(test), deny(indexing_slicing, arithmetic_side_effects,
  unwrap_used, expect_used, panic)) + per-site allow with reason.
- SEC-U4a-5 Low: a2's prefix half never reaches a stream type's body: wire.rs:180-189 and :192-201 cut without
  rewriting the length prefix; every cut of the 11 stream types + RoomControl stops at BadLength (msg.rs:382-384,
  control.rs:562-565). Client->room stream types: Resync, Join, ClientSettings, Leave, SessionControl. Proof: plant
  unchecked slice in BitReader::take (bits.rs:125); prefix half still passes. Fix: rewrite prefix to the cut length.
- SEC-U4a-6 Low: Input accepts ack_tick != 0 with ack_groups == 0 (input.rs:198-199, :24) where Snapshot refuses a
  value without its flag (snapshot.rs:501-509). Proof: decode it -> Ok. Fix: UnknownField when strict; NotCanonical.
- SEC-U4a-7 Low: encode's TooLarge for Snapshot is the 1,150 B receive limit (msg.rs:219-225, :343), not the 1,000 B
  send cap Limits::TYPES (limits.rs:61-66) and error.rs:248 state (catalogue :88). Proof: encode a 1,100 B snapshot ->
  Ok. Fix: encode refuses above Limits::SNAPSHOT.
- Carries (not U4a findings): U4e: Pong's echo_ns is client-echoed; RTT from the server's own send record by ping id
  only (catalogue Pong row), else a client shifts its view-time estimate (rewind still capped). U4e/npt-net: the
  control-stream reader needs the 512 B bound before buffering; channel_max is pub(crate) (msg.rs:242), so it would copy
  the number (a pub Limits::receive_max(way, channel) gives one home). U6: RoomControl (InjectPanic, InjectStall) is in
  measured builds; the local socket's permissions are its only wall.
- Considered, no finding: bites cannot reach a measured build (Quant/Wrap bite fields and with_bite cfg-gated,
  strict()/checks_channel() true; no manifest enables bites by default, resolver 3); note Cc/Cf rule (control.rs:120-152)
  as the catalogue states; no allocation before size checks; slot never on the wire (wire.rs:532-538); hash is a test
  instrument, not a security primitive.

## Anti-cheat coverage (U4a's part: what the wire allows vs. where the server must refuse)
| Player action / field | npt-proto (U4a) | Server check (owner unit) | Gap |
|---|---|---|---|
| move_xy | any i8 pair (input.rs:119-121) | room: length <= 1 (U4e) | none at U4a, by design |
| aim point | each axis in its grid field; out-of-range code refused (quant.rs:197-216) | sim: map bounds, reach clamp | none |
| buttons / presses (fire, reload) | any bits | room: one edge per tick; sim: cooldown, ammo, reload | none |
| command tick / seq | any u32; seq implied newer-1; step 0-255 | room: strictly newer, lead bound, Wrap forward (U4e) | none |
| view time (rewind) | offset <= 65,535 ticks | room: within one interval of the estimate; rewind cap | none at U4a |
| commands per datagram | <= 8 (input.rs:200-203) | room CMD_CATCHUP 8/tick; pump bucket | none |
| ack | any 16-bit tick + mask | room: names a sent snapshot | SEC-6 (non-canonical "no ack") |
| slot | never on the wire (decoder writes 0) | session sets it | none |
| Join token / bind | 16 B each | room: constant-time compare; bind equal | SEC-3 (Debug prints them) |
| SessionControl cap | 16 B | server: constant-time compare | SEC-3 |
| ClientSettings delay | any u16 | room: clamp to 2-5 intervals, 1/s step | none |
| RecordStep note | <= 480 B UTF-8, no Cc/Cf | logged with source | none |
| Ping / Pong | sizes; ids u8 | server: 10/s rate, outstanding id, hold <= 1 tick | carry (echo_ns) |
| what a client may see | GROUPS classes descriptive only | room interest (U4e) | SEC-1 (whole layout to every section), SEC-2 |
