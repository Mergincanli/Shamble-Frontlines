# U5 allocation bounds per fuzz target (test-engineer, 2026-11-21)
Task: for each of U5's 17 fuzz targets (PM-0001-34), the largest single heap allocation any input within the type's
wire limit can make its decode path ask for (option (a) of U5-ALLOC; advisor-u5-brief.md :39-45, :63-77, :244-261).
Read at a94c865; the probe ran at b8b6708 (`git diff --stat a94c865 b8b6708 -- spikes/netcode-proof` is empty: the
spike is the same). Read only, except this file and the scratch copy .gate/u5-probe (removed at the end).
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan
1. Read limits.rs:12-51. 2. Read msg.rs `decode` (:303) and every per-type decode it reaches. 3. Read the seam types.
4. Read the client's receive path and the stress scene. 5. Read ws_frame.rs's Scanner. 6. Probe sizes (disk allowed).
7. One block per target. 8. Summary table.
STATUS: all steps done (2026-11-21). Disk at start 7,705,645,056 B free (> 6 GiB): probe allowed and run.

## The probe (measured, rustc 1.99.1, x86_64-unknown-linux-gnu, debug test build)
Scratch copy `git worktree add .gate/u5-probe HEAD`; one file crates/proto/tests/u5_probe.rs (public API only: encode,
decode, RoomControl, the seam types; a hand bit-writer for snapshots at 1,150 B, since `encode` caps a snapshot at its
1,000 B send limit); run `CARGO_TARGET_DIR=.gate/shared-target flock -x -w 300 .gate/shared-target/.lane.lock cargo
+1.99.1 test --manifest-path .gate/u5-probe/spikes/netcode-proof/Cargo.toml -p npt-proto --test u5_probe --locked
--offline -- --nocapture`: 1 passed, built in 5.42 s, lock taken at once. Then the file deleted and `git worktree remove
.gate/u5-probe` (the guard refuses `--force`; removing the one untracked file first made the plain remove work).
Output, verbatim:
- size_of: Command 32; EntityRecord 104; EntityDelta 104; SimEvent 8; (u16, u8, SimEvent) 12; (u32, SimEvent) 12;
  IpAddr 17; HeldStandIn (EntityRecord + [f32; 12], npt-client's private `Held`'s fields) 152; &EntityRecord 8;
  (u32, u64) 16; (u32, u32) 8; (u8, u64, u64) 24; Message 88.
- Input 8 commands: 42 B on the wire, commands.capacity() = 8. Header-only [1,0,0,0,8]: Err(Truncated) (it reserves,
  then fails: the reservation is not observable from outside, only by reading input.rs:212).
- Snapshot 431 removes: 1,149 B, entities.len() 431, capacity 512. 432 removes: 1,151 B -> Err(TooLarge).
  255 own removes: 687 B, own.capacity() 256. 215 kills: 1,146 B, events.capacity() 256. 216 kills: 1,151 B ->
  Err(TooLarge). 5 removes: 31 B, entities.capacity() 8; 6 removes: 33 B.
- FullState 14,561 doors: 65,540 B, entities.capacity() 16,384. 14,562 doors: encode Err(TooLarge).
- RoomControl 16 peers: 278 B, addrs.capacity() 16. [2,0,7,16]: Err(Truncated) (reserves 16, then fails).
- SessionControl note 480: 504 B, note.capacity() 480. Note 104: 128 B, note.capacity() 104.
- Growth: Vec<EntityDelta> push caps over 512 pushes [4, 8, 16, 32, 64, 128, 256, 512]; Vec<EntityRecord> over 14,561
  pushes [4, ..., 8192, 16384]; clone of 513 Held: capacity 513; after one insert: 1,026; Held inserts from empty over
  238: [4, 8, 16, 32, 64, 128, 256]; exact collect of 14,561 Held: 14,561; with_capacity(16) IpAddr: 16;
  str::to_owned 480: 480.
- A Vec's capacity is the element count it asked the allocator for (Vec keeps the requested capacity), so capacity x
  size_of is the request in bytes. What the probe cannot see: the client's private vectors (computed from the same
  measured rules) and std's sort scratch (needs an unsafe counting allocator, forbidden by the workspace lints: computed).

