# U5 allocation bounds per fuzz target (test-engineer, 2026-11-21)
Task: for each of U5's 17 fuzz targets (PM-0001-34), the largest single heap allocation any input within the type's
wire limit can make its decode path ask for (option (a) of U5-ALLOC; advisor-u5-brief.md :39-45, :63-77, :244-261).
Read at head a94c865 (branch claude/netcode-proof-test-plan-hcky5g). Read only, except this file and .gate/u5-probe.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan
1. Read limits.rs:12-51 (the limits). 2. Read msg.rs `decode` (:303) and every per-type decode it reaches (input.rs,
snapshot.rs, control.rs, bits.rs, quant.rs, error.rs). 3. Read seam types (sim seam.rs). 4. Read client lib.rs receive
path for Snapshot/FullState and the stress scene. 5. Read ws_frame.rs Scanner. 6. If disk > 6 GiB: scratch worktree,
size_of probe under the lane lock; else arithmetic. 7. One block per target, each with sites, reach, element size,
bytes at growth, size-first check. 8. Summary table.
Disk at start: 7,705,645,056 B free (above 6 GiB = 6,442,450,944): probe allowed.
STATUS: steps 1-5 done (notes below); step 6 next.

## Notes (one line per finding, as I go)
- Limits (proto/src/limits.rs:17-51): INPUT 166, SNAPSHOT 1,000 send / SNAPSHOT_RECEIVE 1,150, FULL_STATE 65,536
  (after the 4 B length), RESYNC 8, PING 32, PONG 32, JOIN 128, JOIN_ACCEPTED 256, JOIN_REJECTED 16, CLIENT_SETTINGS 8,
  ROOM_ENDED 64, LEAVE 16, KICK 16, SESSION_CONTROL 512, SESSION_STATUS 32, ROOM_CONTROL 1,024; NOTE 480, PEERS 16,
  REDUNDANCY_CAP 8. A control-stream limit counts its 2 B length (msg.rs:256-263 `after_length`); FullState's limit
  is after its 4 B length, so its largest input is 65,540 B.
- Size-first in `Quant::decode` (msg.rs:396-501): :403 `bytes.len() > channel_max(way, channel)` -> TooLarge; :406 type
  byte; :411 `bytes.len() > frame + receive_limit(t)` -> TooLarge; both before `BitReader::new` (:429) and every type's
  `get`. Nothing on the path allocates before :411. `decode` (:303) is `Quant::default().decode`. channel_max values
  pinned by msg.rs:548-559 (ClientToRoom Datagram 166, Control 512; RoomToClient Datagram 1,150, Control 256,
  FullState 4 + 65,536; BothWays Datagram 32).
- BitReader (bits.rs:57-152): borrows the slice; `get`, `array::<N>` (stack array), `take` (a sub-slice), `align`,
  `finish`: no heap. Quant (quant.rs:98-235): `get`/`value` integer only, no heap. DecodeError (error.rs:9-37): a Copy
  enum, no String. So a scalar type's decode allocates 0 B: the Message is returned by value.
- Cross-type reach: `decode` reads the type byte from the input (msg.rs:406-430), so a target that feeds raw bytes on a
  way and channel reaches every type that travels that way on that channel (direction :422, channel :425-427; a type of
  another channel is refused WrongChannel before its body), not only the named type. Recorded per target below.
- Input::get (input.rs:202-301): count read at :208, refused above 8 at :209-211, then `Vec::with_capacity(count)` at
  :212 BEFORE any command is read; pushes (:294) never pass that capacity, so no regrowth. A 5 B input (type, ack 2,
  groups 1, count = 8) reaches it, then fails Truncated. Element `npt_sim::Command` (sim/src/seam.rs:22-44).
