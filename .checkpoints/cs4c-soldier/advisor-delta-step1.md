# Advisor agent: plan delta for 4c-2's step 1, and step 1's build order (2026-11-28)

## Plan
1. Read the brief, step1-build-order.md, and each item's cited block: engine-architect block 7, rust-reviewer blocks
   13 to 15 and 11.9, netcode-architect 9.2/9.3/9.5, game-designer block 9, security-auditor's third answer,
   pm-wording-4c2.md, question 13.
2. Spot-check against the branch head (world.rs, content, test scene's spawns, the batch's base files).
3. Rule D1 (items 1 to 8), D2 (C1 to C5), D3 (the stop line); findings one line each; verdict; DONE.

## Notes (one line per finding)
- Checked: HEAD 07e9e866; `git diff --stat eebbbcc2 HEAD -- crates packages content` empty (no code since my verdict);
  the batch's five base files unchanged since a4e93679 (`git diff --stat` empty).
- Checked against the code: room/src/lib.rs:159-163 the join answer's literal slot 0 / team 0; :216-223 the fill takes
  buttons and presses only; content map.rs:92 "the point on the ground between the feet"; bundle.rs:53, :65
  `movements`; rules_scan.rs:699 `in_tests`, :767 its use; no devtools/tests/layering.rs and no
  sim/tests/support/ yet (both new). ADR 0004 :1141 and :1156 equal pm-wording-4c2.md's old rows (first 300 chars).
