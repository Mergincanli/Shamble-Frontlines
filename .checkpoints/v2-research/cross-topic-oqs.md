# Cross-topic open questions from the consistency critic (step 3a)

- **Task:** step 3a of `plan-amendments-2.md`, part 3: the critic's other 29 items (13 hidden dependencies, 15 gaps,
  1 wrong cross-reference), as cross-topic open questions for step 3c. Author: engine-architect, 2026-10-30.
- **Source:** `engine-architect-consistency-result.json` (the critic's list) and its notes
  `engine-architect-consistency.md`. The question texts below are the critic's words ("why" and "decided by"); only the
  one-line titles are new. Nothing here is a new claim, and no topic file is edited in this step.
- **Outcome:** 28 items become shared cross-topic open questions; **none is settled by a resolution** (none of the 12
  resolutions answers one; where a resolution touches the same text, a note says so); rank 41 is a nit fix in R5, not a
  question. One item carries a decision code: rank 18, **BASELINE-WHEN** (the Advisor).
- **Ids:** `R<n>-XT-OQ<k>`, numbered per file in rank order. Per file: R1 18 (XT-OQ1–OQ18), R2 15, R3 8, R4 8, R5 14,
  R6 9; 72 insertions in all.
- **Where they go:** in each topic, a new subsection **"### Cross-topic open questions (consistency critic,
  2026-10-29)"**, placed right after the "Shared open questions" subsection of `resolutions.md` ("Where shared open
  questions go"). Each entry is the quoted text below with that file's id at its head. Line numbers inside the critic's
  side quotes are not repeated here (the critic read R5 before its second pass, so some R5 lines have moved); ids are.

## Ids per file

| Rank | Kind | R1 | R2 | R3 | R4 | R5 | R6 | Code |
|---|---|---|---|---|---|---|---|---|
| 2 | hidden dependency | R1-XT-OQ1 | R2-XT-OQ1 |  | R4-XT-OQ1 |  |  |  |
| 3 | hidden dependency | R1-XT-OQ2 |  |  | R4-XT-OQ2 | R5-XT-OQ1 |  |  |
| 4 | hidden dependency | R1-XT-OQ3 | R2-XT-OQ2 | R3-XT-OQ1 |  | R5-XT-OQ2 |  |  |
| 7 | hidden dependency | R1-XT-OQ4 | R2-XT-OQ3 |  |  |  |  |  |
| 8 | gap |  | R2-XT-OQ4 |  |  | R5-XT-OQ3 | R6-XT-OQ1 |  |
| 12 | gap | R1-XT-OQ5 | R2-XT-OQ5 |  |  |  |  |  |
| 13 | gap | R1-XT-OQ6 | R2-XT-OQ6 |  |  |  |  |  |
| 14 | hidden dependency | R1-XT-OQ7 | R2-XT-OQ7 |  | R4-XT-OQ3 |  |  |  |
| 15 | hidden dependency | R1-XT-OQ8 |  |  | R4-XT-OQ4 | R5-XT-OQ4 | R6-XT-OQ2 |  |
| 16 | hidden dependency | R1-XT-OQ9 | R2-XT-OQ8 | R3-XT-OQ2 |  |  |  |  |
| 18 | hidden dependency |  | R2-XT-OQ9 |  |  | R5-XT-OQ5 | R6-XT-OQ3 | BASELINE-WHEN |
| 20 | gap |  | R2-XT-OQ10 |  |  | R5-XT-OQ6 | R6-XT-OQ4 |  |
| 24 | gap |  |  |  | R4-XT-OQ5 | R5-XT-OQ7 |  |  |
| 25 | gap | R1-XT-OQ10 |  | R3-XT-OQ3 |  | R5-XT-OQ8 |  |  |
| 26 | gap |  |  | R3-XT-OQ4 |  |  | R6-XT-OQ5 |  |
| 27 | hidden dependency | R1-XT-OQ11 | R2-XT-OQ11 |  |  |  |  |  |
| 28 | gap | R1-XT-OQ12 | R2-XT-OQ12 |  |  |  | R6-XT-OQ6 |  |
| 30 | hidden dependency |  | R2-XT-OQ13 |  | R4-XT-OQ6 |  |  |  |
| 31 | gap | R1-XT-OQ13 |  |  |  | R5-XT-OQ9 |  |  |
| 32 | hidden dependency |  | R2-XT-OQ14 |  |  | R5-XT-OQ10 |  |  |
| 33 | gap |  |  | R3-XT-OQ5 |  | R5-XT-OQ11 | R6-XT-OQ7 |  |
| 34 | hidden dependency |  |  |  |  | R5-XT-OQ12 | R6-XT-OQ8 |  |
| 35 | gap | R1-XT-OQ14 |  | R3-XT-OQ6 | R4-XT-OQ7 |  |  |  |
| 36 | gap | R1-XT-OQ15 |  |  |  | R5-XT-OQ13 |  |  |
| 37 | gap | R1-XT-OQ16 |  | R3-XT-OQ7 |  |  |  |  |
| 38 | gap | R1-XT-OQ17 |  |  |  |  | R6-XT-OQ9 |  |
| 39 | gap |  | R2-XT-OQ15 | R3-XT-OQ8 |  |  |  |  |
| 40 | hidden dependency | R1-XT-OQ18 |  |  | R4-XT-OQ8 | R5-XT-OQ14 |  |  |
| 41 | wrong cross-reference | | | | | nit (see below) | | |

