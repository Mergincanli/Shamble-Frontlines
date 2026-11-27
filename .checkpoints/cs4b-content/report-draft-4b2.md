# Advisor report: cs4b2-world (2026-11-27, branch `claude/ecstatic-wozniak-5cbdzs`, base main b8f02b0)

DRAFT (notes only). Marked [after]: the reviews, the verifier, docs-keeper, CI on the final head, the final ids.

Change-set 4b-2, "The world", the second of 4b's two pull requests (his SIZE-4B (b)); lane 1; Builder V4 to step 9's
third piece (5c6738a), then the chat he calls Builder V6. The branch stands in for `cs/4b-content`. Risk tier 1 (game
code, tests, scripts, data; one manifest batch held until security-auditor's answer on its digests; no CI, kit or gate
file). **`content/source/` changes** (his SOURCE-GO answer asks a report's first lines to say so): his Testlines
Export at 30 steps, its manifest and the hand-over table. **Nothing a player sees changes in a match that runs:** the
page's scene (step 12) moved, with the map's reading, to the start of 4c (his STEP12-GO (a), 2026-11-27, 20:03:18 UTC).

## Goal
Bring the game's content into the match the right way: one bundle, judged by the game's own loader once per match,
through the constructor, never swapped mid-match; the soldier's movement table written from his Export; the test
scene's static collision world, the same bits on every target; and a check on the file players get that keeps fused
multiply-adds, vector instructions, host maths and foreign imports out of it.

## Outcome
DONE for steps 6 to 11; step 12 moved to 4c by his choice. [after: the reviews and the verifier.]

## Plan and approval
The re-read sheet's revision 3, section G, rows 6 to 12, approved by the advisor agent's short re-read (2026-11-26,
19:58 Amsterdam) under seven conditions, each met: (1) the remade batch resolved parry3d 0.31.1, safe_arch 1.2.0 and
thiserror 2.0.21 as step 0's, the same 56 packages (security-auditor approved its five digests); (2) every changed
manifest in the digests; (3) the feature compare's two named exceptions, each with a fixture, near misses and a
planted mistake (5cc96c5); (4) the first collision query from the match's step is 4c's (no step queries the world in
4b-2), so a recording that covers queries is 4c's; (5) the facing's zero checked against the old game's code by file
and line before the first map file (`map.rs:56-64`: `legacy/shamble-game.js:871`, `:2710`, `:2758`); (6) step 12's
reviewers are not called (moved), netcode-architect reviews steps 9 and 10; (7) 4b-1's conditions: no report calls
PM-0005-3 passed; every movement value is his Export's and no range refused one; 4B-MEMORY-UNIT fixed (a8741fc).
Rulings during the build, all the advisor agent's unless named: the count rule (`advisor-count-rule.md`: the
recordings out as data, the generated TypeScript in); step 9's order in three cuts (`step9-order.md`); cut C's caps,
page parse and dev route (`advisor-cutC.md`, APPROVE with conditions); step 11's four questions (`advisor-step11.md`);
rust-reviewer's two maths allow-list entries (`rust-reviewer-cutC.md`); security-auditor's dependency answers.

## What changed (100 files outside the notes; the commits: `git log b9aad01..<head> -- crates packages content`)
- **Step 6** `Cargo.toml`, `Cargo.lock` (33 packages), `crates/sim` and `crates/sim-wasm` manifests; the dependency
  rules and the feature compare (`dependency_rules.rs`, `sim-feature-lists.json`, `build-checks.mjs`).
- **Step 7** `build-checks.mjs` and `build.mjs`: the built-file check (names, instructions, target features, imports,
  fused multiply-add, banned maths, the maths allow-list, unlinked crates, the stamp); `rules_scan.rs`; native tests.
- **Step 8** `content_bundle.rs`, `content_files.rs`, `testlines.mjs`, `canonical.mjs`; `content/` (his Export, the
  table, `movement/soldier.json`, `maps/test-scene.json`); the tests on the real files.
- **Step 9** `ts_export.rs` and `packages/types/src/generated/content/`; the page folder's plain files and the
  markup-sink scan; `build.mjs` writes `content.json` (cut A); `first-loop.content.json` and every replay's content
  check (cut B); `Sim::new(seed, mode, pad_bytes, bundle)`, the Worker's `start` with the bundle, `content.ts`,
  `boot.ts`, the message caps, the dev server's `/content.json` (cut C).
- **Step 10** `collision.rs` (845 lines with its tests), `vec.rs`, `world_proofs.rs`, the fuzz and hostile maps
  through the builder; `LocalMatch` holds the world, `CreateError::MapMissing` and `::World`, `LOCAL_MAP`; a replay
  of another map's recording refused.
- **Step 11** `schema.rs` (test-only), the two schema files, the export test's schema folder, the Vitest limit test.
The gate's symbol counts are the legacy files' (+0 -0); nothing under `legacy/` changes.