## Notes (one line per finding, as I went)
- Limits (proto/src/limits.rs:17-51): INPUT 166, SNAPSHOT 1,000 send / SNAPSHOT_RECEIVE 1,150, FULL_STATE 65,536 (after
  its 4 B length: largest input 65,540 B), RESYNC 8, PING 32, PONG 32, JOIN 128, JOIN_ACCEPTED 256, JOIN_REJECTED 16,
  CLIENT_SETTINGS 8, ROOM_ENDED 64, LEAVE 16, KICK 16, SESSION_CONTROL 512, SESSION_STATUS 32, ROOM_CONTROL 1,024; NOTE
  480, PEERS 16, REDUNDANCY_CAP 8. A control-stream limit counts its 2 B length (msg.rs:256-263).
- Size-first in `Quant::decode` (msg.rs:396-501): :403 `bytes.len() > channel_max(way, channel)` -> TooLarge; :411
  `bytes.len() > frame + receive_limit(t)` -> TooLarge for the type byte read at :406; both before `BitReader::new`
  (:429) and every type's `get`. `decode` (:303) = `Quant::default().decode`. channel_max pinned by msg.rs:548-559.
- BitReader (bits.rs:57-152), Quant (quant.rs:98-235), DecodeError (error.rs:9-37, a Copy enum): no heap. A scalar
  type's decode allocates 0 B (the Message, 88 B, is returned by value).
- Cross-type reach: `decode` takes the type byte from the input (msg.rs:406), so a target feeding raw bytes on a way
  and channel reaches every type that travels that way on that channel (direction :422; another channel's type is
  refused WrongChannel at :425-427 before its body). Each block gives (a) the named type's own path (type byte pinned)
  and (b) the reach with the type byte free, within the named type's own size.
- Input::get (input.rs:202-301): count :208, refused above 8 :209-211, `Vec::with_capacity(count)` :212 before any
  command is read; pushes (:294) stay within it.
- Snapshot::get (snapshot.rs:556-624): own :589-598 (u8 count), entities :599-608 (u16), events :609-612 (u8), each
  `Vec::new()` + push after a whole item decodes. Smallest delta Remove 21 bits (:330-386; KIND_BITS 3 :182, :739),
  refused only at baseline age 0 (:382-384); smallest event Kill 42 bits (:417-437). At 1,150 B: 9,192 body bits - 128
  header and counts = 9,064: 431 removes, 215 kills (measured above).
- FullState::get (snapshot.rs:690-715): own :701-704, entities :705-708, push-grown. Smallest record Door 36 bits
  (:238-259; seam.rs:290, :222; grid.rs:59-69: OPEN is 1 bit); Door is in the entity section (snapshot.rs:656-667); no
  index check at decode. At 65,540 B: 524,280 body bits - 80 = 524,200: 14,561 doors, a valid message (measured).
- SessionControl::get (control.rs:248-290): `note.to_owned()` :285, len refused above 480 :273-275, bytes present
  (`take` :276), UTF-8 and note_ok before it (:277-280, no heap). At 512 B: 512 - 24 = 488 >= 480. Empty note: 0 B.
- SessionStatus::get (control.rs:307-320): scalars, 0 B.
- RoomControl::decode (control.rs:601-694): size-first :602-604 before BitReader :605; get_peers (:428-451) count :429,
  refused 0 or above 16 :433-435, `Vec::with_capacity(n)` :436 before any address; one peer list per control.
- Client receive (client/src/lib.rs:783-831): `self.quant.decode(Direction::RoomToClient, channel, bytes)` :784 (the
  same size-first checks); Snapshot -> `group` :1023-1113; FullState -> `full_state` :1116-1153. `input` (:532-558)
  takes `&[(Channel, Vec<u8>, u64)]`: the harness's own copy of the input (<= the limit) must sit outside the window.