- Snapshot::get (snapshot.rs:556-624): three `Vec::new()` + push: own :589-598 (n_own u8 <= 255), entities :599-608
  (n_entities u16 <= 65,535), events :609-612 (n_events u8 <= 255). A push happens only after a whole item decodes, so
  the reach is the bits left. Smallest delta: Remove = kind 3 + index 16 + op 2 = 21 bits (get_delta :330-386;
  KIND_BITS 3 at :182, asserted :739); refused only when baseline_age == 0 (:382-384). Smallest event: Kill =
  16 + 8 + 2 + 8 + 8 = 42 bits (:417-437).
  Bits at the 1,150 B receive limit: 1,149 B body = 9,192 bits; header 96 + counts 32 = 128; 9,064 left.
  Entities: floor(9,064 / 21) = 431 removes (valid at 1,149 B) -> pushes 257..512 end at 512 slots. Own: <= 255 ->
  256 slots. Events: floor(9,064 / 42) = 215 -> 256 slots. 513 entity pushes would need 10,773 bits: unreachable.
- FullState::get (snapshot.rs:690-715): two `Vec::new()` + push: own :701-704 (n_own u8 <= 255), entities :705-708
  (n_entities u16 <= 65,535). Smallest record: Door = kind 3 + index 16 + generation 16 + OPEN 1 = 36 bits (get_record
  :238-259; DOOR layout seam.rs:290; OPEN = Field::integer(0, 1) at :222, bits = 64 - lz(span 1) = 1, grid.rs:59-69).
  Door is in FullState's entity section (snapshot.rs:656-667); index uniqueness is not checked at decode.
  Bits at the limit: 65,540 B input, 65,535 B body = 524,280 bits; header 56 + counts 24 = 80; 524,200 left.
  floor(524,200 / 36) = 14,561 doors (524,196 bits, 4 pad bits: a VALID message, so the client path runs whole) ->
  pushes 8,193..16,384 end at 16,384 slots; 16,385 would need 589,860 bits: unreachable. Own: 255 Effect records
  (78 bits each) fit -> 256 slots.
- Layout bits (seam.rs:218-295, grid.rs:106-158, TICK_HZ 30 at ticks.rs:12): Soldier 92 (pinned snapshot.rs:747),
  Vehicle 60, VehiclePart 76, Projectile 142, Door 1, Wall 11, Effect 43, OwnState 48+16+FIRE_TIMER+RELOAD_LEFT+14+24.
- SessionControl::get (control.rs:248-290): RecordStep's note `note.to_owned()` at :285 (a String of exactly len B;
  `str::to_owned` -> `to_vec` -> with_capacity(len)); len refused above 480 at :273-275, bytes must be present (`take`
  :276, bits.rs:121-128), UTF-8 and note_ok checked before (:277-280, no heap: `chars()`). At the 512 B limit: 512 - 2
  length - 1 type - 21 (variant 1, cap 16, step 1, result 1, len 2) = 488 >= 480, so 480 B reachable. Empty note: 0 B.
- SessionStatus::get (control.rs:307-320): scalars only, 0 B.
- RoomControl::decode (control.rs:601-694): size-first :602-604 (> 1,024 B -> TooLarge) before BitReader (:605).
  get_peers (:428-451): count n read :429, refused 0 or > 16 at :433-435, then `Vec::with_capacity(n)` at :436 BEFORE
  any address is read; pushes never pass it. One peer list per control (codes 4, 7, 8, 9, 10, 11). A 4 B input
  [2, 0, 7, 16] reaches with_capacity(16). Element `std::net::IpAddr`.
- Client receive (client/src/lib.rs:783-831): `self.quant.decode(Direction::RoomToClient, channel, bytes)` at :784, so
  the same size-first checks; Snapshot -> `group` (:1023-1113), FullState -> `full_state` (:1116-1153).
  `input` (:532-558) takes `received: &[(Channel, Vec<u8>, u64)]`: the harness's own Vec of the input bytes (<= limit),
  outside the decode path; it must sit outside the counting window or it is counted (it is under every bar anyway).
