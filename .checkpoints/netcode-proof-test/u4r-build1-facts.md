# U4r build copy 1: fact check for the advisor (read-only, adversarial)

Sources: questions u4r-build1-questions.md; notes u4r-build.md; code .gate/u4r-snap1 (2fbec77 + wip/u4r.patch);
logs .gate/u4r/; brief u4r-brief.md; carries u4s-carries.md; plan.md.

## Plan
1. Read notes (282 lines), questions (done).
2. Point 1: logs regen-all.log, regen-precheck.log; check.mjs wasmStep; hash.rs legs/regenerate_all/--only;
   33.1/61.6 base source; WASM_CALL_LIMIT_MS location and scope (carry 84); arithmetic for (a)-(d).
3. Point 2: npt-room RADIUS_CM, AreaFlood visibility; interest set build; mirror.rs checks.
4. Point 3: run file sizes; lab.jsonl -57 per-tick; SERIES.md:66-73; brief 7.1 table; df; du shared-target.
5. Point 4: client refusal causes; L1 P0 fates; logs.rs refused_as_lost; carry 62 (h).
6. Point 5: HISTORY_MS, ROOM_HISTORY_MS, span_ms writer.
7. Point 6: World dump events section; shot counting.
8. Point 7: Behaviour::step view.
9. Point 8: d13.log vs notes table; U4R-Q14 arithmetic.
10. Point 9: --lead, --every-step, --logs 0 refusal/limits.

## Items (a line per item, marked done)

- P1 done (reads): regen-all.log real 3m20.992s (201.0 held); regen-precheck.log real 1m50.068s (110.1 held). mtimes:
  regen-all start 20:28:09.88, synthetic done 20:28:09.996, SS-P2-s1.rec 20:30:32.54 (~142.6 s), ES-P3-s2.rec
  20:31:25.14 (~52.6 s), native-*.lines 20:31:30.85 (~5.7 s legs). regen/: SS-P0-s4.rec 20:27:23.74 (~108.1 s), legs ~1.9 s.
- P1: wasmStep check.mjs:2254; calls :2319 "npt-lab hash" (no --only), :2374 "hash-map native legs" (--only rec0),
  :2432 "std-sin native legs" (--only scan's catch SS-s1; std-sin-scan.txt has catch SS-s1 2). Held: 3 legs calls.
- P1: hash.rs legs :823; selected :484 (replayed only); regenerate_all(&entries) :870ish over whole manifest; same in
  2fbec77 (:751 selected, :770 regenerate_all(&entries)). serve() reads files (:965-975), no regen. Held.
- P1: WASM_CALL_LIMIT_MS check.mjs:2212, applied per call :2263 (cargo 0); no step deadline today. Carry 84 (a)
  (u4s-carries.md:396-410) makes it one shared deadline for non-cargo calls from the first of them, at U4r's change.
- P1: 33.1 s = m5-counted.log:170 (builds 5.4+5.2+1.2+1.2=13.0; non-cargo sum 19.9). 61.6 = cold-run2.log:23 (builds
  41.8; non-cargo 19.6). Latest warm on spike code: l3-manifests/counted.log:172 35.8 s (builds 15.0; non-cargo 20.8).
- P1 done. P2 done: room.rs:29 `const RADIUS_CM` private; interest.rs:77 `pub struct AreaFlood` re-exported lib.rs:27,
  methods new/update/reaches/sees_portal pub(crate) (:86,:99,:140,:147). find_relevant room.rs:904-970: soldier/hull/
  projectile by point() (room.rs:345-351) within r2 AND flood.reaches; parts with relevant hull; doors sees_portal, wall
  reaches (no radius). Snapshot Add/Update only from c.relevant (room.rs:1240); Removes otherwise. FullState tick t
  from c.relevant (room.rs:1106-1115). mirror.rs audit_records :224-275: radius on Soldier/Vehicle (grid0,1), Projectile
  (grid1,2), Part via hull; Door/Wall `_ => None` unchecked; no flood; events: only own-shot (mirror.rs:421-430).
  => cannot flag a legit send of the 4 checked kinds; can pass in-radius unreached-area records, any door/wall, any
  non-own cosmetic/gameplay event. breaches list capped at 16 (mirror.rs breach). r4 log b11-r4r8r9.log:31-34 0 breaches.
- P6 done: World::dump pub (world.rs:1018); events cleared each step (world.rs:2100); Shot pushed per fired
  (world.rs:1550); dump events count + 4 words each (world.rs:1995-2017); pools fixed-size (world.rs:846,855) so
  cached events_at stays valid; logs.rs room_second :520-603 per room tick, only when s.logs (scenario.rs:884-888);
  Room::tick steps sim once (room.rs:814); SimCore::step returns events but room calls it (seam.rs:546-548); no pub
  events getter on SimCore/World/Room (Room.stamped private room.rs:417).
- P7 done: newest_tick = "newest snapshot tick read in a header" (client/src/lib.rs:335-338); mirror.tick fed room's
  datagrams before server.send (scenario.rs:830-852); view = mirror frame of client's newest tick, soldiers only
  (mirror.rs:203-215; scenario.rs:916-921). Frame = mirror's baseline frame + every group of the tick (mirror.rs:1-4).
- P3 done: sizes smoke1 lab 10,961,471 / room 881,985 / client 492,198 (held). lab tick lines bytes/tick (awk):
  smoke1 P0 24,376 (449 lines); relp2 P2 9,270; d13-1 P2 9,226; d13-2 9,020; d13-3 P2 lead3 15,000; d13-4 P3 3,984;
  d13-5 P0 24,691. Tick lines hold only s.pos_err_um/yaw/pitch (python read). room+client d13-1: 3,648,994+1,976,863
  over 1,828 ticks. SERIES.md:66,67,73 are -57's rows (68-72 others). marks.json:4008-4100 -57 parts P0,P2,P3,B1 min 3,
  max checks + p99 report. Brief 7.1 :1153-1170: -57 on -17's runs, 4 parts x 3 seeds, 10 min; bites one run per bite
  at seed 1. df now 5.6G (5,998,694,400 B avail). du shared-target 8.1G; .gate 22G; .gate/u4r 291M.