- group(): decode vectors; baseline clone `held.frame.clone()` :1065 and `tick.frame = base.clone()` :1074 (two exact
  clones of the held frame, Vec<Held> each list); `apply` (:117-150) `list.insert` :135/:145 grows the cloned list from
  capacity == len (measured: 513 -> 1,026 on the first insert); `tick.events.push` :1095; on a complete tick exact
  EntityRecord collects :1107-1109 and client_hash (hash.rs:60-66, :89-95: 8 B per item, only when unsorted).
  Age 0: base `Frame::default()` (no heap), every delta an Add; Door add 38 bits -> 238 adds -> 256 Held (measured).
  Age > 0 needs a complete hashed frame in the ring (:1054-1066): in a fixed joined state, the FullState's tick T.
  Small, per message: out.hashes (16 B items), out.lost (`see` :889-934: at most 17 pushes of 8 B -> <= 256 B),
  out.wraps, bad_headers (<= 32 u32), pings (lib.rs:795-797: pushed while fewer than 8, 24 B each; a cloned list of 7
  grows to 14 -> <= 336 B), ClockSync samples under the controller (clock.rs:159-168; 64 held, 40 B each by arithmetic
  -> at most 126 x 40 = 5,040 B).
- full_state(): decode vectors; `frame.entities` collect :1125 and `frame.own` :1126 (exact: n x 152); `sort_by_key`
  :1128-1129 (std's stable sort scratch, below); exact EntityRecord collects :1130-1131 (n x 104); client_hash (8 B per
  record, only when unsorted: duplicate keys with unordered generations).
- After a refused datagram the client still allocates a little (lib.rs:806-829): `sentinel` -> bad_header or see ->
  out.lost and out.hashes pushes, <= 256 B; the decoder itself allocated nothing. So "one byte over allocates nothing
  before its refusal" holds for the decoder; a window around the whole `input` call sees <= 256 B after the refusal.
- Held (lib.rs:70-74) = 152 B (stand-in measured). Tick (lib.rs:162-173): computed 168 B (4+1+1+1 + 64 sigs + 4 + 48
  frame + 24 events + 16 filed, align 8); the ring is 32 Ticks (lib.rs:306, ACK_RING_TICKS 32 at 30 Hz, ticks.rs:65-70).
- Stress scene (sim/src/scene.rs:166-190; greybox.rs:92-137): 48 soldiers, 15 hulls, 124 parts (4 x 20 + 11 x 4,
  :149-151), 300 live projectiles (pool 320), 6 doors, 1 wall, 8 effect rows per soldier. A client's relevant set
  excludes its own soldier (room.rs:1160-1162) and keeps records within 64 m (RADIUS_CM, room.rs:29) whose area
  connects, parts with their hull, doors and the wall by the flood (room.rs:1137-1206). So its FullState (room.rs:
  1380-1398) holds L_e <= 47 + 15 + 124 + 320 + 6 + 1 = 513 entity records (493 at 300 live projectiles) and L_o = 10
  own records (soldier, OwnState, 8 effects). The exact L_e at the harness's join tick is a run's figure, not computed.
- std rules used, rustc 1.99.1 (no rust-src installed): growth max(2 x cap, cap + 1, 4) for elements of 2 to 1,024 B
  (measured above for 104 B and 152 B); clone, to_vec, to_owned, exact-size collect ask exactly len (measured);
  `sort_by_key` (stable, driftsort since Rust 1.81): no heap for len <= 20, else a scratch of alloc_len = max(len -
  len/2, min(len, 8,000,000 / size_of), 48) elements, on a 4,096 B stack buffer when it fits, else
  Vec::with_capacity(alloc_len): for Held (152 B) and len <= 52,631 that is len x 152 (COMPUTED, from my knowledge of
  std's source, not read at 1.99.1 and not measured).
- Scanner (net/src/l2/ws_frame.rs): fixed state (State :82-92 with a [u8; 14] header buffer; Scanner :100-112, built by
  value :116-145); `scan` :170-246 and `header` :249-311 index and copy only. No heap on any input; the module says so
  (:23-24). No size-first refusal applies: a stream scanner takes any slice and never allocates.
- Counting rule the bars assume: a `realloc` counted as one request of its new size. Every push-grown and insert-grown
  vector reaches its largest size by realloc; an allocator that counts only `alloc` would see the first 4 slots (Snapshot
  416 B, FullState 416 B) and the bars below would be untestable.

## Blocks (one per target)
"Own path" pins the type byte to the named type; "cross" lets it vary within the named type's own size.

### 1. Input: decode(ClientToRoom, Datagram), input <= 166 B
- Site: input.rs:212 `Vec::with_capacity(count)`, count <= 8 (:209-211), reserved before any command is read, so a
  5 B input reaches it. Element Command 32 B. Request 8 x 32 = 256 B (with_capacity: exact). Pushes :294 never regrow.
- Bar 256 B (own path). Cross: Ping, Pong (0 B): bar 256 B either way. Size-first: msg.rs:403 (166), :411.

### 2. Resync: decode(ClientToRoom, Control), input <= 8 B
- Own path: no body (msg.rs:434): 0 B. Cross: Join, ClientSettings, Leave (0 B), SessionControl: a note needs at least
  2 + 1 + 21 + 1 = 25 B, so 0 within 8 B. Bar 0 B either way. Size-first msg.rs:403, :411.

### 3. Ping: input <= 32 B
- Own path: scalars (msg.rs:435-438): 0 B. Size-first msg.rs:403, :411.
- Cross, by the way the harness passes: BothWays (msg.rs:37: Ping and Pong alone): 0 B. ClientToRoom (the room's
  receiver): Input's with_capacity(8) from 5 B: 256 B. RoomToClient (the client's receiver): Snapshot within 32 B:
  17 B header and counts leave 120 bits: 5 removes (measured 31 B; 6 need 33 B) -> 8 slots x 104 = 832 B
  (snapshot.rs:599-608 or :589-598); events 2 kills -> 4 x 12 = 48 B.
