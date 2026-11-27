# His design notes on resource points and repair bots, for the docs (2026-11-27, in the chat of Builder V6)

_Copied by script from the chat's transcript (session_01WLuqnt1v3UNwSqbJsJre7u), never retyped: the user message of 2026-11-27T22:58:25.650Z (uuid 20d8ea82-0a9e-4420-86fc-0e6903b1728c), whole (7117 bytes, sha256 7ad747cb3fbf7c5ec1712ff2909e10a80378eae213984f2371b426bd24fd5253). What it answers: nothing the chat asked: his design direction for the economy and base repair, from his walk of 27 November, with what goes where in the docs (his items 1 to 9). It came after the docs pass of 4b-2 had run, so by his item 6 it goes in with the first docs commit of 4c._

---
Did I paste this earlier btw? I might have forgotten it. 

FOR BUILDER V6. From Mustafa, 27 November, 22:15 Amsterdam (written with my Advisor). Design notes for the docs. No code. It rides with 4b-2's docs pass and must not hold 4b-2.
1. 
WHAT THIS IS: my design direction for the economy and base repair, from my walk of 27 November. The notes under item 7 are mine, word for word. Save this message by script like my other messages. It goes into DECISIONS at your docs pass.
2. 
WHERE IT GOES: docs/LOOP_V2.md gets a new section 14 after section 13, with exactly the text under item 8. docs-keeper places that text and writes no design of its own.
3. 
POINTERS, so the loop doc does not say two things: at each line of LOOP_V2's sections 1, 3, 9 and 11 that speaks of the harvest field in the centre or of the safe field near each base, add the dated pointer "(2026-11-27: resource points replace open fields, see §14)". Keep the old text for the history. The header's "Updated" line gets one sentence that names §14. Change nothing else in the design docs and rewrite no ADR. The register (docs/adr/README.md) gets one dated line that names §14.
4. 
NOTHING IS BUILT FROM THIS in Slice 1. Every number is a placeholder until playtests.
5. 
If a line of mine clashes with something in the docs that item 3 does not cover, do not resolve it. List it in the report under "Decisions needed".
6. 
TIMING: if your docs pass has already run when this arrives, do not reopen 4b-2 for it. Tell me in one line, and it goes in with 4c's first docs commit.
7. 
MY NOTES, word for word:
Design notes from Mustafa, 27 Nov – economy and base repair
This is my design direction, not loose ideas. Numbers are placeholders until playtests. Add them to the game design docs; change no code.
Resource points (decided)
- 
Resource points sit symmetrically around the map. Teams capture them for their harvesters, RTS-style, instead of open ore fields.
- 
Each team starts with a resource silo on its harvester building: a small passive income (about 1 to 2 credits a second, to tune).
- 
A support vehicle drives out and sets up the harvest station on a point. Players drive it and place it themselves.
- 
The first station is free. Each next one costs more the further a team expands, so growing is an investment and snowballing isn't free.
- 
Enemies can destroy a station and build their own on that point, which costs them too.
- 
Points run low but never hit zero: once depleted they still pay about 10 to 20% per run.
Resource points (open, to test)
- 
Depleted points: slowly refill (longer matches) or stay low (shorter matches). A dial for playtests.
- 
If points or objectives move, spawn them in fair sets at equal distance from each base, so no team gets the close one.
- 
Should a station be free again when a team has none left?
- 
Placeholder starting money: about 300.
Repair bots (decided)
- 
Why: in Renegade, a base under siege pulls half the team inside to repair, so they can't fight back.
- 
Built only at a dedicated repair station inside the base: the player holds E for 5 seconds. 300 credits each.
- 
Each bot repairs at half a player's speed. Max 4 per building, so a full set equals 2 players repairing.
- 
They sit against the walls inside buildings. Attackers must push inside to destroy them; they can't be hit from outside.
Repair bots (open)
- 
Should bots pause for a few seconds after a big hit, so a small attack can still crack a base?
- 
Engineer repair values for vehicles and buildings are still to design.
Player experience goal
- 
Tearing down a team's station and putting up your own should feel dominant: bragging rights and banter after the match.
8. 
THE TEXT FOR LOOP_V2, SECTION 14 (place it as it stands):
## 14. Resource points and repair bots
Added 2026-11-27. Mustafa's design direction for the economy and base repair, from his notes of 2026-11-27 (word for
word in DECISIONS.md): "This is my design direction, not loose ideas. Numbers are placeholders until playtests." Where
a line here differs from §1, §3, §9 or §11, this section wins. Nothing here is built in Slice 1.
### Resource points
- 
`[DECIDED]` Resource points sit symmetrically around the map. Teams capture them for their harvesters, RTS-style,
  
instead of open ore fields. (2026-11-27)
- 
`[DECIDED]` Each team starts with a resource silo on its harvester building: a small passive income (about 1 to 2
  
credits a second, to tune). (2026-11-27)
- 
`[DECIDED]` A support vehicle drives out and sets up the harvest station on a point. Players drive it and place it
  
themselves. (2026-11-27)
- 
`[DECIDED]` The first station is free. Each next one costs more the further a team expands, so growing is an
  
investment and snowballing isn't free. (2026-11-27)
- 
`[DECIDED]` Enemies can destroy a station and build their own on that point, which costs them too. (2026-11-27)
- 
`[DECIDED]` Points run low but never hit zero: once depleted they still pay about 10 to 20% per run. (2026-11-27)
- 
`[OPEN]` Depleted points: slowly refill (longer matches) or stay low (shorter matches). A dial for playtests.
- 
`[OPEN]` If points or objectives move, spawn them in fair sets at equal distance from each base, so no team gets the
  
close one.
- 
`[OPEN]` Should a station be free again when a team has none left?
- 
`[OPEN]` Placeholder starting money: about 300. (§3's $300 stays the starting value until a playtest changes it.)
### Repair bots
- 
Why (Mustafa): in Renegade, a base under siege pulls half the team inside to repair, so they can't fight back.
- 
`[DECIDED]` Built only at a dedicated repair station inside the base: the player holds E for 5 seconds. 300 credits
  
each. (2026-11-27)
- 
`[DECIDED]` Each bot repairs at half a player's speed. Max 4 per building, so a full set equals 2 players
  
repairing. (2026-11-27)
- 
`[DECIDED]` They sit against the walls inside buildings. Attackers must push inside to destroy them; they can't be
  
hit from outside. (2026-11-27)
- 
`[OPEN]` Should bots pause for a few seconds after a big hit, so a small attack can still crack a base?
- 
`[OPEN]` Engineer repair values for vehicles and buildings are still to design.
### Player experience goal
- 
Tearing down a team's station and putting up your own should feel dominant: bragging rights and banter after the
  
match. (Mustafa, 2026-11-27)
### Open, noted by the Advisor (not Mustafa's words; his to answer before any of this is built)
- 
`[OPEN]` The silo's income: paid to every player, like a delivery (§3), or one sum for the team?
- 
`[OPEN]` Who pays for a station after the first, and for a repair bot: the player who places it, or the team in
  
shares, like a harvester rebuild (§3)?
- 
`[OPEN]` Does the raised centre (§9) keep a richer point, or are all points equal?
9. 
Rules unchanged: git and gh always start their own command, with no other program in front of them. In a shell command, spell out every file you write with its full path: no variable, substitution or wildcard in the name of a file that is written, moved, copied or deleted, or use the Write and Edit tools.