## Rank 2 (hidden dependency): Who casts the cursor ray for the aim point, and how does the point cross the boundary?
- **Goes into:** R1 as R1-XT-OQ1; R2 as R2-XT-OQ1; R4 as R4-XT-OQ1.
- **Decision code:** none.
- **Ids:** Side A: R1-BND-REC1; R1-BND-REC3; R1-DET-REC2. Side B: R2-SYNC-REC8; R4-LOS-REC3.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who casts the cursor ray for the aim point, and how does the point cross the boundary?** The aim point
  > must come from a Rust ray query against what the page draws (interpolated positions live on the page). But R1
  > allows the page no Wasm calls and no sim maths in TypeScript, and R1's command carries yaw and pitch, not a point.
  > No topic says which side casts the cursor ray, against which interpolated state, or how the result crosses the
  > boundary. Side A: R1-BND-REC1; R1-BND-REC3; R1-DET-REC2. Side B: R2-SYNC-REC8; R4-LOS-REC3. Existing questions:
  > none. Decided by: ADR-0001 (the command format) and ADR-0004 (the boundary), with R2-SYNC-REC8 mark 1 and
  > R1-BND-REC1 mark 2 as the spike marks. (Consistency critic, 2026-10-29, rank 2.)

## Rank 3 (hidden dependency): Which R1 items still assume Rapier, now that R4 recommends parry3d?
- **Goes into:** R1 as R1-XT-OQ2; R4 as R4-XT-OQ2; R5 as R5-XT-OQ1.
- **Decision code:** none.
- **Ids:** Side A: R1-DET-REC3; R1-ECS-REC4; R1-ECS-REC1; R1-TICK-REC2; R1-RR-OQ1. Side B: R4-PHYS-REC1. Also named:
  R4-PHYS-OQ3.
- **Existing questions (the critic's words):** R4 'Wave-1 items R4 changes or answers' (r4:1110-1129) and R4-PHYS-OQ3,
  partly.
- **Note (not inserted):** Resolutions 1, 9 and 11 change other parts of R1-TICK-REC2 and R1-ECS-REC1 but not their
  Rapier reasons; this question stays as the critic wrote it.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Which R1 items still assume Rapier, now that R4 recommends parry3d?** R4 states openly that some of R1's
  > Rapier items change. It does not list R1-TICK-REC2's reason for starting at 60 Hz (Rapier's default step),
  > R1-ECS-REC1's reason and mark 5, R1's TL;DR, or whether R1-RR-OQ1's SIMD/FMA hazard applies to parry3d's "3D f32
  > SIMD" BVH (R4-PHYS-OQ3 asks only by analogy). ADR-0004's determinism and snapshot sections rest on whichever crate
  > wins. Side A: R1-DET-REC3; R1-ECS-REC4; R1-ECS-REC1; R1-TICK-REC2; R1-RR-OQ1. Side B: R4-PHYS-REC1. Also named:
  > R4-PHYS-OQ3. Existing questions: R4 'Wave-1 items R4 changes or answers' (r4:1110-1129) and R4-PHYS-OQ3, partly.
  > Decided by: ADR-0004 (sim core: physics), with R4-PHYS-REC1 marks 2-5 and R1-DET-REC6's suite as the evidence.
  > (Consistency critic, 2026-10-29, rank 3.)

## Rank 4 (hidden dependency): Where does the remote-entity interpolation buffer live: in the worker or on the page?
- **Goes into:** R1 as R1-XT-OQ3; R2 as R2-XT-OQ2; R3 as R3-XT-OQ1; R5 as R5-XT-OQ2.
- **Decision code:** none.
- **Ids:** Side A: R1-TICK-REC5; R1-BND-REC2; R1-BND-REC6. Side B: R2-SYNC-REC4; R5-TEST-REC1.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Where does the remote-entity interpolation buffer live: in the worker or on the page?** A two-block buffer
  > on the page cannot hold R2's 100 to 167 ms delay. So either the worker (Rust) runs R2's adaptive snapshot buffer
  > and publishes already-delayed remote poses, or the page (TypeScript) keeps a snapshot buffer, as R5's test layer
  > assumes. No topic places R2's interpolation and clock estimate on one side of R1's boundary. Side A: R1-TICK-REC5;
  > R1-BND-REC2; R1-BND-REC6. Side B: R2-SYNC-REC4; R5-TEST-REC1. Existing questions: none. Decided by: ADR-0001
  > (interpolation design) with ADR-0004 (boundary: block contents and the number of blocks kept). (Consistency
  > critic, 2026-10-29, rank 4.)

