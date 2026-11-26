# Mustafa's answers of 2026-11-26 to SIZE-4B and FEATURE-COMPARE: 4b in two pull requests, two named exceptions with four conditions

_Copied by script from the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq), never retyped: the user message of 2026-11-26T14:09:13.209Z (uuid d736715d-8d41-4f7f-a8fa-f3011d021464), whole (1697 bytes, sha256 903135dd8d54efe6f74a5ad8ffffbc8f31c5da550c8cf18bae58c50ff937d72c). What it answers: the chat's two questions of 2026-11-26 (.checkpoints/cs4b-content/question-2026-11-26-feature-compare.md, with its two dated additions): SIZE-4B (how 4b is cut: one pull request, two, or a moved stop line) and FEATURE-COMPARE (whether the build's check on the simulation's dependencies may take two named exceptions)._

---
From Mustafa, 26 November, 15:05 Amsterdam (written with my Advisor).

1. SIZE-4B: (b), in engine-architect's cut: the collision library goes wholly into 4b-2. Stop lines 7,450 for 4b-1 and 8,450 for 4b-2. Each stands on its own: lines left over in 4b-1 do not move to 4b-2. At a stop line you stop and ask.
2. Your own numbers say each half is likely to reach its line at 4a's rate (about 7,950 and 9,000 written). So before step 1 of each half, name in the checkpoint what leaves first if the count runs high, in order, and tell me in one sentence where you stand at two thirds of the line.
3. 4b-1 starts now: no outside package, no wait for my Export, no browser check.
4. FEATURE-COMPARE: (a). Two named exceptions, each pinned to exact entries and versions, with tests and planted mistakes; any other difference still stops every build. Conditions: a change of parry3d's version or of the two packages' versions needs a new yes from me; the library's std stays off; the built-file check in 4b-2 keeps looking for SIMD and fused multiply-add in the players' file; and the recorded match exercises the collision queries from the first change-set that calls them, so native, Chrome and Firefox are compared on the real thing and not only on 10,000 rays. Recorded as a dated pointer for ADR 0002's next revision.
5. 4b-1's report says in its first lines what moved 4b from 5,410 to about 7,050 lines by the designs' count, and gives a new estimate for the first build I can move a soldier in.
6. My Export at 30 steps comes before the data step (step 8 in the new numbering), which is in 4b-2.
7. Rule unchanged: git and gh always start their own command, with no other program in front of them.
