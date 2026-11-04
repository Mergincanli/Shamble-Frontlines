# Advisor report: b3-design-walk (2026-11-04, branch `claude/sleepy-tesla-ofsxsr`, base main 4893ab6)
## Goal
Record Mustafa's design walk of 2026-11-04 and his answers to B3-Q1 to B3-Q3 in the design docs, word for word and
marked decided or idea, so his unit list and the drafts' next revision build on his own rules. Docs only.
## Outcome
DONE: his 17 walk items and 3 answers are in LOOP_V2 and UNITS.md; the 15 upheld review findings are handled.
## Plan and approval
Steps: 73d546f (plan revisions 1 and 2, his answers), 6a09137 (build, docs-keeper), bfaf142 (notes), the fix commit.
- **Revision 1: APPROVE with 11 conditions**, all met. (1) The learn-by-doing reading is an Advisor `[DEFAULT]` in
  LOOP_V2 §13. (2, 3) The power and card questions are routed to OQ-0005-AB7 and AB8. (4) §13 has the AB5 pointer.
  (5) His message is in the appendix, and E1 was rerun on it. (6) Register labels read "(the advisor's reading,
  2026-11-04)". (7) "Bears on" ids were checked one by one; 7 were dropped (below). (8) No engine-architect. (9) His
  bot sentence stays whole. (10) Notes are in `.checkpoints/`; no Night queue entry (B3 ran by day). (11) See rev 2 (7).
- **Revision 2 (his answers): APPROVE with 10 conditions**, all met (2, 4 and 6 partly through this report: see
  Deviations). (1) The Tech-2 note is in LOOP_V2 §5 and in "For
  Mustafa". (2) The deviations are listed below, E1 and E2 cover his answers, and the appendix has the questions as
  sent. (3) The HQ tension is in Risks. (4) Line 112 reads as given, and the line-105 risk is in Risks. (5) LOOP_V2
  §12 points to ADR 0005:1263 and 1278-1283; `docs/research/r1-simulation-core.md:1386` ("sidegrade variants") is left
  as a research record. (6, 7) Pointers are replaced: "Open since" is 0, with 4 answered (B3-Q1 to B3-Q3). (8) The
  AB5 and CD5 clauses are in. (9) #35's follow-ups (below). (10) Revision 1's conditions stand.
## What changed
- `docs/LOOP_V2.md` (+109 −16): §2, §4–§5, §7, §8 and §10, a new §13 (amps, loot, ammo), the `[IDEA]` tag, and
  §12's list of replaced and outside lines. `docs/UNITS.md` (+51 −3): a new §0 (factions, tier-one ladders,
  infiltrators, ideas). `docs/DESIGN_NOTES.md` (+12 −2): pointers. `docs/adr/README.md` (+18 −13): 13 register edits.
  STATUS (+70 −25), ROADMAP (+5 −4) and REPO_MAP (+2 −2): docs-keeper, then the fix round. No ADR file. Symbols +0
  −0, as planned.
## Evidence
- E1 (his words byte for byte): the verifier's 45 checks on the docs all PASS (its notes; `.gate/` is scratch).
  Rerun on the appendix: 43 pieces (his 33 sentences, 10 answer clauses), each found once there, byte for byte: 0
  failing.
  The three appendix blocks are identical to their sources (diff).
- E2-E6 and E10, the builder's script rerun after the fixes (its base was bfaf142; the verifier checked the counts
  against main 4893ab6): `PASS: 0 failing checks`. E3: the 9 replaced strings
  appear only in §12. `[OPEN]`: LOOP_V2 6 (4 before), UNITS 0, DESIGN_NOTES 1. `[IDEA]`: 2 and 4. `[DEFAULT]` tags:
  20 → 20. The raw string count rises 22 → 26 only because §12 quotes four (two old lines, two ADR lines).
- E7: 7 docs files and this report; no code. `node tools/gate.mjs --no-smoke`: GATE PASS, [1]-[4] and
  [5b]-[5d] PASS, symbols +0 −0, 5a skipped (CI runs it). `repo-map-check.mjs`: "repo map OK: 113 entries".
