# Claims ledger: ADR 0005, "The ability and effect system (game-designer)"

- **Draft:** `game-designer-adr0005-abilities-section.md` (this folder). **Author:** game-designer, 2026-10-31, xhigh.
- **Lines** are at 9e7db28 (`git diff --quiet 9e7db28 HEAD` on docs/research, LOOP_V2, VISION_V2, LEGACY_INVENTORY,
  LEGACY_TUNING, DESIGN_NOTES and STATUS exits 0 on this branch). r1, r2, r4, r6 = `docs/research/r1-simulation-core.md`,
  `r2-netcode.md`, `r4-physics-navigation.md`, `r6-infrastructure.md`; INV = `docs/LEGACY_INVENTORY.md`; TUN =
  `docs/LEGACY_TUNING.md`; GL = `docs/research/GLOSSARY.md`; DN = `docs/DESIGN_NOTES.md`; map = the decision map
  (`engine-architect-decision-map.md` and `-ids.tsv`, this folder).
- **Tiers:** `research verified` (an R1 or R2 recommendation claim, or a finding R1's or R2's verifier table lists as
  checked; `inputs/findings-checked.tsv` is the guide) · `research, source to check` (R1 or R2 options cells and open
  questions, R4 and R6 recommendations, R7 inventory and tuning rows, a legacy code line) · `arithmetic` (shown) ·
  `settled` (a STATUS code, GLOSSARY §8, or the approved plan and decision map where a row states which draft decides
  what) · `design doc` (VISION_V2, LOOP_V2, EDITOR_V2, LOOK_V2, DESIGN_NOTES) · `proposed mark` (with its reason).
- **Two kinds of rows need a word.** (1) "ADR 000N decides …" rows record the scope the approved plan (revision 3) and
  the decision map's owner table and part j (J3–J5) give each draft; they are tiered `settled` (plan), not a STATUS
  code. (2) The trade-off lines of the design questions (OQ-0005-AB1 to AB11) are the game-designer's reading of the
  design lines they cite, written for Mustafa's choice; they are tiered `design doc` with those lines, and they carry
  no research claim and no number.