- group() per message: decode vectors above; `held.frame.clone()` at :1065 (baseline frame, two Vec<Held>, clone =
  exact len); `tick.frame = base.clone()` at :1074 (second exact clone); `apply` (:117-150) `list.insert` at :135/:145
  grows the cloned list from cap == len (first realloc max(2 x len, len + 1, 4)); `tick.events.push` at :1095
  (Vec<(u32, SimEvent)>, from empty); on a complete tick two exact-size collects of EntityRecord at :1107-1109 and
  client_hash (proto/src/hash.rs:60-66, :89-95: a Vec<&T> of 8 B and its sort only when unsorted). Small: out.hashes,
  out.lost (see :889-934, at most 17 pushes of 8 B), out.wraps, bad_headers (<= 32 u32), pings (<= 8 x 24 B), and under
  the controller ClockSync's samples (clock.rs:159-168, <= 64 held, 40 B each: <= 126 x 40 = 5,040 B).
  With baseline_age 0 the base is `Frame::default()` (no heap) and every delta must be an Add: Door add = 3+16+2+16+1 =
  38 bits -> floor(9,064 / 38) = 238 adds into an empty list -> 256 slots of Held.
  With baseline_age > 0 the baseline must be a complete, hashed frame in the ring (:1054-1066): in a fixed joined state
  that is the FullState's tick T, so a snapshot at T + a with age a (1 to 31) clones it.
- full_state() per message (:1116-1153): decode vectors; `frame.entities` collect at :1125 and `frame.own` at :1126
  (exact-size map over a slice -> n x size_of::<Held>()); `sort_by_key` at :1128-1129 (std's stable sort: a scratch
  buffer, see below); two exact EntityRecord collects at :1130-1131; client_hash (8 B per record, only when unsorted:
  duplicate keys with unordered generations make it so).
- Held (client lib.rs:70-74): EntityRecord + [f32; 12]: 104 + 48 = 152 B by arithmetic (private type; measured by a
  stand-in of the same fields).
- Stress scene (sim/src/scene.rs:166-190): 48 soldiers, 15 hulls, 4 x 20 + 11 x 4 = 124 parts (LARGE_PARTS 20,
  SMALL_PARTS 4, :149-151), 300 live projectiles (pool 320), 8 effect rows per soldier; greybox doors 6 (greybox.rs:
  92-137, one per building, six buildings), 1 wall. A client's relevant set excludes its own soldier (room.rs:1160-1162).
  Entity section at most 47 + 15 + 124 + 320 + 6 + 1 = 513 records (493 at 300 live projectiles); own section 1 soldier +
  1 OwnState + 8 effects = 10 records (room.rs:1380-1398 builds the FullState from own + relevant).
- std behaviour used (rustc 1.99.1; no rust-src installed, so not read from the toolchain's own source): push/insert
  growth RawVec::grow_amortized = max(2 x cap, cap + 1, MIN_NON_ZERO_CAP) with MIN_NON_ZERO_CAP 8 for 1 B elements, 4
  up to 1,024 B, 1 above; Vec::clone, to_vec, to_owned and exact-size collect ask with_capacity(len); stable sort
  (driftsort, std since 1.81): no heap for len <= 20, else scratch alloc_len = max(len - len/2, min(len, 8,000,000 /
  size_of::<T>()), 48) elements, on a 4,096 B stack buffer when it fits, else Vec::with_capacity(alloc_len). The probe
  measures the growth and clone capacities; the sort scratch cannot be measured without an unsafe counting allocator
  (forbidden by the workspace lints), so it stays computed.
- Scanner (net/src/l2/ws_frame.rs:82-311): fixed state (State enum with a [u8; 14] header buffer, :82-92; Scanner
  :100-112, constructed by value :116-145); `scan` (:170-246) and `header` (:249-311) index and copy only: no heap on
  any input. The module text says so (:23-24). No size-first refusal applies: a stream scanner takes any slice; its own
  caps (max_payload, max_head) close the stream, they do not gate an allocation.
