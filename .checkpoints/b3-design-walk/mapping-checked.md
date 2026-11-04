# B3 mapping: Mustafa's 2026-11-04 design walk onto LOOP_V2 and UNITS.md

Prepared by the game-designer checker, 2026-11-04. Research for B3's plan: no repository file is edited here.
Line numbers are from the worktree `scratchpad/docs-pass` (same files as main for these docs).
Abbreviations: LOOP = docs/LOOP_V2.md, UNITS = docs/UNITS.md, VISION = docs/VISION_V2.md, DN = docs/DESIGN_NOTES.md,
README = docs/adr/README.md, 0001/0004/0005 = docs/adr/000N-*.md, LT = docs/LEGACY_TUNING.md.

## Plan
1. Read his message (16 items: 12 Decided, 4 Ideas; plus 1 Open). Item 10 (the ladders) is one item with three
   sub-items (AMK, shotgun, SMG).
2. Read the two targets in full (LOOP 167 lines, UNITS 151 lines), then VISION, DN, LOOK_V2 (Molotov/fire: only
   line 33 touches it), README "Decisions the drafts ask for" (lines 38-157) and "Design questions" (158-199),
   0005 OQ-AB1..AB15 (lines 1220-1419) and OQ-CD1..CD8 (lines 643-711), 0001 (reconnect part 11, upgrades part 12,
   OQ-0001-7 and -10; 0001 has no spawn-protection text), plus STATUS's record of UPGRADE-SCOPE and the 2026-11-03
   answers (STATUS lines 183-238), because B3's new text supersedes some of them.
3. Per item: his words, landing site, marker, conflicts, ADR links, design notes. Saved after each item.
4. List the Open item. Then the cross-cutting decisions for the plan.

## Findings that shape the whole plan (read first)
- **LOOP has no section for amps, loot or ammo.** The 2026-10-31 UPGRADE-SCOPE and the 2026-11-03 answers live only
  in STATUS (lines 183-238) and README (lines 47-50, 60-63, 117-127). Items 1, 3, 4, 5 and 8 need a home.
  Recommended: a new `## 13. Weapons in the match: amps, loot and ammo`, appended after §12, so §1-§12 keep their
  numbers (VISION §2 pillar 7 cites "LOOP_V2 §8"; 0005 cites §2, §4, §5, §7, §8, §11, §12; DN cites §4, §5, §8-9,
  §12). Inserting a section before §8 would break those citations.
- **UNITS.md has no status markers.** It is the legacy "Phase 5" reference roster (title line 1; status block lines
  5-7: "This file remains the **planned** expansion source"). Recommended: a new top section
  `## 0) v2 roster decisions (Mustafa, 2026-11-04)` with a header line "Status tags as in LOOP_V2.md", the way VISION
  (line 4) and LOOK_V2 (lines 4-5) borrow LOOP's tags. The reference tables (§1-§5) stay as they are.
