# netcode-architect: U4r build copy 3, points 1, 3, 4 (causes and options for the advisor)

Read-only. Code at /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + wip/u4r.patch). No cargo, no runs.
No Edit tool: this file is rewritten whole with Write.

## Plan
1. Read the questions file, builder notes :571-906, and points 2 and 7.
2. B1 (point 1): eviction rule, full-state path, budget and priority; logs; plan, carries, rulings.
3. tick16 (point 3): why sets collapse past the wrap; options for carry 73.
4. no-fallback bite (point 4): why 25 of 96 heals land within 500 ms.
5. Return the structured result.

## Items
- [done] 1. questions file; builder notes :571-906.
- [done] 2a. Code (spikes/netcode-proof/crates/):
  - room/src/ack.rs:181-193 baseline (age 1..31, else defaults); :202-205 evicted; :109-114 stale capture in record();
    :207-212 stale(). sim/src/ticks.rs:58-70 ring 32 at 30 Hz.
  - room/src/room.rs:1154-1381 snapshot: base :1163-1170; frame = view(base, this tick's groups) :1351, :1369. Order:
    own :1179-1206, events oldest first resent until acked :1208-1235, entities :1236-1292, cosmetic :1302-1312.
  - encoder.rs:14, :26-31 window 1,066-1,067 B a tick; Packer :108-157.
  - room.rs:1083-1090 FS only when full_state_due (join :656, Resync :734, overflow :1030); full_state :1097-1131 on the
    control outbox, not charged to the window; no snapshot while in flight :1087-1089.
  - lab mirror.rs:314-337 stats; logs.rs:538-541; judge figures.rs:22-55.
  - ADR 0001:1589 PM-0001-2 ("relevant set ... 99 % ... proposed share"); part 6 :276-279 (defaults; ring proposed 32,
    about 1.07 s); :815 row E cost "a spike when a baseline expires".
  - history.md:338 Mustafa 2026-11-11 ~11:50 UTC: "The 9.5 floor (PM-0001-2) does not move." (ES events floor).
  - plan §4.5 :296-300; §4.7 :335-336; §4.12 :416-424 (Q22: -18's reason = shot band); §4.13 :438-440; §4.14 :461.
  - carries 57 (cap 1,024 "above the about 400 a B1 burst adds"), 58 (queue high-water per burst; overflow resync in
    judged -17/-18 runs to the advisor), 77 (b) (the bitten room's age-32 defaults tick: counted apart), 78 (events
    before the nearest 8; "no U4e or U4r run meets it"; U4g fixes).
- [done] 2b. Logs .gate/u4r/r2/b1, b1-nofallback:
  - client 0 wire ~1,028 B every tick before/during/after burst 0. eiv 48 through 64186-64214; 46 at 64215; 14 at 64216;
    0 from 64217 to ~64338; 46-47 by ~64570-64590.
  - lab carried-record counts: 64218-64248 ~47 soldier records a tick over 48 clients (own soldiers only): no entity
    change for ~30 ticks (event backlog: carry 78's case, met at U4r). From ~64250 normal rate, eiv still 0 to ~64340.
  - per-slot zero-flag ticks per burst: burst 0 min 143 / med 224.5 / max 370; burst 1 min 188 / med 244.5 / max 324
    (truncated by the run's end). 1 % of 18,000 = 180 per run = 18 per burst at 10 bursts: one burst already > 180 for
    the worst client.
  - events_queued 20,176 for slot 0 over 1,233 ticks = ~16.4 a tick (~490/s; F121 said ~350/s). queue_max 546-631.
  - b1 heals 96/96 (48-130 ms), full_state 0; evictions all age 0.
  - b1-nofallback: burst 0: 14 fast all eviction age 0, 34 slow 33/34; burst 1: no age 0, 11 fast (8 at 33, 3 at 34).
    slow 71 = overflow resyncs 71 = full_state 71; heals 1.10-1.69 s.
- [done] 2c. Arithmetic: Add bits (seam.rs:249-282, grid.rs, encoder.rs:40-58): soldier 129, projectile 179, part 113,
  hull 97 -> SS set ~8.8 KB of Adds; 47 soldier Adds ~758 B (fits one window with the own section).
- [done] 3. tick16c logs + code:
  - proto/src/snapshot.rs:532 the only header_tick use: snapshot header only; FullState keeps its 32-bit tick.
  - client/src/lib.rs:489-516 too_old/old_header: post-wrap header ticks (0, 1, ...) are before the client's first ->
    bad_header sentinel, newest unmoved, no ack. :775-779 FS passes (32-bit tick).
  - bots/src/lib.rs:183-189: a command only when the endpoint's newest moves -> after the wrap no command (commands
    1,532 ~ 48 x 32). room/src/input.rs:127-140 option B repeats the last command (walk, aim, fire).
  - tick16c room: eiv 48 to 65566; 0 at 65581-65596 (eviction ~wrap+32); overflow FS at 65605 -> 48 at 65611-65626;
    0 again; 30 at 65686-65701; 0 from 65716 on; measured eiv sum 0 although 31 + 13 overflow FSs in the measured
    phase (FS = the room's relevant set whole: room.rs:1104-1107) -> the interest sets themselves lost soldiers.
    Reliable events ~150-270/s vs p0-s1 ~475-600/s; shots hold 672-720.
  - client log: 48,772 sentinels filed under ticks widened past 2^32 (orphans): judge figures.rs:412-420 counts every
    orphan over every phase -> -17 would FAIL if the run were valid. eval.rs:1408-1410 a bite run takes its matched
    parts' validity (B7) -> pre:PM-0001-2 -> INVALID.
- [done] 4. no-fallback mechanism: base chosen (room.rs:1163-1170) before record() captures stale (ack.rs:112-114,
  room.rs:1369); stale() only for the current last_acked. Burst 0: eviction tick A+32 = k 481 (due 16.033 s, after the
  return at 16.029 s) for A = 449 -> defaults -> decoded -> heal (14, carry 77 (b)). Burst 1: all A+32 in-burst; a late
  ack of D = k 900 (client's newest pre-burst decode) reaches the room before tick 932 while held (newest sent 931) ->
  last_acked = 900 -> tick 932 no baseline, stale holds A -> defaults -> heal ~103-136 ms (11; outside 77 (b)'s words).
- [done] 5. Options drafted; returning the structured result.
