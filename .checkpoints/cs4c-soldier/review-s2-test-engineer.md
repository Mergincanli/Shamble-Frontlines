# Step S review, round 2: test-engineer (4c-1, the page's scene), fix 2b2f44c2 on c1de41e5

Worktree head 3c6e3bc7; `git diff 2b2f44c2 3c6e3bc7 -- packages crates content` is empty, so the code reviewed is
2b2f44c2's. `git status --porcelain -- packages crates` empty before my runs.

## Plan
1. Read `git diff c1de41e5 2b2f44c2 -- packages` (scene.test.ts, content*.test, report.test.ts, skeleton.spec.ts and
   the code they test: scene.ts, inventory.ts, content.ts, report.ts, boot.ts, three-picture.ts).
2. For each of TE-1 to TE-8 and G-3, GE-F5, S-2, EA-3, EA-4, EA-5, GE-F1: the new assertion, the plant that shows it
   red, and whether the plant is the finding's own defect, caught by the test it names in that test's words.
3. Read `plants-4c.py`, `plants-4c.txt` and the log's last run (`.gate/plants-4c.log`): F1's timeout against the clean
   run's time, orphaned processes, the files put back.
4. What is still not proven in what changed: my own plants on a scratch copy of src/render at 2b2f44c2
   (`.gate/te-review2/client`, script `.gate/te-review2/plants-te2.py`, probe `cap-probe.test.ts` beside the copy's
   scene tests), never the worktree's source; the plants run is not repeated.

## Notes (one line per finding, as found)
- N1 (provenance, TE-8): `plants-4c.txt:1` names 2b2f44c2 (full id); 45 caught, 3 controls held, 7 files' sha256 back.
  Checked in the log's last run: F1 to F17 and C1 to C3 (C1 exit 0 in 431 ms, C2 and C3 exit 0); T1 to E4 by their
  lines in plants-4c.txt only (unchanged plants, all caught in round 1).
