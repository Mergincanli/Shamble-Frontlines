## Lane 2: what the picture reads from the simulation (first list, Slice 1)

This list covers every field and event that lane 2's units read from the block. The units are 6 Look, M (the first model), 10 Body and 11 Feel.

For each entry it gives:
- who reads it;
- when they read it;
- the source that asks for it;
- whether a card or draft already plans it.

Lane 1 chooses the field names. Names in backticks are ones a source already uses.

**How to read it.**
- **Read by:** 6, M, 10 and 11 are the units.
- **When:**
  - *tick*: a block field, interpolated between the two newest blocks.
  - *rare*: a block field that seldom changes.
  - *event*: from the block's event section.
- **Status:**
  - **Block**: in the block today.
  - **Card N (named)**: change-set N's card names this thing in its own words.
  - **Card N (implied)**: the card names only what holds it (for example "soldier fields"), not this field itself.
  - **Draft**: a decision draft names it, but no card does.
  - **New**: no card or draft names it.
  - "Implied by" in a source cell means the source asks for it without naming it.
- **Short names for the sources:**
  - Plan: `docs/reports/2026-11-17-build-plan-slice-1.md`
  - P15: `…-part-15.md`
  - App: `…-appendix.md`
  - G: `legacy/shamble-game.js`
  - LI: `docs/LEGACY_INVENTORY.md`
  - AER: `docs/research/asset-export-rules.md`
  - 0001 to 0004: `docs/adr/000N-*.md`
  - CS1: `docs/reports/2026-11-18-cs1-types.md`
  - Digest: the Shamble VFX digest, `.checkpoints/part-15/shamble-vfx-digest.md` at commit d2c4caa (cited by P15:14-15)
- **Not asked for here, so nobody counts it twice:**
  - The picture already gives back the muzzle point and the cursor ray through the interface file (P15:117-121).
  - Lane 2 owns, and does not ask for, its camera state, its quality tiers and the data in `content/look/` and `content/feel/`.
  - Health: the health bars stay with lane 1 (P15:113-114).
  - Recoil kick, the reload dip and the hit flash: no field needed. Lane 2 computes them from the events below plus the weapon's content data (G:1471, :2714, :2717, :2934; see gap 10).

### Fields read every tick (15)

