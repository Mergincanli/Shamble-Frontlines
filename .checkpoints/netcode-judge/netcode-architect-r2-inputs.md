# R2-INPUTS ruling (netcode-architect), npt-judge part b, 2026-11-24. STATUS: COMPLETE

## Ruling
Frame point 5 does not trigger: ADR 0006:1387 and SERIES.md:284-285 both hold under the reading below, so I rule.

**1. Inputs and expression** (room log; exact fractions; ticks and seconds of the measured window):
- H = header `tick_hz` (it is the snapshot rate here: room.rs:882 sets `snapshot_hz` = tick_hz; E3's "30 Hz" is the
  snapshot rate, r2:1764).
- E = mean of `tick.s.relevant_count` over every (measured tick, client slot) entry (each list holds `clients` entries).
- V = mean of `tick.v.big_vehicles_count` over measured ticks (EntityKind::Vehicle hulls, tick_loop.rs:220, :1173).
- S, R, J = mean per measured second of `second.v.shots_count`, `second.v.reliable_events_count`,
  `second.v.projectile_spawns_count` (room-wide; tick_loop.rs:1188-1195).
- bits/s per client = 8 x (H x (90 + 15/2 x (E - V) + 47/5 x V) + 4 S + 24 R + 11 J)
  = 4/5 x H x (900 + 75 E + 19 V) + 32 S + 192 R + 88 J.
- Term by term against r2: 90 = 57 transport (r2:1742) + 8 header (:1743) + 16 own state (:1756) + 7 structures (:1757)
  + 2 economy (:1758), i.e. r2:1764's "65 + 16 + ... + 7 + 2": right. 15/2 B per other soldier (:1751): right, but on
  (E - V), not E (THE ONE CORRECTION to the sheet: the set holds the hulls). 47/5 B per vehicle (:1752): right.
  4 B per shot (:1759), 24 B per reliable event = 8 B x 3 sends (:1760), 11 B per spawn (:1754): right.