## Evidence
- **Size:** 7,978 written lines (raw with the recordings 7,996) against his stop line 8,450; 2.5 times the designs'
  count for steps 6 to 11 (3,155 by the sheet's rows: 110, 890, 545, 830, 540, 240; 4b-1 was 2.17, 4a 2.4). His one
  sentence at two thirds came at cut B (5,668 by the count then in use, given at once).
- **Every code head of this seat** (5cef19f, ccde0d4, e1e2dd4, fbbd84d, 885d382, c68b8d5 with dcb5e6b, 46c192f,
  c442a80): the full local checks passed whole (at step 11: 235 Rust tests twice, 234 in release, clippy and format
  clean, the generated files written again unchanged, client 148, sim-worker 53, types 12, net 1, boundary 39, 29
  Chromium tests in 208 s, the recordings what their scripts make), the gate with smoke, CI green (toolchain 9 min 43
  s to 13 min 2 s; the job's limit is 25 minutes). [after: c442a80's toolchain run; the final head.]
- **Planted defects** (`plants-4b2.py`, 84 plants plus 4 stamp and 6 native ones): the first whole run on c442a80, 83
  caught of 83, 10 of 11 controls held (P19's page step ran after a refused build, with no stamp: a defect of the
  script, fixed); the second whole run [after]. Every file put back, the players' file rebuilt to its digest.
- **Loading content changed no state:** at cut B, `first-loop.hashes` lines 2 to 1,801, `first-loop.vector.txt` and
  the 1,800 step records equal main's `git show` copies; since then every recording file is byte for byte cut B's. A
  content byte folded into the shared state fails at step 1 natively, in Node and in Chromium (plants C17 to C19).
- **The players' file:** 54,514 bytes on main, 217,692 with the loader (cut C), 257,421 with the world's tree
  (step 10; gzip 100,215, brotli 82,101), sha256 3660132be46b33a4fbf59404b47a85a2436cfea47251adf386a42e529b800644;
  the built-file check walks 95,441 instructions and refuses nothing; no word of a description or a schema in it.
- **The browser:** the page's content spec (one fetch, the served bytes the built file's, one changed value refused at
  its file and line, seven hostile bundles shown only as the loader's sentence with no element, a failed fetch
  starting no Worker); the isolation spec's dev server serves the build's bundle byte for byte. Firefox runs the rules
  and replay specs only (CI).
- **The table's names** (test-engineer's TE-1, High, at round 1): the second route reads 5 of the table's 15 values,
  and a swap between any two rows it does not read passed every check (its proof: the rows and `soldier.json` swapped,
  `--check` and every cargo test green). Step 8's sentence ("a row swapped within one of those [equal] pairs passes
  every check") stated that gap too small. Closed at the fix pass: a test on the real table holds every row's field
  to the last step of its Export name, but for one named rename, so no two rows can trade fields, equal values or
  not; plant C38 (two rows of equal values trade fields, the content unchanged) is caught by it, and its controls C39
  and C40 show that both routes that compare values pass that swap.
- **Not verified:** the boundary and page specs run in Chromium, not Firefox; no Firefox run of the players' file's
  content refusals; the collision world is built but queried by nothing in a match (4c).

## Reviews [after]
rust-reviewer, security-auditor, netcode-architect, engine-architect, test-engineer, infra-engineer: one round, then
one fix pass. The design reads during the build: rust-reviewer (steps 0, 10, cut C), security-auditor (the batch,
the advisories, the test list), netcode-architect (route (i)), engine-architect, test-engineer (the test list).

## Decisions and deviations
- His: DIFF-LIST (a); GUARD-RETRY (a); STEP12-GO (a); STEP11-ROOM unanswered, so the ladder built step 11.
- The order: step 10's core and the pieces needing no content file before step 8's second part (DIFF-LIST was open);
  step 9 in three cuts; cut C and step 10 committed before their full checks ended (the environment's Stop hook), each
  check then passing on the same files.
- From the plan: the map's reading field by field moved from step 9 to step 12, now 4c (Q2); the caps (Q1: 512 for a
  thrown text, the panic's 1,016-byte room whole, the page's check at 1,040, against the test list's "about 512"); no
  `default` column, the descriptions in a test-only table (step 11, Q1 and Q2); the manifest beside his Export
  written by script; `fma4` dropped from the native assertion; the plants' P19 fixed in the script.
- Two things this change-set leaves for later on purpose (engine-architect's EA-3): the generated content types have
  no reader in 4b-2, because the map's reading moved to 4c with its one reader; until then only their tests hold them
  (the export test, which writes them again unchanged, and, since the fix pass, a test that ties them to the schemas)
  (the advisor agent's cut C ruling, Q2's condition 5). And of engine-architect's block 1.9 only the serving part is
  built: the development server answers `/content.json` with what the build's own content tool writes, on each ask;
  there is no watch and no reload (Q3's condition 7).
- Found and fixed in the build: the constructor said "the content is refused:" twice (cut C, before its commit); a
  Worker script that does not load gives no message (cut C, caught by an existing spec); a replay of another map's
  recording would have run on `test-scene` (step 10's follow-up); a tie test read a file around his LINKS-CHECK scan
  (caught by the local checks before any push).
- Slips of this chat's rules: listed in the card's "Slips" (times typed ahead of the clock, git after another program
  in a read, a merge commit without attribution, the CI limit told as 15 minutes when it is 25).

## Risks
1. The world is built and held but queried by nothing yet: its first real test is 4c's movement and aim.
2. The players' file grew 4.7 times (mostly the standard library's number reading and writing); recorded, not gated.
3. CI's toolchain run varies with the runner (9:43 to 13:02); 12 minutes under its limit.

## Proposed next step
Change-set 4c, starting with the page's scene and the map's reading (moved from 4b-2), then the soldier's walk on the
static world.

## Decisions needed
[after]

## For Mustafa
1. [after] Your go for the merge of this pull request, for the head the advisor agent approves (no Brave check).