| # | What | Per | Read by | When | Source | Status |
|---|---|---|---|---|---|---|
| H1 | **Tick of each block**: lane 2 places each block in time and sets the blend between the two newest | block | 6, 10, 11 | tick | P15:117-118 (the picture gets "the two newest blocks, the events, the time"); layout.ts `tick` | Block |
| H2 | **Number of rows in each section** | block | 6, 10, 11 | tick | Plan:285 ("the header first: tick, layout hash, counts"); 0002:288; CS1:187-189, :263 (moved to change-set 4) | Card 1 (named), moved to change-set 4 |
| F1 | **Position**: a world point in metres, with the height | soldier | 6, M, 10, 11 | tick | Plan:343-344 ("soldier fields in the snapshot, the block's soldier section, interpolation"); 0001:309; G:277 | Card 4 (implied) |
| F2 | **Facing (aim yaw)**: an angle. Legacy uses radians; ADR 0001:280 gives only its width, 16 bits. | soldier | 6, M, 10 | tick | Plan:345 (the stand-in "shows facing and aim"); P15:130-131 ("where it faces", "the aim"); 0001:309; G:204 `aimYaw`, G:282 | Card 4 (named) |
| F3 | **Aim pitch**: an angle. Legacy uses radians; ADR 0001:280 gives 10 bits. | soldier | 10 | tick | Plan:345 ("aim"); P15:131 ("the aim"); 0001:309-310; G:1428-1432 (the gun pitched toward the aim point) | Card 4 (implied) |
| F4 | **How fast and which way it moves against its facing**: the horizontal velocity, in metres per second. Legacy derives it from the change in position. If lane 2 derives it too, a snapped correction of your own soldier (Plan:388) reads as a jump in speed; legacy's teleport threshold (G:278) handles that. | soldier | 10 (gait weights, stride, cadence, hips turned to strafing, lean) | tick | P15:130-131; Plan:422 ("the reaction to strafing"); docs/LOOK_V2.md:20; G:277-279, :282; LI:221 ("it needs only position and aim yaw") | New as a field. **The least lane 2 needs** is F1 plus F10, so it can derive this as legacy does. Lane 1 chooses. |
| F5 | **On the ground or in the air** (yes or no) | soldier | 10 (feet tucked in the air and planted again on landing; no steps in the air) | tick | P15:131 ("whether it is on the ground"); Plan:339-340 (jump); G:283, :2782 `ps.grounded`, :2936 `en.grounded`; LI:222 | New |
| F6 | **Sprinting** (yes or no) | soldier | 10 (each gun's sprint pose) | tick | Implied by App:412, :420 (the sprint poses) and Plan:339 (sprint); 0001:310 (sprint flag). Legacy derives it from speed (G:285). | Draft (0001:310), or derived from F4 and the movement table (gap 10) |
| F7 | **Alive or dead** | soldier | 6, 10 (the death fall, then held), 11 | tick | P15:124-126 ("read from a field of the block"); Plan:403 ("death and respawn"); G:2938 | Card 8 (named) |
| F8 | **Weapon held**: a content id | soldier | 6 (which code-built gun shows), 10 (hold, sockets, recoil value, reload style), 11 (which feel-kit row) | tick | 0001:310 ("the weapon id"); implied by P15:106 (both code-built guns) and Plan:408 (the SMG and the rifle); G:289 `GUN_KIT[gunId]` | Draft (0001:310) |
| F9 | **Soldier id** that stays the same from block to block | soldier | 10 (keeps each soldier's spring state), 11 | tick | Plan:339 ("the soldier table with its ids"); 0001:315-316 | Card 4 (implied) |
| F10 | **Respawn marker**: a new generation, or a flag on the respawn block | soldier | 10 (place the soldier rather than slide it, plant the feet again, clear the death pose) | tick | 0001:316-318 (a generation change "tells the client to place that soldier, not slide it"); G:275, :278 (legacy guesses from a jump faster than 25 m/s) | Draft (0001:316-318) |
| F11 | **Team**, plus each team's faction; or the faction of each soldier | soldier, plus a small table for the match | 6, M (faction colour on the `faction_` slots), 11 (the effect runtime's `faction` and `relation`) | rare | Implied by Plan:1209 ("faction colour per instance"), Plan:797-798, AER:192-196, LOOK_V2:18-19 and Digest:18 | New |
| F12 | **Which soldier is the local player**, and its team | match | 6 (the camera follows it), 10 (it gets the full rig first), 11 (split feedback, own camera kick, blast distance from own soldier, `relation`) | rare | Implied by P15:106 (`camera/` is lane 2's), Plan:346, docs/LOOP_V2.md:119 (split feedback) and CS1:267 ("the page sees only the block"). `JoinAccepted.slot`/`team` is a message, not the block. | New |
| F13 | **Own reload progress** (0 to 1) and the **active-reload state** | the local player (owner-only) | 10 (the reload moves over this progress), 11 | tick | Implied by Plan:33 (simple active reload: "hitting it will make the reload end faster") and Plan:136-138 (change-set 10 places the reload's hand targets); LI:226; G:2782 `rel`, G:2717 | Card 8 (implied: Plan:404, "the local player's own weapon state, predicted") |

### Events (6)

| # | Event | What it carries | Read by | Source | Status |
|---|---|---|---|---|---|
| E1 | **Shot**, one per shot fired. Your own shots arrive in the block of the tick that fired them; other soldiers' shots arrive on their own timeline. | shooter id, weapon id, tick, direction or aim point (for the tracer) | 10 (recoil: legacy adds 0.4 + recoil × 0.05, capped at 1.1, decaying 5 per second: G:1471, :2714); 11 (muzzle flash and tracer from the muzzle socket, casing from `socket_eject`, shot sound, own camera kick: G:1472-1473, :1481) | Plan:403 ("the events (shots, hits, deaths)"); P15:109 ("Recoil and the death fall answer to change-set 8's shot and death events"); P15:131-132 ("each shot … with its place and direction"); Plan:427-428; App:429, :276; 0001:303 (sent once, never to the shooter's own client); 0002:301 | Card 8 (named). Its direction field is New. |
| E2 | **Where a shot ended**, one per ray (so one per pellet) | place (a world point), the surface normal, and what it struck: a soldier, or a world surface class (flesh, metal, dirt…) | 11 (impact puff by surface, impact sound, the tracer's end) | Implied by Plan:427 ("impact"), P15:214 (check 7) and Digest:60 ("each hit per surface"); G:1489-1496 (tracer to the hit point; impact by surface), G:1022-1023; G:1625-1629 (bot shots end at the hit point, or at full range) | New |
| E3 | **Hit on a soldier**, confirmed | attacker id, victim id, weapon id, damage amount, body part, hit point, the point the hit came from, kind (bullet or blast) | 10 (flinch away from where the hit came from, strength = amount ÷ 40, held between 0.2 and 1: G:448-451, :1531-1532); 11 (hit marker and its sound for the local attacker: G:1535, :2495; hit flash: 0003:313, G:2934; when the local player is hit, the camera kicks away from the attacker and shakes: G:1540-1545) | Plan:403, :405 (hit marker on confirmation), :407 (locational damage); P15:131-132 ("hit … with its place and direction"); Plan:427-428 (flinch); LOOP_V2:119; 0001:308-309 | Card 8 (named). The amount, body part, hit point and where it came from are New. |
| E4 | **Death** | victim id, killer id, weapon id, and the point to fall away from: the killer's position for a shot, the blast's position for a blast kill | 10 (the death fall away from that point: G:457-460); 11 (kill confirm for the local killer: G:1536) | Plan:403; Plan:423 ("the death fall"); P15:109; P15:131-132 ("death … with its place and direction"); 0004:545 (killer and victim slot, unit id, weapon id); G:1532, :1775 (a blast kill passes the blast's point) | Card 8 (named). The point to fall from is New (gap 3). |
| E5 | **Blast** | place (a world point), strength; kind (a content id) and owner id, both implied by P15:214 (one effect per event name) and Digest:18 (`faction`, `relation`) | 11 (flash, fragments, dust, scorch and boom at the place: G:1079-1084; camera kick and concussion away from the blast, stronger the closer the local soldier is: G:1085-1090, LOOK_V2:23-24); 10 (through E4) | Plan:417 ("blasts"); Plan:429 ("fragments and the concussion shake"); P15:131-132 ("blast with its place"); 0004:546 ("position and strength, for the concussion shake and the fragments") | Card 9 (named) |
| E6 | **Reload start and reload end**; the end carries the active-reload result (normal or quick) | soldier id, weapon id, tick, and the result at the end | 10 (other soldiers' reload progress, from the start tick and the weapon's reload time, cut short by the end: gap 10); 11 (reload sounds: G:1412 at the start, G:2688 at the end) | 0001:311 (reload is an action start, a cosmetic event); implied by Plan:33 (active reload in Slice 1) and LI:226 ("Remote players need reload progress in the snapshot") | Draft for the start (0001:311). The end and its result are New. |

### Wishes (5): each one helps only if lane 1 builds the mechanic in Slice 1

| # | Wish | Why it would help | Source |
|---|---|---|---|
| W1 | **Aiming down sights** (yes or no), for each soldier | The idle and aiming poses differ (App:408-411, :418). Without this flag, lane 2 reads "aiming" as "fired recently" (E1). | 0001:310 (ADS flag); G:290, :2782, :3284; Plan:857 (the camera panel's sights mode). No card puts ADS in Slice 1. |
| W2 | **Weapon swap**, if one soldier can switch between the SMG and the rifle | The draw lift (0.26 s) and its sound. A change in F8 shows the swap, so no new data is needed. | G:1153, :2716, :1382 |
| W3 | **Grenade throw**: charge start and release, for each soldier | The wind-up and follow-through pose for every soldier, not just the local one. Change-set 9 makes the frag grenade playable (Plan:418), but card 10 names no throw pose. | 0001:311 (throw is an action start); G:453, :2782 `throwC`; LI:227 |
| W4 | **How far the sprint-to-fire swing has got** | The swing from the sprint pose to the aiming pose is where the delay shows. No slice places the delay yet. | docs/LOOP_V2.md:125; App:205; Plan:1405 |
| W5 | **Footsteps and landings** | Step sounds and landing dust. Lane 2 times them from F4 and F5 or from its own foot plants, so no new data is needed. Card 11 names only gun and grenade sounds. | G:2782 (a step every max(0.2, 0.46 − 0.022 × speed) s above 1.6 m/s) |

**Left out on purpose:**
- The bayonet lean, shield, katana and molotov holds: no Slice 1 card has them (G:2782 `lean`; LI:228).
- Hit-stop: "in no card of the plan and I add none now" (P15:27).
- The sound occlusion factor: not researched (0003:445).
- A unit or class id: Slice 1 has one soldier type.

### Gaps or conflicts (10)

1. **ADR 0001's soldier record lacks three fields.** Its remote-soldier fields (0001:309-310) have no velocity (F4), no on-the-ground flag (F5) and no team (F11). Part 15 asks for the first two (P15:130-131), and the plan asks for faction colour per soldier (Plan:1209). The ground flag changes on every jump, so it does not fit the draft's 8 flag bits that "change rarely" (0001:313).
2. **A projectile's blast has no event on the wire.** It is sent as the projectile's last trajectory change (0001:304-306), yet ADR 0002 lists blasts in the block's event section (0002:293). Lane 2 asks for one E5 in the block either way.
3. **The death event has no point to fall away from.** Part 15 asks for each death "with its place and direction" (P15:131-132). ADR 0004's death event carries only ids (0004:545). Legacy falls away from the killer, or from the blast for a blast kill (G:1532, :1775).
4. **Question: who draws projectiles in flight after change-set 9** (the grenade, the arrow)? The hand-over names "the plain tracers and flash" (P15:110, :113), not projectile bodies. If lane 2 draws them, it reads card 9's projectile section (Plan:417).
5. **Question: one yaw or two?** Legacy uses one angle for aim and facing (G:2782). Part 15 names "where it faces" and "the aim" separately (P15:130-131). If v2 keeps them apart, F2 becomes two fields.
6. **Question: what makes a soldier "aiming" in Slice 1?** The art's idle and aiming poses (App:408-411) need one of two answers: ADS (W1), or recent fire (E1).
7. **Online, the true end of your own shot comes from the server.** Spread is authority-only (Plan:403), so the true end of your own shot (E2) arrives with the server's event. Local play is not affected. "The hit event corrects" the tracer (0004:329-330), but the effect runtime cannot change a playing effect's inputs (Digest:90). Lane 1 says what the block carries for your own shots; lane 2 builds to that.
8. **The local player's slot and team are only in a message.** They arrive in `JoinAccepted`, and the page does not read messages (CS1:267). F12 puts them in the block. The interface file could carry them instead; lane 1 chooses.
9. **The surface class (E2) needs a surface tag on each collider in the test scene.** That tag lives in lane 1's `content/` (P15:93-94).
10. **This list counts on the page reading lane 1's content tables.** Several entries need, from those tables, each weapon's recoil and reload time and the walk and sprint speeds (LI:221): the recoil kick (E1), other soldiers' reload progress (E6) and sprinting derived from speed (F6). If the page cannot read those tables, these values become block needs too.

**Count** (from this file):
- **15 fields:**
  - in the block today: 1 (H1)
  - named in a card's own words: 3 (H2, F2, F7)
  - implied by a card: 4 (F1, F3, F9, F13)
  - named only by a draft: 3 (F6, F8, F10)
  - new: 4 (F4, F5, F11, F12)
- **6 events:**
  - named in a card's own words: 4 (E1, E3, E4, E5); E1, E3 and E4 also need new fields
  - named by a draft: 1 (E6), whose end and result are new
  - new: 1 (E2)
- **5 wishes and 10 gaps or questions.**