## Rank 7 (hidden dependency): Does the E3 projectile encoding rest on the cross-target identity R1 says the netcode must not need?
- **Goes into:** R1 as R1-XT-OQ4; R2 as R2-XT-OQ3.
- **Decision code:** none.
- **Ids:** Side A: R1-DET-REC1. Side B: R2-BUD-REC2; R2-SYNC-REC6; R2-BUD-OQ8.
- **Existing questions (the critic's words):** R2-BUD-OQ8 (unanswered by R1).
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Does the E3 projectile encoding rest on the cross-target identity R1 says the netcode must not need?**
  > R2's chosen encoding and its bandwidth figure (146 rather than 188 kbps) rest on client-side flight that matches
  > the server across targets. R1 says the netcode must not depend on that identity, and R1 never answers R2-BUD-OQ8,
  > the question R2 put to it. Side A: R1-DET-REC1. Side B: R2-BUD-REC2; R2-SYNC-REC6; R2-BUD-OQ8. Existing questions:
  > R2-BUD-OQ8 (unanswered by R1). Decided by: ADR-0001 (E3 or E3b) with ADR-0004 (whether cross-target identity is a
  > guarantee for the trajectory function); R2-SYNC-REC6 mark 1 and R2-BUD-REC2's desync count in the spike.
  > (Consistency critic, 2026-10-29, rank 7.)

## Rank 8 (gap): Who hosts, costs and builds the lobby and the room supervisor?
- **Goes into:** R2 as R2-XT-OQ4; R5 as R5-XT-OQ3; R6 as R6-XT-OQ1.
- **Decision code:** none.
- **Ids:** Side A: R2-ROOM-REC2; R2-ROOM-OQ9. Side B: missing answer (R6-HOST-REC2, R6-OBS-REC3, R5-LAYOUT-REC2
  named). Also named: R2-ROOM-REC8, R6-COST-REC1.
- **Existing questions (the critic's words):** R2-ROOM-OQ9 (left to R6, unanswered).
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who hosts, costs and builds the lobby and the room supervisor?** Join tokens, guest identity, room codes
  > and crash reports all go through the lobby, and R2 hands its hosting and store to R6. R6 neither hosts nor costs
  > it, and R5 has no crate for the lobby or the supervisor, so the infrastructure ADR and the cost budget leave out a
  > required service. Side A: R2-ROOM-REC2; R2-ROOM-OQ9. Side B: missing answer (R6-HOST-REC2, R6-OBS-REC3,
  > R5-LAYOUT-REC2 named). Also named: R2-ROOM-REC8, R6-COST-REC1. Existing questions: R2-ROOM-OQ9 (left to R6,
  > unanswered). Decided by: ADR-0006 (hosting and cost of the lobby and its store) with ADR-0002 (crate list) and
  > ADR-0001 (lobby protocol). (Consistency critic, 2026-10-29, rank 8.)

## Rank 12 (gap): Who answers R1's timing questions (sub-tick input timestamps, client tick nudging, shipped tick rates)?
- **Goes into:** R1 as R1-XT-OQ5; R2 as R2-XT-OQ5.
- **Decision code:** none.
- **Ids:** Side A: R1-TICK-OQ3; R1-TICK-OQ4; R1-TICK-OQ8. Side B: missing answer (R2-BUD-OQ1, R2-BUD-REC4 named).
- **Existing questions (the critic's words):** R1-TICK-OQ3, R1-TICK-OQ4, R1-TICK-OQ8; R2-BUD-OQ1.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who answers R1's timing questions (sub-tick input timestamps, client tick nudging, shipped tick rates)?**
  > R1 leaves three timing questions to R2's design, and R2 does not take them up: which tick a client stamps and how
  > it stays just ahead of the server is part of the command path both ADRs rely on. Neither topic could read the
  > shipped references, so R1-TICK-OQ3 cannot be aligned. Side A: R1-TICK-OQ3; R1-TICK-OQ4; R1-TICK-OQ8. Side B:
  > missing answer (R2-BUD-OQ1, R2-BUD-REC4 named). Existing questions: R1-TICK-OQ3, R1-TICK-OQ4, R1-TICK-OQ8;
  > R2-BUD-OQ1. Decided by: ADR-0001 (time sync and command stamping); the R2-SYNC-OQ3 spike; R1-TICK-OQ4 only if the
  > tick falls back to 30 Hz. (Consistency critic, 2026-10-29, rank 12.)

## Rank 13 (gap): Who answers R1's protocol questions (hash comparison, snapshot layout, mid-match tuning, the content hash at join)?
- **Goes into:** R1 as R1-XT-OQ6; R2 as R2-XT-OQ6.
- **Decision code:** none.
- **Ids:** Side A: R1-DET-OQ9; R1-BND-OQ8; R1-DATA-OQ3; R1-DATA-REC6. Side B: missing answer. Also named: R2 room
  control messages.
- **Existing questions (the critic's words):** R1-DET-OQ9, R1-BND-OQ8, R1-DATA-OQ3.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who answers R1's protocol questions (hash comparison, snapshot layout, mid-match tuning, the content hash
  > at join)?** R1 hands these protocol choices to R2 by id, and R2 answers none of them. The join message R2 does
  > propose does not carry the content hash R1's bundle check and R6's match record (r6:757) both rely on. Side A:
  > R1-DET-OQ9; R1-BND-OQ8; R1-DATA-OQ3; R1-DATA-REC6. Side B: missing answer. Also named: R2 room control messages.
  > Existing questions: R1-DET-OQ9, R1-BND-OQ8, R1-DATA-OQ3. Decided by: ADR-0001 (join handshake, snapshot format,
  > divergence telemetry) with ADR-0005 (content hash). (Consistency critic, 2026-10-29, rank 13.)

## Rank 14 (hidden dependency): Is whole-world replay still a spike mark?
- **Goes into:** R1 as R1-XT-OQ7; R2 as R2-XT-OQ7; R4 as R4-XT-OQ3.
- **Decision code:** none.
- **Ids:** Side A: R1-ECS-REC1; R1-XU-OQ2. Side B: R2-SYNC-REC3; R2-SYNC-REC9; R4-NAV-REC4. Also named: R1-ECS-OQ3.
- **Existing questions (the critic's words):** R1-XU-OQ2 (with R1-ECS-OQ3, R1-DET-OQ6, R1-ABL-OQ4).
- **Note (not inserted):** Resolution 5 (ABL-PREDICT) is about abilities only; the glossary's prediction scope
  (R2-SYNC-REC3's local-only replay) does not settle R1-ECS-REC1 mark 3, so this stays open.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Is whole-world replay still a spike mark?** R2 answers R1's question by subject without citing it, and R4
  > already builds on the local-only answer (no navigation in prediction). R1's whole-world replay mark (and the
  > snapshot cost it sizes) is still a proposed spike mark for a model no topic recommends. Side A: R1-ECS-REC1;
  > R1-XU-OQ2. Side B: R2-SYNC-REC3; R2-SYNC-REC9; R4-NAV-REC4. Also named: R1-ECS-OQ3. Existing questions: R1-XU-OQ2
  > (with R1-ECS-OQ3, R1-DET-OQ6, R1-ABL-OQ4). Decided by: ADR-0001 (prediction scope), then ADR-0004 keeps or drops
  > R1-ECS-REC1 mark 3. (Consistency critic, 2026-10-29, rank 14.)

## Rank 15 (hidden dependency): Do browser navigation and server-less previews rest on an undecided offline-play need?
- **Goes into:** R1 as R1-XT-OQ8; R4 as R4-XT-OQ4; R5 as R5-XT-OQ4; R6 as R6-XT-OQ2.
- **Decision code:** none.
- **Ids:** Side A: R1-BND-OQ1. Side B: R4-NAV-REC4; R6-HOST-REC3; R5-PREV-OQ3. Also named: R1-TICK-OQ7, R1-XU-OQ12.
- **Existing questions (the critic's words):** R1-BND-OQ1, R1-TICK-OQ7, R1-XU-OQ12.
- **Note (not inserted):** Resolution 10 changes only the N in R1-TICK-OQ7 (MODES-3), not whether offline play is a
  need.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Do browser navigation and server-less previews rest on an undecided offline-play need?** R4 builds
  > navigation for the browser, R6 serves previews without a game server, and R5 plans the first previews, all on
  > offline play, which R1 leaves as an open VISION question (and, at the largest mode, as R1-TICK-OQ7). Side A:
  > R1-BND-OQ1. Side B: R4-NAV-REC4; R6-HOST-REC3; R5-PREV-OQ3. Also named: R1-TICK-OQ7, R1-XU-OQ12. Existing
  > questions: R1-BND-OQ1, R1-TICK-OQ7, R1-XU-OQ12. Decided by: Mustafa and the Advisor (VISION_V2), then ADR-0004 and
  > ADR-0006. (Consistency critic, 2026-10-29, rank 15.)

## Rank 16 (hidden dependency): Do the snapshot and the output block carry remote soldiers' animation state?
- **Goes into:** R1 as R1-XT-OQ9; R2 as R2-XT-OQ8; R3 as R3-XT-OQ2.
- **Decision code:** none.
- **Ids:** Side A: R1-BND-REC2. Side B: none by id. Also named: R3-CHAR-REC3, R3-CHAR-REC4, R1-BND-OQ6, R2 worked
  estimate.
- **Existing questions (the critic's words):** R1-BND-OQ6 (block size and layout), partly.
- **Note (not inserted):** Resolution 9's stress scene fixes the counts, not the per-soldier fields.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Do the snapshot and the output block carry remote soldiers' animation state?** Animation of remote
  > soldiers online needs those states in R2's snapshot, then in R1's block. R2's bit budget names none of them and R1
  > leaves the layout open, so R3's character path and R2's bandwidth figure may not fit together. Side A:
  > R1-BND-REC2. Side B: none by id. Also named: R3-CHAR-REC3, R3-CHAR-REC4, R1-BND-OQ6, R2 worked estimate. Existing
  > questions: R1-BND-OQ6 (block size and layout), partly. Decided by: ADR-0001 (snapshot fields) with ADR-0004 (block
  > layout) and ADR-0003 (what animation reads). (Consistency critic, 2026-10-29, rank 16.)

## Rank 18 (hidden dependency): Which recommendations wait on the deferred security baseline?
- **Goes into:** R2 as R2-XT-OQ9; R5 as R5-XT-OQ5; R6 as R6-XT-OQ3.
- **Decision code:** BASELINE-WHEN.
- **Ids:** Side A: R5-PREV-REC4; R5-TSLINT-OQ5; R2-ROOM-OQ11; R2-ROOM-OQ2. Side B: R6-SEC-OQ1.
- **Existing questions (the critic's words):** R6-SEC-OQ1.
- **Note (not inserted):** This is the question BASELINE-WHEN asks (resolutions 17 and 23); the four items here wait
  on its answer too.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Which recommendations wait on the deferred security baseline?** Recommendations in R2 and R5 (guest-secret
  > storage, preview headers, a lint rule, token keys) wait on a topic with no recommendation and no date. ADR-0001,
  > 0002 and 0006 cannot close those items. Side A: R5-PREV-REC4; R5-TSLINT-OQ5; R2-ROOM-OQ11; R2-ROOM-OQ2. Side B:
  > R6-SEC-OQ1. Existing questions: R6-SEC-OQ1. Decided by: The Advisor (research the baseline before ADR-0006, or
  > list it open), then security-auditor. Decision code: BASELINE-WHEN. (Consistency critic, 2026-10-29, rank 18.)

## Rank 20 (gap): How do multiplayer previews get their server binary and their certificate hash?
- **Goes into:** R2 as R2-XT-OQ10; R5 as R5-XT-OQ6; R6 as R6-XT-OQ4.
- **Decision code:** none.
- **Ids:** Side A: R6-HOST-REC3; R2-TRN-REC5. Side B: R5-PREV-REC1. Also named: R5-PREV-OQ3.
- **Existing questions (the critic's words):** R5-PREV-OQ3 (asks whether previews need a server; R6 answers yes, but
  not how).
- **Note (not inserted):** Resolution 21 (PREVIEW-WALL) holds that no preview talks to a game server until Mustafa
  answers; how multiplayer previews are delivered stays this question.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. How do multiplayer previews get their server binary and their certificate hash?** Multiplayer previews
  > need a server-binary delivery job (a second CI secret and PR-built native code on a VM) and a way to publish the
  > self-signed hash on the page's origin. R5's preview design covers only the static client. Side A: R6-HOST-REC3;
  > R2-TRN-REC5. Side B: R5-PREV-REC1. Also named: R5-PREV-OQ3. Existing questions: R5-PREV-OQ3 (asks whether previews
  > need a server; R6 answers yes, but not how). Decided by: ADR-0002 (preview workflow) and ADR-0006 (staging
  > server), with ADR-0001 (where the certificate hash is fetched from). (Consistency critic, 2026-10-29, rank 20.)

## Rank 24 (gap): Where do the navigation bake and R4's checks live in the crates and the gate?
- **Goes into:** R4 as R4-XT-OQ5; R5 as R5-XT-OQ7.
- **Decision code:** none.
- **Ids:** Side A: R4-NAV-REC1; R4-FLOORS-REC3. Side B: missing answer (R5-LAYOUT-REC2 named). Also named:
  R4-PHYS-REC2, R5-GATE-REC1.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Where do the navigation bake and R4's checks live in the crates and the gate?** Where the bake code lives
  > decides whether the generator ships in the player's Wasm (R4-NAV-REC4 wants a bot-free build to skip navigation
  > data) and how the editor runs it in a Worker. The gate does not include R4's checks. Side A: R4-NAV-REC1;
  > R4-FLOORS-REC3. Side B: missing answer (R5-LAYOUT-REC2 named). Also named: R4-PHYS-REC2, R5-GATE-REC1. Existing
  > questions: none. Decided by: ADR-0002 (crate list and gate checks) with ADR-0004 (navigation). (Consistency
  > critic, 2026-10-29, rank 24.)

## Rank 25 (gap): Does the editor still open by double-click with a Wasm validator?
- **Goes into:** R1 as R1-XT-OQ10; R3 as R3-XT-OQ3; R5 as R5-XT-OQ8.
- **Decision code:** none.
- **Ids:** Side A: R1-DATA-OQ11. Side B: missing answer (R5-LAYOUT-REC5, R3-WGPU-REC4 named).
- **Existing questions (the critic's words):** R1-DATA-OQ11.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Does the editor still open by double-click with a Wasm validator?** R1 hands this question to R5, and R5's
  > editor build does not address the double-click rule EDITOR_V2 keeps. Side A: R1-DATA-OQ11. Side B: missing answer
  > (R5-LAYOUT-REC5, R3-WGPU-REC4 named). Existing questions: R1-DATA-OQ11. Decided by: ADR-0002 (editor build), or
  > Mustafa if EDITOR_V2 §5's double-click rule must change. (Consistency critic, 2026-10-29, rank 25.)

## Rank 26 (gap): Who creates the client's field data (backend, hardware, frame time)?
- **Goes into:** R3 as R3-XT-OQ4; R6 as R6-XT-OQ5.
- **Decision code:** none.
- **Ids:** Side A: R3-GL2-OQ2. Side B: R6-TEL-REC1; R6-OBS-OQ5. Also named: R3-BUDGET-OQ1.
- **Existing questions (the critic's words):** R3-GL2-OQ2, R3-BUDGET-OQ1, R6-OBS-OQ5.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who creates the client's field data (backend, hardware, frame time)?** Each topic hands client-side field
  > data (backend share, hardware mix, frame time) to the other. No recommendation creates it, yet R3's fallback-share
  > and hardware questions depend on it. Side A: R3-GL2-OQ2. Side B: R6-TEL-REC1; R6-OBS-OQ5. Also named:
  > R3-BUDGET-OQ1. Existing questions: R3-GL2-OQ2, R3-BUDGET-OQ1, R6-OBS-OQ5. Decided by: The synthesis: ADR-0003
  > (what is measured) and ADR-0006 (where it is stored, under R6-OBS-REC4's data rules). (Consistency critic,
  > 2026-10-29, rank 26.)

## Rank 27 (hidden dependency): Do the wire's entity ids and tick counter fit R1's generational ids and a 20-minute match?
- **Goes into:** R1 as R1-XT-OQ11; R2 as R2-XT-OQ11.
- **Decision code:** none.
- **Ids:** Side A: R1-ECS-REC3. Side B: R2-ROOM-REC1. Also named: R1 pointer to R2 (tick wrap), R2 worked estimate.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Do the wire's entity ids and tick counter fit R1's generational ids and a 20-minute match?** R2's byte
  > budget has no room for R1's generational ids, and with a 16-bit tick a 20-minute match at 60 Hz wraps. R2 does not
  > answer either pointer. Side A: R1-ECS-REC3. Side B: R2-ROOM-REC1. Also named: R1 pointer to R2 (tick wrap), R2
  > worked estimate. Existing questions: none. Decided by: ADR-0001 (wire id and tick widths), recomputing
  > R2-BUD-REC2's estimate. (Consistency critic, 2026-10-29, rank 27.)

## Rank 28 (gap): Who owns recovery from a trap in the client's Wasm sim?
- **Goes into:** R1 as R1-XT-OQ12; R2 as R2-XT-OQ12; R6 as R6-XT-OQ6.
- **Decision code:** none.
- **Ids:** Side A: none by id. Side B: R1-RR-OQ3; R6-OBS-REC3. Also named: R2 pointer to R1 (trap recovery).
- **Existing questions (the critic's words):** R1-RR-OQ3.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who owns recovery from a trap in the client's Wasm sim?** R2 says R1 owns client-trap recovery; R1 leaves
  > it to the synthesis. No topic recommends it. Side A: none by id. Side B: R1-RR-OQ3; R6-OBS-REC3. Also named: R2
  > pointer to R1 (trap recovery). Existing questions: R1-RR-OQ3. Decided by: ADR-0004 (worker restart) with ADR-0001
  > (resync path, R2-XU-OQ6) and ADR-0002 (panic strategy). (Consistency critic, 2026-10-29, rank 28.)

## Rank 30 (hidden dependency): Do soldiers block each other? (one rule, asked twice)
- **Goes into:** R2 as R2-XT-OQ13; R4 as R4-XT-OQ6.
- **Decision code:** none.
- **Ids:** Side A: R2-SYNC-OQ5. Side B: R4-PHYS-OQ1; R4-NAV-OQ5. Also named: R2-SYNC-REC7, R4-NAV-REC3.
- **Existing questions (the critic's words):** R2-SYNC-OQ5, R4-PHYS-OQ1, R4-NAV-OQ5.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Do soldiers block each other? (one rule, asked twice)** The same rule is asked twice, with different
  > spikes named. R2-SYNC-REC7 mark 3 and R4-NAV-REC3's avoidance marks both depend on one answer. Side A:
  > R2-SYNC-OQ5. Side B: R4-PHYS-OQ1; R4-NAV-OQ5. Also named: R2-SYNC-REC7, R4-NAV-REC3. Existing questions:
  > R2-SYNC-OQ5, R4-PHYS-OQ1, R4-NAV-OQ5. Decided by: The game-designer with Mustafa, stated once in the synthesis
  > (ADR-0004 and ADR-0001). (Consistency critic, 2026-10-29, rank 30.)

## Rank 31 (gap): Where do content's TypeScript types come from?
- **Goes into:** R1 as R1-XT-OQ13; R5 as R5-XT-OQ9.
- **Decision code:** none.
- **Ids:** Side A: R1-XU-OQ8; R1-BND-REC4; R1-DATA-REC3. Side B: R5-LAYOUT-REC3.
- **Existing questions (the critic's words):** R1-XU-OQ8.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Where do content's TypeScript types come from?** R5 gives the outputs one home but does not choose between
  > the two sources for content types. Side A: R1-XU-OQ8; R1-BND-REC4; R1-DATA-REC3. Side B: R5-LAYOUT-REC3. Existing
  > questions: R1-XU-OQ8. Decided by: ADR-0005 (content data), with ADR-0002. (Consistency critic, 2026-10-29, rank
  > 31.)

## Rank 32 (hidden dependency): Do R2's marks all name one of R5's network profiles?
- **Goes into:** R2 as R2-XT-OQ14; R5 as R5-XT-OQ10.
- **Decision code:** none.
- **Ids:** Side A: R5-TEST-REC4; R2-XU-OQ12. Side B: R2-BUD-REC1; R2-BUD-REC4. Also named: R2-INT-REC1.
- **Existing questions (the critic's words):** R2-XU-OQ12.
- **Note (not inserted):** No resolution touches the profiles; R2-BUD-REC1 and R2-BUD-REC4 keep their ±10 ms marks
  until the synthesis restates them.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Do R2's marks all name one of R5's network profiles?** R5 treats R2's profile set as settled, but R2's own
  > marks still use other jitter values and unnamed latencies, so some R2 marks cannot be run as R5 specifies. Side A:
  > R5-TEST-REC4; R2-XU-OQ12. Side B: R2-BUD-REC1; R2-BUD-REC4. Also named: R2-INT-REC1. Existing questions:
  > R2-XU-OQ12. Decided by: The synthesis (ADR-0001) restates every R2 mark against a named profile. (Consistency
  > critic, 2026-10-29, rank 32.)

## Rank 33 (gap): Who sets the CI runner costs (GPU, macOS, reference machines, cache)?
- **Goes into:** R3 as R3-XT-OQ5; R5 as R5-XT-OQ11; R6 as R6-XT-OQ7.
- **Decision code:** none.
- **Ids:** Side A: R5-CI-OQ5; R5-TEST-OQ4; R3-MEASURE-OQ2. Side B: missing answer. Also named: R5-XQ-OQ1,
  R6-COST-REC1.
- **Existing questions (the critic's words):** R5-CI-OQ5, R5-TEST-OQ4, R5-XQ-OQ1, R3-MEASURE-OQ2.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who sets the CI runner costs (GPU, macOS, reference machines, cache)?** Real-Safari runs, WebGPU coverage
  > in CI and reference-machine runs all depend on costs no topic sets. Side A: R5-CI-OQ5; R5-TEST-OQ4;
  > R3-MEASURE-OQ2. Side B: missing answer. Also named: R5-XQ-OQ1, R6-COST-REC1. Existing questions: R5-CI-OQ5,
  > R5-TEST-OQ4, R5-XQ-OQ1, R3-MEASURE-OQ2. Decided by: ADR-0006 (costs) with ADR-0002 (CI); Mustafa for any paid
  > plan. (Consistency critic, 2026-10-29, rank 33.)

## Rank 34 (hidden dependency): Where do the Sentry upload job and the smoke test's network rule fit?
- **Goes into:** R5 as R5-XT-OQ12; R6 as R6-XT-OQ8.
- **Decision code:** none.
- **Ids:** Side A: none by id. Side B: R5-GATE-REC3. Also named: R6-OBS-REC3, R5-PREV-REC1.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Where do the Sentry upload job and the smoke test's network rule fit?** R6's crash reporting adds a CI
  > secret job and an external endpoint that R5's CI design and smoke check do not allow for. Side A: none by id. Side
  > B: R5-GATE-REC3. Also named: R6-OBS-REC3, R5-PREV-REC1. Existing questions: none. Decided by: ADR-0002 (CI jobs,
  > secrets and the smoke test's network rule) with ADR-0006. (Consistency critic, 2026-10-29, rank 34.)

## Rank 35 (gap): Who owns audio occlusion rays, if option O-B is chosen?
- **Goes into:** R1 as R1-XT-OQ14; R3 as R3-XT-OQ6; R4 as R4-XT-OQ7.
- **Decision code:** none.
- **Ids:** Side A: R3-AUDIO-REC3; R3-AUDIO-OQ5. Side B: missing answer (R4-LOS-REC1 named). Also named: R1-BND-REC1.
- **Existing questions (the critic's words):** R3-AUDIO-OQ5.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who owns audio occlusion rays, if option O-B is chosen?** Page-side audio rays would need the sim's query
  > across a boundary that allows no page calls. This only matters if option O-B is adopted. Side A: R3-AUDIO-REC3;
  > R3-AUDIO-OQ5. Side B: missing answer (R4-LOS-REC1 named). Also named: R1-BND-REC1. Existing questions:
  > R3-AUDIO-OQ5. Decided by: ADR-0003 with ADR-0004, only if R3-AUDIO-REC3's spike asks for O-B. (Consistency critic,
  > 2026-10-29, rank 35.)

## Rank 36 (gap): Which bench method measures R1's budget marks?
- **Goes into:** R1 as R1-XT-OQ15; R5 as R5-XT-OQ13.
- **Decision code:** none.
- **Ids:** Side A: R1-ABL-REC6. Side B: R5-TEST-REC1; R5-TEST-OQ8. Also named: R1 pointer to R5 (tick benches).
- **Existing questions (the critic's words):** R5-TEST-OQ8.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Which bench method measures R1's budget marks?** R1's budget marks wait on a measuring method R5 has not
  > chosen. Side A: R1-ABL-REC6. Side B: R5-TEST-REC1; R5-TEST-OQ8. Also named: R1 pointer to R5 (tick benches).
  > Existing questions: R5-TEST-OQ8. Decided by: ADR-0002 (bench tool and method). (Consistency critic, 2026-10-29,
  > rank 36.)

## Rank 37 (gap): Does R3 need OffscreenCanvas, or a per-entity or per-field block layout?
- **Goes into:** R1 as R1-XT-OQ16; R3 as R3-XT-OQ7.
- **Decision code:** none.
- **Ids:** Side A: R1-BND-OQ6. Side B: missing answer. Also named: R1 pointer to R3.
- **Existing questions (the critic's words):** R1-BND-OQ6.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Does R3 need OffscreenCanvas, or a per-entity or per-field block layout?** R1 asks R3 for two inputs to
  > the block layout, and R3 gives neither. Side A: R1-BND-OQ6. Side B: missing answer. Also named: R1 pointer to R3.
  > Existing questions: R1-BND-OQ6. Decided by: ADR-0003 with ADR-0004 (block layout), measured in the R1-BND-REC2
  > spike. (Consistency critic, 2026-10-29, rank 37.)

## Rank 38 (gap): Who answers R1-BND-OQ5 (embeds, cross-origin popups, CORP and the login flow under isolation)?
- **Goes into:** R1 as R1-XT-OQ17; R6 as R6-XT-OQ9.
- **Decision code:** none.
- **Ids:** Side A: R1-BND-OQ5. Side B: none by id. Also named: R1-BND-REC5, R1 pointer to R6.
- **Existing questions (the critic's words):** R1-BND-OQ5.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Who answers R1-BND-OQ5 (embeds, cross-origin popups, CORP and the login flow under isolation)?** These are
  > part of R1-BND-REC5's isolation cost, which R6 was asked to report. Side A: R1-BND-OQ5. Side B: none by id. Also
  > named: R1-BND-REC5, R1 pointer to R6. Existing questions: R1-BND-OQ5. Decided by: Mustafa with ADR-0006, only if
  > production isolation is adopted (R1-BND-REC2 mark 5). (Consistency critic, 2026-10-29, rank 38.)

## Rank 39 (gap): Does the client's cutaway follow the server's visibility rule?
- **Goes into:** R2 as R2-XT-OQ15; R3 as R3-XT-OQ8.
- **Decision code:** none.
- **Ids:** Side A: R2-INT-REC2. Side B: R3-WGPU-REC3. Also named: R2-INT-OQ1.
- **Existing questions (the critic's words):** R2-INT-OQ1.
- **Note (not inserted):** Look brief pending: it stays pending (R2-INT-OQ1).
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Does the client's cutaway follow the server's visibility rule?** This is a pointer R3 does not take up. It
  > stays look brief pending and depends on R2-INT-OQ1. Side A: R2-INT-REC2. Side B: R3-WGPU-REC3. Also named:
  > R2-INT-OQ1. Existing questions: R2-INT-OQ1. Decided by: The look brief and Mustafa (R2-INT-OQ1), then ADR-0003.
  > (Consistency critic, 2026-10-29, rank 39.)

## Rank 40 (hidden dependency): Does R5's cache question size a workspace no topic proposes?
- **Goes into:** R1 as R1-XT-OQ18; R4 as R4-XT-OQ8; R5 as R5-XT-OQ14.
- **Decision code:** none.
- **Ids:** Side A: R5-CI-OQ1. Side B: R1-ECS-REC1; R4-PHYS-REC1.
- **Existing questions (the critic's words):** none.
- **Text to insert** (the same in each file, headed by that file's id):
  > **<id>. Does R5's cache question size a workspace no topic proposes?** R5's cache question assumes crates R1 and
  > R4 recommend against, so it sizes a workspace no topic proposes. Side A: R5-CI-OQ1. Side B: R1-ECS-REC1;
  > R4-PHYS-REC1. Existing questions: none. Decided by: ADR-0002 (crate list), then the cache spike (R5-CI-REC1 mark
  > 2). (Consistency critic, 2026-10-29, rank 40.)

## Rank 41 (wrong cross-reference): a nit fix in R5, not a question
- **Ids:** R5-PREV-REC4 (side A); R1-BND-REC5, R1-BND-REC2 (side B).
- **The critic's words:** "This is an ambiguous pointer to the test that decides production isolation." Decided by: "An
  editorial fix in the synthesis (cite R1-BND-REC2 mark 5)"; the advisor's condition 11 makes it a nit fix now.
- **Change** (r5:1617, R5-PREV-REC4; line number at commit 8dee746):
  - before: `production stays non-isolated until REC5's test decides`
  - after: `production stays non-isolated until R1-BND-REC2 mark 5's test decides`
- **Why:** R1-BND-REC5 says "Adopt it in production only if the spike's B2-against-B3 test passes REC2's threshold"
  (r1:2249); that test is R1-BND-REC2 mark 5 (r1:2224-2226), and inside R5's section a bare "REC5" reads as
  R5-PREV-REC5.
- **Overlap:** resolution 22's change C22.1 rewrites the same sentence and already carries this fix ("until
  R1-BND-REC2 mark 5's test decides"). If C22.1 is approved, this nit is applied through it; if not, apply this change
  alone. Logged by the R5 owner as a nit.
