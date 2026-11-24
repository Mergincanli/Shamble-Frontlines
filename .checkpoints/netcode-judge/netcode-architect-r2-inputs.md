# R2-INPUTS ruling (netcode-architect), npt-judge part b

## Ruling
(pending)

## Plan
1. Read r2-netcode.md:1735-1780 (E3, item sizes, worked number).
2. Read ADR 0006 ~1387 (PM-0006-17) and ADR 0001 ~1479, 1598-1603 (PM-0001-12).
3. Read SERIES.md rows: r2_down_formula_bits, r2_egress_formula_bytes, egress_vs_r2_ratio, tick.s.relevant_count,
   tick.v.big_vehicles_count, second.v.shots/reliable_events/projectile_spawns_count.
4. marks.json PM-0001-12 and PM-0006-17 by jq.
5. Writers: tick_loop.rs; room crate `relevant`; u6-brief/u6-facts.
6. Check golden arithmetic. Rule (a)/(b)/(c); frame point 5 check.

## Findings (as I go)
- F1 r2:1764-1765 E3 = 65+16+(P-1)x7.5+Vx9.4+7+2 B/snapshot (const 90: 57+8 hdr r2:1742-1743, 16 own r2:1756, 7 r2:1757,
  2 r2:1758) + events 15.8xP B/s (shots 4 B r2:1759, reliable 8 Bx3 sends r2:1760, spawn 11 B r2:1754). Golden checked:
  583.5x30+288+240+232.32 = 18265.32 B/s = 146122.56 b/s = 3653064/25; x48x3600/8 = 3,156,247,296 B. Matches r2:1775.
- F2 ADR 0006:1387 names "P, V, shots and projectile spawns" + "at the counts that ran"; ADR 0001:1600 (-12) names
  "entities in each client's set, shots, spawns and events per second" and says "as ADR 0006's PM-0006-17 evaluates it".
- F3 room.rs:1159-1228 find_relevant: set = other soldiers (own excluded :1182), hulls, projectiles (trajectories),
  VehicleParts of relevant hulls (:1208-1211), doors/walls in reached areas (:1213-1221). NOT soldier-only.
  EntityKind variants seam.rs:194-215. No interest-off switch in room/server (grep).
- F4 tick_loop.rs:1169-1179 writes soldiers_count, snapshot_clients_count, big_vehicles_count (hulls), projectiles_count;
  :1188-1195 per-second shots, reliable_events, projectile_spawns. No relevant_count writer.
