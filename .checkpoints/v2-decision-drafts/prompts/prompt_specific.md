### 0001 netcode-architect
Title: "ADR-0001: Multiplayer and netcode". Draft file: netcode-architect-adr0001-draft.md. Research: r2-netcode.md (every
unit), r1-simulation-core.md (the tick unit's rate rule, R1-TICK), r6-infrastructure.md (the servers, as cited), GLOSSARY
§1–§4. Your agent file's ADR list applies in full: requirements (player count, tick, latency), components and data flow,
a state ownership table, the message catalogue, a bandwidth estimate, failure modes, anti-cheat implications, what a
spike must prove, a phased migration plan, a test plan. Must also:
- Set the budgets the netcode proof test is judged against (brief §2): server tick time (the server tick budget and its
  two shares, at 30 Hz, from GLOSSARY §1), bandwidth per client, memory per room; each with its research basis.
- TICK-30 and carry-forward 2: you own the 30 Hz baseline marks (R2-BUD-REC6 restated from GLOSSARY §1's 30 Hz column)
  and the 60 Hz margin mark exactly as the plan's TICK-60-HEADROOM paragraph rules it (the advisor's reading of
  2026-10-31, confirmed by Mustafa and the Advisor at the stop: base = the server tick budget at 60 Hz, 8.33 ms; p99
  server tick time at most 6.25 ms on one vCPU of the reference server in the stress scene at N = 48 with a
  snapshot-receiving client on every soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included; plus every
  60 Hz research mark as written: R2-BUD-REC6's two shares, R1-TICK-REC2 P1–P3; otherwise 30 Hz). List
  TICK-60-HEADROOM as an open question whose decider is Mustafa and the Advisor, "confirm at the stop". State three lines
  as replaced by TICK-30: R1-TICK-REC2's rule (r1-simulation-core.md:2029), "the rule does not change"
  (r2-netcode.md:2415–2416) and "which does not change" (GLOSSARY.md:85). Where you restate r2-netcode.md:2402–2403,
  add "(at N = 54, MODES-3 (b))" (the advisor's PR #28 review, finding 2): at N = 48 the re-flood snapshot tick fits by
  0.05 ms (8.28 of 8.33 ms) while its server-only share is over its own 25 % (4.41 of 4.17 ms) (GLOSSARY.md:62-75).
- SEC-SPREAD (a): the spread is known only to the server; tracers use a similar-looking spread that the hit corrects.
  ABL-PREDICT stays open (interim: the player's own weapons are predicted, damage never, abilities not until the Feel
  Lab test decides; resolution 5).
- SLICE-ORDER's conditions: from slice 1 a client plus a local server (the sim in a Web Worker) talking through the
  network's own messages, no offline-only shortcuts; movement and shooting input-driven and replayable from day one; the
  netcode proof test (brief §2's definition, word for word) runs before the foundation. Name the research spikes it
  subsumes.
- The transport choice the research leaves to a spike (the WebTransport layer on quinn), the WebSocket fallback behind
  one transport interface, rooms and private matches (VISION_V2 §3, §6.1), bots on the server filling empty slots.
- Certificates: one per hostname (R6-HOST-REC4, carry-forward 4); ADR 0006 owns them, you cite it.
- UPGRADE-SCOPE as Mustafa updated it on 2026-10-31 (verbatim in
  /home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md), the netcode
  part as the decision map's addendum assigns it: the server is the authority for tracks, offers, picks, weapon drops
  and pickups; the messages that carry an offer and a pick; how pickups replicate and are interest-managed; the
  anti-cheat side (a pick or a pickup the server did not offer or place is refused). Decide no rule.

### 0002 engine-architect
Title: "ADR-0002: Code structure, build and toolchain". Draft file: engine-architect-adr0002-draft.md. Research:
r5-toolchain-quality.md (LAYOUT, TSLINT, RUSTLINT, TEST, CI, GATE, and PREV's build half), r1-simulation-core.md (the
boundary-worker unit, R1-BND; the data unit where it sets the content crate's build), GLOSSARY §5 for previews.
Must also:
- The foundation, as brief §2 defines it (word for word), and its end condition: an empty scene runs through the full
  stack in the browser, with CI green and a preview link.
- The repository restructure (the legacy game moves to `legacy/` in phase 1 and stays playable by double-click; its hard
  constraints then apply to `legacy/` only; spikes live in `spikes/`, SPIKE-SCOPE (a)).
- The Cargo workspace plus TypeScript with Vite, pinned toolchains, strict TypeScript and lint, clippy and rustfmt, the
  test layers, CI with Rust caching, the gate v2 that replaces today's gate, the preview build (hosting and privacy are
  ADR 0006's), TypeScript types generated from the Rust code, dependency rules (the sim depends on nothing above it).
- The Rust ↔ TypeScript boundary and the client + local server skeleton: the sim in one Web Worker behind the same
  message protocol the network uses (ADR 0001 owns the messages), bulk data in one transferred block per tick, no call
  per entity per frame. The boundary spike's claims (with Wasm size).
- Which CLAUDE.md hard constraints end for v2 code (classic scripts, r128, file://, no external assets: LOOK_V2 §8 ends
  it for v2), once Mustafa accepts the drafts.

### 0003 graphics-engineer
Title: "ADR-0003: Rendering and presentation". Draft file: graphics-engineer-adr0003-draft.md. Second output (a section
for ADR 0005): graphics-engineer-adr0005-assets-section.md. Research: r3-rendering.md (every unit), GLOSSARY §2–§3.
**The look is decided now where LOOK_V2 says [DECIDED]** (brief §1: "The look is docs/LOOK_V2.md"). Your agent file
(written before LOOK_V2 existed) says the look is not decided; for this draft LOOK_V2 is the look, and you still decide
nothing it leaves [OPEN] (the tree style) and change none of its items. Must:
- Show how each LOOK_V2 [DECIDED] item is achieved (LOOK_V2 §9): a trace table, one row per item of §1–§8 tagged
  [DECIDED] or [DEFAULT] (item · the technique or the system that delivers it · the research id, or "not researched:
  open question or spike mark" (plan rule 2) · what the look proof test shows · the draft that owns it if not 0003).
  [DEFAULT] items are built as defaults; the Advisor's additions (§6) are [DEFAULT].
- The look proof test: judged by Mustafa in the browser (Brave on Windows), side by side with screenshots of the legacy
  game (LOOK_V2 §9); also its measurable marks (GPU time per pass, frame time on the reference machines).
- The renderer (WebGPURenderer with TSL, a pinned release) and the WebGL 2 fallback (what it loses); characters
  (skinning, instancing, LOD) for slimmer, serious proportions with detail up close (LOOK_V2 §3, §5); the budgets for
  the minimum (integrated graphics, largest mode, 30+ fps on Low) and recommended (mid-range card, 60 fps on High)
  hardware (VISION_V2 §4); the spectacle budget (VISION_V2 §6.9); how it is measured (never a headless frame rate as a
  hardware result); spatial audio with Web Audio.
- The asset section for ADR 0005 (separate file): the GPU-side rules of LOOK_V2 §8's pipeline, proposing only what R3-CHAR
  and R3-BUDGET support (LODs, the faction colour material slots, what "the engine applies the comic look" means for an
  imported model). ASSET-RESEARCH (a), the advisor's ruling: everything else is an open question marked "not
  researched" (compression formats, the LOD tool, export checks, the material-slot renaming step, the per-unit budget
  numbers), grouped under one research unit, "asset-pipeline research" (graphics-engineer, verified like R3, its own
  daytime session), placed by the spike plan before the asset spike. The look proof test says whether it needs authored
  models; if it does, it uses plain glTF/GLB exports and decides nothing about compression or LODs.

### 0004 rust-reviewer
Title: "ADR-0004: The simulation core". Draft file: rust-reviewer-adr0004-draft.md. Research: r1-simulation-core.md
(the ecs, tick and determinism units, plus its rust-reviewer risks R1-RR-OQ1–7), r4-physics-navigation.md (every unit),
GLOSSARY §1 (the sim step share), §3 (the stress scene), §4 (random streams, the state hash), §7 (the hash-map ban).
Must also:
- ECS or plain data; the fixed tick (integer ticks, durations in ticks; the 30 Hz baseline and the 60 Hz rule are
  ADR 0001's, you own the mechanism and the sim step share); entity identity; snapshot and restore for re-simulation.
- Identical results on every machine (Wasm and native): the float rules, seeded random streams (shared and
  authority-only; SEC-SPREAD (a): the spread stream is authority-only), the state hash per tick, the bans in the sim
  crate, and how a test proves it on every target.
- Physics (the research recommends no physics engine: parry3d plus our own kinematic movement, Rapier's controller the
  measured fallback), navigation (baked into the map document, queried by the sim), bots moving between floors, line of
  sight across heights, obstacles placed mid-match, vehicles avoided, never carved (VISION_V2 §6.2, §6.4).
- LOOK_V2 §4's places as sim needs: buildings with interiors and upper floors, damage stages where debris and planks
  become routes ([DEFAULT]), roofs that fade (a render matter, ADR 0003; the sim's part is only what the camera needs).
- Bots run in the sim on the server (they fill empty slots); moments and telemetry events as the decision map assigns.
- UPGRADE-SCOPE as Mustafa updated it on 2026-10-31 (verbatim in
  /home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md), the sim's
  part as the decision map's addendum assigns it: a player record apart from the soldier entity (the PlayerState, not the
  Pawn) that holds every unit and weapon-type track for the match and survives death and switching; weapon drops from
  killed enemies as world pickup entities; which random stream draws the offers of three (shared or authority-only, with
  the SEC-SPREAD reasoning); the reset at match start. Decide no rule: carry slots, ammo for looted weapons and track
  levelling speed are open questions ADR 0005 lists for Mustafa.

### 0005 game-designer (the ability system section)
Section file: game-designer-adr0005-abilities-section.md (a section of "ADR-0005: Content data, abilities and assets";
engine-architect assembles the record). Research: r1-simulation-core.md (the abilities unit, R1-ABL; the data unit
where tuning is concerned), LOOP_V2 (§4 tiers, §5 units and abilities, §6 vehicles, §7 the feel kit and active
reload), VISION_V2 (§2 pillars 3 and 4, §6.3, §6.4), docs/LEGACY_TUNING.md where a legacy ability is a starting value.
Write the ability and effect system's decision: a reduced GAS subset as data over a closed set of Rust operations (the
research's starting point), how LOOP_V2's abilities and upgrades are expressed in it (Molotov, Riot Shield, Banzai
blade, active reload, the Javelin's knockback, the Hammer's leech, the Spy's disguise and hack: [LATER] items only as
"the data model must not rule them out"), tech-tier gating (LOOP_V2 §4), combos and counters as data (pillar 3), the
per-tick cost mark (R1-ABL-REC6), and **UPGRADE-SCOPE as Mustafa updated it on 2026-10-31** (his words, verbatim, in
/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/inputs/UPGRADE-SCOPE-mustafa-2026-10-31.md; it replaces the
brief's upgrade note and answers the brief's question about switching): every unit and every weapon has its own
roguelike upgrade track (pick one of three per level), owned by the player; tracks survive death and switching (switch
back and the track is still there) and reset at the start of every match; a class always comes with its own weapon (no
weapon choice when buying); other weapons come only from killed enemies' drops, and dropped weapons are world pickups;
progress belongs to the weapon type, not the physical gun (when a gun is lost, its track waits for the next gun of that
type); in Unreal terms the tracks live on the PlayerState, not the Pawn. Show that the ability and effect system, as
data, carries all of that (the track definitions per unit type and weapon type, the offers of three, the picks as
effects on the player's current unit and weapon, the reset at match start); design no upgrade, no track content and no
numbers. List as open questions for Mustafa, each with its options and their trade-offs and **no pick** (the advisor's
condition 11 and Mustafa's own words: "list them as open questions, don't decide them"): carry slots; ammo for looted
weapons; how fast tracks level (including catch-up when switching). Where UPGRADE-SCOPE sits next to another design
line (for example LOOP_V2 §8's third gate, buying an unlocked item each life, or VISION_V2 §1's "Finally unlocked that
weapon"), list the question the same way and resolve nothing. Which random stream draws the offers, and the player
record that holds the tracks, are ADR 0004's (cite it); their messages and authority are ADR 0001's. ABL-PREDICT stays
open. ADR 0005 designs no upgrade, ability, unit or rule: you write how the engine carries Mustafa's rules, never new
rules.

### 0005 engine-architect (content data, the asset pipeline, and the assembly)
Draft file: engine-architect-adr0005-draft.md (the whole record, assembled). Research: r1-simulation-core.md (the data
unit, R1-DATA), r3-rendering.md (R3-CHAR, R3-BUDGET as cited), LOOK_V2 §8, VISION_V2 §6.3, §6.4, §6.7. Write the
content data part (JSON as the one content format through one Rust content crate, validated at load, hot reload in
development only, units, vehicles, weapons, abilities, effects and tuning as data, the faction content pipeline) and
the asset pipeline part (LOOK_V2 §8's flow taken as given: concept sketch → a model in Spline → glTF/GLB → an automated
step that compresses, builds LODs and renames materials into the faction colour slots → the engine applies the comic
look; budgets per unit; the game never runs Spline's runtime; the tree style stays [OPEN]; propose only what R1-DATA,
R3-CHAR and R3-BUDGET support, everything else "not researched" under the asset-pipeline research unit, ASSET-RESEARCH
(a)), plus EDITOR_V2 §2's document rules where they bind content data (the map document). Then assemble, after the two
sections have landed: place game-designer's ability section and graphics-engineer's asset section verbatim as sections
(each headed with its author), and write the record's frame (header, Context, the joined Decision, Consequences, Open
questions numbering, Appendix A) around them (the advisor's condition 11).

### 0006 infra-engineer
Title: "ADR-0006: Infrastructure and operations". Draft file: infra-engineer-adr0006-draft.md. Research:
r6-infrastructure.md (every unit), r5-toolchain-quality.md (PREV's hosting half; CI's hosting and cost where the map
gives it to you), GLOSSARY §5 (previews and hosting) and §6 (the deferred security baseline). Must also:
- The client host and per-PR previews (PREVIEW-WALL (a): previews that reach the staging game server are private from
  the first one; a preview with unreleased art is behind the access wall; PREV-DELETE stays a spike item: does a closed
  PR's preview really disappear).
- EU game servers (one region to start, VISION_V2 §4), one room per process on the reference server, its orderability
  (R6-HOST-OQ1), the shared-vCPU machine measured beside it; certificates: one per hostname (R6-HOST-REC4,
  carry-forward 4; the wildcard is an option the security baseline weighs).
- Costs at the start modes and at the goal (with the dates of the prices, from the research); logs, metrics and crash
  reports; balance telemetry storage and analysis (VISION_V2 §6.6; recorded by the sim as the decision map says).
- The security baseline, listed open with interim positions (BASELINE-WHEN (b), the Advisor, 2026-10-31: "List it in
  ADR 0006 as open, with temporary positions. Research the security baseline before the first online playtest, in its
  own fresh session."): R5's scheduled advisories check stays the mechanism; one certificate per hostname stays; R6's
  wildcard is an option the baseline weighs.
- Never propose that a session creates accounts, buys services or changes repository settings: those are Mustafa's.

### Pass marks (test-engineer, one unit per draft)
Input: the assembled draft docs/adr/000N-*.md (its "What a spike must prove" and its decision), its research, R5's test
and gate units (R5-TEST, R5-GATE), GLOSSARY §1 (time statistics: a mark with no statistic is a p99), §2 (N), §3 (the
stress scene). Output file: test-engineer-passmarks-000N.md with (1) the "### Pass marks" subsection text, whole: a
table `| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise |
basis |`, every number sourced or marked "proposed" with its reason, hardware named precisely (one vCPU of the
reference server; the minimum laptop; the recommended machine; Mustafa's machine in Brave on Windows for anything he
judges), method precise enough to repeat (scene, duration, warm-up, statistic, tool), the run count and how noise is
judged, and the pass/fail rule; (2) the draft's spike entries for docs/SPIKE_PLAN.md: spike name · what it proves ·
its marks · dependencies · rough size (S: a day, M: 2–4 days, L: a week or more of session work) · which research spikes
it subsumes.