- Checked: item 9's sum: step 1 3,933 - 3,350 = +583; step 2 1,025 - 820 = -205; 8,550 + 583 - 205 = 8,928.
- G1 (Medium) `Room.spawns` is stored and never read (engine-architect block 7 item 3: "No accessor for `rules` or
  `spawns` (no reader)"; netcode-architect 9.2: "no tick of 4c reads again"). rustc warns `field is never read`
  (shown on a scratch crate, rustc 1.99.1), and local-checks.sh:14 runs clippy with `-D warnings`: red. It is also a
  field kept for change-set 5. Fix: `Room::new` and `LocalMatch::new` borrow the table (`spawns: &[SpawnPoint]`) for
  `World::new` and keep nothing; no `#[allow(dead_code)]`.
- G2 (Medium) the build order does not place items 4 and 5. C4 (the fill starts copying move and aim into the hashed
  row) must carry the room's whole half with it: refused_move, refused_aim, refused(), the doc, the NONE tie, the
  protocol sentence, PLAYER_SLOT/PLAYER_TEAM in the join answer, any_bytes (T2-7) and T2-6 bar its wish-cap case. C5:
  the wish cap and T2-6's direct-feed case, W-3's moving half, Q-1. C4: W-1, W-2, W-4, W-3's stand-and-join half.
- G3 (Medium) a field that enters a commit before its first reader trips the same lint (C3's `rules`, `spawns`,
  `query_counts` when nothing reads them until C4/C5). Rule: every commit passes local-checks.sh whole, no new allow.
- G4 (Low) T-B (row 28) in C4 passes with a soldier who cannot move; from C5 it asserts the route reaches the box's top.
- G5 (Low) 11.8 still puts the re-freeze and the first soldier in "the batch's commit"; the build order splits them
  (C2, C3, C4). 11.8 follows the order. C2's evidence: the `.hashes` files byte-identical, only headers and manifest move.
- G6 (Medium) pm-wording-4c2.md: (i) "every form pair, the ramp included" says more than test 21 checks (it leaves out
  the ramp's low end and slope face, tops and bottoms, counted not compared): the wording names the compared and
  left-out faces as row 21 lists them; (ii) the old pair list's "other convex shapes" is dropped: keep it ("when the
  format allows one") or say in the side-by-side why the players' file meets none; (iii) the flag-on times, native and
  in a browser, mean and p99, sit in neither "In 4c-2" nor "Open in STATUS": place them; (iv) the "form no sweep
  handles" bite: say how it is placed (the map format allows only box and ramp).
- G7 (Low) question 13, :338 "Needed before the soldier's movement step" contradicts :344-346 and :352: reword to "not
  blocking: the build goes on under (b); his answer is wanted by the early look", before he reads it.
- G8 (Low) marking.rs: say how devtools/tests/layering.rs includes sim's tests/support/marking.rs (a `#[path]` module
  in a test file; layering's own `#[path]` plant walks only room's and the endpoint's src/, so no clash).
- G9 (Low) grounded set at spawn, jump (2b) before standing (2h): a soldier spawned off the ground (test 23's 1 m) with
  a press on tick 1 jumps from the air. No recording meets it (tick 1 command-less, B9-N2; map.rs:92's contract).
  Write it in 11.1 as a known edge, or pin it in test 23.
- Checked: `holds_cm` exists nowhere in crates/ today (grep); collision.rs has `narrow` (:103) only. `refused_aim`
  needs it, so `bounds_cm` and `holds_cm` (with `build`'s derivation and `WorldError::Bounds`) move from C5 to C4.
- Checked: sim's lib.rs:18-23 declares its modules `pub`; vec.rs's items are `pub`. A `pub` item in a `pub mod` draws
  no dead-code warning; a `pub(crate)` or private one with no caller does (C1's helpers, until C5).
- G10 (Medium, from G1 and G3) once no Room field may sit before its reader, the players' build first reaches exp
  (`Rules::take`) and sincos (`spawn_points`) in C4, not C3, so C3's "exact maths-allowed.mjs entries" cannot match.
  Ruled in D2: the entries move to C4; my Q4 condition 2 amended in writing.
- G11 (Low) item 3's one condition was missing: game-designer block 9 asks the early look's step 1 to name street_1 at
  (-20, 0, -0.9) and call it a stand-in until the barracks; joined to my F4 condition.
- G12 (Low) D3: `walk` is step 6, after the early look; past 18,700 it waits on his word, yet pm-wording-4c2.md's
  "In 4c-2" list puts PM-0004-20's hash half on `walk`. The draft marks it "if step 6 runs in 4c-2".
- Item 7: my own earlier scene check (advisor-steps1to7.md:22-24: every shape's bottom at y 0 but the four roof ramps,
  on their houses' 2.6 m tops) agrees with B13-3 (no floating underside inside 1.8 m of another top). The re-run itself
  (.gate/rr7/) not opened.

## D1. Items 1 to 8 as plan deltas to steps 1 and 2: APPROVE, with conditions
Risk tier unchanged: 1, needs-mustafa (his Brave check; the PM rows' wording reaches a decision doc's report). No
Critical or High. Conditions:
1. G1 (Medium), item 1: `Room` does not keep the spawn table. Default: `Room::new(setup, content_hash, rules, statics,
   spawns: &[SpawnPoint])` and `LocalMatch::new(.., spawns: &[SpawnPoint])` borrow it for `World::new` and keep
   nothing; `Sim::create` builds it and drops it after. No `#[allow(dead_code)]`. engine-architect and rust-reviewer each
   write a one-line yes (or a reader they name) before C3; it rewrites block 7 items 2, 3 and 5.9, 11.6's room line and
   9.2's "the room keeps the spawn table". C1 and C2 may start meanwhile.
2. Item 2 (B5 (c)): approved as written; row 33 lands in C4 (where `Sim::create` first takes the movement).
3. Item 3 (B4 (a), grounded set): approved; G11: the early look's step 1 names street_1 at (-20, 0, -0.9), facing +x,
   a stand-in until the barracks (game-designer N37), with my F4 condition. G9 (Low): the tick-1 jump of a soldier
   spawned off the ground written in 11.1 as a known edge or pinned in test 23; rides the next notes commit.
4. Item 4 (the room's half into step 1): approved; step 2's evidence as 9.3 writes it (the empty `git diff` over room's
   src and sim's world, movement, body and collision, `recordings.rs` green and the recordings' `--stat` empty). Its
   placement is D2 condition 2.
5. Item 5 (Q-1): approved; lands in C5 with the step's first write of the counts.
6. Item 6 (layering.rs, the shared marking): approved; G8 (Low): the session says how devtools' layering.rs includes
   sim's tests/support/marking.rs; security-auditor reads the batch's commit with the shared copy in it (my F9).
7. Item 7 (question 13): approved; G7 (Low): its :338 "Needed before the soldier's movement step" reworded to "not
   blocking: the build goes on under (b); his answer is wanted by the early look", before he reads it.
8. Item 8 (F2's draft): approved as a draft for rust-reviewer's and netcode-architect's review, with G6 (Medium) applied
   before that review: (i) the compared and left-out faces named as row 21 lists them, not "every form pair"; (ii) the
   old list's "other convex shapes" kept ("when the format allows one") or the side-by-side says why the players' file
   meets none; (iii) the flag-on times (native and browser, mean and p99) placed in "In 4c-2" or "Open in STATUS";
   (iv) how the "form no sweep handles" plant is placed; (v) G12: the `walk` half marked "if step 6 runs in 4c-2".
9. Lows G4, G5, G7 to G9, G11, G12 ride the next notes commit; no push of their own.

## D2. The build order C1 to C5: APPROVE, with these changes
C2 before C3 holds: `Rules::take` (exp's first caller) needs content's 13 fields, and the old frozen bundle cannot be
loaded once the schema needs them, so the re-freeze belongs with them. C4 before C5 holds: one hash move per commit
gives a bisectable soldier who stands before he moves; the cost is one more pin re-make, made by the script.
1. G3 (Medium): every commit passes local-checks.sh whole (clippy `-D warnings` included) with no new `allow`: a field,
   or a `pub(crate)`/private helper, enters in the commit that first reads it, or is `pub` in a `pub mod`. C1 says so
   for its V3 helpers and `resolve_mut`.
2. G2 (Medium): C4 holds the fill copying move and aim together with the room's whole half (9.3 a to h: refused_move,
   refused_aim, refused(), the doc, the NONE tie, the protocol sentence, PLAYER_SLOT and PLAYER_TEAM in `World::new`
   and the join answer, any_bytes as T2-7, T2-6 bar its direct-feed cap case), `bounds_cm` and `holds_cm` with
   `build`'s derivation (moved from C5), and W-1, W-2, W-4 and W-3's stand-and-join half. C5 holds the wish's cap and
   T2-6's direct-feed case, W-3's moving half, Q-1, T-D and the rest of 11.7. No commit copies a wire move into the
   hashed row unjudged (B9-N11).
3. G10 (Medium): C3 holds the batch, fmath, `Rules::take`, `SpawnPoint` and `spawn_points` (called by sim's own tests
   only), the vectors, the libm scan, F1's dependency rules and layering.rs. C4 holds the wiring (`local_constants`,
   `CreateError`'s two variants, `Room`'s fields as each gains a reader, `LocalMatch::new`) and the maths list's exact
   entries, checked in the commit where the players' build first prints exp and sincos. My Q4 condition 2 is amended
   to that, in writing here: the dependency still lands with its scans and vectors; the list follows the shipped build.
   If the maths check fails for an entry before its print (or passes without it), the session reports it, not edits it.
4. G5 (Low): 11.8's "the batch's commit" follows the order; C2's evidence: the recordings' `.hashes` byte-identical,
   only the headers and manifest move. G4 (Low): T-B asserts from C5 that the route reaches the box's top.
5. Standing: the gate, the full local checks and CI green before the next code push; pins re-made in each of C4 and C5;
   step 1 done only with test 21 green and F2's wording drafted.

## D3. The stop line: nothing new now beyond his rule
His SIZE-4C (b) (answers file, item 5) acts on the count reaching 18,700, measured by my Q4 condition 8 (`git diff
--numstat` from the merge-base), not on an estimate: "do not stop: tell me in two lines in your next report and build
on to the early look. Past the line, nothing after the early look starts without my word." No question for him and no
cut to stay under it (evidence is never shrunk to save lines). Two things only:
1. The next report may carry the forecast as information (design about 8,930; about 19,300 to 24,000 written), not an
   ask, and my Q3 rung (14,960) told before it moves, as my Q3 says.
2. When the two lines go, they say what sits behind his word: steps 6, 6b, 6c and 7 (`walk`, walk_reaches.rs, the far
   corner map, and PM-0004-20's hash half on `walk`, G12).

## Verdict
D1 APPROVE items 1 to 8 with D1's conditions 1 to 9. D2 APPROVE the order with changes 1 to 5 (C3 waits on D1
condition 1's two yeses; C1 and C2 may start). D3: nothing new beyond his rule, with its two notes.

Not checked:
- Nothing built or run in the workspace: no gate, cargo, test or prototype. The dead-code warning was shown on a
  scratch crate with the system rustc 1.99.1, not on a workspace build; that the workspace toolchain behaves the same is
  my reading.
- Block 13's re-run (10,111 + 1,021 + 12 + 329 runs) and its traces under .gate/rr7/: not opened; taken from its notes.
- Whether maths-allowed.mjs's check is exact both ways (G10's "if it fails" sentence covers either).
- netcode-architect blocks 1 to 8 and engine-architect's earlier blocks: read only where the new blocks cite them;
  the counts (3,933; 820; 8,930) are the agents' estimates, my sum only.
- Whether question 13 has reached him.
DONE
