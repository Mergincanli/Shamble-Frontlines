# Design notes: decisions, open questions and lessons

Game-design context that the other docs don't hold yet: Mustafa's decisions from the design discussions (Sept–Oct
2026), the questions they leave open, and lessons about the look. **Decisions here are Mustafa's.** Suggestions are
marked as the Advisor's and are NOT decisions. When a question is settled, docs-keeper moves it to "Decided".

## Progression: the highest-priority open design question
Gun tiers T1→T4 stay on hold until progression is settled: whether power is gated in the match (era) or by the
account changes what a tier even is.

### The framework: three timescales, and only one may touch raw power
- **Identity:** faction and class chosen at the Armory terminal, era-gated, with soft counters. Rock-paper-scissors
  lives in the selection layer; aim and positioning decide the actual gunfight.
- **Momentum:** progression within a match, fed by kills and objectives (the current idea: roguelike "pick one of
  three" upgrades). Resets every match.
- **Mastery:** account XP unlocks breadth (more classes, more options), never raw power.

### Decided by Mustafa
- **Death tax.** Momentum persists until the end of the match: you keep your money. Dying costs the unit you bought:
  you respawn as a Conscript and must buy again. Buying and dying with higher-tier units should have a consequence.
- **Tiers are sophistication, not damage.** Higher tiers feel noticeably but subtly stronger, mainly by unlocking more
  sophisticated units with new capabilities, not through bigger damage numbers.
- **The Spy**, a hybrid of the TF2 Spy and the Red Alert 3 Allied Spy: looks like the enemy, can backstab, and can
  reach the enemy terminal to steal their tech points, so a team can lose Tech. This creates mid-game twists and gives
  teams a reason to contest the Tech Box objectives.
- **Multiplayer scale:** up to 16v16, or 12v12v12 in squads of 4 (one team per faction).

### Advisor suggestions (not decided)
- A comeback valve from day one: extra bounty for killing an expensive unit, and a CS/MOBA-style economic catch-up.
- "Keep the roll, lose the body": Momentum belongs to the player, the unit belongs to the life.
- Start Momentum as a contained, earned burst (Overwatch's ultimate, Titanfall's Titan meter) before any card deck;
  if a deck comes later, bias it toward sidegrades over raw power.
- Keep respawns short and re-buying quick, so death costs tempo and investment, not a long bench as well.
- Test every new unit with "a new verb, or just a bigger number?". Keep tier stat gaps small (around 10–25 %) and
  balance sophisticated units through cost and fragility.
- Spy: make tech theft a contestable channel that alerts defenders; start with denial or partial transfer; scope the
  first disguise tightly with clear counterplay; the disguise can re-tint the uniform through `applyFaction`; its full
  payoff arrives with multiplayer (bots make poor spy-checkers).

### Open questions (decide before building the content)
1. How Momentum is earned, and what the first small upgrade pool looks like.
2. Snowball control within a match, given that Momentum survives death.
3. Spy rules: steal or deny; what breaks the disguise; cloak or not; how lethal the backstab is.
4. Three-team matches: a three-way map (today's map mirrors two bases), win conditions when a team falls, and
   counters to two teams ganging up on one.
5. Squads of 4: squad spawning, roles, pings, and how squads read from the overhead camera.

## The look: lessons learned
- Visual fidelity comes from relief (geometry that catches light), painted light and value contrast, not texture
  noise. An earlier "add detail" pass looked worse.
- Real-time global illumination is off the table. Palette warmth, rounded forms, fake ambient occlusion, soft contact
  shadows and painterly materials are reachable. Tiny Glade is an aspirational reference (open, nothing locked).
- Cozy comes from prop and set-dressing density, not lighting tech: that's the cozy-village pass. Known artifact: the
  brick-and-flint farmhouse reads as candy stripes.
- Soviet primary colour: the build and docs use 0xD41414; the original target was 0xE11020. Confirm 0xD41414 is final.

## Reference games from the progression discussion
Deadlock (in-match growth inside a base-assault shooter) · League of Legends / Dota 2 (comeback economy) ·
Overwatch (contained burst) · Titanfall 2 (per-life escalation) · Counter-Strike / Valorant (loss bonus) ·
Team Fortress 2 (no vertical progression; the Spy) · Halo (contestable power on the map) · Renegade X (take its
economy shape, avoid its raw-damage tiers). See docs/reference/.

## Platform note
Shift+right-click opens Brave's native context menu during play. The browser never fires that event to the page,
so no game code can suppress it.