- N2 (F1, GE-F1's hang): caught by the script's outer timeout (`plants-4c.py:28-40`, start_new_session and killpg),
  90 s against the clean scene file's 0.33 to 0.43 s in the same run (over 200 times the margin); its wanted text
  "TIMED OUT" is the script's own, so the catch is the harness's, not the test's words; any hang in that file counts,
  and the plant's one edit (the cap removed) is the only change. `ps` after the run: no vitest or worker process left.
  In CI the same defect would show as a job hung until the job's own timeout, not as a failed test (by reading).
- N3 (F2, F14): each errors its named test through an uncaught TypeError, not an assertion: F2 `view.position` of null
  (`scene.ts:316`, log :6332-6361), F14 `content.map is not a function` (the readout throws on "refused"). Each is the
  real failure its code prevents (a throw at every resize; a readout that cannot be drawn), so a real catch.
- N3b (F3 to F17, the failing line read in the log, :6363-7102): F3 `expected 500 to be greater than 500`; F4 `made
  ground triangle 0 faces up: expected -1`; F5 `raised ground corners` (y 0 against 2); F6 `made street corners`; F7
  `made ground corners` (-15 against -32); F8 `made wall edge ... drawn 2 times`; F9 `made line 0: [0, 24]` against
  [-24, 24]; F10 `expected 4 to be 105` (the level count); F11 `expected 0.0761 to be greater than 0.92` (the 0.99
  reach); F12 the own-`__proto__` test; F13 the ready test's refused flag; F15 the word-list tie; F16 the message
  mismatch (N5); F17 `skeleton.spec.ts:95:111`, the corner compare with `street_house_a_roof_north`, not the counts.
  Each is caught by its finding's own assertion. Besides, F4, F6, F7, F9, F10 and F11 are my round-1 P2, P1, P3, P4,
  P6 and P5, which passed every test at c1de41e5: red now shows the new assertions bite.
- N4 (F8, TE-2/GE-F5): wanted texts include the edge check's own "drawn 2 times": the edge pairing, not the volume,
  is what catches it. The volume is now taken from (-1000.5, -1000.25, -1000.125), off every face's plane.
- N5 (F16, EA-4): the stray mesh (`scene.test.ts:339-344`) has an empty geometry, so with the material check removed
  the reader throws a TypeError, not INVENTORY_UNKNOWN; `toThrow(INVENTORY_UNKNOWN)` matches the message, so the plant
  is caught by the assertion either way. The kind list's typing (`inventory.ts:14`, `satisfies`) is proven by the
  type check only; no plant (not needed: a compile error).
- N6 (F12, S-2; F15, EA-3): real. F12's Object.assign drops the own `__proto__` (its [[Set]] runs the setter), exactly
  S-2's case. The ties test uses `toEqual` on whole lists, so a word the loader gains and the page lacks is caught as
  well as F15's reverse. The three consts stand at `crates/content/src/map.rs:69-71` in the regex's form.
- N7 (LARGEST, checked): `map.rs:37-40` gives 32,767 cm reach each way and 65,535 cm height: the test's
  ±327.67 and 655.35 are the loader's limits.
- N8 (my plants, scratch copy, 11 scene tests clean): Q1 `frameCamera` without `camera.lookAt` (`scene.ts:318`): NOT
  CAUGHT. Q2 without `camera.updateProjectionMatrix()` (`:320`): NOT CAUGHT. Q3 every z-running grid line at the first
  x, count and ends kept (`:234`): NOT CAUGHT. Q4 the overview's target at y 0, not the bounds' bottom (`:274`): NOT
  CAUGHT. Q5 the patches at y 0 (`:211`): CAUGHT by the new flat test (G-3 on the patches holds).
- N9 (the cap's place, measured on the copy): the overview's distance on the largest floor is 749.6 m at 16:9, 1,010 m
  at 1:1, 1,605 m at 9:16, 4,066 m at 0.2, 76,626 m at 0.01, none at 0.001. With MOST_DOUBLINGS planted at 10 (1,024 m)
  all 11 scene tests pass while 9:16 and every narrower window give no view: the tests bracket the cap from one side only.
- N10 (EA-5's wiring): `boot.ts:166` (`content.refused ? "refused"`) is reached by no test: no Node test imports
  boot.ts (grep), and no browser spec makes the page itself refuse (content.spec.ts covers the loader's refusals,
  content "none"). Dropping that branch would pass every test (by reading, not run).
- N11 (Q1 in the browser): E1 compares the exact render counts (`skeleton.spec.ts:117`, 32 draws), and a camera that
  never turns would likely cull blocks and change them; not run, so unverified.

## Claim, proof, gap
| Finding | Proof at 2b2f44c2 | Plant | Closed? |
|---|---|---|---|
| TE-1 ground and patches: corners, height, facing | `scene.test.ts:144-171` on MAP and RAISED (corners f32, 6 points, right-hand normal +y, normals +y); browser `skeleton.spec.ts:95-102` | F4, F5, F6, F7 (and my Q5) | yes |
| TE-2 closed mesh | `scene.test.ts:113-127`, each directed edge once, its reverse once | F8 ("drawn 2 times") | yes |
| TE-3 as near as fits | `scene.test.ts:276-294`, reach(1) within the margin, reach(0.99) past it | F11 | yes |
| TE-4 grid ends | `scene.test.ts:221-239` on MAP and RAISED | F9 | ends yes; line positions not (Q3) |
| TE-5 greys over the range | `scene.test.ts:196-206`, 1 cm steps never darker, all 105 levels taken | F10 | yes |
| TE-6 every corner in the browser | `skeleton.spec.ts:28-47`, `:95` | F17 | yes |
| TE-7 framing tested in Node | `frameCamera` in scene.ts, `scene.test.ts:312-328` | F2, F3 | in part: the turn and the projection update not held (Q1, Q2); the width clamp `three-picture.ts:97` untested (a second guard) |
| TE-8 whole run on the fix head | `plants-4c.txt` on 2b2f44c2, 45 of 45, 3 of 3 | (itself) | yes |
| GE-F1 search ends | `scene.test.ts:304-310`: 0, NaN, LARGEST 0.001 null, LARGEST 16:9 not | F1 (outer timeout) | yes; the cap's place held from one side only (N9) |
| G-3 the bounds' bottom | RAISED in the flat, grid and solid tests | F5 (and Q5) | ground, patches, grid yes; the overview's target not (Q4) |
| GE-F5 volume blind spot | volume from an off-plane point plus the edge pairing | F8 | yes |
| S-2 own `__proto__` | `content.test.ts:274-297`, six objects and from JSON text | F12 | yes |
| EA-3 word lists | `content-ties.test.mjs:40-44` | F15 | yes |
| EA-4 kinds and material | `scene.test.ts:338-344`; typing by the type check | F16 | yes |
| EA-5 the page's refusal | `content.test.ts` (refused true/false at five points), `report.test.ts:143,159` | F13, F14 | in part: boot.ts:166's wiring untested (N10) |

## Verdict: APPROVE
Every round-1 finding is closed (TE-4, TE-7, G-3 and EA-5 with the Low remainders below); each plant F1 to F17 is the
finding's own defect, caught by the test it names (F1 by the harness's outer timeout, with a 200-fold margin and no
process left; F2 and F14 by an uncaught TypeError inside the named test, the real failure each guard prevents). No Critical, High or Medium. The new gaps
are Low: STATUS follow-ups under the review budget, none a reason to push on its own.

1. TE2-1 Low. `frameCamera`'s turn and projection update are not held: `scene.ts:318` (`lookAt`) and `:320`
   (`updateProjectionMatrix`) can each be removed with all 11 scene tests passing (Q1, Q2); the success case asserts
   position and far only (`scene.test.ts:315-318`). Fix: project the floor's corners through the framed camera's own
   matrices (the overview test's `reach`) without updating them in the test.
2. TE2-2 Low. The cap's place is held from one side: MOST_DOUBLINGS at 10 passes every test while a 9:16 window on the
   largest floor gets no view (N9; `scene.test.ts:306-309`). No such map exists yet. Fix: LARGEST at 9:16 and 0.01 not
   null, beside 0.001 null.
3. TE2-3 Low. EA-5's readout wiring `boot.ts:166` is reached by no test (N10): proven only in its parts (F13, F14).
   Fix: a browser case where the page refuses the map (a bundle the loader takes and readMap refuses), reading
   content "refused"; or accept the parts as the proof.
4. TE2-4 Low. The grid test checks the count and the ends, not where the lines stand (`scene.test.ts:233-238`): every
   z-running line at one x passes (Q3). Fix: the x set equals the whole metres from ceil(minX) to maxX, likewise z.
5. TE2-5 Low. G-3 on the overview: its target's height is never checked on the raised map (Q4, `scene.ts:274`;
   `scene.test.ts:295` checks [24, 0, 0] on MAP only). Fix: `overview(RAISED.bounds, 16 / 9)!.target[1]` is 2.

Not verified: Q1 to Q4 against the browser specs (by reading; N11 makes Q1 likely caught by E1's counts); F1's
behaviour under CI's own job timeout; the WebGPU path. Scratch files: `.gate/te-review2/` only; the worktree's source
untouched (`git status --porcelain -- packages crates` empty before and after my runs).

Note, not a finding (my round-1 ask, item 1: a negative aspect and Infinity): measured on the copy
(`cap-probe.test.ts`), every call returns in under 4 ms: Infinity and -Infinity give no view; a negative aspect
gives a view at the positive aspect's distance (-1: 1,010.4 m on the largest floor, as 1). So the fix is a cap, not
the "not a positive finite number" guard the round's disposition names; a negative aspect cannot reach it
(`three-picture.ts:97` holds width and height at 1 or more), and no test pins either case.

DONE
