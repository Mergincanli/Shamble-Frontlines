# Mustafa's game rule VISION-RULE, version 2: a player sees an enemy only on their own soldier's clear line of sight (2026-11-26)

_Copied by script from the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq), never retyped: the message he sent while the chat was working (stored as a queued command) of 2026-11-26T15:51:38.348Z (uuid 809fbcd0-8830-4627-b624-3db516eef947), whole (3990 bytes, sha256 fd315366c33e9a9db9934cab2b02c8b49fb821e6c3e4b758bc52d585f37154c1). What it answers: the open design questions R2-INT-OQ1 and R4-LOS-OQ2 of docs/adr/README.md (what a player may see: the camera's view or the character's sight), and, for sight, part of R4-LOS-OQ1 and R4-LOS-OQ4 (what blocks it). It is his own decision, brought to this chat to record at 4b-1's docs pass; by its own words it replaces his block of 16:35 Amsterdam of the same day (15:35:38 UTC, message dcae7c82-851e-46e8-afe6-b0ee784ab13c), which is therefore not recorded. No code is asked for it and it is not part of Slice 1._

---
Some adjustments to the previous prompt that I had send:
FOR BUILDER V4. From Mustafa, 26 November, 16:50 Amsterdam (written with my Advisor). A game rule to record: VISION-RULE, version 2. If you already have my block of 16:35 on this, this one replaces it. No code for it now, and 4b-1 does not stop for it.

1. VISION-RULE, my decision: a player sees an enemy only when their own soldier has a clear line of sight to that enemy. It ends the third-person camera trick of standing behind a wall and still watching enemies. It answers the open design questions R2-INT-OQ1 and R4-LOS-OQ2 in docs/adr/README.md (what a player may see: the camera's view or the character's sight). My answer: the character's sight.
2. The rule:
   a. Each soldier has a vision cone that starts at the soldier and points where the soldier looks. It is drawn on screen as a white cone.
   b. Its width starts at 190 degrees. That is a bit generous on purpose, like seeing from the corner of your eye, and an enemy must still be able to sneak up behind another player. The final value is tuned later.
   c. An enemy is drawn for me only while some part of their body is inside my cone with nothing solid in between. Otherwise that enemy is not drawn at all. That holds in the open behind my back too.
   d. No maximum reach. As far as the screen shows, I see an enemy if nothing blocks the line.
   e. My own eyes only. What a teammate or squad mate sees is not shown to me.
   f. What blocks sight: walls, buildings, static objects, vehicles, hedges and smoke. Soldiers do not block sight. While I sit inside a vehicle, my own vehicle does not block my view.
   g. An enemy that leaves my sight is gone at once. No fade and no last-seen mark.
   h. Projectiles are always drawn. Bullets and their traces are never filtered or hidden by this rule, whoever fired them and from wherever, also from outside my cone. If someone shoots towards me I must be able to see the traces, even as a last-second glimpse.
   i. Sounds are not hidden either: gunfire and footsteps of a hidden enemy reach me.
   j. The ground, walls and buildings always stay visible. Only enemy players are hidden.
3. Not decided yet, so record it as open: the cone's final width; how the cone is drawn (a look decision); whether fences block sight; whether shots are blocked by the same things as sight.
4. What to do, at 4b-1's docs pass and not before:
   a. Save this message by script and enter it in docs/DECISIONS.md with the question it answers.
   b. In docs/adr/README.md mark R2-INT-OQ1 and R4-LOS-OQ2 answered with a pointer to that entry. For R4-LOS-OQ1 and R4-LOS-OQ4 note what point 2f answers for sight and what stays open.
   c. Rewrite no ADR text now. Add one follow-up line in STATUS for the next edit of ADR 0001 (part 9 (c) and its table Q14: hiding enemies by line of sight is now asked for, so R2-INT-OQ5's cost measurement at 48 players becomes due; part 9 (f): shots and their traces reach every player who can have them on screen, hidden shooter or not), of ADR 0004 part 13 and of ADR 0003's cutaway rule.
5. No code for this rule, and it is not part of Slice 1. It lands where the plans already put it: the sight check with Slice 2's bots, the hiding over the network in Slice 3. I try the cone in Testlines first, and its values reach the game only through my Export, as before.
6. One question, answered in two lines when your current step is done: should anything in part 4b (the map kind, the collision world) already carry a "blocks sight" flag per object, as ADR 0004 part 13 describes, so that this rule needs no rework later? Do not build it without my word.
7. The second guard fix (#73) and the CI Builder's #74 are on main (3b949c1a9f9c99d6d54570a0f2805c3d65661d8b). When your current step is done, bring your chat's main folder up to main by a fast-forward, as your rules allow, so the new guard loads.
8. Rule unchanged: git and gh always start their own command, with no other program in front of them.
