# His text for LOOP_V2 section 14, as the paste brought it (item 8 of his design notes of 2026-11-27)

_Copied by script from `mustafa-2026-11-27-design-notes.md`, never retyped: its lines 61 to 128 of 130 (3063 bytes, sha256 2bd82216e21f819b172b816ca256700c173ac98c2f4c0c0384036b293703a945)._

---
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