- Golden (python Fraction, checked): E 62 (= 47 + 15), V 15, S 72, R 10, J 528/25, H 30 -> 3653064/25 bits/s
  (146,122.56; r2:1775 prints 146.1); x 48 x 3,600 / 8 -> 3,156,247,296 B (ADR 0006's 3.156 GB): confirmed. The sheet's
  "E 47" becomes E 62. Plant: reading E as soldiers only (E 47, V 15) gives 471 B/snapshot, not 583.5.

**2. Option (c), with relevant_count's meaning fixed; no added row.** Sentence for SERIES.md:407: "one per client slot:
the soldiers other than its own plus the big-vehicle hulls in its relevant set this tick (vehicle parts, trajectories,
doors and walls not counted); 0 for a slot with no client or a kicked one; with interest off, every other soldier and
every hull".
- (a) is wrong under either meaning: soldiers + hulls prices each hull twice (7.5 + 9.4 B); every record (the set as
  room.rs:1159-1228 builds it: parts, trajectories, doors, walls too) prices ~300 SS trajectories (ADR 0001:1653) and
  >= 80 parts at 7.5 B, which E3 prices elsewhere (spawns as events r2:1754; parts inside the 9.4 B r2:1752; structures
  in the 7 B r2:1757): kilobytes per snapshot off.
- (b) gives (c)'s value when the set holds every hull, but makes -10's "relevant entities" soldiers only, dropping the
  vehicles R2's "players x visible entities" (r2:2297-2298) and ADR 0001:1476 ("all 47 other soldiers and all 15
  vehicles") count; and needs a writer change anyway.
- (c) takes ADR 0001:1600's "entities in each client's set" (relevant_count) and ADR 0006:1387's "the run's V"
  (big_vehicles_count) literally; all in view, E - V = P - 1 (E3's term, r2:1764). The writer already counts both per
  client at tick_loop.rs:277-287 (`others`, `vehicles`, for everyone_in_view).
- Residual: when a client's set lacks hulls, (c) overstates by 19/10 B x (V - hulls in set) per snapshot, at most
  28.5 B (6.84 kbps at 30 Hz, 15 hulls). It touches reports only (-12's SS, -17's interest-on); the arithmetic printed.

**3. One shared formula; both hold.** ADR 0006:1387 leads with "R2's E3 estimate at the counts that ran"; its
parenthesis names the counts it expected to differ, never "only". ADR 0001:1600 lists "events per second" and calls the
-12 evaluation "as ADR 0006's PM-0006-17 evaluates it": the one reading that keeps both ADR rows true has the run's
reliable events in both, as SERIES.md:285 says. It is also the faithful one: the spike's reliable events are hits and
kills (tick_loop.rs:127), queued to every client that sees shooter or victim and resent until acknowledged
(room.rs:1251-1266; events.rs:1-3), R2's 8 B x 3 sends model (r2:1760); R2's fixed 10 a second holds only at R2's
counts, ADR 0006's own reason for taking the run's shots. At the golden counts both readings give 1,920 bits/s. If the
advisor reads the list as exhaustive (reliable term 40 x P bits/s for -17 only), the figures stop sharing one formula:
that is frame point 5's conflict, his to call.

**4. The two runs.** -17's check part (interest off, 48 bots): relevant_count = 47 + V every tick, so E - V = 47 = P - 1
and V = the run's V: ADR 0006's P and V exactly. -12's "SS at P2": soldiers all in view by PM-0001-2 (>= 99 % of
snapshots, SERIES.md:288); vehicles are not guaranteed in SS (the vehicle clause is ES's, tick_loop.rs:184-185, :289),
so the residual applies (report). -17's "interest on (reported)" also gets r2_egress_formula_bytes (that report names
no parts, marks.json); E3 assumes every entity in view (r2:1765-1766) and S, R, J are room-wide, so it overstates there.

**5. For the writer (board note):** write `tick.s.relevant_count` = others + vehicles exactly as tick_loop.rs:280-287
counts them (own soldier excluded, room.rs:1182), one per client slot on every tick record, 0 included; under
`params.interest` "off" it must still count every other soldier and hull the client is sent. The egress run's measured
window should be the counter's span (the room-hour between the two iface reads), so the formula's means and the
interface bytes cover the same interval.

**Findings.** Medium: fixing relevant_count to soldiers + hulls also fixes -10's denominator (SERIES.md:293). I read it
as R2's "visible entities" (r2:2297-2298), but the advisor should confirm at the build; if he wants every record there,
a second count is needed and one figure row's cite changes: back to him. Low: E3 does not price stand-in projectiles'
per-change updates (tick_loop.rs:234-235) or a snapshot split over several datagrams (57 B each, r2:1742; E3 assumes
one); the checks compare against E3 as written. Low: SERIES.md:284 cites "ADR 0001 lines 1754-1779", r2's lines (the
sheet's fix stands).

## Plan (done)
1. r2:1735-1780. 2. ADR 0006:1387, ADR 0001:1474-1479, :1596-1603. 3. SERIES.md rows. 4. marks.json by jq.
5. Writers: tick_loop.rs, room.rs `relevant`, u6 notes. 6. Golden arithmetic; options; frame point 5.

## Findings (as I went)
- F1 r2:1764-1765 E3 = 65+16+(P-1)x7.5+Vx9.4+7+2 B/snapshot + events 15.8xP B/s (6 shots + 5 reliable + 4.8 spawns).
  Golden: 583.5x30+288+240+232.32 = 18265.32 B/s = 3653064/25 b/s; x48x3600/8 = 3,156,247,296 B (r2:1775).
- F2 ADR 0006:1387: "P, V, shots and projectile spawns" + "at the counts that ran"; ADR 0001:1600 (-12): "entities in
  each client's set, shots, spawns and events per second", "as ADR 0006's PM-0006-17 evaluates it".
- F3 room.rs:1159-1228 find_relevant: other soldiers (own excluded :1182), hulls, trajectories, parts of relevant hulls
  (:1208-1211), doors/walls in reached areas (:1213-1221). EntityKind: seam.rs:194-215. No interest-off switch (grep).
- F4 tick_loop.rs:1169-1179 writes soldiers, big_vehicles (hulls), projectiles counts; :1188-1195 per-second counts.
  No relevant_count writer.
- F5 tick_loop.rs:277-287 counts per client `others` and `vehicles` in c.relevant(). Soldiers always have a point
  (room.rs:446-451): dead soldiers in range are in the set.
- F6 reliable = Hit + Kill (tick_loop.rs:123-131), routed room.rs:1251-1266, resent until acked (events.rs:1-3).
- F7 snapshot_hz = tick_hz (room.rs:882).
- F8 relevant_count also feeds -10 (SERIES.md:293, :407; ADR 0001:1598); R2: "players x visible entities"
  (r2:2297-2298); ADR 0001:1476: set holds "47 other soldiers and all 15 vehicles".
- F9 marks.json: -12 reports the formula on "SS at P2" only; -17 checks on params.interest "off"; its formula report
  has no parts (both parts).