- **"Ideas" has no exact LOOP tag.** LOOP's tags (line 6-7): `[DECIDED]` Mustafa decided · `[DEFAULT]` the Advisor
  proposed, stands unless vetoed · `[OPEN]` needs a decision before it is built · `[LATER]` not in the greybox.
  None means "Mustafa's sketch, to iterate". `[DEFAULT]` would be wrong (it is the Advisor's tag and "stands").
  Options: (a) a new tag `[IDEA]` ("Mustafa's sketch, to iterate; not decided"), defined once in LOOP line 6-7 and
  borrowed by UNITS §0; (b) `[OPEN]` plus the words "Idea (Mustafa, 2026-11-04), to iterate:". Recommend (a): it
  keeps his own word, and `[OPEN]` already means "blocks the build", which an idea does not.
- **"Parked" (suppression, dual-wield) maps to `[LATER]`.** "First sketch, to iterate" on a Decided item stays
  `[DECIDED]` with those words kept (LOOP line 8 already says every number is a starting value).
- **Supersession of UPGRADE-SCOPE.** Item 5 ("amps are on weapons only") replaces the "every unit and every weapon"
  track of UPGRADE-SCOPE (STATUS line 236). That text also sits in 0005 (Settled inputs, lines 66-67; AB-4 table line
  1027) and 0001 part 12 (line 541: "levels and picks per unit type and per weapon type"). B3 edits only LOOP and
  UNITS; recommended: LOOP §12 gains a bullet naming what item 5 supersedes, and the drafts pick it up at their next
  revision, as README line 8-10 already does for the 2026-11-03 answers.

---

## Item 1 (Decided): looting keeps the class ability
1. **His words:** "Looting keeps the class ability (a Conscript with a looted shotgun still throws Molotovs). Rare
   strong combos are welcome; watch for any combo that becomes a must-have."
2. **Lands:** LOOP, new §13 (see plan), new bullet. Nothing in LOOP touches loot today. Touches LOOP §5 lines 63-66
   (the T1 units and their abilities) only by reference.
3. **Marker:** `[DECIDED]`. The "watch for a must-have combo" half is a standing instruction to balance work, not a
   rule: keep it in the same bullet, no separate tag.
4. **Conflicts:** none in LOOP or UNITS. It confirms the assumption in Mustafa's 2026-11-03 carry-slot answer
   (README line 61; STATUS line 185): "Assumed, to confirm: the class's own abilities stay." That answer ("a looted
   weapon replaces the main (class) weapon; the sidearm stays as backup") is not in LOOP yet; §13 needs it as the
   rule this item qualifies. Decision B3-SCOPE below.
5. **ADR links:** OQ-0005-AB1 (carry slots; README line 61; 0005 lines 1220-1231): its open "to confirm" part is now
   answered. OQ-0005-AB9 (drop rules; README line 121): bears on it (what a dead looter drops), not answered.
6. **Design notes (not resolved):**
   - Must-have combo candidates, from the T1 kit (LOOP §5; LT `WEAPONS`/`UNIT_TYPES`, lines 169-230):
     - Conscript + looted shotgun: the Peacekeeper's close game (8 pellets × 11) plus the Molotov, which LOOP's
       triangle uses as the anti-shield tool (0005 line 1009; AB11 answer, README line 123: "the Conscript's Molotov
       punishes the shield hard"). Risk: it beats both its counter (the Warrior up close) and its prey.
     - Warrior + looted AMK: 300 m reach (LT line 181) plus the Banzai blade's one-hit kill (9999, LT line 173)
       and speedMult 1.5 (LT line 221). Risk: the Conscript's whole answer to the Warrior is range.
     - Peacekeeper + looted SMG: Riot Shield up front plus a 36-round SMG.
   - Counter check (pillar 3) per combo: the looted gun keeps its own range profile, so the triangle's answer is the
     same gun's counter, not the class's. That is the measurable claim for sims: unit pick and win rate per
     (class, held weapon) pair, two-team and three-team modes separately (VISION §3, §6.6).
   - Telemetry need: "a combo that becomes a must-have" needs a threshold (for example a pair's pick share and win
     rate above a set line). Not set by his words; the game-designer proposes it once telemetry exists.

## Item 2 (Decided): spawn protection
1. **His words:** "Spawn protection: 3 s; firing ends it early."
2. **Lands:** LOOP §2, line 23, **replacement** of part of the bullet. Existing text:
   "`[DEFAULT]` Spawn safety: 3 s of spawn protection, and enemies cannot enter a barracks interior."
   Recommended split: `[DECIDED]` "3 s of spawn protection; firing ends it early." and the barracks-interior half kept
   as `[DEFAULT]` (his words do not touch it).
3. **Marker:** `[DECIDED]` (was `[DEFAULT]`).
4. **Conflicts:** LOOP line 23 `[DEFAULT]` is superseded for the 3 s and gains the early end; his 2026-11-04 words win.
   README line 126 and STATUS lines 196-197 still call "Firing ends it early" "the Advisor's pencil, which Mustafa may
   veto": now his decision. 0005 line 991 cites "§2 `[DEFAULT]`".
5. **ADR links:** OQ-0005-AB14 (README line 126; 0005 lines 1390-1407): "what ends it early" answered (option (ii),
   in part: see note); "what it covers" ((a) damage only or (b) damage and every enemy effect) still open. 0001 has no
   spawn-protection text (checked: grep "spawn protection" in 0001 finds nothing); its respawn handling (lines 317
   and 340) is unaffected.
6. **Design notes:**
   - The math 0005 gives (line 1395-1397): a Warrior empties its 36-round SMG in 35 × 0.072 = 2.52 s, up to
     36 × 8 = 288 damage, inside a 3 s shield (LT lines 173-177). With "firing ends it early", damage dealt while
     protected is 0 by construction. That closes the pillar 3 hole AB14 named.
   - Unstated: does an **ability press** (Molotov throw, Riot Shield, Banzai) end it? AB14's option (ii) was "the first
     shot or ability press"; his word is "firing". Flag for Mustafa (B3-SP below).
   - Unstated: does **leaving the barracks** end it (AB14 option (iii))? Not in his words; stays open.

## Item 3 (Decided): ammo
1. **His words:** "Ammo: limited but generous, refilled at the barracks and at supply points (keeps an engineer or
   supply role possible later)."
2. **Lands:** LOOP, new §13, new bullet. Nothing in LOOP covers ammo today (0005 line 1234-1235: "LOOP_V2 does not say
   how class weapons refill either"). The engineer or supply role: a `[LATER]` sub-bullet.
3. **Marker:** `[DECIDED]`; the engineer or supply role `[LATER]`.
4. **Conflicts:** none in LOOP or UNITS. It turns the Advisor's lean (README line 62; STATUS lines 187-188: "limited
   but generous, refilled at the barracks and supply points", "not an answer") into his decision. LOOP §9's map layout
   (lines 118-123) has no supply points: a new map element for the octopon-3 layout (not a conflict; a gap).
5. **ADR links:** OQ-0005-AB2 (looted ammo; README line 62; 0005 lines 1232-1239): answered, with the 2026-11-03
   "LATER" routing now moot for this question. His words read as covering all guns, class and looted alike; that
   matches AB2's option (c) "refilled the same way as class weapons". Record the link; the Advisor confirms the
   reading.
6. **Design notes:**
   - AB2 (line 1238-1239) warns option (c) is "the strongest pull away from class identity": with item 1 (a looted gun
     replaces the class gun; the ability stays) a looted gun can last a whole life. Combined with item 1's combos, this
     raises the must-have risk. Measure in sims: share of lives that end holding a looted gun.
   - "Generous" needs numbers per gun (magazine and reserve as data; LOOP line 8). Legacy reference: AMK reserve 999,
     Bagpipe 24, SMG 144 (LT line 184); legacy also has a resupply action on E (`tryResupply`, shamble-game.js:3267),
     so "supply points" has a legacy precedent.
   - Does a dead player's dropped gun carry its remaining ammo (AB2 option (a)) or a refill? Unstated; sits with AB9.

## Item 4 (Decided): rejoining, and bots with loot and amps
1. **His words:** "Rejoining keeps a player's amp tracks; bots can loot weapons and pick amps."
2. **Lands:** two places, both new bullets:
   - Rejoin: LOOP new §13 (LOOP has no rejoin or multiplayer-session text; §11 line 157 puts multiplayer outside v0).
   - Bots: LOOP §10, after line 133-134, which reads "`[DEFAULT]` Bots must fight across three teams, buy T1 units,
     escort and hunt harvesters, and carry, defend and steal uplinks." New bullet: bots can loot weapons and pick amps.
     (Alternatively one bullet in §13 with a pointer from §10.)
3. **Marker:** `[DECIDED]` for both. Bots picking amps needs amps to exist, so for the greybox v0 it is effectively
   `[LATER]` (v0 scope, §11, has no amps); say so in the bullet or leave the v0 list to decide.
4. **Conflicts:** none in LOOP or UNITS.
5. **ADR links:**
   - OQ-0004-10 (README line 107; 0004 line 937): "does a player who rejoins after the grace window get their player
     record back?": answered for the tracks. Money is not named in his words (0001 line 507: "their record (tracks,
     money)").
   - OQ-0004-12 (README line 109; 0004 line 962): "do bots take world pickups?": answered yes, for weapons.
   - OQ-0005-AB10 (README line 122; 0005 lines 1339-1352): "do bots have tracks and make picks": answered yes
     (options (a) or (b)); "what a human gets when taking a bot's slot" still open.
   - 0001 part 11, lines 505-508 (reconnect, the 60 s grace window, GD-REJOIN): bears on it; the record's key
     (slot or player) follows the answer, per that text.
6. **Design notes:**
   - Unstated: whether "rejoining" means after the grace window too (that is OQ-0004-10's exact case); inside the
     window the slot is held anyway (0001 line 505-506). Flag (B3-REJOIN).
   - Unstated: money on rejoin; and a human taking a bot's slot (AB10's second half).
   - Bots looting changes AB9's balance "when most slots are bots" (0005 line 1349-1350). Three-team modes with uneven
     bot counts (VISION §3) need their own sims: bot-heavy teams now gain loot and amps too.

## Item 5 (Decided): amps on weapons only; account progression gives breadth only
1. **His words:** "Amps (amplifications) are on weapons only. No class perks and no learn-by-doing layer. Account
   progression gives breadth only (class variants, cosmetics), never power."
2. **Lands:** split in two.
   - Amps: LOOP new §13, new bullets (amps defined, weapons only, no class perks, no learn-by-doing layer). LOOP never
     says "amp" today (grep: 0 hits); the name is Mustafa's of 2026-11-03 (README line 47; STATUS line 191).
   - Account: LOOP §8, line 105, **replacement**. Existing text: "`[DECIDED]` Unlocks give breadth, never raw power:
     more classes, sidegrade variants, cosmetics." Also touches line 111-112: "`[DEFAULT]` Every role that counters
     something (...) is available from level 1. Unlocks add variants and specialists, so new players are never short
     of an answer."
3. **Marker:** `[DECIDED]` for all parts.
4. **Conflicts (his 2026-11-04 words win):**
   - STATUS line 236 (UPGRADE-SCOPE, Mustafa 2026-10-31): "Every unit and every weapon has its own roguelike upgrade
     track" → unit tracks are gone. Same text in 0005 Settled inputs (lines 66-67) and AB-4's table (line 1027:
     "a track per unit type and per weapon type"), and 0001 part 12 (line 541: "levels and picks per unit type and per
     weapon type"). Not B3's files; recommend LOOP §12 records the supersession.
   - README line 63 / STATUS lines 189-190 (AB3 answer): "XP from kills, assists and objective work while using that
     unit or weapon" → the "unit" half has no track to feed now.
   - LOOP line 105 lists "more classes"; his list is "(class variants, cosmetics)". Possibly narrower: are new classes
     (and the "specialists" of line 112) still account unlocks? Flag (B3-UNLOCK); his newest words win on whatever
     he confirms.
   - DN lines 17-18 (Momentum: "roguelike 'pick one of three' upgrades") and line 21 (Mastery: "breadth ... never raw
     power"): consistent, no conflict.
5. **ADR links:**
   - OQ-0005-AB4 (README line 117; 0005 lines 1260-1272, "what is one track"): narrowed: unit and vehicle-unit tracks
     out. Still open: shared tracks (the Luger in two classes), ability-granted weapons (Katana, Molotov) and vehicle
     weapons.
   - OQ-0005-AB5 (README line 60; WEAPON-UNLOCKS): his 2026-11-03 lean "class variants" now appears inside a
     Decided line. Bears on it strongly (option (a)); whether it closes AB5 is the Advisor's reading, not mine.
   - OQ-0005-AB7 (README line 119, "may a pick add raw power"): "never power" is said of **account** progression only.
     For amps, item 10's ladders include raw-power amps with a cost (hollow points: "harder close, even faster
     falloff"; slugs "hit hard and fire slowly"), which reads as AB7 option (c) mixed or (b) with trade-offs. Link only.
   - OQ-0005-AB6 (README line 118, tracks beside tech tiers): untouched, still open.
   - OQ-0005-AB15 (README line 127): enemies see a build (2026-11-03); with weapon-only amps the build is the held
     gun's amps. Link only.
6. **Design notes:**
   - "No learn-by-doing layer" is not defined in any doc. Reading A: no separate skill layer that grows a class from
     use (Battlefield-style class mastery). Reading B: amps do not level by use, which would contradict the AB3 answer
     ("XP ... while using that ... weapon"). Flag (B3-LBD); recommend reading A, confirmed by Mustafa.
   - Earned-power vs way-back (VISION §2 pillars 1 and 2, `[OPEN]` there): weapon-only amps plus item 1 (loot replaces
     the class gun) mean a looter starts the looted gun's own track (progress belongs to the weapon type, STATUS
     line 240). Catch-up rule from AB3: "a track that's behind the player's best levels faster." Measure both sides
     once numbers exist: TTK between a levelled and a fresh gun per matchup, and the time for a fresh track to close
     the gap (0005 lines 1252-1256).

## Item 6 (Decided): active reload on every gun, gentle
1. **His words:** "Active reload on every gun, gentle: a perfect reload is faster, with little or no damage bonus; a
   miss is a normal reload, no jam. Amps can widen the sweet spot."
2. **Lands:** LOOP §7, line 94, **replacement**. Existing text: "`[DECIDED]` Gears-style **active reload**: tap R at
   the right moment for a faster reload and a short damage boost." The "amps can widen the sweet spot" half can sit in
   the same bullet or in §13 (amp candidates). Touches UNITS §4 line 111, which describes Gears' own rule ("nail the
   sweet spot for a faster reload + briefly boosted damage, miss it and you jam"): that is a reference description of
   Gears, not our rule; optional one-line note "v2's version is gentler: LOOP_V2 §7".
3. **Marker:** `[DECIDED]` (unchanged tag, new content).
4. **Conflicts:** LOOP line 94 "a short damage boost" → "little or no damage bonus"; the 2026-11-04 words win. Also
   0005 AB-3 line 936 ("active reload's short damage boost") and UNITS line 137 ("AR (Lancer feel, active reload ...)")
   are compatible after the change only if the boost is near zero; 0005 is not B3's file.
5. **ADR links:** OQ-0001-7 (README line 72; 0001 lines 1147-1154): the reload tap's moment inside a 30 Hz tick.
   A gentle rule lowers the cost of an edge-of-window misjudgement (a miss is a normal reload, no jam), which bears on
   how much the Feel Lab check matters; decides nothing. R1-ABL-OQ6 (README line 163): "active reload numbers are game
   rules": the sweet-spot width and the speed gain are now data the amps modify. 0001 line 631 (own weapon state,
   active reload, predicted for the local player): unaffected.
6. **Design notes:**
   - "Every gun" includes the shell-by-shell shotgun (item 10): an active reload per shell, or once per reload cycle?
     And ability weapons (Molotov: reload 0.1 s, Katana: none, LT line 183): presumably excluded. Flag (B3-AR).
   - Stacking on the AMK: reload time = 3.5 s × (1 − L1 gain) × (1 − perfect gain) on legacy numbers (LT line 183).
     The AMK's TTK vs a Warrior (120 HP, 4 hits of 34, mag 1) is 3 reloads: 3 × 3.5 = 10.5 s legacy (matches CD13,
     0005 line 1008). With, for example, 20 % and 20 % gains (illustrative, not proposed): 3 × 2.24 = 6.72 s. The
     Warrior's 0.86 s close-range kill still dominates; the triangle holds at close range. The risk sits at mid range,
     where the SMG falls off (15-35 m, LT lines 193-195). Needs sims, 3 runs per build.
   - A floor on reload time is a data value to add (so stacked amps cannot reach a near-zero cycle).

## Item 7 (Decided): sprint-to-fire delay
1. **His words:** "Every gun gets a short sprint-to-fire delay, longer for heavy guns."
2. **Lands:** LOOP §7, new bullet after line 95 (the AMK rhythm bullet). Nothing in LOOP or UNITS covers it today.
3. **Marker:** `[DECIDED]`. The delay per gun is a data value (LOOP line 8).
4. **Conflicts:** none. Legacy has sprint speed (`sprintMult`, LT `UNIT_TYPES`) but no sprint-to-fire delay.
5. **ADR links:** no open question names it. Bears on 0001's prediction scope (line 186-187 and line 631: "own weapon
   state (ammo, fire timers, reload and active reload)" is predicted for the local player): the delay is a fire timer,
   so it falls inside that scope. Link only.
6. **Design notes:**
   - "Heavy guns" needs a weight class per gun in data; item 16 (weapon weight → movement speed, an Idea) would reuse
     the same field. Recommend one weight field serving both, if item 16 is adopted.
   - Item 10's SMG L1 ("faster sprint-to-fire") is the first amp that edits this value: the Warrior's kit already has
     the highest sprint (sprintMult 1.62 vs 1.214 for the Conscript, LT `UNIT_TYPES`), so its sprint-in burst is its
     strength; the delay is its counterweight. Pillar 3 measure: time from sprint-stop to first hit, per class.

## Item 8 (Decided): picking an amp with Alt+1/2/3
1. **His words:** "Picking an amp: when a gun levels up, three cards wait at the screen's edge; pick with Alt+1/2/3.
   Rebindable, and tested early in Brave and Firefox, because Alt can steal browser focus or switch tabs; fallback:
   hold B + 1/2/3."
2. **Lands:** LOOP new §13, new bullet (with the test and the fallback as sub-bullets). Nothing in LOOP or UNITS
   covers it today.
3. **Marker:** `[DECIDED]` for the offer and the binding; the fallback is his own conditional ("fallback: hold B +
   1/2/3"), so `[DECIDED]` as a fallback, used if the early test fails. The early browser test is a task, not a rule:
   it belongs in the build plan (and in the "Brave check" list of the change-set that builds it), noted in the bullet.
4. **Conflicts:** none in LOOP or UNITS. Legacy binds plain Digit1/2/3 to weapon slots (shamble-game.js:3268-3270:
   `equipPrimary`, a dev gun cycle, `equipSpecial`), which is presumably why a modifier is needed; legacy is frozen,
   so no conflict for v2.
5. **ADR links:** OQ-0005-AB8 (README line 120; 0005 lines 1308-1321, "the offer: when does it appear, and when can
   the player answer it?"): answered as option (a) "at once, in play, answerable any time". Still open in AB8: what a
   full queue does when a track levels again (i/ii/iii), whether candidates always differ, and two guns levelling
   together (a looted gun and the sidearm). Offer size 3 matches 0001 part 12 line 535 ("three choices by default")
   and UPGRADE-SCOPE (STATUS line 236). The cards are ADR 0003's to draw (0003 lines 61 and 458, owner-only offers);
   the look is Mustafa's.
6. **Design notes:**
   - The browser-focus risk is his own flag. Add to the test list the other three desktop browsers VISION §4 names
     (Chrome, Edge, Safari; line 52), not only Brave and Firefox, and both OSes he can reach. On macOS, Alt is Option
     and types special characters; whether the page still sees the digit's key code is a test item, not a claim.
   - A mid-fight pick costs a hand off the movement keys (pillar 1 vs readability). The "cards wait" wording implies a
     queue: an unanswered offer waits, which needs AB8's queue rule.
   - The fallback key B: no v2 binding exists yet to collide with, but Renegade-style games often put the buy menu on
     B; reserve it in the key map once v2 has one.
   - "Rebindable" implies a settings store for key bindings; no VISION §6 item names one. It is a client setting, not
     an engine feature, so the trace question (CLAUDE.md "every engine feature traces to a VISION_V2 §6 need") likely
     does not apply; the engine-architect confirms in the build plan.

## Item 9 (Decided): the Molotov is a real fire patch
1. **His words:** "The Molotov in v2 is a real fire patch that denies an area, not a grenade."
2. **Lands:** LOOP §5, line 64, a sub-bullet under the Conscript. Existing text: "**Conscript:** AMK rifle; ability
   Molotov." Touches LOOP §4 table line 58 ("Tech 1 | T1 class abilities (Molotov, Riot Shield, Banzai blade)"): no
   change needed there. UNITS line 30 (RA3 reference: "cheap rifle + **Molotov** cocktail (anti-garrison)"): a
   reference row, unchanged.
3. **Marker:** `[DECIDED]`.
4. **Conflicts:** none in LOOP or UNITS (LOOP is silent on how the Molotov works). It supersedes, outside B3's files:
   0005 line 985 (the Molotov "ends in an area blast", "no fire area (GC11)"); and the legacy behaviour (GAME_DESIGN.md
   line 115: "lobbed incendiary, AoE"; LT `molotov` row and the shared blast 140 damage in 6.5 m, per 0005 line 985).
   LOOK_V2 line 33 ("grenades show fragments flying, not just smoke and fire") applies to grenades; "not a grenade"
   means the Molotov is outside it. No conflict.
5. **ADR links:**
   - 0005 line 985, the Molotov row's open cell: "whether it leaves a burning area: legacy has none and LOOP_V2 is
     silent; an area that lingers would be a new operation": answered (yes). The new operation is engine work for
     0005's next revision (a reviewed Rust change per AB-1/AB-6), not B3's.
   - ABL-PREDICT / OQ-0001-4 (README line 57): the Feel Lab test throws the Molotov (0005 line 2421): the thrown part
     is unchanged; the patch is a new thing the test may also judge.
   - OQ-0005-AB11 (README line 123): "the Conscript's Molotov punishes the shield hard": the counter now runs through a
     lingering area, not a blast.
   - OQ-0005-AB14 (README line 126): an enemy fire patch is an "effect another team applies" (option (b)) if it
     burns a spawn-protected soldier at a barracks opening.
   - OQ-0004-11 (README line 108, which row gives way when a pool is full) and R2-BUD-OQ5 (README line 167, caps on
     live projectiles): fire patches are new lingering entities that need a pool and a cap.
   - R1-ABL-OQ6 (README line 163): stacking (two patches, or a patch plus item 10's incendiary rounds).
6. **Design notes:**
   - **Triangle check (pillar 3).** Today's leg "the Conscript beats the Peacekeeper" rests on a blast that skips the
     frontal block (0005 lines 1009 and 1447: same damage with the Shield up or down). Legacy burst: 140 damage
     within 6.5 m (linear falloff) vs the Peacekeeper's 150 HP (LT line 218). A patch deals d per second for as long
     as the target stands in it; a Peacekeeper who steps out takes d × t_exit. The patch must still "punish the shield
     hard": the shield-carrier moves at ×0.72 (0005 line 986), so t_exit for a patch of radius R is about
     R / (0.72 × v_walk × 0.95). Formula only: R, d and v_walk are unset. Sims at Tech 1, 3 runs per build, check that
     the Conscript still wins the matchup.
   - Area denial is a new verb in the T1 kit: good for uplink fights (LOOP §4) and barracks openings, and a risk of
     stalemate at chokepoints. Who it hurts: own team (legacy owner ×0.5, 0005 line 985)? Unstated.

## Item 10 (Decided, "first sketch, to iterate"): tier-one weapon ladders
1. **His words:**
   "Tier-one weapon ladders (first sketch, to iterate):
   - Conscript AMK (bolt action): L1 faster reload; L2 incendiary rounds (a small fire on hit). Suppression parked; if
     ever, a small screen-edge effect that doesn't stack.
   - Peacekeeper shotgun (shells load one by one): L1 faster reload (two shells at a time); L2 a choice of buckshot or
     slugs (slugs hit hard and fire slowly, and still lose a long-range duel to the rifle); L3 fork: buckshot leads to a
     sawed-off (lighter, faster), slugs to a longer barrel (steadier aim).
   - Imperial Warrior SMG (high recoil, strong close, falls off at mid range): L1 faster sprint-to-fire; L2 hollow points
     (harder close, even faster falloff); L3 drum mag. Dual-wield parked for a tier-two SMG or a variant. Its amps never
     add range."
2. **Lands:** UNITS, new §0 (v2 roster decisions), sub-section "Tier-one weapons and their amp ladders", one block per
   gun. Plus LOOP §5 lines 64-66, **touch** (gun descriptions), existing text:
   "- **Conscript:** AMK rifle; ability Molotov." / "- **Peacekeeper:** shotgun; ability Riot Shield." /
   "- **Imperial Warrior:** SMG, strong up close; ability Banzai blade (one-hit kill while active)."
   Recommended: LOOP keeps the unit line and adds the gun type ("AMK bolt-action rifle", "shotgun, shells loaded one by
   one", "SMG: high recoil, strong close, falls off at mid range") plus a pointer "amp ladders: UNITS.md §0". The
   ladders are weapon content, so UNITS (the task's split: units and factions there). A judgement call; B3-HOME below.
3. **Marker:** `[DECIDED]` with "first sketch, to iterate" kept in the text. Suppression and dual-wield: `[LATER]`.
   "Its amps never add range": `[DECIDED]`, a rule (it can also sit in LOOP §13 as a general amp constraint for the
   SMG only, as he wrote it).
4. **Conflicts:**
   - LOOP §7 line 95: "`[DECIDED]` Fix the AMK's rhythm (one shot, then a 3.5 s reload): a small magazine, the active
     reload, or both." Not contradicted: "bolt action" plus L1 faster reload plus item 6 bear on it. Whether bolt
     action is the chosen fix (a small magazine cycled by the bolt) is not said. Keep line 95; flag (B3-AMK).
   - UNITS §5 line 137 ("Rifleman / basic ... → AR (Lancer feel, active reload, melee built in)"): the Conscript's gun
     is now a bolt action, not an assault rifle. A reference synthesis; add a pointer to §0 or leave it.
   - GAME_DESIGN.md line 115 (legacy "AMK Rifle I (semi-auto)"): legacy, not a v2 conflict.
   - Mustafa's three-card offer (item 8) vs the ladders: each level lists one amp (L1, the AMK's L2, the SMG's L2/L3) or
     two (the shotgun's L2 choice, the L3 fork). Three cards per level-up and one or two named amps per level do not
     match. Not a contradiction in text, but the plan must say how they fit (B3-CARDS).
5. **ADR links:** OQ-0005-AB4 (what is one track: the sawed-off and longer barrel are forks of one shotgun track, so
   "variants of one weapon" share a track, option (b)); OQ-0005-AB7 (raw power: hollow points and slugs add damage
   with a cost); OQ-0005-AB8 (offer size vs the ladder); AB-5's candidate requirements (0005: a candidate can require
   a tag or the track's level) carry the L3 fork's dependency on the L2 pick. OQ-0005-CD8 (0005 lines 702-711, top
   speed when amps add speed): the sawed-off "lighter, faster" (if "faster" means movement) touches it. Links only.
6. **Design notes (math on legacy numbers, LT lines 173-195 and 218; illustrative, not proposals):**
   - **AMK:** shots to kill n = ceil(HP / 34): Conscript 3, Warrior 4, Peacekeeper 5. With a 1-round magazine,
     TTK = (n − 1) × reload: 7.0 s, 10.5 s, 14.0 s. Bolt action changes the cycle to (n − 1) × bolt time inside a
     magazine; the numbers are unset. **Incendiary L2:** a burn of b total damage per hit lowers n where 34 + b crosses
     a breakpoint: b ≥ 16 makes the Conscript a 2-hit kill (2 × 50 = 100), b ≥ 6 makes the Warrior 3 hits
     (3 × 40 = 120), b ≥ 4 makes the Peacekeeper 4 hits (4 × 38 = 152). Small burns move breakpoints: the triangle
     risk (Conscript vs Warrior) sits at b ≥ 6. Burn stacking with the Molotov patch: R1-ABL-OQ6.
   - **Shotgun:** legacy 8 pellets × 11, magazine 2, reload 2.6 s, range 34 m. Kills a Warrior in 0.85 s only if 11
     of 16 pellets land (0005 line 1006). Shell-by-shell reload: reload(k shells) = k × t_shell; L1 halves the count of
     load steps. **Slugs must lose the long-range duel to the rifle:** at range r, TTK_slug(vs Conscript, 100 HP) >
     TTK_AMK(vs Peacekeeper, 150 HP) = 14.0 s legacy. With slug damage s, fire interval f, magazine m and reload R:
     TTK_slug = (ceil(100/s) − 1) × f + reloads. The condition is checkable once s, f, m, R and the slug's range
     falloff exist; the cleanest guarantee is a slug falloff that ends below the AMK's reach.
   - **SMG:** legacy 8 damage, 0.072 s, 36 rounds, falloff from 15 m to 35 m down to 0.2 (LT lines 193-195). Warrior
     kills a Conscript in 12 × 0.072 = 0.864 s (13 hits). Hollow points: damage × h close, falloffEnd moved below 35 m.
     "Never add range" is checkable as a content rule: no SMG amp may raise range, falloffStart or falloffEnd (a
     content-check rule in data, 0005 CD-2's whole-set check could carry it).
   - **Drum mag (L3):** 36 → more rounds lengthens the close-range kill streak (one mag kills 36 × 8 / 100 = 2.88
     Conscripts at full damage); with Banzai and item 1's loot, watch the Warrior's multi-kill rate.
   - **Two-team vs three-team:** in three-team fights the SMG's close strength and the AMK's reach meet two enemy teams
     at once; run the ladder sims in both modes (VISION §3).

## Item 11 (Decided): factions
1. **His words:** "Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
   specialists and vehicles are unique). Personalities: Soviets heavy and brutal, Allies precise and technical, Empire
   fast and tricky."
2. **Lands:**
   - UNITS new §0, sub-section "Factions" (personalities, and the shared-vs-unique rule), new text.
   - LOOP §5 lines 67-68, **replacement**. Existing text: "`[DEFAULT]` All three are shared by every faction, because
     the counter triangle needs them (Warrior beats Conscript beats Peacekeeper beats Warrior). Factions differ through
     uniforms and signature vehicles."
   - Touches LOOP §4 table line 59 ("Tech 2 | T2 units (Officer, Javelin) and vehicles (Juggernaut; Hammer tank once
     built)"), §5 lines 71-73 (Javelin, Officer), §6 line 82 (Juggernaut shared) and line 83-84 (signature tanks).
   - Touches UNITS line 5 (status block: factions as "a **visual identity layer**"): legacy wording; recommend a
     pointer to §0.
3. **Marker:** `[DECIDED]` (LOOP 67-68 goes from `[DEFAULT]` to `[DECIDED]` for the shared T1 and triangle).
4. **Conflicts (his 2026-11-04 words win, but two need his reading):**
   - LOOP line 68 "Factions differ through uniforms and signature vehicles" → superseded: tier-two specialists differ
     too, and differences grow with tech.
   - **LOOP §6 line 82:** "`[DECIDED]` The **Juggernaut** stays as the shared heavy walker." vs "vehicles are unique".
     Literal reading: no shared vehicle, so the Juggernaut is no longer shared. Possible reading: "signature vehicles
     are unique" (LOOP line 83 already gives each faction one), Juggernaut still shared. Both are his words; the newer
     wins only if he means the literal reading. Flag (B3-JUG).
   - **LOOP §5 lines 71-73 and §4 line 59:** the Javelin (`[DECIDED]`, T2 `[DEFAULT]`) and the Officer (T2,
     `[DEFAULT]`) are shared T2 units today. "Tier-two specialists are unique" would make them per-faction. LOOP §8
     line 111 also counts the Javelin among the counter roles every player has from level 1, and LOOP §5 line 71 makes
     it the answer to vehicles: if it becomes one faction's unit, the other two need their own anti-vehicle answer
     (pillar 3). Flag (B3-T2).
   - VISION §2 pillar 5 line 36-37 and §7 line 86: "`[OPEN]` How much is faction-specific?": answered in principle.
     VISION is Mustafa's and the Advisor's, not a B3 target: note only.
   - README lines 48-50 / STATUS line 199: "Still open: faction asymmetry (Mustafa works on the unit list first)":
     answered in principle by this item; the unit list itself (which T2 specialists) is still to come.
5. **ADR links:** OQ-0005-CD5 (README line 114; 0005 lines 682-687, faction variants: full definitions or overrides;
   "What decides it: how much factions differ"): this item supplies that input; decides nothing about the format.
   VISION §6.7 (faction content pipeline) unchanged.
6. **Design notes:**
   - Pillar 3 with unique T2: every faction's T2 specialist and vehicle needs an answer available to both other
     factions. Today's only listed answer to vehicles is the shared Javelin. List per faction once the unit list exists.
   - Pillar 5 vs balance: asymmetry grows with tech, so the late game is where faction balance must be measured, in
     two-team and three-team modes separately (VISION §3; three-team adds the case where two factions' strengths both
     beat the third).
   - Personalities give a test for each new unit ("does it read heavy and brutal / precise and technical / fast and
     tricky?"), beside DN line 43's "a new verb, or just a bigger number?".

## Item 12 (Decided): infiltrators
1. **His words:** "Infiltrators (tier two, every faction): one shared core, where entering an enemy building does the
   same for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus a
   faction verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB
   agent interrogates a downed enemy to briefly reveal the enemy team on the map."
2. **Lands:**
   - UNITS new §0, sub-section "Infiltrators (tier two)": the shared core and the three faction verbs. New text.
   - LOOP §5 lines 74-79, **replacement** of the Spy block. Existing text: "`[DECIDED]` `[LATER]` **The Spy** (Tier 2
     or 3), inspired by the Red Alert 3 and Team Fortress 2 spies. It can disguise itself as the enemy, and hack an
     enemy harvester building (refinery) to steal that team's tech. It is built for real players, who create the
     memorable spy moments (bots make poor spy-checkers). / `[OPEN]` Mustafa's idea is that it costs tech. The Advisor
     recommends gating it behind Tech 3 and charging money, so one player can't spend the team's shared tech. / Also
     open: what breaks the disguise, how long the hack takes, how much it steals, and how defenders are warned."
     Recommended: the LOOP block becomes "Infiltrators" with the shared core (a rule), the Spy as the Allies' one, and
     a pointer to UNITS §0 for the three verbs.
   - LOOP §4 table line 60: "Tech 3 | T3 units (the Spy) and heavy options" → the Spy moves to Tech 2 (line 59).
   - LOOP §12 line 163: "The Spy's hack target is now the enemy **harvester building**, not the terminal." → superseded.
3. **Marker:** `[DECIDED]`; keep `[LATER]` (LOOP §11 line 156 lists the Spy outside v0; his words do not change the
   build order). The cost (`[OPEN]`, line 77-78) and "also open" (line 79) stay `[OPEN]`, minus what this answers.
4. **Conflicts (his 2026-11-04 words win):**
   - LOOP line 74 "(Tier 2 or 3)" and line 60 "Tech 3 | T3 units (the Spy)" → Tier 2.
   - LOOP line 75-76 "hack an enemy harvester building (refinery) to steal that team's tech" → refinery steals
     **money**; a **tech building** steals tech; the power plant cuts power. LOOP line 163 (§12) and DN lines 29-33
     (terminal, then refinery) superseded for the target.
   - LOOP line 77-78: the Advisor's "gating it behind Tech 3" no longer fits Tier 2; the cost question (tech or
     money) stays open.
   - The Spy as the only spy (line 74) → one infiltrator per faction.
5. **ADR links:** R2-INT-OQ8 (README line 172: "the Spy's rules are LOOP_V2 §5 `[OPEN]`"): partly answered (targets,
   tier); disguise-breaking, hack time, amounts and warnings still open. DN open question 3 (line 53): partly. VISION
   §6.6 / OQ-0005-CD6 (moments: a steal is a moment). R2-INT-OQ3 (README line 170, team-private economy and objective
   state) and R2-INT-OQ4 (README line 171, what the minimap shows): the KGB reveal is a minimap rule. LOOK_V2 none.
6. **Design notes:**
   - **"Tech building" does not exist in LOOP.** Tech comes from uplinks (§4) and is a team total; LOOP's base has a
     barracks (§2), a refinery (§3), a power plant (§3 line 37) and an HQ (§8 line 99). Which building is the tech
     building? Flag (B3-TECHBLD).
   - **"Steals tech" vs LOOP §4 line 48-49** ("Players never spend it ... A tier, once reached, is never lost. Tech
     stolen from a team slows its next tier"): consistent if a steal only slows the next tier; say so.
   - **"Cuts their power" vs LOOP §3 line 37** ("a destroyed power plant raises that team's prices"): is a cut the same
     price rise, for a time? Unstated.
   - **"Steals money" vs LOOP §3**: money is personal (§3 heading "Money (personal $)"): from whom, to whom? Unstated.
   - **"A downed enemy"**: LOOP has no downed state (grep "downed", "revive" in LOOP, VISION, DN, 0004, 0005: 0 hits).
     The KGB verb needs a new rule (a downed state before death, or a corpse window). Flag (B3-DOWNED).
   - **Three-team modes:** "reveal the enemy team" is ambiguous with two enemy teams: the downed enemy's team, or
     both? Refinery and tech steals in three-team modes need the same answer. Flag with B3-DOWNED.
   - Pillar 3 per verb: disguise (answer: what breaks it, open); stealth and fast (answer: detection, open); reveal
     (answer: its short length). Bots are poor spy-checkers (LOOP line 76-77): bot-heavy teams are exposed.

---
Ideas (all four: marker `[IDEA]` if B3-TAG (a) is chosen; else `[OPEN]` + "Idea (Mustafa, 2026-11-04), to iterate:").

## Item 13 (Idea): Soviet crewed walker
1. **His words:** "Soviets: a crewed walker; the pilot's leap stuns infantry and the gunners finish them (every seat
   has a verb)."
2. **Lands:** UNITS new §0, sub-section "Faction ideas (sketches to iterate)", Soviets. New text. Touches LOOP §6
   (vehicles, lines 81-87) only by pointer, if at all.
3. **Marker:** `[IDEA]` (no exact LOOP tag; see the plan).
4. **Conflicts:** none. Beside LOOP §6 line 82 (the shared Juggernaut walker) and line 83-85 (the Hammer as the Soviet
   signature tank): an idea, so it contradicts nothing; B3-JUG decides whether the Juggernaut stays shared.
5. **ADR links:** VISION §6.4 "`[OPEN]` seats for several players" (line 73); R2-SYNC-OQ7 (README line 166, seats);
   OQ-0005-AB4 (whose vehicle track applies with several seats: now "weapons only", so each seat's weapon?);
   R1-ABL-OQ6 (README line 163, stacking of stuns and slows); OQ-0001-9 (README line 74, melee-range lag compensation:
   the leap is a short-range ability).
6. **Design notes:** a stun-then-finish combo inside one vehicle is a guaranteed kill chain: the must-have risk item 1
   warns of. Needs an answer (pillar 3): the stun's length and a counter (the Javelin, or a stun break). The
   stun is a crowd-control effect no T1 kit has today.

## Item 14 (Idea): Allied Cryo specialist and an IFV
1. **His words:** "Allies: a tier-two Cryo specialist: the freeze builds up visibly, then it leaps and shatters the
   target; it's exposed while it channels. An IFV whose turret weapon depends on the passenger's class (Red Alert 3's
   Multigunner as the reference)."
2. **Lands:** UNITS new §0, "Faction ideas", Allies. New text. UNITS line 24 (RA3 reference: "Cryo Legionnaire
   *(Uprising)* | freeze beam that shatters frozen targets") is the source row: unchanged.
3. **Marker:** `[IDEA]`.
4. **Conflicts:** none (an idea). The Guardian stays the Allied signature tank (LOOP §6 line 83-84).
5. **ADR links:** VISION §6.4 seats (line 73), R2-SYNC-OQ7 (README line 166): an IFV with a passenger seat; OQ-0005-AB4
   (the turret's weapon track when it depends on the passenger); R1-ABL-OQ6 (build-up and stacking); OQ-0005-CD8 (the
   leap's speed against the top-speed bound).
6. **Design notes:** the IFV multiplies combos (passenger classes × turret) and is the clearest pillar 3 "combo nobody
   saw coming" (VISION line 9); each turret form needs its own answer. The Cryo's shatter is a one-hit kill like the
   Banzai blade; its counter is "exposed while it channels": the channel time vs a defender's TTK is the number to
   measure.

## Item 15 (Idea): Empire hover skimmer and a disguise transport
1. **His words:** "Empire: a hover skimmer as its first vehicle (fast, crosses water, sideways dash, rides just above
   the ground; no true flight under the top-down camera), and a disguise transport at tier two."
2. **Lands:** UNITS new §0, "Faction ideas", Empire. New text. Touches LOOP §6 line 83-84 ("The Empire tank is
   open"): the skimmer as its first vehicle bears on it without settling it (an idea).
3. **Marker:** `[IDEA]`.
4. **Conflicts:** none (an idea).
5. **ADR links:** OQ-0005-CD8 (0005 lines 702-711; fast plus a dash against the top-speed bound); VISION §6.2 (true 3D
   simulation: "crosses water" needs water as walkable-by-vehicle terrain; the map has none in LOOP §9); R4-OBST-OQ1
   (README line 190, what a vehicle does to soldiers in its path).
6. **Design notes:** a disguise transport overlaps the Allies' faction verb (item 12: "the Allies' Spy disguises");
   pillar 5 wants faction verbs distinct. Flag for his iteration, not a conflict. "No true flight" fits the camera;
   LOOP §5 line 71 still names "aircraft" among the Javelin's targets.

## Item 16 (Idea): weapon weight sets movement speed
1. **His words:** "Weapon weight could set movement speed."
2. **Lands:** LOOP §7 (combat feel), new bullet after item 7's sprint-to-fire bullet; or LOOP §13 beside loot.
   Recommend §7, next to the sprint-to-fire delay that also reads weight ("longer for heavy guns").
3. **Marker:** `[IDEA]` (his "could").
4. **Conflicts:** none. Legacy speed is per unit (`speedMult`, LT line 221).
5. **ADR links:** OQ-0005-CD8 (a light looted gun could raise speed above the unit's top speed, which 0001's velocity
   bounds assume, 0005 line 703-704).
6. **Design notes:** with item 1, weight makes loot a movement trade: a Warrior (speedMult 1.5) with a heavy looted gun
   loses its speed edge, which protects the triangle (good); the sawed-off "lighter" and the drum mag (item 10) gain
   a movement meaning. If adopted, cap the result at the unit's top speed (CD8's option (b)).

---

## The Open item
- **His words:** "Open: the war around the matches, in its own design session."
- **Where it is noted today:** README lines 47-50 ("Still open, in his words: ... and the war around the matches."; "the
  war around the matches maps to no id"); STATUS line 199 (2026-11-03). Not in LOOP, VISION or DN.
- **Where B3 notes it (recommended):** LOOP §8 (winning and account progression, the nearest section about what
  outlives a match), a new `[OPEN]` bullet: "The war around the matches: its own design session (Mustafa,
  2026-11-04)." VISION §7 (open design questions, lines 83-88) is the natural second home but is not a B3 target:
  a note for the Advisor. No ADR id; none is created.

## Summary table
| # | Item | Kind | File · section · lines | New / replace | Marker |
|---|---|---|---|---|---|
| 1 | Loot keeps the class ability | Decided | LOOP §13 (new) | new | `[DECIDED]` |
| 2 | Spawn protection 3 s, firing ends it | Decided | LOOP §2 line 23 | replace (split) | `[DECIDED]` (was `[DEFAULT]`) |
| 3 | Ammo limited but generous | Decided | LOOP §13 (new) | new | `[DECIDED]`; supply role `[LATER]` |
| 4 | Rejoin keeps tracks; bots loot and pick | Decided | LOOP §13 (new) + §10 after line 134 | new | `[DECIDED]` |
| 5 | Amps on weapons only; account breadth | Decided | LOOP §13 (new) + §8 line 105 (and 111-112) | new + replace | `[DECIDED]` |
| 6 | Gentle active reload | Decided | LOOP §7 line 94 (UNITS line 111 optional note) | replace | `[DECIDED]` |
| 7 | Sprint-to-fire delay | Decided | LOOP §7 after line 95 | new | `[DECIDED]` |
| 8 | Alt+1/2/3 amp cards | Decided | LOOP §13 (new) | new | `[DECIDED]` |
| 9 | Molotov fire patch | Decided | LOOP §5 line 64 | new sub-bullet | `[DECIDED]` |
| 10 | T1 weapon ladders | Decided | UNITS §0 (new) + LOOP §5 lines 64-66 touch | new + touch | `[DECIDED]`; parked `[LATER]` |
| 11 | Factions | Decided | UNITS §0 (new) + LOOP §5 lines 67-68 | new + replace | `[DECIDED]` (67-68 was `[DEFAULT]`) |
| 12 | Infiltrators | Decided | UNITS §0 (new) + LOOP §5 lines 74-79, §4 line 59-60, §12 line 163 | new + replace | `[DECIDED]` `[LATER]` |
| 13 | Soviet crewed walker | Idea | UNITS §0 (new) | new | `[IDEA]` (no exact tag) |
| 14 | Allied Cryo + IFV | Idea | UNITS §0 (new) | new | `[IDEA]` |
| 15 | Empire skimmer + disguise transport | Idea | UNITS §0 (new) | new | `[IDEA]` |
| 16 | Weapon weight → speed | Idea | LOOP §7 | new | `[IDEA]` |
| Open | War around the matches | Open | LOOP §8 | new | `[OPEN]` |

Count by primary home: LOOP 10 (items 1-9 and 16), UNITS 6 (items 10-15); items 10, 11 and 12 also touch LOOP §4/§5,
and item 6 optionally UNITS §4. Plus the Open item in LOOP §8.

## Decisions for B3's plan (for the main session and the Advisor; Mustafa only where marked)
- **B3-TAG** (Advisor): which marker do his "Ideas" get? (a) a new `[IDEA]` tag defined in LOOP line 6-7 and reused by
  UNITS §0; (b) `[OPEN]` + "Idea, to iterate". Recommend (a).
- **B3-HOME** (Advisor): where do amps, loot and ammo live? (a) new LOOP §13 after §12, no renumbering; (b) a new
  section inserted after §7, renumbering §8-§12 and every citation of them. Recommend (a). Ladders in UNITS §0 with a
  pointer from LOOP §5.
- **B3-SCOPE** (Advisor): does §13 also carry the 2026-11-03 answers it builds on (carry slots, levelling, the amps
  name, "enemies can see a player's build"), quoted and dated? Recommend yes: item 1 qualifies the carry-slot rule
  and would read incomplete without it. Without them B3 records only the 2026-11-04 words.
- **B3-SUPERSEDE** (Advisor): LOOP §12 gains a bullet: weapon-only amps replace UPGRADE-SCOPE's "every unit and every
  weapon" track (STATUS line 236; 0005 lines 66-67, 1027; 0001 line 541), for the drafts' next revision. Recommend
  yes; docs-keeper updates STATUS separately.
- Questions for **Mustafa** (his words leave them open; B3 records his words as they are and lists these):
  - **B3-JUG:** "vehicles are unique": does the Juggernaut stop being shared (LOOP §6 line 82)? Options: shared
    stays / per-faction. Recommend asking; no default.
  - **B3-T2:** are the Javelin and the Officer per-faction now (LOOP §5 lines 71-73)? If the Javelin is not shared,
    each faction needs its own anti-vehicle answer from level 1 (LOOP §8 line 111). Recommend: keep the Javelin's
    role shared (its look per faction) until the unit list exists.
  - **B3-UNLOCK:** do account unlocks still include new classes and specialists (LOOP §8 lines 105, 112), or only
    class variants and cosmetics?
  - **B3-LBD:** "no learn-by-doing layer": does it rule out only a separate class-mastery layer (amps still level from
    use, the 2026-11-03 answer), or more? Recommend the first reading.
  - **B3-SP:** does an ability press end spawn protection, or only firing a gun?
  - **B3-REJOIN:** do tracks come back after the 60 s grace window, and does money?
  - **B3-AR:** active reload on the shell-by-shell shotgun: per shell or per reload? Ability weapons excluded?
  - **B3-AMK:** is bolt action the fix LOOP §7 line 95 asked for?
  - **B3-CARDS:** three cards per level-up vs one or two named amps per level in the ladders: what fills the other
    cards?
  - **B3-TECHBLD:** which building is the "tech building" infiltrators enter?
  - **B3-DOWNED:** the KGB verb needs a downed state LOOP lacks; and in three-team modes, which "enemy team" is
    revealed?

## Conflicts found (newest words win unless marked "needs his reading")
1. LOOP line 23 `[DEFAULT]` spawn safety → `[DECIDED]` 3 s + firing ends it.
2. LOOP line 94 "short damage boost" → little or no damage bonus.
3. LOOP line 105 "more classes, sidegrade variants, cosmetics" → "class variants, cosmetics" (needs his reading,
   B3-UNLOCK; also line 112 "specialists").
4. LOOP lines 67-68 `[DEFAULT]` "Factions differ through uniforms and signature vehicles" → T2 specialists and vehicles
   unique.
5. LOOP line 82 `[DECIDED]` shared Juggernaut vs "vehicles are unique" (needs his reading, B3-JUG).
6. LOOP lines 71-73 / line 59 shared Javelin and Officer at T2 vs "tier-two specialists are unique" (needs his
   reading, B3-T2).
7. LOOP line 74 "(Tier 2 or 3)" and line 60 "T3 units (the Spy)" → Tier 2.
8. LOOP lines 75-76 refinery hack steals tech → refinery steals money, tech building steals tech; LOOP line 163 and DN
   lines 29-33 superseded for the target; line 77-78's "gating it behind Tech 3" no longer fits.
9. Outside B3's files: STATUS line 236, 0005 lines 66-67 and 1027, 0001 line 541 ("every unit and every weapon" /
   "per unit type") → weapons only; README line 63 / STATUS lines 189-190 (AB3's "unit or weapon") → the unit half
   has no track; README line 126 / STATUS lines 196-197 ("the Advisor's pencil") → his decision; 0005 line 985 (no
   fire area) → fire patch; 0005 line 936 (damage boost) → little or none.
Not conflicts (compatible): LOOP line 95 (AMK rhythm), UNITS lines 111 and 137 (Gears and synthesis references),
LOOK_V2 line 33 (grenade fragments), DN lines 17-21 (Momentum, Mastery).

## ADR links found (links only, nothing decided)
- Answered or answered in part: OQ-0005-AB1 (README 61, the "to confirm" part), AB2 (62), AB8 (120, option (a)),
  AB10 (122, bots' tracks and picks), AB14 (126, what ends it early), OQ-0004-10 (107, tracks on rejoin), OQ-0004-12
  (109, bots take weapon pickups), 0005 line 985's Molotov open cell (burning area: yes).
- Bears on: OQ-0005-AB4 (117), AB5 (60), AB6 (118, untouched), AB7 (119), AB9 (121), AB11 (123), AB15 (127);
  OQ-0005-CD5 (114), CD8 (0005 lines 702-711, not in README); OQ-0001-4/ABL-PREDICT (57), OQ-0001-7 (72), OQ-0001-9
  (74); OQ-0004-11 (108); R1-ABL-OQ6 (163), R2-SYNC-OQ7 (166), R2-BUD-OQ5 (167), R2-INT-OQ3 (170), R2-INT-OQ4 (171),
  R2-INT-OQ8 (172), R4-OBST-OQ1 (190); 0001 part 11 (lines 505-508) and part 12 (lines 533-541); VISION §2 pillar 5
  and §7 (faction asymmetry), §6.4 seats.
- Open item: no ADR id (README line 50: "maps to no id").

## Skeptic check
Checked 2026-11-04 against the worktree `scratchpad/docs-pass` (HEAD 76c35bf). No repository file edited. "M" = this
mapping's line; "spec" = `b3-spec-2026-11-04.md`. What holds: every LOOP and UNITS landing text quoted here matches the
file word for word (LOOP 23, 58-60, 64-68, 71-79, 82-84, 94-95, 105, 111-112, 133-134, 163; UNITS 5, 24, 30, 111, 137);
the spot-checked README, STATUS, 0001, 0003, 0004, 0005, LT, GAME_DESIGN and shamble-game.js cites are right, and so is
the arithmetic.

**Counts:** (a) 2 minor · (b) 4 · (c) 2 · (d) 9 missed conflicts · (e) 8 (5 missed links, 3 places that decide by
reading) · (f) 1, plus b1, which is also an invention. **Total 26.**

### (a) Coverage and verbatim
- No item dropped, none merged away: 12 Decided, 4 Ideas and 1 Open (spec 4-24) all have a section. A script compared
  every "His words" quote (M46-506) with the spec: all 17 match word for word.
- a1 (minor). M76-77: the proposed LOOP §2 text "3 s of spawn protection; firing ends it early." rewords spec 5
  ("Spawn protection: 3 s; firing ends it early."). The plan should either quote him or say the text is a rewording,
  with his words kept beside it.
- a2 (minor). M310-311: the proposed LOOP §5 descriptions reword him ("shells loaded one by one"; spec 15 says "shells
  load one by one"). Same fix.

### (b) Markers
- b1. M98-99: "the engineer or supply role `[LATER]`". His words only keep it "possible later" (spec 6). In LOOP,
  `[LATER]` marks something planned that is not in the greybox (LOOP 7; paired with `[DECIDED]` at 74, 85, 87, and
  bare at 113 for the planned catalogue), so the tag turns a reason into a planned role. Keep it as plain words inside
  the ammo bullet, with no tag.
- b2. M35 and M313: "parked" mapped to `[LATER]`. His "Suppression parked; if ever, ..." (spec 14) is weaker than a
  planned-later item. Keep his word "parked" in the text; if a tag is used, say that it is not a commitment.
- b3. M32-33, M443, M539-540: option (b) (`[OPEN]` + "Idea") goes against spec 2 ("marked decided or idea exactly as
  below") and against LOOP 7 (`[OPEN]` = "needs a decision before it gets built"). Option (a) is the only one that
  fits his instruction. Cite spec 2 and drop (b).
- b4. Pairing `[LATER]` with `[DECIDED]` is uneven. Item 12 gets `[DECIDED] [LATER]` (M412) and item 4's bot half is
  hedged (M123-124). Items 1, 3, 5, 8 and 10 (loot, ammo, amps, cards, ladders: none is in v0 per LOOP 140-157) get
  `[DECIDED]` alone. The plan should pick one rule. Note that LOOP §7 itself keeps `[DECIDED]` alone for feel items
  that §11 156 puts outside v0.

### (c) Landing places and cites
- c1. M41: "as README line 8-10 already does for the 2026-11-03 answers". README 8 is blank, and 9-10 cover the
  legacy ADR-001..003 and pointers without line numbers. The real source is STATUS 184 ("the drafts take them up at
  their next revision"), or README 42-47.
- c2. M440: "bots make poor spy-checkers (LOOP line 76-77)". The text is on LOOP 76; line 77 is the cost `[OPEN]`.

### (d) Conflicts the mapping missed (grep of LOOP, UNITS, DN, VISION for the listed topics)
- d1. **LOOP 164-165 (§12, a B3 target):** "The Momentum question is largely answered: in-match progression is the
  team's tech tiers". Weapon amps (items 5, 8, 10) add a per-player in-match layer, so this line becomes incomplete.
  The same claim appears in DN 19-20 and DN 50-51 ("Largely answered by LOOP_V2 §4 (tech tiers)"), and DN 50 asks for
  "the first small upgrade pool", which item 10 now sketches.
- d2. **UNITS 151 (a B3 target):** the guns "replacing the placeholder AMK/Bagpipe/Shabzon/etc. ... held until the
  progression design is decided". Item 10 keeps the "Conscript AMK" as v2's tier-one gun, and item 5 settles the
  progression frame. M25-28 says §1-§5 "stay as they are", but this is UNITS's own plan line, not a reference row.
- d3. **UNITS 149 (and 142):** the era mapping puts "T3 shock/stealth (... cloak)" and T2 as "(AT, flame, sniper)".
  Item 12 puts the stealthy Shinobi and every infiltrator at tier two (spec 18).
- d4. **DN 10-12:** "Progression: the highest-priority open design question ... whether power is gated in the match
  (era) or by the account". Item 5 answers the account half ("never power"). M163-164 cites only DN 17-21. The DN
  pointers at 19-20, 32-33 (Spy target "harvester building (refinery)", now superseded by spec 18) and 50-51 go stale
  once LOOP changes (DN 7-8: superseded notes carry a pointer). The plan names no follow-up for docs-keeper.
- d5. **DN 27-28 (Mustafa, Decided):** "Tiers are sophistication, not damage ... not through bigger damage numbers".
  Against it stand item 10's damage amps (hollow points "harder close", slugs "hit hard") and the Advisor's DN 40-41
  ("bias it toward sidegrades over raw power"). 0005 AB7 (lines 1298-1300) names exactly these lines. The mapping links
  AB7 (M171-173) but lists no tension. It needs his reading: are amp levels "tiers"?
- d6. **UNITS 137:** the "Imperial Warrior ... → AR" archetype. His v2 Warrior gun is an SMG (spec 16). M320-321 flags
  only the Conscript half.
- d7. **LOOP 117 and 156-157:** structures players can enter are `[LATER]` and not in v0. Infiltrators work by
  "entering an enemy building" (spec 18), so they depend on that. Not noted (compatible with LOOP 23, since the barracks
  is not a target).
- d8. **VISION 59:** "`[DECIDED]` `[LATER]` Cosmetics and account progression get designed when their time comes."
  Item 5 now fixes part of account progression. Note it beside the VISION §2/§7 notes (M379-382).
- d9 (minor, about an Idea). **UNITS 3** ("then aircraft") and the flying rows (UNITS 39, 44) sit against item 15's "no
  true flight under the top-down camera". The mapping notes only LOOP 71 (M488-489). The idea speaks of the skimmer, so
  this is a tension to flag, not a conflict.
- No hits in any of the four docs for: ammo, rejoin/reconnect, perks, Alt, KGB, IFV, hover vehicles, downed. Sprint
  appears only in reference rows (UNITS 42, 118).

### (e) ADR links
- Every ADR link cited is real (OQ ids and lines checked in README, 0001, 0003, 0004 and 0005).
- Obvious links the mapping missed:
  - e1. OQ-0001-10 "Are weapon pickups predicted?" (README 75; 0001 line 1165): bears on items 1 and 4. M14 says it was
    read, yet no item links it.
  - e2. R2-INT-OQ1 (README 168) and R4-LOS-OQ2 (README 187), "what a player may see": bear on the Spy's disguise, the
    Shinobi's stealth and the KGB reveal (item 12). Only R2-INT-OQ4 (minimap) is linked.
  - e3. R2-ROOM-OQ3 (README 173, a dropped player's soldier during the grace window) and R2-ROOM-OQ4 (README 174,
    humans joining a running match): bear on item 4.
  - e4. OQ-0005-2 (README 133), "Is a weapon definition the same thing as a weapon type?": bears on item 5 (tracks on
    weapons only) and on item 10's sawed-off and longer-barrel forks.
  - e5 (not an ADR). DN 75-76, Brave's Shift+right-click that the page never receives: a direct precedent for item 8's
    worry that Alt steals browser focus.
- Places that decide by reading:
  - e6. M591-592 lists OQ-0005-AB2 as "Answered", yet M104-106 leaves the reading to the Advisor. His words (spec 6)
    say nothing about what a looted gun comes with (AB2's first half, 0005 1232). At most "answered in part, subject to
    the Advisor's reading".
  - e7. M592 lists OQ-0004-10 as answered "(tracks on rejoin)". That question is specifically about a rejoin after the
    grace window (README 107), and M136-137 itself flags that case as unstated (B3-REJOIN). It should be "bears on".
  - e8 (soft). M501: "If adopted, cap the result at the unit's top speed (CD8's option (b))" recommends an answer to
    open OQ-0005-CD8 (0005 702-711; decider: ADR 0004 with the game-designer). Make it a link only.

### (f) Beyond his words
- f1. M235-236: "the fallback ... used if the early test fails". His words (spec 11) do not tie the hold-B fallback to
  the test's result. Record "fallback: hold B + 1/2/3" as he wrote it.
- (b1 above is also an invention: a `[LATER]` engineer or supply role.)
- The rest of the design notes are clearly labelled as notes or recommendations and are kept out of the landing text.
