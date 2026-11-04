# B3 plan revision 1, with the advisor's 11 conditions applied (2026-11-04)

## Conditions applied
1. Q-LBD becomes a reading: L17 (the `[DEFAULT]` sub-bullet in §13); Q-LBD dropped from §8.
2. Q-POWER routed to OQ-0005-AB7: U0 (UNITS §0's pointer), N4, R8, §8.4 (new row).
3. Q-CARDS routed to OQ-0005-AB8: L17 (§13's pointer), U0 (the card line), R9, §8.4 (new row).
4. The AB5 pointer: L17 (after the 2026-10-31 "no weapon choice when buying" line).
5. His message in the repository: §6 (E1), §10 (step 8).
6. Register labels "(the advisor's reading, 2026-11-04)": §5.3 (intro, R0, R1, R2, R5, R7, R9, R10, R11), §7.
7. R0's "bears on" list verified one id at a time: §5.3 (after R0), §11 (verifier).
8. Reviewers: §11 (engine-architect removed; the verifier's two extra checks).
9. L14: the §13 pointer moved after the attribution.
10. Process: §1 (branch, notes path, the stand-in and PR #35 line), §10 (step 0, the Night queue).
11. Evidence: §6 (E4, E5), §8 (intro), Counts; the open-question count also in §1, §3, §7, §11, §12 and For Mustafa.

Status: PLAN, for the advisor's review (CLAUDE.md protocol step 2). Nothing is edited until the advisor approves.
Inputs: his message word for word (`scratchpad/queue/b3-spec-2026-11-04.md`), the game-designer's mapping with the
skeptic's check (`scratchpad/queue/b3-mapping.md`; every skeptic correction a1-f1 is applied, listed in §9), and the
worktree `scratchpad/docs-pass` (main 868e165 plus PR #35, HEAD 76c35bf). Line numbers below are that worktree's.
Every edit is also anchored by its old text, so a line shift after #35 merges cannot misplace it.

## 1. Goal and scope
**Goal.** Record Mustafa's 2026-11-04 walk (12 Decided items, 4 Ideas, 1 Open) in LOOP_V2 (rules) and UNITS.md
(units and factions), in his words, each marked `[DECIDED]`, `[IDEA]` or `[OPEN]` as his message labels it. Where
his newer words plainly replace older text, replace it and list the old text. Where they leave a conflict, record an
`[OPEN]` question next to the text and do not resolve it.

**In scope:** `docs/LOOP_V2.md`, `docs/UNITS.md`; pointer lines only in `docs/DESIGN_NOTES.md` (choice B3-DN) and in
`docs/adr/README.md`'s register (choice B3-REG); at the end, docs-keeper's `docs/STATUS.md`, `docs/ROADMAP.md`,
`docs/REPO_MAP.md`.

**Out of scope:**
- Any ADR's text (`docs/adr/0001`-`0006`): untouched. Their stale lines are listed as pointers for their next revision.
- VISION_V2 (pillar 5's `[OPEN]`, §5's account line, §7's faction question): handed to B2, which edits VISION_V2.
- Deciding anything his words leave open: the 4 questions in §8 stay `[OPEN]`; 12 smaller gaps go to the open questions
  or build plans that already own them (§8.4).
- Numbers, balance, build order (§11's v0 list is unchanged), the look of the amp cards, legacy code.
- The agent files' tag lists (`.claude/agents/game-designer.md` line 16 lists four tags): a note for kit-v3-2.

**Change type:** docs only (design record). **Symbols:** none; +0 −0. **Branch:** `cs/b3-design-walk`.
**Checkpoint:** `.checkpoints/b3-design-walk.md`. Agents write their notes to
`.checkpoints/b3-design-walk/<agent>-<part>.md`.
**Order:** a cloud session limited to a `claude/…` branch uses that as the stand-in and says so; work starts only after
PR Mergincanli/Shamble-Frontlines#35 has merged to main, on a branch from that main.

## 2. Plan-level choices (the advisor rules)
- **B3-TAG: how are his Ideas marked?** Recommend: a new tag `[IDEA]`, defined once in LOOP_V2's tag line ("Mustafa's
  idea, a sketch to iterate: not decided") and borrowed by UNITS §0. His instruction says "marked decided or idea
  exactly as below", and `[OPEN]` means "needs a decision before it gets built", which an idea does not. The mapping's
  option (b) (`[OPEN]` plus the word "Idea") is dropped: it goes against his instruction (skeptic b3).
- **B3-HOME: where do amps, loot and ammo live?** Recommend (a): a new `## 13. Weapons in the match: amps, loot and
  ammo`, appended after §12, so §1-§12 keep their numbers (VISION_V2, ADR 0005 and DESIGN_NOTES cite §2, §4, §5, §7,
  §8, §9, §11 and §12). The units and factions items (ladders, factions, infiltrators' faction verbs, the three faction
  ideas) go to a new UNITS `## 0) v2: Mustafa's roster decisions and ideas (2026-11-04)`, with pointers from LOOP §5.
  Option (b), a section inserted after §7, renumbers §8-§12 and breaks those citations.
- **B3-SCOPE: does §13 carry the earlier decisions it builds on?** Options: (a) only the 2026-11-04 words; (b) plus the
  2026-11-03 answers it builds on (carry slots, levelling, the name "amps", upgrade visibility); (c) (b) plus the
  2026-10-31 UPGRADE-SCOPE lines that still stand. Recommend (c): LOOP_V2 is written "for readers with no other
  context", and "rejoining keeps a player's amp tracks" and "looting keeps the class ability" read incomplete without
  the track and carry-slot rules. Each carried line is quoted word for word from STATUS.md lines 185-195 and 236-240,
  dated, and its superseded part marked. Not carried: AB11 (Shield and blade), fragments, the 2026-11-03 spawn words
  (now replaced by his 2026-11-04 decision), AB5's lean, "In Unreal terms, the tracks live on the PlayerState" (an
  engine note, already in ADR 0005) and UPGRADE-SCOPE's "Still open" line (answered since).
- **B3-SUPERSEDE: where is the replaced text listed?** Options: (a) one new §12 bullet: the in-file text replaced,
  word for word, plus pointers for what it supersedes outside LOOP_V2 (UPGRADE-SCOPE's unit tracks, ADR 0001 part 12,
  ADR 0005's Molotov blast and damage boost); (b) only the outside pointers, in-file history left to git and the report;
  (c) "was: ..." notes inline at each site. Recommend (a): one place, readable body, and the check in §6 can prove each
  old string survives only there.
- **B3-MARKS: one marker rule.** Recommend: B3 adds only `[DECIDED]`, `[IDEA]` and `[OPEN]`, never a new `[LATER]`.
  Existing `[LATER]` tags stay where they are (the Spy's moves to the infiltrator line that replaces it, so they stay
  out of v0 as the Spy was). His own words stay as he wrote them: "parked" (suppression, dual-wield) gets no tag (skeptic
  b2); "keeps an engineer or supply role possible later" gets no tag (b1, which was an invention); "first sketch, to
  iterate" stays in the `[DECIDED]` line. Build timing stays with §11, which already leaves amps, loot and ammo out of
  v0 by not listing them (skeptic b4).
- **B3-DN: DESIGN_NOTES pointers.** Recommend (a): seven pointer lines in B3, because DESIGN_NOTES lines 7-8 promise a
  pointer wherever LOOP_V2 supersedes a note, and B3 supersedes four (skeptic d4). Option (b): leave them to a later
  docs pass (the notes go stale meanwhile).
- **B3-REG: the ADR register's pointers.** Recommend (a): `docs/adr/README.md` gets one dated clause on each row his
  words touch (11 rows) and one sentence after its design-answers paragraph (lines 47-50) naming the ids they bear on,
  labelled no stronger than his words, as that paragraph's own rule asks. No ADR file is edited. Option (b): leave the
  register to the drafts' next revision (then the register keeps calling "Firing ends it early" "the Advisor's pencil").
- **B3-VISION:** VISION_V2 lines 36-37, 59 and 86 go to B2 as notes (B2 edits VISION_V2 next). Recommend yes.

## 3. Conventions for the new text
- **His words verbatim, unquoted**, followed by "(Mustafa, 2026-11-04)" (in §13 and UNITS §0, whose headers name him:
  "(2026-11-04)"). No bold or other markup inside his words, so the check can find them byte for byte after joining
  wrapped lines. Older words carried by B3-SCOPE are dated the same way (2026-10-31, 2026-11-03).
- **Intentional deviations, all listed here and nowhere else:** (1) a capital first letter where a quoted clause starts
  a bullet ("A looted weapon …", "Bots can loot weapons and pick amps", "Enemies cannot enter a barracks interior");
  (2) labels before a quoted line, taken from STATUS's record ("Levelling:", "Name:"); (3) item 11's first sentence
  appears alone in LOOP §5, both sentences in UNITS §0; item 4's bot half appears again in LOOP §10. Line wrapping at
  120 columns is whitespace only.
- **Open questions in the docs** are worded as questions to him, self-contained, and end with "(Open since
  2026-11-04.)" exactly once each; pointers to them say "See §N's open question" and carry no such ending, so a count
  proves there are four.
- Edits are applied bottom-up in each file, so the line numbers below stay valid while editing.

## 4. Exact edits: docs/LOOP_V2.md (20 edits)

**L0. Header, lines 3-4.** Append one sentence after "It is written for readers with no other context, including Claude
Code.":
```
Updated on 2026-11-04 with Mustafa's design walk: the lines dated 2026-11-04, and §13.
```

**L1. Status tags, lines 6-7** (B3-TAG). Old:
```
**Status tags.** `[DECIDED]` Mustafa decided it. `[DEFAULT]` the Advisor proposed it and it stands unless Mustafa
vetoes it. `[OPEN]` needs a decision before it gets built. `[LATER]` not in the greybox prototype.
```
New:
```
**Status tags.** `[DECIDED]` Mustafa decided it. `[DEFAULT]` the Advisor proposed it and it stands unless Mustafa
vetoes it. `[OPEN]` needs a decision before it gets built. `[LATER]` not in the greybox prototype. `[IDEA]` Mustafa's
idea, a sketch to iterate: not decided (tag added 2026-11-04).
```

**L2. §2, line 23** (item 2; superseded S1). Old:
```
- `[DEFAULT]` Spawn safety: 3 s of spawn protection, and enemies cannot enter a barracks interior.
```
New:
```
- `[DECIDED]` Spawn protection: 3 s; firing ends it early. (Mustafa, 2026-11-04)
  - `[OPEN]` Still open (OQ-0005-AB14 in docs/adr/README.md): what it covers (damage only, or damage and every effect
    another team applies), and whether using an ability counts as firing.
- `[DEFAULT]` Spawn safety: enemies cannot enter a barracks interior.
```
(The sub-bullet records an existing open question and AB14's own option "the first shot or ability press"; it is
routed there, not a new question, §8.4.)

**L3a. §4 table, line 59** (item 12). Old: `| Tech 2 | T2 units (Officer, Javelin) and vehicles (Juggernaut; Hammer tank
once built) |` New:
```
| Tech 2 | T2 units (Officer, Javelin, each faction's infiltrator) and vehicles (Juggernaut; Hammer tank once built) |
```

**L3b. §4 table, line 60** (item 12; S2). Old: `| Tech 3 | T3 units (the Spy) and heavy options |` New:
```
| Tech 3 | T3 units and heavy options |
```

**L4. §5, after line 64** ("  - **Conscript:** AMK rifle; ability Molotov.") (item 9). Insert:
```
    - `[DECIDED]` The Molotov in v2 is a real fire patch that denies an area, not a grenade. (Mustafa, 2026-11-04)
```

**L5. §5, after line 66** (the Imperial Warrior line) (item 10, pointer; no rewording of the gun lines, skeptic a2).
Insert:
```
  - Their guns and the guns' first amp ladders: UNITS.md §0 (Mustafa, 2026-11-04).
```

**L6. §5, lines 67-68** (item 11; S3). Old:
```
- `[DEFAULT]` All three are shared by every faction, because the counter triangle needs them (Warrior beats Conscript
  beats Peacekeeper beats Warrior). Factions differ through uniforms and signature vehicles.
```
New:
```
- `[DECIDED]` Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
  specialists and vehicles are unique). (Mustafa, 2026-11-04; personalities and each faction's units: UNITS.md §0.)
  The counter triangle: Warrior beats Conscript beats Peacekeeper beats Warrior.
```

**L7. §5, after line 73** ("- `[DEFAULT]` The **Officer** moves to Tier 2.") (Q-UNIQUE). Insert:
```
- `[OPEN]` Shared or unique? The Javelin and the Officer are shared tier-two units today, and the Juggernaut (§6) is
  the shared heavy walker, while the faction rule above makes tier-two specialists and vehicles unique. Does each of
  the three become a per-faction unit, or stay shared? The Javelin is today's only listed answer to vehicles, and §8
  makes it available from level 1. (Open since 2026-11-04.)
```

**L8. §5, lines 74-79** (item 12; S4, S5, S6). Old:
```
- `[DECIDED]` `[LATER]` **The Spy** (Tier 2 or 3), inspired by the Red Alert 3 and Team Fortress 2 spies. It can
  disguise itself as the enemy, and hack an enemy harvester building (refinery) to steal that team's tech. It is
  built for real players, who create the memorable spy moments (bots make poor spy-checkers).
  - `[OPEN]` Mustafa's idea is that it costs tech. The Advisor recommends gating it behind Tech 3 and charging money,
    so one player can't spend the team's shared tech.
  - Also open: what breaks the disguise, how long the hack takes, how much it steals, and how defenders are warned.
```
New:
```
- `[DECIDED]` `[LATER]` Infiltrators (tier two, every faction): one shared core, where entering an enemy building does
  the same for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus
  a faction verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB
  agent interrogates a downed enemy to briefly reveal the enemy team on the map. (Mustafa, 2026-11-04; each faction's
  infiltrator: UNITS.md §0.)
  - `[DECIDED]` **The Spy** (the Allies' infiltrator), inspired by the Red Alert 3 and Team Fortress 2 spies. It can
    disguise itself as the enemy. It is built for real players, who create the memorable spy moments (bots make poor
    spy-checkers).
  - `[OPEN]` Which building is the "tech building"? The base today has a barracks (§2), a refinery and a power plant
    (§3) and an HQ (§8), and tech comes from uplinks (§4). And does "cuts their power" work like a destroyed power
    plant (their prices rise, §3), and for how long? (Open since 2026-11-04.)
  - `[OPEN]` The KGB agent interrogates a downed enemy, but no rule here has a downed state: a soldier dies and
    respawns (§5). What is "downed": for example a short downed state before death, or a short window on a fresh body?
    And with two enemy teams, does the reveal show the downed enemy's team, or both? (Open since 2026-11-04.)
  - `[OPEN]` Mustafa's idea is that it costs tech. The Advisor recommends gating it behind Tech 3 and charging money,
    so one player can't spend the team's shared tech. (2026-11-04: infiltrators are tier two, so the Tech 3 gate no
    longer fits; the cost, tech or money, stays open.)
  - Also open: what breaks the disguise, how long the hack takes, how much it steals, and how defenders are warned.
```
(The `[LATER]` moves with the Spy line it replaces, B3-MARKS. The Spy line loses only "(Tier 2 or 3)" and ", and hack
an enemy harvester building (refinery) to steal that team's tech"; its other words stay.)

**L9. §6, after line 82** ("- `[DECIDED]` The **Juggernaut** stays as the shared heavy walker.") (Q-UNIQUE pointer).
Insert:
```
  - `[OPEN]` Is it still shared, now that "vehicles are unique" (Mustafa, 2026-11-04)? See §5's open question.
```

**L10. §7, line 94** (item 6; S7). Old:
```
- `[DECIDED]` Gears-style **active reload**: tap R at the right moment for a faster reload and a short damage boost.
```
New:
```
- `[DECIDED]` Gears-style **active reload**: tap R at the right moment. Active reload on every gun, gentle: a perfect
  reload is faster, with little or no damage bonus; a miss is a normal reload, no jam. Amps can widen the sweet spot.
  (Mustafa, 2026-11-04)
```
(Line 95, "Fix the AMK's rhythm …", stays: his words do not say whether bolt action is that fix, §8.4.)

**L11. §7, after line 95** (items 7 and 16). Insert:
```
- `[DECIDED]` Every gun gets a short sprint-to-fire delay, longer for heavy guns. (Mustafa, 2026-11-04)
- `[IDEA]` Weapon weight could set movement speed. (Mustafa, 2026-11-04)
```

**L12. §8, after line 105** ("- `[DECIDED]` Unlocks give breadth, never raw power: more classes, sidegrade variants,
cosmetics.") (item 5's account sentence; Q-UNLOCK). Line 105 stays, because whether "more classes" survives is the
open question. Insert:
```
- `[DECIDED]` Account progression gives breadth only (class variants, cosmetics), never power. (Mustafa, 2026-11-04)
  - `[OPEN]` Do account unlocks still include new classes and specialists, as the line above ("more classes") and the
    role line below ("Unlocks add variants and specialists") say, or only class variants and cosmetics? (Open since
    2026-11-04.)
```

**L13. §8, after line 113** (the `[LATER]` unlock catalogue line) (the Open item). Insert:
```
- `[OPEN]` The war around the matches, in its own design session. (Mustafa, 2026-11-04)
```

**L14. §10, after line 134** (the first bots bullet) (item 4's bot half). Insert:
```
- `[DECIDED]` Bots can loot weapons and pick amps. (Mustafa, 2026-11-04; §13)
```

**L15a. §12, line 163** (S8). Old: `  - The Spy's hack target is now the enemy **harvester building**, not the terminal.`
New:
```
  - The Spy's hack target is no longer the terminal: since 2026-11-04, every faction's infiltrator enters an enemy
    power plant, refinery or tech building (§5).
```

**L15b. §12, lines 164-165** (S9; skeptic d1). Old:
```
  - The Momentum question is largely answered: in-match progression is the team's tech tiers, which unlock abilities,
    units and vehicles.
```
New:
```
  - The Momentum question is largely answered: in-match progression is the team's tech tiers, which unlock abilities,
    units and vehicles, and each player's weapon amps (§13).
```

**L16. §12, after line 167** (the last bullet) (B3-SUPERSEDE (a)). Append:
```
- Mustafa's design walk of 2026-11-04 (the lines dated 2026-11-04, and §13):
  - Inside this file it replaced, word for word: in §2, "`[DEFAULT]` Spawn safety: 3 s of spawn protection, and
    enemies cannot enter a barracks interior."; in §4, "T3 units (the Spy)"; in §5, "`[DEFAULT]` All three are shared
    by every faction, because the counter triangle needs them (Warrior beats Conscript beats Peacekeeper beats
    Warrior). Factions differ through uniforms and signature vehicles.", and in the Spy line "(Tier 2 or 3)" and ", and
    hack an enemy harvester building (refinery) to steal that team's tech"; in §7, "for a faster reload and a short
    damage boost"; in §12, "The Spy's hack target is now the enemy **harvester building**, not the terminal."
  - Outside this file, pointers only (the drafts take them up at their next revision; no ADR is edited here):
    UPGRADE-SCOPE's "Every unit and every weapon has its own roguelike upgrade track" (STATUS.md; ADR 0005's settled
    inputs and AB-4; ADR 0001 part 12's "per unit type") now holds for weapons only (§13). ADR 0005's Molotov row ("an
    area blast", no fire area) meets the fire patch (§5). ADR 0005 AB-3's "active reload's short damage boost" meets
    "little or no damage bonus" (§7). The register (docs/adr/README.md) points each open question his words touch.
```
(The last sentence only with B3-REG (a).)

**L17. New §13, appended at the end** (items 1, 3, 4, 5, 8; B3-SCOPE (c); S10, S11; Q-LBD, Q-CARDS).
```
## 13. Weapons in the match: amps, loot and ammo
Added 2026-11-04. Mustafa's rules for the weapons a player holds in a match, in his words: his frame of 2026-10-31
(UPGRADE-SCOPE) and his answers of 2026-11-03, as STATUS.md records them word for word, and his design walk of
2026-11-04. Each line carries its date. The guns and their first amp ladders: UNITS.md §0.

- `[DECIDED]` Amps (amplifications) are on weapons only. No class perks and no learn-by-doing layer. (2026-11-04)
  This narrows the frame of 2026-10-31, "Every unit and every weapon has its own roguelike upgrade track (pick one of
  three per level), owned by the player.", to weapons.
  - Name: the in-match picks are "amplifications" ("amps"). (2026-11-03)
  - `[DEFAULT]` The Advisor's reading (2026-11-04): "no learn-by-doing layer" rules out a separate layer that grows a
    class or a player from use; guns still level from XP earned while using them (the levelling line below), as the
    same walk says "when a gun levels up". Mustafa may veto it.
- `[DECIDED]` Tracks survive death and switching (switch back and the track is still there). They reset at the start of
  every match. (2026-10-31)
- `[DECIDED]` Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the
  next gun of that type. (2026-10-31)
- `[DECIDED]` Rejoining keeps a player's amp tracks; bots can loot weapons and pick amps. (2026-11-04)
- `[DECIDED]` Levelling: XP from kills, assists and objective work while using that unit or weapon; fast early levels;
  a track that's behind the player's best levels faster. (2026-11-03; amps are on weapons only since 2026-11-04, so
  only the weapon half applies.)
- `[DECIDED]` Picking an amp: when a gun levels up, three cards wait at the screen's edge; pick with Alt+1/2/3.
  Rebindable, and tested early in Brave and Firefox, because Alt can steal browser focus or switch tabs; fallback:
  hold B + 1/2/3. (2026-11-04)
  - What fills the other cards when a ladder level names fewer than three amps (UNITS.md §0) is OQ-0005-AB8's
    question in docs/adr/README.md: whether the candidates always differ, and what a track with fewer candidates than
    the offer's size does.
- `[DECIDED]` Upgrade visibility: enemies can see a player's build. (2026-11-03)
- `[DECIDED]` A class always comes with its own weapon: no weapon choice when buying. (2026-10-31) How an
  account-unlocked weapon then reaches a match (§8's third gate) is open: OQ-0005-AB5 in docs/adr/README.md.
- `[DECIDED]` Other weapons come only from killed enemies' drops; dropped weapons are world pickups. (2026-10-31)
- `[DECIDED]` A looted weapon replaces the main (class) weapon; the sidearm stays as backup. (2026-11-03)
- `[DECIDED]` Looting keeps the class ability (a Conscript with a looted shotgun still throws Molotovs). Rare strong
  combos are welcome; watch for any combo that becomes a must-have. (2026-11-04; it confirms the 2026-11-03 "Assumed,
  to confirm: the class's own abilities stay.")
- `[DECIDED]` Ammo: limited but generous, refilled at the barracks and at supply points (keeps an engineer or supply
  role possible later). (2026-11-04)
```
(The fallback is recorded as he wrote it, not tied to the test's result: skeptic f1.)

## 5. Exact edits: the other files

### 5.1 docs/UNITS.md (5 edits)
**U0. New §0, inserted after line 10** (the blank line after the first `---`, before `## 1) Red Alert 3 …`) (items 10-15;
Q-POWER). Insert, followed by a blank line, `---` and a blank line:
```
## 0) v2: Mustafa's roster decisions and ideas (2026-11-04)

From Mustafa's design walk of 2026-11-04, in his words. Status tags as in [`LOOP_V2.md`](LOOP_V2.md): `[DECIDED]`
Mustafa decided it · `[IDEA]` Mustafa's idea, a sketch to iterate: not decided · `[OPEN]` needs a decision before it
gets built. The rules these units play by are in LOOP_V2 (§5 units, §7 combat feel, §13 amps, loot and ammo). The
reference rosters in §1-§5 below are unchanged.

### Factions
- `[DECIDED]` Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
  specialists and vehicles are unique). Personalities: Soviets heavy and brutal, Allies precise and technical, Empire
  fast and tricky. (2026-11-04)
  - `[OPEN]` Which of today's shared units stay shared (the Javelin, the Officer, the Juggernaut)? See LOOP_V2 §5's
    open question.

### Tier-one weapons and their amp ladders
- `[DECIDED]` Tier-one weapon ladders (first sketch, to iterate): (2026-11-04)
  - Conscript AMK (bolt action): L1 faster reload; L2 incendiary rounds (a small fire on hit). Suppression parked; if
    ever, a small screen-edge effect that doesn't stack.
  - Peacekeeper shotgun (shells load one by one): L1 faster reload (two shells at a time); L2 a choice of buckshot or
    slugs (slugs hit hard and fire slowly, and still lose a long-range duel to the rifle); L3 fork: buckshot leads to a
    sawed-off (lighter, faster), slugs to a longer barrel (steadier aim).
  - Imperial Warrior SMG (high recoil, strong close, falls off at mid range): L1 faster sprint-to-fire; L2 hollow
    points (harder close, even faster falloff); L3 drum mag. Dual-wield parked for a tier-two SMG or a variant. Its amps
    never add range.
  - Whether amps may add raw power is OQ-0005-AB7 in docs/adr/README.md (Mustafa's); these ladders bear on it: faster
    reloads, a drum mag, and hollow points and slugs, whose damage gain comes with a cost.
  - How many cards each level offers: see LOOP_V2 §13's pointer to OQ-0005-AB8.

### Infiltrators (tier two)
- `[DECIDED]` Infiltrators (tier two, every faction): one shared core, where entering an enemy building does the same
  for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus a faction
  verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB agent
  interrogates a downed enemy to briefly reveal the enemy team on the map. (2026-11-04)
  - `[OPEN]` The tech building and a downed state are not in the rules yet: see LOOP_V2 §5's open questions.

### Faction ideas (sketches to iterate)
- `[IDEA]` Soviets: a crewed walker; the pilot's leap stuns infantry and the gunners finish them (every seat has a
  verb). (2026-11-04)
- `[IDEA]` Allies: a tier-two Cryo specialist: the freeze builds up visibly, then it leaps and shatters the target;
  it's exposed while it channels. An IFV whose turret weapon depends on the passenger's class (Red Alert 3's
  Multigunner as the reference). (2026-11-04)
- `[IDEA]` Empire: a hover skimmer as its first vehicle (fast, crosses water, sideways dash, rides just above the
  ground; no true flight under the top-down camera), and a disguise transport at tier two. (2026-11-04)
```
(Weapon weight, his fourth idea, is a rule: LOOP_V2 §7, edit L11.)

**U1. Status block, after line 5** (S12). Insert inside the blockquote:
```
>
> **v2 (2026-11-04):** Mustafa's roster decisions and ideas for v2 are in §0. Where they differ from this block (here:
> factions as a visual identity layer only), §0 holds.
```

**U2. §5, line 137** (S13; skeptic d6). Append to the line, after "melee built in).":
```
 *(v2, 2026-11-04: the Conscript's gun is the AMK, bolt action, and the Imperial Warrior's an SMG; see §0.)*
```

**U3. §5, line 149** (S14; skeptic d3). Append to the line, after "gates already in place.":
```
 *(v2, 2026-11-04: the infiltrators, the Empire's stealthy Shinobi among them, are tier two; see §0.)*
```

**U4. Line 151** (S15; skeptic d2). Append to the blockquote line, after "`era` + `minLevel` gates.":
```
 *(v2, 2026-11-04: the tier-one guns, the AMK still the Conscript's, and their first amp ladders are in §0.)*
```
Not edited: line 111 (Gears' own active reload, a reference description), line 3 and rows 39 and 44 (aircraft; a
tension with an idea's "no true flight", §8.5), the reference rows 22-24, 30 and 43.

### 5.2 docs/DESIGN_NOTES.md (7 pointer lines; only with B3-DN (a))
In the file's own pointer style ("→ *…*"), each a pointer, no new decision:
- **N1. After line 12** (S16; skeptic d4): `→ *The account half: LOOP_V2 §8, "Account progression gives breadth only (class
  variants, cosmetics), never power." (Mustafa, 2026-11-04).*`
- **N2. Lines 19-20** (S17), the pointer becomes: `→ *LOOP_V2 §4 largely answers this: in-match progression is the
  team's tech tiers, which unlock abilities, units and vehicles; and each player's weapon amps (LOOP_V2 §13).*`
- **N3. After line 21** (the Mastery line, "more classes"): `→ *Whether new classes stay account unlocks is open in
  LOOP_V2 §8 (2026-11-04).*`
- **N4. After line 28** ("Tiers are sophistication, not damage"; skeptic d5): `→ *Whether this covers in-match amps is
  OQ-0005-AB7 (docs/adr/README.md); Mustafa's first amp ladders bear on it (UNITS.md §0, 2026-11-04).*`
- **N5. After line 33** (the Spy pointer; S18): `→ *2026-11-04: every faction's infiltrator enters an enemy power plant,
  refinery or tech building; the Spy is the Allies' one (LOOP_V2 §5).*`
- **N6. Line 51** (open question 1; S19), the pointer becomes: `→ *Largely answered by LOOP_V2 §4 (tech tiers); UNITS.md
  §0 sketches the first amp ladders (2026-11-04).*`
- **N7. After line 53** (open question 3, Spy rules): `→ *In part: LOOP_V2 §5 (2026-11-04): the infiltrators steal
  (money or tech) or cut power; the rest stays open there.*`

### 5.3 docs/adr/README.md (12 pointer edits; only with B3-REG (a))
Each clause is appended to the row's question cell, before its Decider cell; nothing in any row is removed. Labels
follow the register's own convention (README line 43): each reading is marked "(the advisor's reading, 2026-11-04)",
with the advisor's reading of each row (skeptic e6, e7).
- **R0. Lines 47-50**, after "… the war around the matches maps to no id.": `Mustafa's design walk of 2026-11-04
  (LOOP_V2, UNITS.md §0) touches the rows dated 2026-11-04 below, each answered at most in part (the advisor's
  reading, 2026-11-04); the drafts take it up at their next revision. It bears on faction asymmetry and keeps the war
  around the matches open, "in its own design session". It also bears, without answering, on OQ-0001-4, OQ-0001-7,
  OQ-0001-9, OQ-0001-10, OQ-0004-11, OQ-0005-2, OQ-0005-AB5, AB9, AB11, AB15 and CD8, and on the design questions
  R1-ABL-OQ6, R2-SYNC-OQ7, R2-BUD-OQ5, R2-INT-OQ1, R2-INT-OQ3, R2-INT-OQ4, R2-INT-OQ8, R2-ROOM-OQ3, R2-ROOM-OQ4,
  R4-LOS-OQ2 and R4-OBST-OQ1.`
  The verifier checks this list one id at a time: it reads each of the 22 ids' question and keeps only those where his
  words plainly touch it, and lists the dropped ones in the report.
- **R1. Row 61, AB1:** `Mustafa (2026-11-04): its "Assumed, to confirm" half is confirmed by "Looting keeps the class
  ability" (LOOP_V2 §13) (the advisor's reading, 2026-11-04).`
- **R2. Row 62, AB2:** `Mustafa (2026-11-04), answered in part (the advisor's reading, 2026-11-04): "Ammo: limited but
  generous, refilled at the barracks and at supply points (keeps an engineer or supply role possible later)." (LOOP_V2
  §13). What a looted gun comes with is not said.`
- **R3. Row 63, AB3:** `Mustafa (2026-11-04) bears on it: amps are on weapons only, so the "unit" half has no track
  (LOOP_V2 §13).`
- **R4. Row 107, OQ-0004-10:** `Mustafa (2026-11-04) bears on it: "Rejoining keeps a player's amp tracks" (LOOP_V2 §13);
  a rejoin after the grace window is not said.`
- **R5. Row 109, OQ-0004-12:** `Mustafa (2026-11-04), answered in part (the advisor's reading, 2026-11-04): "bots
  can loot weapons" (LOOP_V2 §10, §13); other pickups are not said.`
- **R6. Row 114, CD5:** `Mustafa (2026-11-04) bears on it: "the differences grow with tech (tier-two specialists and
  vehicles are unique)" (UNITS.md §0).`
- **R7. Row 117, AB4:** `Mustafa (2026-11-04), answered in part (the advisor's reading, 2026-11-04): amps are on
  weapons only (LOOP_V2 §13). Shared tracks, ability weapons and vehicle weapons are not said.`
- **R8. Row 119, AB7:** `Mustafa (2026-11-04) bears on it: the first amp ladders (UNITS.md §0) include faster reloads
  and damage gains with a cost, and account progression gives "never power" (LOOP_V2 §8). B3's power question is this
  one.`
- **R9. Row 120, AB8:** `Mustafa (2026-11-04), answered in part (the advisor's reading, 2026-11-04): "when a gun
  levels up, three cards wait at the screen's edge; pick with Alt+1/2/3" (LOOP_V2 §13). A full queue, whether the
  cards always differ and two guns levelling together are not said. B3's card question is this one.`
- **R10. Row 122, AB10:** `Mustafa (2026-11-04), answered in part (the advisor's reading, 2026-11-04): "bots
  can loot weapons and pick amps" (LOOP_V2 §13). What a human gets on taking a bot's slot is not said.`
- **R11. Row 126, AB14:** `Mustafa (2026-11-04): "Spawn protection: 3 s; firing ends it early." (LOOP_V2 §2).
  What ends it early is now his words, and the Advisor's pencil is his decision (the advisor's reading, 2026-11-04).
  What it covers, and whether an ability press counts as firing, stay open.`

### 5.4 docs-keeper's step (after the reviews; Opus 5.5, high)
- **STATUS.md:** a "Design walk, Mustafa, 2026-11-04" entry beside the 2026-11-03 ones (pointing to LOOP_V2 and UNITS §0,
  not repeating them); pointers on the lines his walk supersedes there: line 236 (UPGRADE-SCOPE's "every unit"),
  lines 187-188 (ammo "still open"), 196-197 ("Firing ends it early" as the Advisor's pencil) and 199 (faction
  asymmetry "still open"); the open-question counts if B3-REG adds none (it adds none: B3 adds no register id).
- **ROADMAP.md:** one terse entry.
- **REPO_MAP.md:** line 36 (LOOP_V2's tag list gains `IDEA`, and §13); line 50 (UNITS.md: "§0: Mustafa's v2 roster
  decisions and ideas (2026-11-04)").

## 6. Evidence (docs change: greps and a check script, no sims or renders)
A throwaway script, `scratchpad/b3-check.mjs` (never committed), reads the spec and the edited files, joins wrapped
lines (newline plus indentation becomes one space) and prints a PASS/FAIL table; its output goes into the report.
- **E1 every item verbatim.** E1 runs against the copy of his 2026-11-04 message that the report (`docs/reports/…`)
  carries word for word as an appendix, so the PR Advisor can reproduce it from git alone. Each sentence of the 17 items
  (12 Decided, 4 Ideas, 1 Open) is found byte for byte in its file(s), case-sensitive except the first letter (§3's
  listed deviations only). Expected: items 1, 3, 4, 5 (amps), 8 in LOOP §13; 2 in §2; 5 (account) in §8; 6 and 7 in §7;
  9 in §5; 11's first sentence in LOOP §5 and both in UNITS §0; 12 in LOOP §5 and UNITS §0; 10, 13, 14, 15 in UNITS §0;
  16 in LOOP §7; the Open item in LOOP §8; 4's bot half also in LOOP §10. Same check for the carried older lines
  (B3-SCOPE): 5 from 2026-10-31, 4 from 2026-11-03.
- **E2 every marker.** For each found sentence, the tag of the bullet holding it: `[DECIDED]` for the 12 Decided
  items (plus `[LATER]` on the infiltrator line only), `[IDEA]` for the 4 Ideas, `[OPEN]` for the Open item. Each
  carried older line is `[DECIDED]` (or untagged label lines "Name:"), with its own date.
- **E3 every superseded line handled.** For the 7 replaced strings (S1-S5, S7, S8): zero hits anywhere in the file
  outside the new §12 bullet, and exactly one hit inside it. For the 14 marked lines (S6, S9-S21): the marker text is
  present on or right after the old line (the script checks the next 3 lines). For the 12 lines left to others: each is
  named in L16 (ADR lines) or in the B2 hand-over (VISION) or in docs-keeper's brief (STATUS).
- **E4 the open questions.** `grep -c "Open since 2026-11-04"`: "Open since 2026-11-04" is 4 in LOOP_V2 (Q-UNIQUE,
  Q-TECHBLD, Q-DOWNED, Q-UNLOCK), 0 in UNITS, 4 in all, the same four as §8 (B3-Q1 to B3-Q3); each pointer ("See …
  open question") names a section that holds one.
- **E5 the tag.** `[IDEA]`: LOOP_V2 2 hits (the tag line and §7), UNITS 4 (the header and three ideas). No `[IDEA]`
  anywhere on a Decided item; no new `[LATER]` (count of `[LATER]` in LOOP_V2 unchanged at 6). One new `[DEFAULT]` in
  LOOP_V2 (the Q-LBD reading in §13, condition 1), and no other new `[DEFAULT]`.
- **E6 no renumbering.** `grep -n "^## " docs/LOOP_V2.md` lists §1-§13 in order; UNITS lists §0-§5. Every
  `LOOP_V2 §N` citation in docs/ still resolves (a grep of citations against the headings).
- **E7 scope.** `git diff --name-only main...HEAD`: LOOP_V2.md, UNITS.md, DESIGN_NOTES.md and adr/README.md (the last two
  only as approved), then docs-keeper's three, plus the checkpoint while in progress. No `docs/adr/000*` file, no code.
- **E8 nothing invented.** `git diff -U0` added lines, each classed by the script as his words, a carried older line, a
  pointer, or an open question; the verifier reads every added line against the spec and flags any rule, number or
  reading his words do not hold.
- **E9 gate.** `node tools/gate.mjs` PASS (docs only: symbols +0 −0, smoke and layering unchanged) and
  `node .claude/scripts/repo-map-check.mjs` PASS after docs-keeper.
- **E10 links.** Every new relative link resolves (`LOOP_V2.md` from UNITS, the README's `LOOP_V2`/`UNITS.md` names).

## 7. Risks
- **Rewording his words.** Mitigation: unquoted verbatim text with no markup inside, E1 byte for byte, §3's deviations
  listed, the verifier on every added line.
- **Deciding by placement or label** (for example calling a link "answered", or tagging "parked" as `[LATER]`).
  Mitigation: B3-MARKS; register labels marked "(the advisor's reading, 2026-11-04)", as the advisor read each row;
  the four conflicts stay `[OPEN]` beside the text.
- **Two `[DECIDED]` lines that may clash** (LOOP §8 line 105 and the new account line; DESIGN_NOTES' Mastery line).
  Mitigation: the `[OPEN]` question sits right under them, and neither is removed until he answers.
- **Drift in docs B3 does not edit** (ADR drafts, VISION_V2, STATUS). Mitigation: L16's pointer list, the B2
  hand-over, docs-keeper's brief; each named in the report.
- **PR #35 shifting lines.** It does not touch LOOP_V2, UNITS or DESIGN_NOTES; it does touch the README and STATUS, and
  the worktree already holds it. Step 1 re-checks that the four files on main equal the worktree's; every edit is
  anchored by its old text, not only its line.
- **A new tag in tooling.** `[IDEA]` is not in REPO_MAP line 36 or the game-designer's tag list; docs-keeper fixes
  REPO_MAP; the agent file is a note for kit-v3-2 (agents read LOOP_V2's tag line anyway).
- **Length.** LOOP_V2 grows by about 75 lines. §13 keeps one rule per bullet so Mustafa can scan it.

## 8. Questions for Mustafa (his words leave these open; B3 records them as `[OPEN]` and decides none)
B3 records 4 open questions, sent to him as B3-Q1 to B3-Q3 (word for word from the advisor's verdict's "Decisions
needed", 2026-11-04), 12 gaps routed (§8.4) and 3 tensions (§8.5). Q-LBD is a reading (§13); Q-POWER and Q-CARDS
are routed (§8.4).

### 8.1 Factions
- **B3-Q1: Which of today's shared units become faction-specific?**
  - *The question:* On 2026-11-04 you decided that tier-two specialists and vehicles are unique to each faction.
    Today the Javelin (rockets) and the Officer are shared tier-two units, and you decided earlier that the Juggernaut
    "stays as the shared heavy walker".
  - *Why it matters:* the Javelin is the only listed answer to vehicles. The rules promise every counter role from
    level 1, so whatever changes, each faction still needs an anti-vehicle unit.
  - *Options:*
    - (a) All three become per-faction (each faction gets its own version).
    - (b) All three stay shared, and "unique" means each faction's own extra units.
    - (c) Decide unit by unit in the unit list you are writing.
  - *Recommendation:* (c), with one rule kept: every faction keeps an anti-vehicle answer from level 1. Until you
    answer, the docs keep them shared and mark the question open.

### 8.2 Progression and amps
- **B3-Q2: Are new classes still account unlocks?**
  - *The question:* Your 2026-11-04 line says account progression gives "class variants, cosmetics". Your older line
    says unlocks give "more classes, sidegrade variants, cosmetics".
  - *Why it matters:* it decides whether a new class is earned on your account or only reached through the team's
    tech in a match.
  - *Options:*
    - (a) Only class variants and cosmetics; new classes come through tech in the match.
    - (b) Your list was examples; new classes can still be unlocked, as sidegrades.
  - *Recommendation:* (a). It is your newest and narrowest wording, and it matches your 2026-11-03 lean towards
    "class variants". No hurry: the unlock catalogue is not built yet.

### 8.3 Infiltrators
- **B3-Q3: Two infiltrator rules (no hurry: infiltrators come after the prototype).**
  - *(i) The tech building:* the base has a barracks, a refinery, a power plant and an HQ, and tech comes from uplinks.
    Which building is the "tech building": a new one, or an existing one such as the HQ? And does "cuts their power"
    work like a destroyed power plant (prices rise) for a set time?
  - *(ii) "Downed":* no rule has a downed state; a soldier dies and respawns. What is "a downed enemy" for the KGB
    agent: a short downed state before death (a big change that invites revives), or a short window on a fresh body?
    And does the reveal show the downed enemy's team only, or both enemy teams?
  - *Why it matters:* one of the three targets and the Soviet verb don't exist in the rules yet.
  - *Recommendation:* a new tech building, with a power cut that works like a destroyed plant for a set time; and a
    short window on a fresh body, revealing that enemy's team only. These are the smallest rules that keep your words.

### 8.4 Gaps routed to existing owners (no new question now)
| Gap | Routed to |
|---|---|
| Does using an ability end spawn protection (only "firing" is said)? What does protection cover? | OQ-0005-AB14 (recorded `[OPEN]` in LOOP §2) |
| Do tracks come back after the 60 s grace window, and does money? | OQ-0004-10, OQ-0005-AB10 |
| Active reload on the shell-by-shell shotgun: per shell or per reload? | the feel pass's plan and Feel Lab (OQ-0001-7 for the tap's timing) |
| Is bolt action the fix LOOP §7 line 95 asks for (magazine, active reload, or both)? | the feel pass's plan; line 95 stays |
| Does a dropped gun keep its ammo? | OQ-0005-AB2 (rest), AB9 |
| Supply points are not on the Octopon-3 layout (§9) | the map change-set's plan |
| Other browsers for the Alt test (VISION_V2 §4 names Chrome, Edge, Safari) | the cards' build plan (his words name Brave and Firefox) |
| The threshold for "a combo that becomes a must-have" | balance telemetry (VISION_V2 §6.6), the game-designer |
| Steals money: from whom, how much | LOOP §5's "Also open: … how much it steals" |
| Infiltrators need enterable buildings (LOOP §9 and §11: `[LATER]`) | compatible; the infiltrator line keeps `[LATER]` |
| Q-POWER: may amps add power? The first ladders have faster reloads, a drum mag, and hollow points and slugs whose damage gain comes with a cost; DESIGN_NOTES records "Tiers are sophistication, not damage" | OQ-0005-AB7 in docs/adr/README.md (Mustafa's; pointers in UNITS §0, N4 and R8) |
| Q-CARDS: what fills the three cards when a ladder level names fewer than three amps? | OQ-0005-AB8 in docs/adr/README.md (pointers in LOOP §13, UNITS §0 and R9) |

### 8.5 Tensions noted for his iteration of the Ideas (no question now)
- The Empire's disguise transport and the Allies' disguising Spy share a verb; faction verbs read best when distinct.
- "No true flight under the top-down camera" (said of the skimmer) sits beside "aircraft" in LOOP §5's Javelin line and
  UNITS lines 3, 39 and 44.
- The crewed walker's stun-then-finish is a guaranteed kill chain: the "must-have combo" risk his item 1 names.

## 9. The skeptic's corrections, applied
a1, a2 (rewording): his words verbatim, unquoted; LOOP §5's gun lines are not reworded, a pointer is added instead.
b1, b2, b4: B3-MARKS (no new `[LATER]`; "parked" and the supply role stay his words). b3: option (b) of B3-TAG dropped.
c1: the "drafts take them up" source is STATUS line 184. c2: "bots make poor spy-checkers" is LOOP line 76.
d1 LOOP 164-165: L15b. d2 UNITS 151: U4. d3 UNITS 149 (142): U3. d4 DESIGN_NOTES 10-12, 19-20, 32-33, 50-51: N1, N2,
N5, N6. d5 DESIGN_NOTES 27-28: Q-POWER and N4. d6 UNITS 137: U2. d7 LOOP 117 and 156-157: §8.4's last row. d8 VISION
59: the B2 hand-over. d9 UNITS 3, 39, 44: §8.5. e1-e4 (missed links): in R0 (OQ-0001-10, R2-INT-OQ1, R4-LOS-OQ2,
R2-ROOM-OQ3, R2-ROOM-OQ4, OQ-0005-2). e5 (Brave's Shift+right-click, DESIGN_NOTES 75-76): cited for the cards' build
plan beside the Alt test. e6, e7: no register label says "answered"; AB2 is "in part", OQ-0004-10 "bears on". e8: the
CD8 recommendation is dropped; CD8 is a link only. f1: the fallback is recorded as written.

## 10. Steps
0. Night queue. Once these conditions are folded into the plan file, the work goes into the Night queue (issue #16)
   with a link to this verdict. It is gated on PR #35 being merged; a night run never merges. (The verdict:
   `scratchpad/queue/b3-advisor-plan-rev1.md`.)
1. After PR #35 merges: branch `cs/b3-design-walk` from main; check that LOOP_V2, UNITS, DESIGN_NOTES and the README
   equal the worktree's (else re-anchor by old text); open the checkpoint.
2. LOOP_V2 edits L17 → L0 (bottom-up). Checkpoint.
3. UNITS edits U4 → U0. Checkpoint.
4. DESIGN_NOTES (B3-DN (a)) and README (B3-REG (a)) pointers, as approved. Checkpoint.
5. E1-E10 and the gate. Commit.
6. Reviews on the final diff (§11); fix or defer every finding; on fixes, review only what changed.
7. docs-keeper (§5.4). Gate again.
8. Report with `/advisor-report`, carrying his 2026-11-04 message word for word as an appendix (E1 runs against that
   copy), the checkpoint folded in and deleted; PR marked ready; the advisor's APPROVE.

## 11. Reviewers (all Opus 5.5, `model: opus`; effort as stated)
- **advisor** (high): this plan (step 2 of the protocol) and the final head before merge.
- **game-designer** (high): rules and balance consistency of the final diff: every marker, every conflict caught,
  the four questions' options fair and complete, no balance call slipped into the text.
- **verifier** (high): reruns E1-E10 on its own, reads every added line against the spec (E8), and spot-checks the
  register labels against the ADR questions they cite. It also confirms that no ADR decision text moved and no
  register row lost words, and checks R0's 22 "bears on" ids one at a time (§5.3).
- **docs-keeper** (high): STATUS, ROADMAP and REPO_MAP at the end (§5.4).
- Not called: art-director (nothing drawn; the cards' look stays Mustafa's), netcode-architect (no ADR edit; it meets
  these pointers at ADR 0001's next revision), perf, security, graphics, rust, test, infra.

## 12. What Mustafa must check in Brave
Nothing (docs only). Whether the PR needs his reconfirmation as a rule change is the PR Advisor's call; the four
questions in §8 (B3-Q1 to B3-Q3) are his to answer whenever he likes, and B3 does not wait for them.

## Counts
- Edits: LOOP_V2 20 (L0-L17, with L3 and L15 two each); UNITS 5; DESIGN_NOTES 7 (B3-DN (a)); README 12 (B3-REG (a));
  then docs-keeper's three files.
- Superseded lines: 21 handled in B3 (7 replaced: S1-S5, S7, S8; 14 marked: S6, S9-S21, of which S20 and S21 are the
  README's AB2 "still open" and AB14 "the Advisor's pencil", marked by R2 and R11); 4 more in STATUS, marked by
  docs-keeper; 8 listed for others (ADR 0005 lines 66-67, 936, 985 and 1027 and ADR 0001 line 541 for the drafts' next
  revision; VISION_V2 lines 36-37, 59 and 86 for B2).
- Open questions: 4 (Q-UNIQUE; Q-UNLOCK; Q-TECHBLD, Q-DOWNED), plus 12 gaps routed (Q-POWER and Q-CARDS among
  them) and 3 tensions noted; Q-LBD is a reading.

## Decisions needed (the advisor's, on this plan)
- **B3-TAG:** mark his Ideas with a new `[IDEA]` tag (recommended) or not. 
- **B3-HOME:** a new LOOP_V2 §13 appended, no renumbering, units and factions in a new UNITS §0 (recommended).
- **B3-SCOPE:** carry into §13 the 2026-11-03 answers and the 2026-10-31 lines that still stand, word for word and
  dated (recommended (c)).
- **B3-SUPERSEDE:** one new §12 bullet listing the replaced text word for word and the outside pointers (recommended (a)).
- **B3-MARKS, B3-DN, B3-REG, B3-VISION:** as in §2 (recommended: the one marker rule; DESIGN_NOTES and register
  pointers in B3; VISION_V2 to B2).

## For Mustafa
Nothing now: no Brave check. Once the advisor approves the plan, the four questions in §8 (B3-Q1 to B3-Q3) go to him,
and he can answer them before or after B3 merges.
