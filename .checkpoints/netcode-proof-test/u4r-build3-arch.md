# netcode-architect: U4r build copy 3, points 1, 3, 4 (causes and options for the advisor)

Read-only. Code at /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + wip/u4r.patch). No cargo, no runs.
No Edit tool: this file is rewritten whole with Write.

## Plan
1. Read the questions file (done), builder notes :571-906, and points 2 and 7.
2. B1 (point 1): npt-room eviction rule (burst_eviction), full-state path after eviction, datagram budget and priority
   order; npt-sim / npt-proto where they lead. Logs c3-*.log for the B1 suite. U4e plan / carries / rulings on burst
   recovery and -18's heal. history.md for PM-0001-2's floor.
3. tick16 (point 3): why sets collapse past the wrap under the bite; options for carry 73.
4. no-fallback bite (point 4): why 25 of 96 heals land within 500 ms.
5. Return the structured result.

## Items (a line per item as done)
- [done] 1. questions file read; builder notes :571-906 read.
- [done] 2a. Code read (paths under spikes/netcode-proof/crates/):
  - room/src/ack.rs:181-193 `baseline(now)`: newest complete entry at age 1..31; none -> defaults. :202-205 `evicted`.
    The ring holds 32 entries (sim/src/ticks.rs:58-70 ACK_RING_TICKS = 32 at 30 Hz).
  - room/src/room.rs:1154-1381 `snapshot`: base = baseline frame or the empty frame (:1163-1170); the frame recorded
    in the ring is `view(base, groups)` (:1351, :1369) = baseline + THIS tick's deltas only. Packing order: own
    section (:1179-1206), then every queued gameplay event oldest first, resent until acked (:1208-1235), then entity
    changes (nearest 8 enemies, skipped >= EVERY-1, skipped, distance, id: :1236-1292), then cosmetic (:1302-1312).
  - room/src/encoder.rs:14 window 32,000 B/s -> 1,066-1,067 B a tick (:26-31); Packer :108-157.
  - room/src/room.rs:1083-1090: a full state only when `full_state_due` (join :656, Resync :734, overflow :1030).
    No full state on eviction. room.rs:1097-1131 `full_state` goes on the control outbox, not charged to the window;
    while in flight no snapshot (:1087-1089).
  - lab/src/mirror.rs:314-337 `stats` (everyone = the mirror frame holds every other soldier); logs.rs:538-541;
    judge figures.rs:22-55 (min over slots of the share of measured ticks with flag 1).
  - ADR 0001:1589 PM-0001-2: "every client's relevant set holds all 47 other soldiers on at least 99 % of its
    snapshots ("everyone in view"; proposed share)". ADR part 6 :276-279: "with no usable baseline the server codes
    against the per-entity defaults ... a ring of the snapshots it sent (data, proposed 32, about 1.07 s at 30 Hz)";
    options table :815 row E cost "a spike when a baseline expires; per-client rings".
  - marks.json PM-0001-18 heal: "from the link's return to the decode of the first snapshot the room writes at or after
    the return instant (a fresh delta ...)"; lab scenario.rs:989-1012 (h != 0, due_ns(tick) >= end, not a FullState's).
  - history.md:338 Mustafa 2026-11-11 ~11:50 UTC: "The 9.5 floor (PM-0001-2) does not move." (ES reliable events,
    point 2's). The 0.99 share: ADR says "proposed share"; no Mustafa decision on it found.
  - u4-build-plan.md §4.5 :296-300 (ring 0..31; defaults beyond; burst_eviction for -18's bite); §4.7 :335-336 (queue
    cap sized so a burst does not resync); §4.12 table :416-424 (Q22: -17, -18 get pre:PM-0001-2; -18's reason only
    the shot band; -22's exception for a designed drop); §4.13 :438-440 (B1 arithmetic: shots only).
- [done] 2b. Logs (.gate/u4r/r2/b1, b1-nofallback):
  - b1 room.jsonl: snap_wire_bytes ~1,028 B for client 0 every tick before, during and after burst 0 (window full in
    steady state). eiv count 48 through the burst (64186-64214; the mirror codes against the held baseline), 46 at
    64215, 14 at 64216, 0 from 64217 to ~64338, rebuilding to 46-47 by ~64570-64590.
  - b1 lab.jsonl carried-record counts (errors of decoded carried fields, all clients): 64218-64248 about 47 soldier
    records a tick across 48 clients (each its own soldier only): the entity section carried nothing for ~1 s (the
    event backlog). From ~64250 ~900-1,000 position records a tick (normal rate) yet eiv stays 0 to ~64340.
  - b1 burst_heal: 96 of 96 <= 500 ms (48-130 ms), queue_max 546-631, full_state 0. burst_eviction: 96, all age 0.
  - b1-nofallback: 25 of 96 <= 500 ms. Burst 0: the 14 fast all have eviction baseline_age 0; the 34 slow 33/34.
    Burst 1: no age 0 at all; 11 fast with age 33 (8) / 34 (3).
  - Timing: burst 0 start 15.029 s (k 450.88), end k 480.88, first tick after k 481 = 64217; burst 1 start k 900.33,
    end k 930.33, first tick 931 = 64667. L1 loses by send time (l1/src/lib.rs:1097, :1271, :1426).
- Size arithmetic (seam.rs:249-282 layouts; grid.rs; encoder.rs:40-58): Add bits = 3 + 16 + 2 + 16 + layout. Soldier
  129 bits (16 B), projectile 179 (22 B), part 113 (14 B), hull 97 (12 B). Full SS set ~47 x 16 + 300 x 22 + >=80 x 14
  + 15 x 12 = about 8.8 KB of Adds: >= 8 whole windows before any event or update.
- Point 4 mechanism (derived): the bite's encoder chooses base before `record()` captures the stale frame (room.rs
  :1163-1170 vs :1369; ack.rs:109-114, :207-212), so the first tick with no baseline after each new last_acked is coded
  from the defaults (the `_ => (&empty, 0)` arm). Burst 0: A+32 = k 481 (after the return) for clients whose last ack
  was k 449 -> decodable -> heal (14). Burst 1: all A+32 fall in-burst; a late ack of D (k 900, the client's newest
  pre-burst decode) arrives while D is held (newest sent 931) -> last_acked = D -> tick 932 has no baseline and no
  stale for D -> defaults -> decodable -> heal at ~100-136 ms (11). Slow 71 = overflow resyncs 71 = full_state 71.
- [next] 3. carries / rulings on burst recovery and -18; then tick16 (point 3).
