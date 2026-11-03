# VISION V2: north star and targets

Written by Mustafa and the Advisor, 17 October 2026: step 1 of the v2 planning. It guides every trade-off in the
rebuild. Status tags as in LOOP_V2.md: `[DECIDED]` Mustafa decided it · `[DEFAULT]` stands unless vetoed ·
`[OPEN]` needs a decision · `[LATER]` not now.

## 1. North star
**Matches that turn into stories players retell:** a last-second steal, a saved-up rush that flips the game, a combo
nobody saw coming. All of it inside factions people feel loyal to.

What players should say after a match, in Mustafa's words:

| What a player says | Pillar |
|---|---|
| "That steal in the last ten seconds was insane." | Thrilling swings |
| "Wow, the tank rush from that team was crazy. They saved up and turned the whole game around at the end." | Thrilling swings |
| "We really dominated that team last game." | Power that feels earned |
| "The combo of that unit with that vehicle is actually pretty crazy." | Depth through combos and counters |
| "Good that you went for the tech box, but we can't do much without harvester money. Can we focus on that too?" | Every choice costs something |
| "Faction X for the win!" | Faction pride |
| "I had a great angle on that map: I sat in a building and kept harassing them." | Places worth fighting over |
| "Finally unlocked that weapon. Can't wait to use it against them." | Progression worth chasing |
| "Holy moly, what even is that?" (a new player meeting a Juggernaut or a heavy tank) | Spectacle |

## 2. The pillars
1. **Thrilling swings.** Big objectives can flip late: tech box steals, saved-up rushes. A lead is never fully safe.
2. **Power that feels earned.** Winning teams feel strong and can see it (tiers, vehicles): a healthy snowball that
   players want to come back for.
   - `[OPEN]` Pillars 1 and 2 pull against each other on purpose. Leaders must feel dominant *and* trailing teams
     must keep a real way back. The mechanics need their own design session.
3. **Depth through combos and counters.** Units, vehicles and abilities interact. Experimenting is rewarded, and every
   strong thing has an answer.
4. **Every choice costs something.** Money (harvesters), tech (the tech box) and fighting all compete for the same
   players. Teams must split up and talk.
5. **Faction pride.** Each faction has its own identity, look, units and character, so players pick a side and stay.
   - `[OPEN]` How much is faction-specific? LOOP_V2 says "infantry shared, tanks per faction". Faction pride pulls
     toward more unique units. Needs a design session.
6. **Places worth fighting over.** Verticality, buildings you can enter, angles, and maps that suit different playstyles.
7. **Progression worth chasing.** Unlocks players look forward to: the WARDOGS-style three gates (LOOP_V2 §8).
8. **Spectacle.** Big, readable, awe-inspiring units that new players remember.

## 3. Modes and scale
- `[DECIDED]` **Start small:** two teams with 2–8 players, and three teams with 6–18 players (in total: 1v1 to 4v4,
  and 2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31).
- `[DECIDED]` **The goal:** 12v12v12 or 24v24. Two-team and three-team battles are both tested, and maps are designed
  per mode.
- `[DECIDED]` **Faction choice for organized play:** a clan or guild can deliberately fight another faction in a
  private match.
- `[DEFAULT]` Team count, team size, the faction of each team and the mode rules are all data. Bots fill empty slots.

## 4. Platforms and performance
- `[DECIDED]` Desktop browsers first: Chrome, Edge, Firefox and Safari. Mobile comes later but is not designed out.
- `[DECIDED]` **Minimum:** a laptop with integrated graphics runs the largest mode at 30+ fps on Low.
- `[DECIDED]` **Recommended:** a mid-range graphics card runs it at 60 fps on High.
- `[DEFAULT]` Matches feel fair up to about 100 ms of ping. Servers start in one EU region.

## 5. Accounts and business
- `[DECIDED]` Guest play first; a simple login arrives with account progression (Service points).
- `[DECIDED]` `[LATER]` Cosmetics and account progression get designed when their time comes.
- `[DECIDED]` Never paid loot boxes, and team colours never change (DESIGN_NOTES).

## 6. What this asks of the engine
This is the input for the architecture decisions:
1. **Modes as data:** 2 or 3 teams of any size, one faction per team, plus rooms and private matches.
2. **A true 3D simulation:**
   - floors and building interiors, ledges, and line of sight across heights;
   - bots that move between levels;
   - aiming at height.
3. **A data-driven ability and effect system,** like UE's Gameplay Ability System. Units, vehicles, weapons and
   abilities are built from data, so combos and counters can emerge and tuning needs no code changes.
4. **Vehicles as first-class:**
   - big and multi-part, with their own weapons and abilities (leech, marking);
   - `[OPEN]` seats for several players.
5. **Economy and objectives:** money, tech, steals, saving up and team-funded actions. All server-side and tunable.
6. **Balance telemetry:** every match records the numbers that matter: who led, the swings, steals, and unit pick and
   win rates. Snowball and counters get tuned on evidence.
7. **A faction content pipeline:** per-faction units, uniforms and variants, as data.
8. **Moments:** the server recognizes the big moments, like a last-second steal or a comeback.
   - `[LATER]` These feed kill-cams, replays and highlights.
9. **A spectacle budget:** the renderer handles a few very large units and their effects without dropping frames.
10. `[LATER]` **Accounts:** server-side ownership of unlocks and cosmetics.

## 7. Open design questions (for later sessions)
- **The healthy snowball:** what makes leaders feel dominant while trailing teams keep a real path back?
  (Pillars 1 and 2.)
- **Faction asymmetry:** shared core infantry plus faction specialists and vehicles, or more? (Pillar 5.)
- **Balance across modes:** two-team vs three-team balance, and map design for each.
- **Clan and guild features.**