## Intro and Context
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 1 | A reduced GAS subset as data over a closed set of Rust operations; weapons keep their own bookkeeping; one damage pipeline | Intro | r1, Recommendations §5, R1-ABL-REC1, REC2, lines 2342–2400 | research verified |
| 2 | The section designs no upgrade, ability, unit or rule | Intro | plan.md revision 3, ADR 0005 paragraph; advisor-verdicts.md §1, condition 11 | settled |
| 3 | VISION_V2 §6.3 quoted ("A data-driven ability and effect system, like UE's Gameplay Ability System. …") | Context | VISION_V2 §6.3, lines 69–70 | design doc |
| 4 | §6.4: vehicles with their own weapons and abilities, "leech, marking"; seats `[OPEN]` | Context | VISION_V2 §6.4, lines 71–73 | design doc |
| 5 | §6.5 server-side and tunable (tech gates); §6.6 pick and win rates; §6.7 faction content as data | Context | VISION_V2 §6.5–§6.7, lines 74–77 | design doc |
| 6 | Pillar 3 "every strong thing has an answer"; pillar 4 "Every choice costs something" | Context | VISION_V2 §2, lines 31–34 | design doc |
| 7 | Pillars 1 and 2 pull against each other on purpose; their mechanics `[OPEN]` | Context | VISION_V2 §2, lines 29–30; §7, lines 84–85 | design doc |
| 8 | LOOP_V2 §4: abilities locked until Tech 1 with a minute-5 floor `[DECIDED]`; tier ladder `[DEFAULT]` | Context | LOOP_V2 §4, lines 50–60 | design doc |
| 9 | LOOP_V2 §5 tags: T1 `[DECIDED]`, triangle `[DEFAULT]`, Javelin `[DECIDED]` with knockback `[DEFAULT]`, Spy `[DECIDED]` `[LATER]` with `[OPEN]` cost and rules | Context | LOOP_V2 §5, lines 63–79 | design doc |
| 10 | LOOP_V2 §6: Juggernaut `[DECIDED]`; Hammer's leech `[DECIDED]` `[LATER]` | Context | LOOP_V2 §6, lines 82–86 | design doc |
| 11 | LOOP_V2 §7: feel kit, split feedback, flinch and knockback, active reload, AMK rhythm all `[DECIDED]` | Context | LOOP_V2 §7, lines 90–95 | design doc |
| 12 | LOOP_V2 §8: three unlock gates `[DECIDED]`; catalogue `[LATER]` | Context | LOOP_V2 §8, lines 106–113 | design doc |
| 13 | Every number is a starting value, kept in one data table | Context | LOOP_V2, line 8 | design doc |
| 14 | UPGRADE-SCOPE quoted verbatim (six bullets and the "Still open" line) | Context | inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md (bullets byte-compared by script, no difference) | settled |
| 15 | ABL-PREDICT open, Mustafa after the Feel Lab test | Context | STATUS.md, Decisions pending, lines 110, 121–123 | settled |
| 16 | SEC-SPREAD (a): weapon spread in an authority-only stream | Context | STATUS.md lines 108–109; GL §4 "Shared stream and authority-only stream", lines 525–545 | settled |
| 17 | MODES-3 (a): the largest mode is 24v24, N = 48 | Context | STATUS.md lines 104–105; GL "The largest mode, and its N", lines 335–354 | settled |
| 18 | TICK-30: a 30 Hz server tick is the baseline | Context | STATUS.md lines 118–120 | settled |
| 19 | SLICE-ORDER condition 2: movement and shooting input-driven and replayable from day one | Context | STATUS.md lines 99–100 | settled |
| 20 | SPIKE-SCOPE (a): spikes are throwaway | Context | STATUS.md lines 187–188 | settled |
| 21 | R1 recommends options A2, B1 and E1 | Context | r1, Recommendations §5, R1-ABL-REC1, REC2, REC5, lines 2342–2463 | research verified |
| 22 | Every GAS fact from one community write-up (tranek's GASDocumentation, UE 5.3); Epic's documentation and the engine source not read | Context | r1, Findings §5, method note, lines 1185–1189; R1-ABL-OQ1, lines 2756–2758; UNREAD_SOURCES.md rows at lines 27, 133–134 | research, source to check |
| 23 | Fortnite "does its own bookkeeping for weapon firing rates" | Context | r1, Findings §5, R1-ABL-F17, lines 1286–1295 | research verified |
| 24 | Paragon: "Higher latency connections would have a lower ROF with basic attacks" | Context | R1-ABL-F17, lines 1286–1295 | research verified |
| 25 | Quake III's weapon timer is an integer counter that carries its overshoot | Context | R1-ABL-F22, lines 1322–1330 (the carry reading of `+=` is marked inferred there) | research verified |
| 26 | The research has no finding on upgrade tracks; its "playerstate" hits are Quake III's player state (r1:1320, r2:199–237) | Context | `grep -i -c 'roguelike\|pick one of three\|upgrade track\|in-match upgrade\|PlayerState'` over r1–r6 and GL: r1 1 (line 1320), r2 5 (lines 199, 200, 203, 235, 237), r3–r6 and GL 0 | arithmetic |
| 27 | ADR 0004 executes the closed set and the pipeline in the sim step; owns R1-ABL-REC3, OQ5, OQ10, REC6 | Context | map ids table (owner 0004 for R1-ABL-REC3, REC6, OQ5, OQ10); map seam S5 | settled |
| 28 | ADR 0004 decides the player record, the offers' random stream, drops as world pickups | Context | map J5 table | settled |
| 29 | ADR 0001 decides the prediction scope (R1-ABL-REC4, ABL-PREDICT), R1-ABL-OQ8, the authority's checks, messages, visibility classes | Context | map ids table (owner 0001 for R1-ABL-REC4, OQ8, ABL-PREDICT); map J5 | settled |
| 30 | ADR 0006 owns R6-TEL-REC1 (pick fields); ADR 0003 draws cues and HUD, offers included | Context | map ids table (R6-TEL-REC1 owner 0006); map J5 (UI: 0003) | settled |
| 31 | The content data section decides file format, schema, validation, hot reload (R1-DATA) and the feel kit's format | Context | map part a, ADR 0005 item 1; map J4 | settled |
| 32 | Legacy: seven weapons in one table, special-case fields for falloff, burst, melee and charge | Context | INV CD12, line 93 | research, source to check |
| 33 | Legacy: four classes with HP 100 / 150 / 120 / 110 and their abilities | Context | INV CD13, line 94; TUN `UNIT_TYPES` `maxHealth`, lines 205–230 | research, source to check |
| 34 | Legacy: the Juggernaut as data | Context | INV CD14, line 95 | research, source to check |
| 35 | Legacy faults: bot and colour fields in unit data (CD13), multipliers in two places (CS3), silent fallback to the AMK (CC9) | Context | INV CD13 line 94, CS3 line 171, CC9 line 138; R1-ABL-REC5 reason, r1:2455–2456 | research verified |
| 36 | One damage path; the frontal shield block skips blasts | Context | INV CD8 line 89; GC7 line 246 ("not to blasts") | research, source to check |
| 37 | Four class abilities in code; none checks a tech tier | Context | INV GC11 line 250; GE4 line 290 ("useAbility :1374-1375 checks no era"); r1:1373 ("Legacy abilities check no tech tier") | research, source to check |
| 38 | A purchase status enum | Context | INV GE13, line 299 | research, source to check |
| 39 | Fire cooldowns reset, not carried | Context | INV GC5 line 244; R1-ABL-N5 confirmed by R1's verifier, r1:3486 | research verified |
| 40 | No live in-match weapon progression: `RIFLE_TIERS` gun-game ladder is dead code, marked drop | Context | INV GE17, line 303 | research, source to check |

## Decision AB-1 to AB-5
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 41 | The model: base and current attributes; instant, timed or permanent effects, optionally periodic; counted hierarchical tags; abilities gated by required and blocked tags with cost and cooldown; effects grant tags or abilities; cues presentation only | AB-1 | R1-ABL-REC1, r1:2343–2346 | research verified |
| 42 | Content chooses and parameterizes operations; cannot run arbitrary code; each new operation a reviewed Rust change | AB-1 | R1-ABL-REC1, r1:2346–2347; options A2, r1:1797 | research verified |
| 43 | Aggregation order fixed per attribute; GAS's default sums its multipliers, a rule designers must know | AB-1 | R1-ABL-REC1, r1:2353–2354; R1-ABL-F4, r1:1216–1222 | research verified |
| 44 | Stacking an explicit policy from a short list (none, refresh, add up to a limit, strongest only) | AB-1 | R1-ABL-REC1, r1:2354–2355 | research verified |
| 45 | Left out: instancing policies, replication modes, ability tasks as a framework; anything no legacy or LOOP_V2 test case needs | AB-1 | R1-ABL-REC1, r1:2347–2349 | research verified |
| 46 | Why: §6.3 asks for GAS-like data; the subset covers every legacy ability and weapon and §6.4's primitives without generic-engine features | AB-1 | R1-ABL-REC1 reason, r1:2359–2361 | research verified |
| 47 | Rests on F1–F11, F16, F19 | AB-1 | R1-ABL-REC1, r1:2362 | research verified |
| 48 | Need §6.3, §6.4, §6.5, §6.7 | AB-1 | R1-ABL-REC1 "Serves", r1:2363 | research verified |
| 49 | Pipeline stages: locational multiplier, range falloff, frontal-arc block, target-tag multiplier ("strong against vehicles"), owner and team rules | AB-2 | R1-ABL-REC1, r1:2350–2352 | research verified |
| 50 | Each hit becomes an instant damage effect through the pipeline | AB-2 | R1-ABL-REC2, r1:2384–2385 | research verified |
| 51 | The outcome logic GAS gives to execution code (F9) | AB-2 | R1-ABL-REC1, r1:2352; R1-ABL-F9, r1:1244–1247 | research verified |
| 52 | CS3 shows the cost of two homes for one number | AB-2 | INV CS3, line 171 | research, source to check |
| 53 | Need §6.3, "combos and counters can emerge" | AB-2 | VISION_V2 §6.3, lines 69–70 | design doc |
| 54 | Weapon definition holds the sim fields only (list) | AB-3 | R1-ABL-REC2, r1:2381–2383 | research verified |
| 55 | Per-soldier state machine: timer, ammo, burst count, charge, reload, active-reload window | AB-3 | R1-ABL-REC2, r1:2384 | research verified |
| 56 | The feel kit is a separate client definition keyed by the same id | AB-3 | R1-ABL-REC2, r1:2383–2384 | research verified |
| 57 | Abilities may grant or swap weapons (Banzai equips the Katana; Molotov a thrown weapon) and apply effects to weapon attributes (active reload's boost) | AB-3 | R1-ABL-REC2, r1:2385–2387 | research verified |
| 58 | One weapon model for bots and humans fits; whether v2 keeps it is Mustafa's | AB-3 | R1-ABL-REC2 mark, r1:2399–2400; R1-ABL-OQ6, r1:2772–2775 (DESIGN in the map) | research verified |
| 59 | With cooldowns as effects the server overwrites, high-latency players fire more slowly (F17) | AB-3 | R1-ABL-F17, r1:1286–1295 | research verified |
| 60 | A shipped shooter runs an integer weapon timer inside the player code both sides run (F21, F22) | AB-3 | R1-ABL-F21, r1:1317–1321; F22, r1:1322–1330 | research verified |
| 61 | Need §6.3 (weapons from data), §6.4 (vehicle weapons on mounts) | AB-3 | R1-ABL-REC2 "Serves", r1:2395 | research verified |
| 62 | Active reload: need LOOP_V2 §7 only; ADR 0004 executes it | AB-3 | LOOP_V2 §7, line 94; map J3 (0004 lists active reload) | design doc |
| 63 | Kinds: tag registry, attribute set, effect, ability, weapon, feel kit, unit, vehicle (parts, mounts, seats `[OPEN]`), faction | AB-4 | R1-ABL-REC5, r1:2449–2452 | research verified |
| 64 | Ids resolve to dense handles at load; every reference, tag and range checked; failure names file and field; no silent default | AB-4 | R1-ABL-REC5, r1:2452–2453 | research verified |
| 65 | Added: an upgrade-track kind; every unit and weapon definition names its track | AB-4 | UPGRADE-SCOPE bullets 1 and 5 | settled |
| 66 | Added: drop and pickup values as data that ADR 0004's pickup entities read; their values Mustafa's | AB-4 | map J5 ("their lifetime and pickup rule as data from 0005"); UPGRADE-SCOPE bullet 4 | settled |
| 67 | The unit kind names the class's own weapon(s); a purchase gives exactly those (quote) | AB-4 | UPGRADE-SCOPE bullet 3 | settled |
| 68 | File format, schema tooling, file layout, hot reload are the content data section's (R1-DATA; R1-XU-OQ9) | AB-4 | map part a, 0005 item 1 (R1-XU-OQ9 there); map J4 | settled |
| 69 | Legacy tables already have this shape; their faults are mixing, duplication, silent fallbacks | AB-4 | R1-ABL-REC5 reason, r1:2455–2456 | research verified |
| 70 | Need §6.3, §6.4, §6.6, §6.7 | AB-4 | R1-ABL-REC5 "Serves", r1:2459 | research verified |
| 71 | The track kind and the drop and pickup values: need UPGRADE-SCOPE | AB-4 | map J3 (the session's reading: "need: UPGRADE-SCOPE") | settled |
| 72 | The team's tier is team state the sim keeps (ADR 0004) | AB-5 | map part a, 0004 item 7 (economy and objectives) | settled |
| 73 | An ability requires its tier's tag; a unit or vehicle is buyable at its tier; tier and cost are unit data | AB-5 | R1-ABL-REC1, r1:2344–2345; R1-ABL-REC5, r1:2449–2450 | research verified |
| 74 | T1 class abilities require Tech 1 `[DECIDED]` | AB-5 | LOOP_V2 §4, lines 50–52, 58 | design doc |
| 75 | T2 and T3 require their tier (`[DEFAULT]` ladder); quote "the team's tech tier still decides when a unit or vehicle becomes buyable" | AB-5 | LOOP_V2 §4, lines 53–60; §8, line 110 | design doc |
| 76 | Tier thresholds and the minute-5 floor are tuning numbers in data | AB-5 | LOOP_V2 line 8; §4 line 53 ("thresholds to be tuned") | design doc |
| 77 | Run by the economy and objective phases (ADR 0004) | AB-5 | map part a, 0004 item 7 | settled |
| 78 | The authority refuses a press or purchase before its tier | AB-5 | R1-ABL-REC4 mark, r1:2443 ("a rejected press (for example before Tech 1)"); r1:1385 ("Tech-tier gates become activation and purchase requirements read from team state") | research verified |
| 79 | Whether the client predicts the press is ABL-PREDICT | AB-5 | GL "The prediction scope", Depends on, lines 505–510 | settled |
| 80 | R1-ABL-REC1 names "team tech tiers" among the tag gates; legacy checks no tier (GE4, GC11) | AB-5 | r1:2344–2345; INV GE4 line 290, GC11 line 250 | research verified |

## Decision AB-6 (LOOP_V2's abilities)
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 81 | Legacy numbers are starting values, never decisions | AB-6 | LOOP_V2 line 8; TUN lines 3–5 ("not decisions for v2") | design doc |
| 82 | Molotov: the Conscript's ability `[DECIDED]`, locked until Tech 1 | AB-6 | LOOP_V2 §5 line 64; §4 line 58 | design doc |
| 83 | Molotov is a thrown weapon | AB-6 | R1-ABL-REC2, r1:2386 | research verified |
| 84 | Its projectile is swept per tick (R4-PHYS-REC3, ADR 0004) | AB-6 | r4, Recommendations, R4-PHYS-REC3, lines 747–751; map ids table (R4 owner 0004) | research, source to check |
| 85 | The blast runs through the pipeline with radius, falloff, owner and team rules; a blast skips the frontal block, as in legacy | AB-6 | r1:1375 ("Area damage with falloff and owner/team rules as data"); INV GC7 line 246 | research, source to check |
| 86 | Molotov fuse 1.4 s, building ×1.8 | AB-6 | r1:1373; INV GC14 line 253 | research, source to check |
| 87 | Shared blast: 140 damage in 6.5 m, linear falloff, owner ×0.5 | AB-6 | r1:1375; INV GC14 line 253; TUN `T` grenadeDamage 140 (line 94), grenadeRadius 6.5 (line 96) | research, source to check |
| 88 | No fire area in legacy | AB-6 | INV GC11 line 250 ("no fire area") | research, source to check |
| 89 | LOOP_V2 is silent on a burning area | AB-6 | LOOP_V2 §5 (line 64 names "Molotov" only) | design doc |
| 90 | A lingering area would be a new operation | AB-6 | options A2, r1:1797 ("A needed mechanic may lack an operation (then it is a Rust change …)") | research, source to check |
| 91 | Riot Shield `[DECIDED]`; Tech 1 | AB-6 | LOOP_V2 §5 line 65; §4 line 58 | design doc |
| 92 | Shield as a toggled ability applying a permanent effect (tag and movement modifier); the frontal-arc stage reads the tag | AB-6 | r1:1373 ("a toggled stance with a conditional damage reduction (Shield)"); R1-ABL-REC1, r1:2343–2345, 2350–2352 | research verified |
| 93 | Block 70 % within arc cos 0.30, move ×0.72 | AB-6 | TUN `T` lines 132–135; R1-ABL-N11 (×0.72 confirmed, r1:3492) | research, source to check |
| 94 | Banzai blade `[DECIDED]` "one-hit kill while active"; Tech 1 | AB-6 | LOOP_V2 §5 line 66; §4 line 58 | design doc |
| 95 | Banzai as a timed ability whose timed effect grants the melee weapon and a speed modifier, with a cooldown | AB-6 | R1-ABL-REC2, r1:2385–2386 ("Banzai equips the Katana"); r1:1373 ("a timed equip with a speed buff"); R1-ABL-F10, r1:1248–1251 | research verified |
| 96 | Katana for 10 s, 45 s cooldown, speed ×1.3 | AB-6 | r1:1373; TUN `UNIT_TYPES` banzaiCd 45, banzaiDur 10 (lines 226–227); R1-ABL-N11 (×1.3 confirmed, r1:3492) | research, source to check |
| 97 | Katana damage 9999, arc 120°, impact 0.13 s, swing 0.4 s, range 3.6 m | AB-6 | TUN `WEAPONS`, lines 167–203 (damage, arcDeg, impact, swingDur, range rows) | research, source to check |
| 98 | Whether melee is judged at the attacker's view time is ADR 0001's (R2-SYNC-OQ9) | AB-6 | r2, Open questions, R2-SYNC-OQ9, lines 2933–2935; map ids table (owner 0001) | settled |
| 99 | Active reload quoted `[DECIDED]` | AB-6 | LOOP_V2 §7, line 94 | design doc |
| 100 | The reload state machine's window; a tap shortens the reload and applies a timed effect on the weapon's damage attribute | AB-6 | R1-ABL-REC2, r1:2384–2387 | research verified |
| 101 | The authority judges the tap from the command's tick | AB-6 | SLICE-ORDER condition 2, STATUS.md lines 99–100 | settled |
| 102 | The timing is own weapon state, inside the prediction scope; the boost is damage, which only the authority applies | AB-6 | GL "The prediction scope", lines 501–508; R2-SYNC-REC3, r2:2068–2089 | research verified |
| 103 | None in legacy (active reload) | AB-6 | INV GC4 line 243 ("LOOP_V2 §7 adds active reload") | research, source to check |
| 104 | Its numbers are Mustafa's (R1-ABL-OQ6) | AB-6 | r1:2772–2775 ("active reload numbers (LOOP_V2 §7)") | research, source to check |
| 105 | AMK rhythm quoted `[DECIDED]` | AB-6 | LOOP_V2 §7, line 95 | design doc |
| 106 | The magazine is a weapon field and active reload the row above, so each fix is a data change | AB-6 | R1-ABL-REC2, r1:2381 (magazine), r1:2384 (active-reload window) | research verified |
| 107 | AMK magazine 1, reload 3.5 s, interval 0.2 s | AB-6 | TUN `WEAPONS`, lines 167–203 (mag, reload, fireInterval rows) | research, source to check |
| 108 | Javelin quotes (`[DECIDED]`; knockback `[DEFAULT]`); §7 "Targets react: flinch and knockback" | AB-6 | LOOP_V2 §5 lines 71–72; §7 line 93 | design doc |
| 109 | Target-tag multipliers (above 1 against vehicles, below 1 against infantry) | AB-6 | R1-ABL-REC1, r1:2351 | research verified |
| 110 | Knockback is one new operation; the research reads it as "a velocity change on the kinematic soldier" | AB-6 | r4, R4-PHYS-REC3, lines 750–751; r4 line 601 | research, source to check |
| 111 | The flinch is presentation (the feel kit) | AB-6 | LOOP_V2 §7, lines 90–91 ("target flinch" in the feel kit) | design doc |
| 112 | The Javelin is new; LOOP_V2 §11 leaves it out of the greybox v0 | AB-6 | LOOP_V2 §11, line 156 | design doc |
| 113 | Hammer `[DECIDED]` `[LATER]` | AB-6 | LOOP_V2 §6, lines 85–86 | design doc |
| 114 | Two primitives kept possible (an effect removed when its source dies; a last-hit or tick trigger moving the victim's item), which §6.4 needs | AB-6 | R1-ABL-REC1 mark 3, r1:2372–2376; reason r1:2359–2361 | research verified |
| 115 | Marking as a tag-granting effect (the research's reading) | AB-6 | R1-ABL-REC1, r1:2359–2361 | research verified |
| 116 | Who sees the tag is ADR 0001's interest management (R1-ABL-OQ8) | AB-6 | r1:2779–2780; map ids table (owner 0001) | settled |
| 117 | What marking does is Mustafa's (R1-ABL-OQ6) | AB-6 | r1:2774 | research, source to check |
| 118 | Spy `[DECIDED]` `[LATER]`; cost, disguise breaks, hack length, amount, warnings `[OPEN]` | AB-6 | LOOP_V2 §5, lines 74–79 | design doc |
| 119 | An enemy client receives the disguise as the identity it sees | AB-6 | R2-INT-REC5, r2:2502–2512 | research verified |
| 120 | The cost as an instant effect on a named attribute, so money and team tech both stay possible | AB-6 | R1-ABL-F10, r1:1248–1251 (a cost is an Instant effect); LOOP_V2 §5, lines 77–78 (tech or money) | research verified |
| 121 | The Spy's rules are a design question (R2-INT-OQ8) | AB-6 | map ids table (R2-INT-OQ8: DESIGN) | settled |
| 122 | The Bayonet and the Torque Bow are test cases only (REC1 mark 1); regen "a candidate periodic effect" | AB-6 | R1-ABL-REC1 mark 1, r1:2365–2368; r1:1380 | research verified |
| 123 | Bayonet: 1.3 s charge, kills within 2.4 m, 10 s cooldown | AB-6 | r1:1373; TUN `UNIT_TYPES` bayonetCd 10, bayonetDur 1.3 (lines 229–230) | research, source to check |
| 124 | Torque Bow fuse 0.55 s | AB-6 | r1:1376; R1-ABL-N9 confirmed, r1:3490 | research verified |
| 125 | Regen after 3.5 s at 16 HP/s | AB-6 | TUN `T` lines 126–127; INV GC19 line 258 | research, source to check |
| 126 | None of the three is in LOOP_V2; whether each stays is Mustafa's (R1-ABL-OQ6) | AB-6 | INV GC11 line 250, GC15 line 254, GC19 line 258; r1:2772–2773 | research, source to check |
| 127 | Need §6.3, §6.4; the rows' rules LOOP_V2 §4–§7 | AB-6 | R1-ABL-REC1 and REC2 "Serves", r1:2363, 2395 | research verified |

## Decision AB-7 (counters)
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 128 | Any effect, tag or weapon can be put on any unit or vehicle by data alone (mark 2) | AB-7 | R1-ABL-REC1 mark 2, r1:2369–2371 | research verified |
| 129 | Every kill names its source weapon or ability id | AB-7 | r6, Recommendations, R6-TEL-REC1, lines 791–793 | research, source to check |
| 130 | Counters get tuned on evidence | AB-7 | VISION_V2 §6.6, lines 75–76 | design doc |
| 131 | The triangle: Warrior beats Conscript beats Peacekeeper beats Warrior `[DEFAULT]` | AB-7 | LOOP_V2 §5, lines 67–68 | design doc |
| 132 | Peacekeeper kills a Warrior in 0.85 s only if 11 of 16 pellets land, else 3.45 s, against the Warrior's 1.30 s; "leans on the Riot Shield", locked until Tech 1 | AB-7 | INV CD13, line 94; recomputed: 11 × 11 = 121 ≥ 120 HP within the 2-shell magazine at 0.85 s; 0.85 + 2.6 s reload = 3.45 s; ceil(150 ÷ 8) = 19 shots, 18 × 0.072 = 1.296 s | research, source to check |
| 133 | With the Shield up, an SMG hit deals 30 %: 63 hits instead of 19 | AB-7 | 8 × (1 − 0.7) = 2.4; ceil(150 ÷ 2.4) = ceil(62.5) = 63; ceil(150 ÷ 8) = ceil(18.75) = 19; inputs TUN `WEAPONS` shabzon damage 8 (R1-ABL-N10 confirmed, r1:3491), `T` shieldBlock 0.7 (line 133), `UNIT_TYPES` peacekeeper maxHealth 150 | arithmetic |
| 134 | A blocked blade hit deals 9999 × 0.3 = 2999.7, above 150 HP, so the blade kills through the Shield | AB-7 | 9999 × (1 − 0.7) = 2999.7; inputs TUN `WEAPONS` katana damage 9999, `T` shieldBlock 0.7; the rule `amount*=(1-T.shieldBlock)` for non-blast hits in the arc, shamble-game.js:1514–1515 | arithmetic |
| 135 | LOOP_V2 states no answer to the Banzai blade | AB-7 | LOOP_V2 §5, lines 62–79 (no answer named) | design doc |
| 136 | Warrior kills a Conscript in 0.86 s | AB-7 | INV CD13 line 94; R1-ABL-N6 confirmed (12 × 0.072 = 0.864 s), r1:3487 | research verified |
| 137 | 10.5 s the other way | AB-7 | INV CD13 line 94; recomputed: ceil(120 ÷ 34) = 4 hits with a 1-round magazine, so 3 reloads × 3.5 s = 10.5 s | research, source to check |
| 138 | The Conscript wins only beyond the Bagpipe's 34 m | AB-7 | INV CD13 line 94; TUN `WEAPONS` bagpipe range 34 | research, source to check |
| 139 | The Molotov's blast is not blocked by the Shield ("not to blasts") | AB-7 | INV GC7 line 246; shamble-game.js:1514 (`kind!=='blast'`) | research, source to check |
| 140 | Javelin "strong against vehicles and aircraft" `[DECIDED]` | AB-7 | LOOP_V2 §5, line 71 | design doc |
| 141 | Carried by a target-tag multiplier | AB-7 | R1-ABL-REC1, r1:2351 | research verified |
| 142 | Legacy never reduces the Juggernaut's 1500 HP | AB-7 | INV GC17, line 256 | research, source to check |
| 143 | Javelin "weak against infantry" | AB-7 | LOOP_V2 §5, lines 71–72 | design doc |
| 144 | Track content and levelling are Mustafa's | AB-7 | UPGRADE-SCOPE, "Still open" line | settled |
| 145 | These are sums on starting values, not balance evidence; CD13 asks for sims of each matchup (3 or more runs) | AB-7 | INV CD13 line 94 ("sims of each matchup (3+ runs) before keeping these values") | research, source to check |
| 146 | Research line: R1-ABL-REC5 serves §6.6 (ids key pick and win rates) | AB-7 | r1:2459 | research verified |

## Decision AB-8 to AB-10
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 147 | The section adds only the track kind and its load checks, and no new operation: a pick is an effect or a granted ability | AB-8 | UPGRADE-SCOPE; R1-ABL-REC1, r1:2345 ("effects that grant tags or abilities"); proven by spike mark u1 | proposed mark |
| 148 | A track per unit type and per weapon type, candidates per level, each an effect or granted ability with optional requirements | AB-8 | UPGRADE-SCOPE bullet 1; R1-ABL-REC1, r1:2343–2345 | settled |
| 149 | The offer size is a data value, 3, so no number is hard-coded | AB-8 | UPGRADE-SCOPE bullet 1 ("pick one of three"); LOOP_V2 line 8's rule | settled |
| 150 | The player's record holds per track its level, its picks and any open offer (ADR 0004 decides) | AB-8 | map J5 (0004: the player record) | settled |
| 151 | ADR 0001 decides the pick command and the visibility class; R2-INT-REC5's classes are public, team, owner, spatial | AB-8 | map J5; R2-INT-REC5, r2:2502 | research verified |
| 152 | The record, not the soldier, holds every track; on each spawn the picks of that unit type's track apply | AB-8 | UPGRADE-SCOPE bullets 2 and 6 | settled |
| 153 | Respawns and purchases are commands the authority checks | AB-8 | INV GE13 line 299 ("Purchases must be server-validated commands"); GL "The authority", lines 485–500 | research, source to check |
| 154 | Nothing is saved; accounts `[LATER]` | AB-8 | UPGRADE-SCOPE bullet 2 (reset every match); VISION_V2 §6.10, line 81 | settled |
| 155 | The record starts empty at every match start, with ADR 0001's room lifecycle | AB-8 | map J5 ("its reset in the match lifecycle (0001's room lifecycle cites)") | settled |
| 156 | The unit names its weapons; a purchase gives exactly those and names a unit, never a weapon | AB-8 | UPGRADE-SCOPE bullet 3 | settled |
| 157 | A drop is a world pickup entity (ADR 0004); the authority checks who may take it, refuses a pickup it did not place, and sets the drops' visibility class (ADR 0001) | AB-8 | UPGRADE-SCOPE bullet 4; map J5 ("refusing … a pickup it did not place") | settled |
| 158 | A weapon type's picks apply to whichever gun of that type the player holds and leave the gun when lost; a pickup carries none of its last holder's picks; the track keeps its state | AB-8 | UPGRADE-SCOPE bullet 5 | settled |
| 159 | The record is per player slot, beside the soldier tables | AB-8 | UPGRADE-SCOPE bullet 6; map part a, 0004 item 7 ("held on the player's slot, not the unit's") | settled |
| 160 | The authority draws the three with a sim random stream; which stream is ADR 0004's | AB-8 | map J5; R1-DET-REC4, r1:2184–2193 (one stream per purpose) | research verified |
| 161 | A shared stream would let the owner's client foresee its next offers, the reasoning that kept spread authority-only (SEC-SPREAD (a)) | AB-8 | GL "Shared stream and authority-only stream", lines 526–533 (a shared stream is held by the client's predicting sim; a seed the client knows lets a cheat cancel the spread); an inference from that definition | research verified |
| 162 | A pick that was not offered, or for a closed offer, is refused | AB-8 | map J5 ("refusing a pick the server did not offer") | settled |
| 163 | The pick and track fields of the match record are ADR 0006's, keyed by AB-4's ids | AB-8 | map J5; R6-TEL-REC1, r6:784–811; R1-ABL-REC5, r1:2459 | settled |
| 164 | If a pick changes something in the prediction scope, the predicting sim needs its owner's track state; ADR 0001's | AB-8 | GL "The prediction scope", lines 502–504 (own movement and weapon state replayed from the authoritative state); an inference | research verified |
| 165 | No research on tracks | AB-8 | row 26 | arithmetic |
| 166 | Need: UPGRADE-SCOPE, outside VISION_V2 §6, LOOP_V2, EDITOR_V2 and LOOK_V2 | AB-8 | map J3 ("UPGRADE-SCOPE … is cited the same way ('need: UPGRADE-SCOPE')") | settled |
| 167 | If ABL-PREDICT says yes: own activation, cost, cooldown, granted tags, movement changes; the command's sequence number as the key; cues keyed to play once | AB-9 | R1-ABL-REC4, r1:2423–2428; ABL-PREDICT, r1:2967–2978 | research verified |
| 168 | If no: ability presses wait for the server's confirmation | AB-9 | ABL-PREDICT, r1:2977–2978; GL "The prediction scope", lines 505–510 | research verified |
| 169 | Nothing in AB-1 to AB-8 depends on the answer; only the prediction scope changes | AB-9 | GL "The prediction scope", Depends on, lines 505–510 (the two readings differ only in what the scope adds) | research verified |
| 170 | Need §6.3 | AB-9 | R1-ABL-REC4 "Serves", r1:2440 | research verified |
| 171 | The mark: at most 1 ms per tick at p99 in the stress scene, up to 8 active effects per soldier, on one vCPU of the reference server and the minimum laptop in Wasm in a Worker; nothing allocated on the heap in steady state | AB-10 | R1-ABL-REC6, r1:2465–2472; GL "The stress scene", lines 397–418 | research verified |
| 172 | N = 48 soldiers | AB-10 | MODES-3 (a) | settled |
| 173 | 48 × 8 = 384 active effects | AB-10 | 48 × 8 = 384; GL line 405 | arithmetic |
| 174 | At 30 Hz, 1 ms is 12 % of the sim step share | AB-10 | 1 ÷ 8.33 = 0.120; 8.33 ms = 25 % of 33.3 ms (GL "The sim step share …", lines 190–191; GL §1 table, line 63) | arithmetic |
| 175 | At 60 Hz, 24 % of 4.17 ms | AB-10 | 1 ÷ 4.17 = 0.240; 4.17 ms from GL §1 table, line 63 | arithmetic |
| 176 | The 1 ms is the same at both rates | AB-10 | GL §1 table, line 57 (abilities, effects, weapons: 1.00 at 60 Hz and 1.00 at 30 Hz) | research verified |
| 177 | Proposed: run the step at 8, 16 and 32 active effects per soldier (384, 768, 1,536 at N = 48) and report the count that reaches 1 ms p99 | AB-10 | reason: picks are long-lived active effects whose count is not designed (OQ-0005-AB3); 48 × 16 = 768, 48 × 32 = 1,536 | proposed mark |
| 178 | The stress scene's 8 per soldier predates UPGRADE-SCOPE | AB-10 | GL dated 2026-10-30 (line 9), its 8 from R1-ABL-REC6 (line 415); UPGRADE-SCOPE dated 2026-10-31 | settled |
| 179 | Need VISION_V2 §3 and §4 only; a budget, not a feature | AB-10 | R1-ABL-REC6, r1:2478 | research verified |

## Options and trade-offs
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 180 | Question C is ADR 0004's (R1-ABL-REC3); question D is ADR 0001's (R1-ABL-REC4) | Options | map ids table (REC3 owner 0004; REC4 owner 0001) | settled |
| 181 | The research has no option for how tracks are carried; a separate upgrade system is not researched | Options | row 26 | arithmetic |
| 182 | A1: 11 concept sections (F1–F20), several for Unreal's object and replication model; the highest effort; performance unknown | Options A1 | r1, Options §5, Question A, line 1796 | research, source to check |
| 183 | A1: execution code is not predicted anyway (F9) | Options A1 | line 1796; R1-ABL-F9, r1:1244–1247 | research verified |
| 184 | A1: legacy has none of it (CD12, CD13, GC11 are tables plus code) | Options A1 | line 1799 (A4: "The legacy shape (CD12, CD13, GC11): each ability is code, only tuning is data") | research, source to check |
| 185 | A1: builds generic-engine features "for later" (CLAUDE.md: REVISE); over-serves §6.3 | Options A1 | line 1796 | research, source to check |
| 186 | A2: Medium; one generic interpreter over plain data; every new operation a reviewed Rust change; bounded by data size, no script VM, testable for zero allocation | Options A2 | line 1797 | research, source to check |
| 187 | A2: held to 1 ms p99 (AB-10) | Options A2 | R1-ABL-REC6, r1:2465–2472 | research verified |
| 188 | A2: content cannot add code paths; definitions validated at load | Options A2 | R1-ABL-REC1, r1:2346–2347; R1-ABL-REC5, r1:2452–2453 | research verified |
| 189 | A2: a number change needs no code (mark 4) | Options A2 | R1-ABL-REC1 mark 4, r1:2377–2378 | research verified |
| 190 | A2: the legacy tables convert; each operation traces to a legacy or LOOP_V2 test case | Options A2 | R1-ABL-REC5 mark, r1:2460–2461; line 1797 | research verified |
| 191 | A2 pros and cons (serves §6.3–§6.7; a needed mechanic may lack an operation, then a reviewed Rust change; knockback is one) | Options A2 | line 1797; r4:750–751 | research, source to check |
| 192 | A3: a second language and runtime in both builds; performance not researched | Options A3 | line 1798 | research, source to check |
| 193 | A3: determinism and Wasm support not researched (R1-ABL-OQ9); the options table's "(OQ10)" points to the effects-as-entities question | Options A3 | line 1798 says "(OQ10)"; R1-ABL-OQ9 is line 2781, R1-ABL-OQ10 line 2783 (a reference slip, stated, not edited: plan rule 8) | research, source to check |
| 194 | A3: no legacy precedent (legacy abilities are code, GC11) | Options A3 | line 1799; INV GC11 line 250 | research, source to check |
| 195 | A3: Overwatch's Statescript a shipped example, the talk not read; adds a runtime no §6 item asks for | Options A3 | line 1798; R1-ABL-OQ2, r1:2759–2762 | research, source to check |
| 196 | A4: low at first, grows per ability; best performance; combos and counters need code per pairing (content waits for programmers); the legacy shape; meets "tuning without code changes", not "built from data" | Options A4 | line 1799 | research, source to check |
| 197 | A5: medium plus glue; not measured; Bevy-bound, one slice each (F23–F25); ties to the ECS choice; none documents fixed ticks, determinism or prediction (inferred) | Options A5 | line 1800 | research, source to check |
| 198 | A5: leafwing's cooldowns tick by a `Duration` from `f32` seconds | Options A5 | R1-ABL-F23, r1:1332–1341 | research verified |
| 199 | Cells marked "not researched" or "not measured": the research gives no assessment there | Options | lines 1796–1800, 1803–1808, 1827–1832 | research, source to check |
| 200 | B1: one more mechanism to learn; keeps the per-shot cost low | Options B1 | Options §5, Question B, lines 1803–1805 | research, source to check |
| 201 | B1: the authority decides ammo, cooldowns and reloads (GC4's mark "the server must decide them") | Options B1 | INV GC4, line 243 | research, source to check |
| 202 | B1: avoids Paragon's lower rate of fire at high latency (F17); Fortnite's and Quake III's shape (F17, F22) | Options B1 | R1-ABL-F17, F22 | research verified |
| 203 | B1: legacy's player weapon state is already a state machine (switching, swap and fire cooldowns, timed reloads, spread) | Options B1 | INV GC4, line 243 | research, source to check |
| 204 | B2: one mechanism; an effect per shot at up to 13.9 shots per second per soldier (cost inferred); the predicted-cooldown problem F17 points at | Options B2 | lines 1806–1808 | research, source to check |
| 205 | 13.9 shots per second | Options B2 | 1 ÷ 0.072 s = 13.9; R1-ABL-N4 confirmed, r1:3485 | research verified |
| 206 | E1: ids resolved to dense integer handles at load; an unknown id a load error; no silent fallback (CC9); matches the legacy tables | Options E1 | Options §5, Question E, lines 1827–1829 | research, source to check |
| 207 | E1 pro: shared content written once (the three T1 classes serve every faction) | Options E1 | LOOP_V2 §5, lines 67–68; line 1831 | design doc |
| 208 | E1 con: harder to read than E2, which the research calls "simpler to read" | Options E1 | line 1830 | research, source to check |
| 209 | E2: simpler to read; duplicates shared content; blocks reuse across factions (§6.7) | Options E2 | lines 1830–1832 | research, source to check |
| 210 | A2, B1 and E1 are the smallest model that writes every legacy and LOOP_V2 ability as data and keeps counters in data | Trade-offs | R1-ABL-REC1 reason, r1:2359–2361; AB-6, AB-7 | research verified |
| 211 | A4 cheaper today but every combo is code; A1 and A3 add machinery no §6 item asks for; A5 ties abilities to Bevy and float time | Trade-offs | lines 1796–1800 | research, source to check |
| 212 | B1 keeps Paragon's latency problem out of the weapons | Trade-offs | R1-ABL-F17 | research verified |
| 213 | E1 keeps one definition, and so one track id, for a class every faction shares | Trade-offs | LOOP_V2 §5 lines 67–68; UPGRADE-SCOPE bullet 1 (a track per unit type) | design doc |
| 214 | The strongest argument against: one community write-up (R1-ABL-OQ1); Overwatch's scripted abilities not read (R1-ABL-OQ2) | Trade-offs | r1:2756–2762 | research, source to check |
| 215 | A closed set means a new mechanic without an operation waits for a reviewed Rust change | Trade-offs | line 1797 | research, source to check |
| 216 | The spike's marks 1 and 2 test exactly this | Trade-offs | R1-ABL-REC1 marks 1–2, r1:2365–2371 | research verified |
| 217 | Tracks that survive death let power grow (pillar 2); levelling and catch-up decide the way back (pillar 1) | Trade-offs | VISION_V2 §2, lines 26–30; UPGRADE-SCOPE bullet 2 | design doc |
| 218 | No track numbers exist, so neither side can be measured yet | Trade-offs | UPGRADE-SCOPE "Still open" line | settled |
| 219 | A mode can carry its own values: a room is a mode id plus allowed overrides | Trade-offs | R2-ROOM-REC7, r2:2857–2867 | research verified |
| 220 | Balance is checked per mode, separately; "Two-team and three-team battles are both tested" | Trade-offs | VISION_V2 §3, lines 45–46 | design doc |

## Open questions
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 221 | AB1 to AB11 have no pick: Mustafa's words "list them as open questions, don't decide them" | OQs | UPGRADE-SCOPE "Still open" line; advisor-verdicts.md §1, condition 11 | settled |
| 222 | AB1: each held gun's picks apply | OQ-AB1 | UPGRADE-SCOPE bullet 5; AB-8 | settled |
| 223 | AB1 options (a)–(c) and trade-offs (class identity in the triangle; tracks acting at once; more active effects; top-down readability) | OQ-AB1 | LOOP_V2 §5, lines 67–68; AB-10; game-designer's reading | design doc |
| 224 | AB1: the Javelin comes with rockets and a Luger sidearm | OQ-AB1 | LOOP_V2 §5, lines 71–72 | design doc |
| 225 | AB1: playtests with bots, sims 3 runs per build | OQ-AB1 | CLAUDE.md, change-set protocol 4 ("gameplay / AI → `tools/sim.js` ×3 runs") | settled |
| 226 | AB1, AB2: the research has no finding on loot | OQ-AB1 | row 26 (no topic file mentions drops or loot of weapons as v2 rules; R2-SYNC-F17 is Quake III item pickups) | arithmetic |
| 227 | AB2: LOOP_V2 does not say how class weapons refill | OQ-AB2 | `grep -n 'refill\|resupply\|ammo' docs/LOOP_V2.md`: 0 hits | arithmetic |
| 228 | AB2: in legacy, normal guns have unlimited reserve by a quirk | OQ-AB2 | INV GC4, line 243 | research, source to check |
| 229 | AB2 options (a)–(c) and trade-offs | OQ-AB2 | game-designer's reading of LOOP_V2 §5 (class identity) | design doc |
| 230 | AB3: sets the snowball (VISION_V2 §2 pillars 1 and 2 `[OPEN]`); DESIGN_NOTES open question 2 quoted | OQ-AB3 | VISION_V2 §2 lines 26–30; DN line 52 | design doc |
| 231 | AB3 options (a)–(d) and (i)–(iii) with trade-offs | OQ-AB3 | game-designer's reading of VISION_V2 §2 pillars 1–4 (lines 26–34), LOOP_V2 §4 (uplinks), §5 (triangle) | design doc |
| 232 | AB3: both sides measured once numbers exist (time to kill both ways, per matchup; catch-up time), sims 3 runs per build, two-team and three-team separately, plus telemetry | OQ-AB3 | proposed method; reason: VISION_V2 §2 asks for both sides, §3 for both mode kinds; CLAUDE.md sim rule | proposed mark |
| 233 | AB3: nothing can be computed now | OQ-AB3 | UPGRADE-SCOPE "Still open" | settled |
| 234 | AB3: the snowball design session VISION_V2 §2 and §7 call for | OQ-AB3 | VISION_V2 §2 lines 29–30; §7 lines 84–85 | design doc |
| 235 | AB3: the match record is ADR 0006's (R6-TEL-REC1) | OQ-AB3 | map ids table | settled |
| 236 | AB4: UPGRADE-SCOPE ties progress to "the weapon type" | OQ-AB4 | UPGRADE-SCOPE bullet 5 | settled |
| 237 | AB4: the same Luger for the Recruit and as the Javelin's sidearm | OQ-AB4 | LOOP_V2 §5, lines 69–72 | design doc |
| 238 | AB4: abilities hand out weapons (Katana; Molotov thrown) | OQ-AB4 | R1-ABL-REC2, r1:2385–2386 | research verified |
| 239 | AB4: the Juggernaut's mounted cannon | OQ-AB4 | INV GC17 line 256; CD14 line 95; r1:1372 | research, source to check |
| 240 | AB4: LOOP_V2 §8 plans "sidegrade variants" | OQ-AB4 | LOOP_V2 §8, line 105 | design doc |
| 241 | AB4: seats `[OPEN]`; R2-SYNC-OQ7 a design question | OQ-AB4 | VISION_V2 §6.4 line 73; map ids table (R2-SYNC-OQ7: DESIGN) | design doc |
| 242 | AB4 options (a)–(c) and trade-offs; the data carries all three (each definition names its track, or none) | OQ-AB4 | AB-4; game-designer's reading | design doc |
| 243 | AB5 (WEAPON-UNLOCKS, the map's code): LOOP_V2 §8 third gate quoted; VISION_V2 §1 quoted; UPGRADE-SCOPE quoted | OQ-AB5 | LOOP_V2 §8 line 109; VISION_V2 §1 line 22; UPGRADE-SCOPE bullets 3–4; map J5 (the code) | design doc |
| 244 | AB5 options (a)–(d) and trade-offs (pillar 7) | OQ-AB5 | LOOP_V2 §8 lines 105–110; VISION_V2 §2 line 39; game-designer's reading | design doc |
| 245 | AB5: the catalogue `[LATER]`; accounts `[LATER]` | OQ-AB5 | LOOP_V2 §8 line 113; VISION_V2 §6.10 line 81 | design doc |
| 246 | AB5: ADR 0006 cites this | OQ-AB5 | map J5 ("0006 cites it") | settled |
| 247 | AB6: LOOP_V2 §12 and DESIGN_NOTES call the team's tiers the answer to in-match progression ("Momentum") | OQ-AB6 | LOOP_V2 §12, lines 164–165; DN lines 19–20 | design doc |
| 248 | AB6 options (a)–(c) and trade-offs (pillars 1 and 4) | OQ-AB6 | VISION_V2 §2 lines 26, 33–34; AB-5; game-designer's reading | design doc |
| 249 | AB7: "sophistication, not damage" (decided); "give breadth, never raw power" (decided); the Advisor's not-decided "toward sidegrades over raw power" | OQ-AB7 | DN lines 27–28, 40–41; LOOP_V2 §8 line 105 | design doc |
| 250 | AB7: none of these lines names in-match picks | OQ-AB7 | DN lines 27–28 (tiers), 40–41 (a deck, suggestion); LOOP_V2 §8 line 105 (unlocks) | design doc |
| 251 | AB7 options and trade-offs; LOOP_V2 §8 every counter role from level 1 | OQ-AB7 | LOOP_V2 §8 lines 111–112; AB-1 (stacking limit) | design doc |
| 252 | AB8: the open offer is state on the record (ADR 0004); the answer a command the authority checks (ADR 0001); ADR 0003 draws it | OQ-AB8 | map J5 | settled |
| 253 | AB8 options; everyone spawns in front of the buy terminal | OQ-AB8 | LOOP_V2 §2 line 22; game-designer's reading | design doc |
| 254 | AB9: ADR 0004's pickups read these values from data | OQ-AB9 | map J5 | settled |
| 255 | AB9 options; his words name "killed enemies' drops" only | OQ-AB9 | UPGRADE-SCOPE bullet 4 | settled |
| 256 | AB9: Quake III predicts item pickups (R2-SYNC-F17); whether a pickup is predicted is ADR 0001's | OQ-AB9 | r2, Findings, R2-SYNC-F17, lines 230–236 (checked: yes, findings-checked.tsv); map seam (prediction scope 0001) | research verified |
| 257 | AB10: bots fill empty slots `[DECIDED]` | OQ-AB10 | LOOP_V2 §2 lines 19–20; VISION_V2 §3 line 49 | design doc |
| 258 | AB10: a joining human takes a bot's slot (R2-ROOM-REC6) | OQ-AB10 | r2:2842–2846 | research verified |
| 259 | AB10 options and trade-offs (three-team modes with uneven bot counts); how a bot picks is bot behaviour (ADR 0004) | OQ-AB10 | VISION_V2 §3; map part a, 0004 item 7 (bot behaviour) | design doc |
| 260 | AB11: the triangle `[DEFAULT]`; the legacy numbers of AB-7 (11 of 16 pellets; 63 vs 19; 2999.7 vs 150) | OQ-AB11 | rows 131–134 | arithmetic |
| 261 | AB11 options (a)–(c); the data carries all three | OQ-AB11 | R1-ABL-REC1, r1:2350–2352 (frontal-arc and target-tag stages) | research verified |
| 262 | AB11: sims at Tech 0 and Tech 1 (3 or more runs, as CD13 asks), then the Feel Lab | OQ-AB11 | INV CD13 line 94 | research, source to check |
| 263 | AB11 research ids: R1-ABL-OQ6 (a design question); R2-SYNC-OQ9 (ADR 0001's) | OQ-AB11 | map ids table (R1-ABL-OQ6: DESIGN; R2-SYNC-OQ9: 0001) | settled |
| 264 | AB12: every GAS concept from one community write-up | OQ-AB12 | R1-ABL-OQ1, r1:2756–2758 | research, source to check |
| 265 | AB12 option (b): SRC-ACCESS (a), "official-docs domains such as Epic's can be added in the next kit PR" | OQ-AB12 | STATUS.md lines 167–171 | settled |
| 266 | AB12 proposed (a): the subset is defined by our own test cases and proven by the spike | OQ-AB12 | reason: R1-ABL-REC1 marks 1–4 test our own content, r1:2364–2378 | proposed mark |
| 267 | AB13: the Overwatch talk (GDC 2017) not read | OQ-AB13 | R1-ABL-OQ2, r1:2759–2762 | research, source to check |
| 268 | AB13 proposed (a): revisit only if mark 1 or 2 fails or new operations are needed often | OQ-AB13 | reason: marks 1–2 test the closed set, r1:2365–2371; R1-ABL-OQ9, r1:2781–2782 | proposed mark |

## What a spike must prove
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 269 | The content and ability spike is a working name; `docs/SPIKE_PLAN.md` names and places the spikes | Spike | plan.md rule 5; map part g | settled |
| 270 | Item 1: R1-ABL-REC1's four marks as written | Spike 1 | r1:2364–2378 | research verified |
| 271 | Item 2: R1-ABL-REC2's mark (multipliers; Shabzon falloff at 10, 15, 25, 35, 50 m; pellets, magazine, reload, burst; integers exactly, floats within 1e-6 relative, proposed in the research; one weapon model for bots and humans) | Spike 2 | r1:2396–2400 | research verified |
| 272 | Item 3: R1-ABL-REC5's mark (every legacy number present once; load errors name file and field; 10,000 fuzzed definitions never panic) | Spike 3 | r1:2460–2463 | research verified |
| 273 | Item 3 proposed: fuzzing and load checks include track definitions | Spike 3 | reason: AB-4 adds the kind | proposed mark |
| 274 | Item 4 proposed: an ability press before Tech 1 and a T2 purchase before Tech 2 refused in 100 % of 1,000 scripted attempts, accepted after | Spike 4 | reason: LOOP_V2 §4 `[DECIDED]`, legacy checks no tier (GE4); 1,000 is R1-ABL-REC4's count, r1:2444 | proposed mark |
| 275 | Item 5: the Warrior kills a Conscript in 0.86 s within one tick (R1-ABL-REC3's mark, ADR 0004's) | Spike 5 | r1:2416–2418; map ids table (REC3 owner 0004) | research verified |
| 276 | Item 5 proposed golden cases: Peacekeeper 0.85 s with 11 of 16 pellets; 63 SMG body hits behind the Shield; knockback with no Javelin-specific code | Spike 5 | reason: the numbers the counter check uses (rows 132–133); AB-6 | proposed mark |
| 277 | Item 6 proposed u1–u6 (no track-specific code; death; switching; a lost gun; reset; offers and forged picks; byte-identical offer logs on two runs; native and Wasm where the client holds the stream), counts of 1,000 | Spike 6 | reason: UPGRADE-SCOPE, no research mark; 1,000 is R1-ABL-REC4's count (r1:2444); the byte-identical log follows R1-ABL-REC3's mark (r1:2418–2419) | proposed mark |
| 278 | Item 7: R1-ABL-REC6's mark (ADR 0004's), measured with the method ADR 0002 sets (R1-XT-OQ15) | Spike 7 | r1:2465–2472; R1-XT-OQ15, r1:3097–3100 ("Decided by: ADR-0002") | research verified |
| 279 | Item 8: no spike before acceptance; the Feel Lab test compares predicted and server-confirmed Molotov and Shield once abilities are playable; Mustafa judges the feel | Spike 8 | ABL-PREDICT, r1:2967–2980; R2-SYNC-OQ10, r2:2936–2939; map J6 (8), LE-N (after abilities exist) | settled |

## Appendix A and For the frame
| # | The claim, as written (short) | Section | Citation (file, id, lines) | Tier |
|---|---|---|---|---|
| 280 | R1-ABL-REC5 changed because UPGRADE-SCOPE came after the research | Appendix | R1 verified 2026-10-28 (r1:3233–3236); UPGRADE-SCOPE 2026-10-31 | settled |
| 281 | R1-ABL-OQ3: the cue-once rule is tested where a replay happens: own weapon cues in the netcode proof test (the prediction scope covers own weapon state and cosmetic shot feedback), own ability cues in the Feel Lab test if ABL-PREDICT says yes; R1-ABL-REC4's mark (1,000 presses) is ADR 0001's | Appendix | R1-ABL-OQ3, r1:2763–2765; R1-ABL-REC4, r1:2427–2428, 2444; GL "The prediction scope", lines 502–504; GL §8 "The netcode proof test" (map J1) | research verified |
| 282 | R1-ABL-OQ9 matters only if A3 returns | Appendix | r1:2781–2782 | research, source to check |
| 283 | The decision in three lines | Frame | AB-1 to AB-8 (rows 41–166) | research verified |
| 284 | README trace list: track kind and drop/pickup values (UPGRADE-SCOPE); active reload fields (LOOP_V2 §7 only); the AB-10 curve (VISION_V2 §3 and §4 only) | Frame | map J3 (the trace rule and its reading) | settled |
| 285 | Consequences: easier, harder, revisit-when lines | Frame | rows 42, 43, 110, 128, 146, 177, 214–216 | research verified |
| 286 | Hard constraints: none because of this section; tracks need no persistence; cues stay presentation only, nothing animated decides a hit | Frame | UPGRADE-SCOPE bullet 2; R1-ABL-REC1, r1:2345 ("cues as presentation events only"); CLAUDE.md hard constraint "Animation is cosmetic" | research verified |
| 287 | Legacy rows: no mark changes; CD8, CD12, CD13, CD14, GC4, GC5, GC7, GC11, GC14, GC15, GC17 rewrite in Rust; CS3 drop; CC9 port to TypeScript; GE13 redesign; GE17 drop | Frame | INV mark column of lines 89, 93–95, 243–244, 246, 250, 253–254, 256 (rewrite in Rust); 171 (drop); 138 (port to TypeScript); 299 (redesign); 303 (drop) | research, source to check |
| 288 | The "look brief pending" notes on GC11, GC14, GC15, CC9 and CC12 concern presentation, ADR 0003's | Frame | INV lines 250, 253, 254, 138, 141 (mark column); map part f, LE-H (0003 takes presentation rows) | research, source to check |
| 289 | Migration places in SLICE-ORDER (slice 1 weapons as data; slice 2 T1 classes; slice 4 tiers, abilities, tracks after AB1–AB3; slice 6 REC6 with real content) | Frame | SLICE-ORDER, STATUS.md lines 94–95 (the order); the places are proposals | proposed mark |
| 290 | Test plan: CI goldens; R1-ABL-REC3's 120 s, 3-seed, native and Wasm log in ADR 0004's suite; sims 3 runs per build in both mode kinds; telemetry | Frame | r1:2418–2419; R1-ABL-REC5 mark r1:2462–2463; CLAUDE.md sim rule; VISION_V2 §3 | research verified |
| 291 | Rollback: a failing mechanic becomes a reviewed operation; many failures fall back to A4 with E1 kept and reopen A3; if u1 fails, tracks get their own operations | Frame | options A2, r1:1797 (the Rust-change route); A4, r1:1799 | research, source to check |
| 292 | Seams: pickups are a new entity kind, which R1-ECS-OQ7 asks ADR 0004 to weigh | Frame | R1-ECS-OQ7, r1:2633–2636; map ids table (owner 0004) | research, source to check |
| 293 | Seams: whether pickups are predicted (R2-SYNC-F17) | Frame | r2:230–236 | research verified |
| 294 | Seams: a knocked-back local player's predicted movement is corrected (ADR 0001) | Frame | GL "The prediction scope", lines 502–504 (own movement replayed from the authoritative state on every snapshot); r4:750–751; an inference | research verified |
| 295 | The map gave this section seven ids, and all belong here | Frame | engine-architect-decision-map-ids.tsv (owner 0005: R1-ABL-REC1, REC2, REC5, OQ1, OQ2, OQ3, OQ9) | settled |