- **The fix round** was checked by the verifier: PASS WITH NOTES; its five report nits are fixed in the last commit.
- **Not verified:** CI on the final head (the PR's checks show it); smoke locally (5a skipped).
## Reviews
All three said PASS WITH NOTES: game-designer, the verifier and a completeness critic. There were 18 findings. The
skeptic upheld 15 (13 distinct: F3 = CC-3, F5 = CC-7) and refuted 3.
- Fixed: GD-1 (the Shinobi's stealth reveal and sabotage listed as open), F1 (the register list), F2 ("bears on",
  not "answered in part"), F3/CC-3 (STATUS's `[OPEN]` list), F4 (four pointers on superseded STATUS lines), F5/CC-7
  (a comma), F6 (the AB1 exception), CC-1 (§12 also names ADR 0004:502 and :518, and ADR 0005:796-797, :991 and
  :994; :518 is in part 18, not part 17), CC-4 (the VISION_V2 hand-over to B2, in the STATUS and ROADMAP queues),
  CC-5 (the register's "B3" qualified), CC-6 (queue marks), CC-8 (tag wording), and F7 (this report).
- Refuted, kept as is: GD-2 (the two colons are his words after a lead-in the advisor approved); GD-3 (the plan
  leaves §11's v0 list unchanged, and it is still true); CC-2 (the old pointer still holds in part).
## PR #35's follow-ups
B3 took items 1, 2, 3 and 6, plus item 5 in STATUS and ROADMAP; the queue file marks them "Taken by B3 (PR #36)".
These stay in the queue:
- Item 5 in the ADR index line 91 (OQ-0002-9; line 86 on main), ADR 0006's OQ-0006-23 and the #35 report.
- The two routine proofs. The PR Advisor's first scheduled run (18:45:37 UTC) failed on the usage limit; STATUS
  records that (Routines). Its next run, 2026-11-05 06:45 UTC, and tonight's night shift are the next proof.
## Decisions and deviations
- Changes to his words: "I" became "Mustafa" (B3-Q1). Each answer letter became a lead-in that restates the
  question, cited "B3-Qn (x)"; after a lead-in, his clause keeps its lowercase.
- Superseded STATUS lines: docs-keeper wrote one summary instead of a pointer on each line, and F4 added the pointers.
- The plan file's own text was not edited (its deviations list, §8 "Answered", §8.5, "4 open questions"). It is
  deleted with the checkpoint, and this report carries those items instead (revision 2's conditions 2, 3, 4 and 6).
- The register's "bears on" list went from 21 ids to 14.
  - Dropped, as his words don't plainly touch them: OQ-0001-9 (lag compensation for cover and melee; the skeptic
    would keep it for the leaps, the verifier and the main session drop it), R2-BUD-OQ5 (caps on vehicles and
    projectiles: no cap is set), R4-OBST-OQ1 (a vehicle and soldiers in its path), OQ-0005-AB15 (who sees the picks:
    the walk says nothing on it), OQ-0004-11 (which row gives way in a full pool: no pool rule is set), R2-INT-OQ3
    (which economy state is team-private: a steal doesn't say), R2-ROOM-OQ3 (a dropped player's soldier in the grace
    window: his rejoin line is about tracks).
  - Kept: OQ-0001-4, OQ-0001-7, OQ-0001-10, OQ-0005-2, AB9, AB11, CD8, R1-ABL-OQ6, R2-SYNC-OQ7, R2-INT-OQ1,
    R2-INT-OQ4, R2-INT-OQ8, R2-ROOM-OQ4 and R4-LOS-OQ2.
## Risks
1. Line 105's "more classes" is replaced (B3-Q2 (a)). §12 keeps the old words; ADR 0005 AB4/AB5 quote them for now.
2. "The HQ holds the team's tech" (§5) sits beside §8's "Destroying an HQ is a big points swing". What a destroyed HQ
   does to that tech is not said. It is noted, with no question.
3. The drafts drift until their next revision. §12 lists every stale ADR line; no ADR is edited.
## Proposed next step
kit-v3-2 (A2 on his list), plan first.
## Decisions needed
None. His three answers settle B3-Q1 to B3-Q3, and the gaps they leave are noted for his unit list.
## For Mustafa
1. No Brave check: B3 changes docs only.
2. No action needed: you wrote that every faction has an anti-vehicle answer "from tech 1", where the question said
   "level 1". The docs keep your words. Today the Javelin unlocks at Tech 2, and the docs note it for your unit list.
3. After the advisor approves the final head with CI green, reconfirm the merge: say "merge #36" in a session's chat.

## Appendix: Mustafa's words

### His design walk (2026-11-04), word for word
```text
B3, a docs-only change-set after part B, plan first: record Mustafa's design decisions from his 2026-11-04 walk in LOOP_V2 (rules) and UNITS.md (units and factions), marked decided or idea exactly as below.
Decided:
- Looting keeps the class ability (a Conscript with a looted shotgun still throws Molotovs). Rare strong combos are welcome; watch for any combo that becomes a must-have.
- Spawn protection: 3 s; firing ends it early.
- Ammo: limited but generous, refilled at the barracks and at supply points (keeps an engineer or supply role possible later).
- Rejoining keeps a player's amp tracks; bots can loot weapons and pick amps.
- Amps (amplifications) are on weapons only. No class perks and no learn-by-doing layer. Account progression gives breadth only (class variants, cosmetics), never power.
- Active reload on every gun, gentle: a perfect reload is faster, with little or no damage bonus; a miss is a normal reload, no jam. Amps can widen the sweet spot.
- Every gun gets a short sprint-to-fire delay, longer for heavy guns.
- Picking an amp: when a gun levels up, three cards wait at the screen's edge; pick with Alt+1/2/3. Rebindable, and tested early in Brave and Firefox, because Alt can steal browser focus or switch tabs; fallback: hold B + 1/2/3.
- The Molotov in v2 is a real fire patch that denies an area, not a grenade.
- Tier-one weapon ladders (first sketch, to iterate):
  - Conscript AMK (bolt action): L1 faster reload; L2 incendiary rounds (a small fire on hit). Suppression parked; if ever, a small screen-edge effect that doesn't stack.
  - Peacekeeper shotgun (shells load one by one): L1 faster reload (two shells at a time); L2 a choice of buckshot or slugs (slugs hit hard and fire slowly, and still lose a long-range duel to the rifle); L3 fork: buckshot leads to a sawed-off (lighter, faster), slugs to a longer barrel (steadier aim).
  - Imperial Warrior SMG (high recoil, strong close, falls off at mid range): L1 faster sprint-to-fire; L2 hollow points (harder close, even faster falloff); L3 drum mag. Dual-wield parked for a tier-two SMG or a variant. Its amps never add range.
- Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two specialists and vehicles are unique). Personalities: Soviets heavy and brutal, Allies precise and technical, Empire fast and tricky.
- Infiltrators (tier two, every faction): one shared core, where entering an enemy building does the same for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus a faction verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB agent interrogates a downed enemy to briefly reveal the enemy team on the map.
Ideas (sketches to iterate):
- Soviets: a crewed walker; the pilot's leap stuns infantry and the gunners finish them (every seat has a verb).
- Allies: a tier-two Cryo specialist: the freeze builds up visibly, then it leaps and shatters the target; it's exposed while it channels. An IFV whose turret weapon depends on the passenger's class (Red Alert 3's Multigunner as the reference).
- Empire: a hover skimmer as its first vehicle (fast, crosses water, sideways dash, rides just above the ground; no true flight under the top-down camera), and a disguise transport at tier two.
- Weapon weight could set movement speed.
Open: the war around the matches, in its own design session.
```

### B3-Q1 to B3-Q3 as sent (the advisor's revision-1 verdict, "Decisions needed", word for word)
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

### His answers (2026-11-04), word for word
```text
Answers to B3's questions (Mustafa):
- B3-Q1: (c), unit by unit when I write the unit list, with two fixed rules: every faction has an anti-vehicle answer from tech 1; and a role every faction has (anti-vehicle, infiltration) shares one core with a faction twist on top, like the infiltrators.
- B3-Q2: (a). Account progression gives class variants and cosmetics only. New classes come through the team's tech in the match, so nobody is ever missing a counter.
- B3-Q3: (i) no new building: the HQ holds the team's tech, so infiltrating the HQ steals tech; "cuts their power" works like a destroyed power plant for a set time. (ii) a short window on a fresh body, no downed state; the reveal shows only that enemy's team.
```