- Bar: 0 B pinned or BothWays; 256 B on ClientToRoom; 832 B on RoomToClient.

### 4. Pong: input <= 32 B
- As Ping (msg.rs:439-445 scalars): 0 B pinned or BothWays; 256 B ClientToRoom; 832 B RoomToClient.

### 5. Join: decode(ClientToRoom, Control), input <= 128 B
- Own path: three arrays and a u16 (msg.rs:446-451; `array` is a stack copy, bits.rs:109-118): 0 B.
- Cross: SessionControl's note within 128 B: 128 - 24 = 104 B (measured: 128 B input, note.capacity() 104; control.rs:
  285). Bar 0 B pinned; 104 B with the type byte free (480 B if inputs up to the channel's 512 B were allowed).
- Size-first msg.rs:403 (512 for the channel), :411 (Join's 126 + 2).

### 6. JoinAccepted: decode(RoomToClient, Control), input <= 256 B
- Own path scalars and arrays (msg.rs:452-462): 0 B. Cross: JoinRejected, RoomEnded, Kick, SessionStatus, all 0 B
  (Snapshot, FullState, Ping, Pong refused WrongChannel at :425-427 before a body). Bar 0 B. Size-first :403, :411.

### 7. JoinRejected: decode(RoomToClient, Control), input <= 16 B
- 0 B own and cross (as 6). Size-first msg.rs:403, :411.

### 8. ClientSettings: decode(ClientToRoom, Control), input <= 8 B
- 0 B own; cross 0 within 8 B (as 2). Size-first msg.rs:403, :411.

### 9. RoomEnded: decode(RoomToClient, Control), input <= 64 B
- 0 B own and cross (as 6). Size-first msg.rs:403, :411.

### 10. Leave: decode(ClientToRoom, Control), input <= 16 B
- 0 B own; cross 0 within 16 B (a note needs 25 B). Size-first msg.rs:403, :411.

### 11. Kick: decode(RoomToClient, Control), input <= 16 B
- 0 B own and cross (as 6). Size-first msg.rs:403, :411.

### 12. SessionControl: decode(ClientToRoom, Control), input <= 512 B
- Site: control.rs:285 `note.to_owned()`: a String of exactly len B (measured 480 -> 480), len <= 480 (:273-275); at
  512 B, 488 B are available, so the cap 480 binds. Element u8. Bar 480 B (own and cross: the other client control
  types are 0 B). Size-first msg.rs:403 (512), :411 (510 + 2).

### 13. SessionStatus: decode(RoomToClient, Control), input <= 32 B
- Own path scalars (control.rs:307-320): 0 B; cross 0 (as 6). Size-first msg.rs:403, :411.

### 14. RoomControl: RoomControl::decode(bytes), input <= 1,024 B
- Site: control.rs:436 `Vec::with_capacity(n)` in get_peers, n 1 to 16 (:433-435), reserved before any address, so a
  4 B input [2, 0, 7, 16] reaches it (measured: Err(Truncated) after it). Element IpAddr 17 B. Request 16 x 17 =
  272 B (measured capacity 16 on a 278 B control). One peer list per control (codes 4, 7-11). Bar 272 B.
- Size-first control.rs:602-604 (above 1,024 B -> TooLarge) before the BitReader (:605).

### 15. Snapshot through npt-client's receive path (ClientEndpoint::input, RoomToClient, Datagram), input <= 1,150 B
- Decoder sites (snapshot.rs): entities :599-608 push growth: 431 removes at most (9,064 / 21) -> 512 slots x 104 =
  53,248 B (measured); own :589-598: 255 -> 256 x 104 = 26,624 B (measured capacity); events :609-612: 215 -> 256 x
  12 = 3,072 B (measured). **Decoder alone: 53,248 B.**
- Client sites (lib.rs), with the fixed state's held frame of L_e entity and L_o own records:
  - :1065 and :1074 two exact clones of the baseline frame (age > 0): L_e x 152 B each (and L_o x 152).
  - :135/:145 `insert` into the cloned entity list (capacity L_e): at most 238 new keys (Door adds, 38 bits), so the
    list's largest request is the first capacity of the chain L_e -> max(2 x cap, cap + 1, 4) -> ... that reaches
    L_e + 238. For L_e >= 238 that is one realloc to **2 x L_e x 152 B** (measured 513 -> 1,026). Own list from 10:
    113 Effect adds at most (80 bits) -> 160 x 152 = 24,320 B. Age 0: from empty, 238 adds -> 256 x 152 = 38,912 B
    (measured chain).
  - :1095 events from empty: 215 -> 256 x 12 = 3,072 B. :1107-1109 exact collects: (L_e + 238) x 104 and
    (L_o + 113) x 104. client_hash: <= (L_e + 238) x 8 when unsorted. Small pushes <= 336 B; ClockSync <= 5,040 B.
- **Bar through the client:** max(53,248; 152 x the chain's top; 104 x (L_e + 238)). For L_e >= 238 that is
  max(53,248, 304 x L_e) = 304 x L_e. Stress scene upper bound L_e 513: **155,952 B** at lib.rs:135/:145 (1,026 x 152);
  at 493 (300 live projectiles): 149,872 B. Smaller held frames: L_e 0 -> 53,248 (decoder); 10 -> 53,248 (chain top 320
  x 152 = 48,640); 50 -> 60,800 (chain top 400); 200 -> 121,600 (chain top 800).
- Cross (type byte free on RoomToClient Datagram): Ping -> pings.push (<= 336 B), Pong -> ClockSync under the
  controller (<= 5,040 B): under the bar.
- Size-first: msg.rs:403 (1,150), :411, reached through lib.rs:784. After a refusal the client's sentinel bookkeeping
  allocates <= 256 B (lib.rs:806-829), outside the decoder.

### 16. FullState through npt-client's receive path (RoomToClient, FullState channel), input <= 65,540 B
- Decoder sites: entities snapshot.rs:705-708: 14,561 doors at most -> 16,384 slots x 104 = **1,703,936 B (decoder
  alone; measured capacity 16,384 on a 65,540 B message)**; own :701-704: 256 x 104 = 26,624 B.
- Client sites (lib.rs): :1125 `frame.entities` exact collect 14,561 x 152 = **2,213,272 B** (measured exactness and
  152 B); :1128 `sort_by_key` scratch, alloc_len 14,561 -> 2,213,272 B (computed: driftsort); :1126/:1129 own <= 255 x
  152 = 38,760 B; :1130 exact collect 14,561 x 104 = 1,514,344 B; :1131 own <= 26,520 B; client_hash <= 14,561 x 8 =
  116,488 B (and its sort scratch the same) when unsorted. No copy of the held world: the message's frame replaces it.
- **Bar through the client: 2,213,272 B** (lib.rs:1125, measured rule; lib.rs:1128 ties it, computed). Cross: none
  (only FullState travels that channel; msg.rs:425-427).
- Size-first: msg.rs:403 (4 + 65,536), :411, through lib.rs:784.
- Note: the advisor's draft (advisor-u5-brief.md:45, "passes 65,536 B once it holds more than 512 records, about
  106 KB") gives the point where it passes the wire limit, not the reach: the reach is 16,384 slots.

### 17. DelayStream's scanner: Scanner::client(..) and Scanner::server(), scan
- No heap site: state fixed (ws_frame.rs:82-112), `scan` :170-246 and `header` :249-311 copy into the [u8; 14] buffer
  and index only. Bar 0 B both directions (any input length). Size-first: not applicable (never allocates; a stream
  scanner has no input limit of its own; its caps close the stream).

## The client targets' fixed scene (proposal)
- The stress scene (SS), slot 0's JoinAccepted and the room's FullState for it at the join tick (room.rs:1380-1398),
  built once at the harness's start; per input, a clone of that endpoint made OUTSIDE the counting window, then one
  `input` call inside it. Why SS: it is the largest held state any proof run gives a client (every pass mark that holds
  48 players and the vehicles runs it), so the baseline clone and regrowth paths are fuzzed at their real size; a
  smaller scene (CO: 1 soldier) would leave them nearly empty.
- Held state from that FullState (setup, not per message): frame L_e x 152 B (<= 77,976 B at 513) + 10 x 152 = 1,520 B;
  the ring 32 x Tick (computed 168 B: 5,376 B, lib.rs:306); ClockSync and queues small. Per message, a Snapshot at age
  > 0 copies the held frame twice (lib.rs:1065, :1074: <= 77,976 B each) and regrows one copy (<= 155,952 B); a
  FullState copies none of it.
- The Snapshot bar depends on L_e, a run's figure: the harness prints L_e and computes the bar by Block 15's rule in its
  header; FullState's bar does not depend on the scene.

## Summary table (bar = the largest single request, bytes)
| # | Target | Bar (type byte pinned) | Site | Type byte free, within the type's size |
|---|---|---|---|---|
| 1 | Input | 256 | input.rs:212 (8 x 32) | 256 |
| 2 | Resync | 0 | none | 0 |
| 3 | Ping | 0 | none | 0 BothWays; 256 C->R (Input); 832 R->C (Snapshot) |
| 4 | Pong | 0 | none | as Ping |
| 5 | Join | 0 | none | 104 (control.rs:285) |
| 6 | JoinAccepted | 0 | none | 0 |
| 7 | JoinRejected | 0 | none | 0 |
| 8 | ClientSettings | 0 | none | 0 |
| 9 | RoomEnded | 0 | none | 0 |
| 10 | Leave | 0 | none | 0 |
| 11 | Kick | 0 | none | 0 |
| 12 | SessionControl | 480 | control.rs:285 | 480 |
| 13 | SessionStatus | 0 | none | 0 |
| 14 | RoomControl | 272 | control.rs:436 (16 x 17) | n/a |
| 15 | Snapshot (client) | 155,952 at L_e 513; decoder alone 53,248 | client lib.rs:135/:145 (1,026 x 152); snapshot.rs:599-608 | same |
| 16 | FullState (client) | 2,213,272; decoder alone 1,703,936 | client lib.rs:1125 (and :1128); snapshot.rs:705-708 | same |
| 17 | Scanner (both) | 0 | none | n/a |
Every decoder refuses an over-limit input before any allocation (msg.rs:403/:411; control.rs:602-604); none lacks it.
The scanner never allocates. The client allocates <= 256 B after a decoder refusal (lib.rs:806-829).

## Open points for the brief (my read, not decisions)
- Pin the type byte per target (the harness writes framing and type, the fuzzer the rest) or state the bar per way and
  channel; Ping and Pong need their receiver's way named (BothWays, or the room's and the client's).
- The counting allocator must count a realloc as one request of its new size, or every push-grown bar is unmeasurable.
- The harness's input copy and the endpoint clone sit outside the window; the window ends at `input`'s return.
- The bars rest on std's growth rule at 1.99.1 (measured) and its sort scratch (computed): a toolchain change re-runs
  this probe. The advisor's draft figures: Input 256 holds; Snapshot "about 53,000" holds for the decoder alone, not
  through the client (155,952 at L_e 513); FullState's figure is 1,703,936 (decoder) or 2,213,272 (client).
