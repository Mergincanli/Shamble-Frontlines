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
DONE for steps 6 to 11; step 12 moved to 4c by his choice. Two review rounds (every reviewer PASS by the end,
Lows only left as follow-ups) and their fix passes; [after: the verifier, the advisor agent.]

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

## What changed (102 files outside the notes, 10,381 lines added and 339 removed, `docs/DECISIONS.md` among them, at the final code 914f3b4 against main b8f02b0; the commits: `git log b9aad01..914f3b4 -- crates packages content Cargo.toml Cargo.lock`, 23 of them)
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
- **The two fix passes** (70f11e1, 914f3b4): `collision.rs` (the reach, the cap, the ramp's guard), `map.rs`
  (`MIN_SPAWNS`), `schema.rs`, `content.ts` and `boot.ts` (the page's copy), `vite.config.mjs` (the two refusals),
  `testlines.mjs` and `canonical.mjs` (the folder walk, the resolved path), their tests, the specs' pinned
  sentences, the types-to-schemas tie, `content-ties.test.mjs` (new).
- **The commits, in order** (Builder V4: a8741fc to 5c6738a; Builder V6: from 5cef19f): a8741fc (4B-MEMORY-UNIT),
  5cc96c5 and 298837a (step 6), d313162 (step 7), 54e89d2 (step 8, first part), 2f058b2 and e3acb41 (the advisory
  rules), 8e5a268 (step 10's core), c09f459 and 6290c85 (step 9's first two pieces), ad88341 (step 10, the fuzz run
  and hostile maps), 5c6738a (step 9's third piece), 5cef19f (step 8, second part), ccde0d4, e1e2dd4, fbbd84d (step
  9's cuts A to C), 885d382, c68b8d5, dcb5e6b (step 10's wiring), 46c192f (step 11), c442a80 (the close begins),
  70f11e1 (round 1's fix pass), 914f3b4 (round 2's fix pass); and 35e96ad (main b8f02b0 taken in).
The gate's symbol counts are the legacy files' (+0 -0); nothing under `legacy/` changes.

## Evidence
- **Size:** 8,408 written lines (raw with the recordings 8,426) against his stop line 8,450, at the final code
  (914f3b4): 7,978 at step 11's end, 8,347 after round 1's fix pass (369 lines), 8,408 after round 2's (61). 2.7 times
  the designs' count for steps 6 to 11 (3,155 by the sheet's rows: 110, 890, 545, 830, 540, 240; 4b-1 was 2.17, 4a
  2.4). His one sentence at two thirds came at cut B (5,668 by the count then in use, given at once).
- **Every code head of this seat** (5cef19f, ccde0d4, e1e2dd4, fbbd84d, 885d382, c68b8d5 with dcb5e6b, 46c192f,
  c442a80, 70f11e1, 914f3b4): the full local checks passed whole (on the final code, 914f3b4: 236 Rust tests twice,
  235 in release, clippy and format clean, the generated files written again unchanged, client 153, sim-worker 53,
  types 13, net 1, boundary 39, 29 Chromium tests in 215 s, the recordings what their scripts make), the gate with smoke,
  CI green (toolchain 8 min 48 s to 13 min 2 s; the job's limit is 25 minutes). CI keeps only the newest push's
  toolchain run on a branch, so a notes push cancels the code commit's own run; the run read is the newest head's,
  whose code is the same. On the final code 914f3b4: gate and smoke green (run 46073641654), toolchain green (run
  46073641671, 8 min 48 s). [after: the final head's runs.]
- **Planted defects** (`plants-4b2.py`): on c442a80 (84 plants plus 4 stamp and 6 native ones), the first whole run
  caught 83 of 83 and held 10 of 11 controls (P19's page step ran after a refused build, with no stamp: a defect of
  the script, fixed), the second held all. Round 1's fix pass added 20 entries, each fix shown red on the defect it
  closes and green again (W14 to W16, C38 to C54), and made a control hold only if it also says its words; the whole
  run on 70f11e1 caught 97 of 97 and held 17 of 17. Round 2's fix pass added 9 (W17, C55 to C62) and moved W16's
  anchor. **The whole run on the final code, 914f3b4** (`plants-4b2.txt`, 21:58 to 22:04 UTC): **104 defects caught
  of 104, 19 controls held of 19**, every file put back (32 files by sha256), the players' file rebuilt to its
  digest (a624ff32b1cc...), the clean tree green. Not planted, said so: the spawns check against `MIN_SPAWNS` itself
  (no test can see it while `MIN_SPAWNS` is 1) and the docs' words.
- **Loading content changed no state:** at cut B, `first-loop.hashes` lines 2 to 1,801, `first-loop.vector.txt` and
  the 1,800 step records equal main's `git show` copies; since then every recording file is byte for byte cut B's. A
  content byte folded into the shared state fails at step 1 natively, in Node and in Chromium (plants C17 to C19).
- **The players' file:** 54,514 bytes on main, 217,692 with the loader (cut C), 257,421 with the world's tree
  (step 10; gzip 100,215, brotli 82,101); the built-file check walks 95,441 instructions and refuses nothing; no
  word of a description or a schema in it. At each fix pass the same size and instruction count, and a new sha256:
  2e8866474b22... (round 1's), then **a624ff32b1cca3b48712d85662fba01f2c21f3304c638d663369a228737400b6** (round 2's,
  the final code) (was 3660132b...): the file carries the path `crates/sim/src/collision.rs` once (panic places of
  the world's build) and the fix passes moved lines there and in `map.rs`; the ray cast is in no step yet, so not in
  the file (rust-reviewer's own builds of round 1's: the code section byte-identical; after round 2's spawns check
  the same instruction count, its sameness not compared byte by byte).
- **The browser:** the page's content spec (one fetch, the served bytes the built file's, one changed value refused at
  its file and line, seven hostile bundles each shown only as the loader's own sentence for its class, whole, with
  no element, a failed fetch starting no Worker); the isolation spec's dev server serves the build's bundle byte for
  byte, refuses another site's ask with 403 and a fixed sentence, and passes an address with no path to Vite's own
  handlers. Firefox runs the rules and replay specs only (CI).
- **The table's names** (test-engineer's TE-1, High, at round 1): the second route reads 5 of the table's 15 values,
  and a swap between any two rows it does not read passed every check (its proof: the rows and `soldier.json` swapped,
  `--check` and every cargo test green). Step 8's sentence ("a row swapped within one of those [equal] pairs passes
  every check") stated that gap too small. Closed at the fix pass: a test on the real table holds every row's field
  to the last step of its Export name, but for one named rename, so no two rows can trade fields, equal values or
  not; plant C38 (two rows of equal values trade fields, the content unchanged) is caught by it, and its controls C39
  and C40 show that both routes that compare values pass that swap. Since round 2 the same test also holds each
  row's Export group to the soldier's own (test-engineer's TE-9: his Export also holds the hornet's body radius and
  step; C58 caught, C59 held).
- **Not verified:** the boundary and page specs run in Chromium, not Firefox; no Firefox run of the players' file's
  content refusals; the collision world is built but queried by nothing in a match (4c); the ray's new paths (the
  reach, the cap, the ramp's guard) run natively only, their sameness in WebAssembly read from their form
  (comparisons, no new arithmetic), not run; the players' file's new digests put down to moved line numbers (the
  same size and instruction count; rust-reviewer's own builds at the first fix pass), not proved byte by byte.

## Reviews
Round 1 on c442a80, six reviewers, then one fix pass (70f11e1); round 2 on what changed (c442a80 to 70f11e1), five
reviewers, then its one fix pass (914f3b4). The design reads during the build: rust-reviewer (steps
0, 10, cut C), security-auditor (the batch, the advisories, the test list), netcode-architect (route (i)),
engine-architect, test-engineer (the test list).
- **Round 2: every reviewer PASS, Lows only, every round-1 finding closed or deferred as agreed; no third round.**
  Fixed in its one fix pass: the ramp's guard now refuses an infinity as well as a NaN (rust-reviewer's two probes,
  each a wrong hit at t = 0 in a code-built world; W17); `RAY_REACH`'s doc says the limit is in t, metres only for a
  unit direction (rust-reviewer); the spawns check reads `MIN_SPAWNS` itself (rust-reviewer; no plant can see it
  while it is 1); the table's rows are held to the soldier's groups of his Export, since it also holds the hornet's
  body radius and step (test-engineer's TE-9; C58 caught, C59 held); the dev server's refusal tested for `same-site`
  (C55), the canonical tool's base tested with `content/` as a link (C56), the tools resolve a path before they
  check and open it (C57 for reads, C62 for writes) (security-auditor); the types-to-schemas test checks the shape's
  corners too (engine-architect's F1; C60); EA-1's sentence below (engine-architect's F2). Left as follow-ups:
  `serve.test.mjs`'s two link tests still skip under CI (outside this diff, from 3a); RR-1's three new paths run
  natively only, so 4c commits one ray of each kind (netcode-architect); member kinds are not compared by the types
  test (4c's reader). No reviewer read round 2's fix pass itself; its one change to the players' file is `map.rs`'s
  spawns check (the ramp's guard in `collision.rs` is product code no step calls yet, so not in that file: the
  verifier's wording). [after: the advisor agent's reading of it.]
- **Round 1, test-engineer: REVISE, one High.** TE-1 (the second route left a swap of two of the ten rows it does not read
  open; step 8's sentence stated the gap too small): fixed (the table-name test; C38 caught, C39 and C40 held; its
  own case C52 caught, C53 and C54 held). Lows: TE-2 (two native `cfg!` assertions can never fail, sim's
  `compile_error!` stops such a build first: said here), TE-3 (a link test skipped where it cannot make a link:
  fails under CI now), TE-4 (no meeting ray with a -0.0 direction part: added; the per-class minimum counts of the
  committed rays: 4c), TE-5 (the hostile bundles passed on any refusal: each pinned to the loader's sentence), TE-6
  (a wall-clock bound in a correctness spec: kept as a backstop), TE-7 (a test title that claimed too much:
  retitled), TE-8 (plants run on a folded players' file with no paired control: clean builds first, controls C50
  and C51).
- **Round 1, rust-reviewer: PASS with one Medium and two Lows.** RR-1 (Medium: a ray with huge finite numbers overflowed in the cast and gave
  a wrong hit; no caller in 4b-2): fixed (the reach, the cap, the NaN guard; W14 to W16 caught). RR-2 (Low: the
  schema said nothing of the fewest spawns): fixed. RR-3 (Low: a stale doc): fixed. Rulings (1), (8), (9): fine.
- **Round 1, engine-architect: one Medium, the rest Low or carried to 4c.** EA-1 (Medium: after the loader's refusal the page kept its copy of the
  bundle, up to 8 MiB, for its life): fixed, the page's handling moved into two tested functions that `boot.ts`
  calls (C41, C42 caught); those two calls of `boot.ts` are read and type-checked, not tested (engine-architect's
  round 2). Lows: EA-2 (types and
  schemas from two sources, untied): a tie test (C47); EA-3 (two sentences for this report): written above, under
  "Decisions and deviations"; EA-4 (the page's own copy of the format and version): tied to the loader's (C46); EA-5
  (= NA-2) fixed; EA-6 (test helpers in the shipped crate's docs): hidden; EA-7 and EA-8: 4c's plan. Ruling (3)
  (the map's reading moved with its reader) confirmed.
- **Round 1, security-auditor: PASS with three Lows.** SA-1 (an address with no path made the plugin's error page): fixed
  (C44); SA-2 (another site's page could make the developer's browser start the content tool): fixed, 403 and
  nothing started (C43); SA-3 (the tools checked one folder for a link, and opened following one): fixed (C45).
  Rulings (2) and (4) confirmed, (1) fine.
- **Round 1, netcode-architect: PASS, two Lows.** NA-1 (the Node runner checked the content before what it can replay):
  fixed; NA-2 (a stale doc): fixed.
- **Round 1, infra-engineer: PASS, two forward Lows, nothing to fix in 4b-2** (no round 2: nothing in its lane
  changed). IE-1 (CI's Chromium run uses 3.6 to 3.9 of its
  5 minutes: a split or a new limit before 4c adds a long spec; a CI file, so the CI Builder's) and IE-2
  (`content.json` keeps a fixed name beside hashed files: no-cache or a hashed name when hosting is built).

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
- In round 1's fix pass: C4 now runs the hand-over's own check alone (the new table-name test in the same file
  catches C4's pair); a control holds only if it also says the words it names (C4, C40 and C54 must show their one
  test passing); the new plants' words are the failure's own (Playwright prints the failing line's neighbours too,
  so C43 and C44 read the assertions' own messages); the ray test's first form used `next_up`, which the simulation
  crate's source scan refuses (ADR 0004 part 7's list), so the next float is made from its bits (found by the local
  checks before the commit).
- In round 2's fix pass: security-auditor's finding on the normalised path (its notes' L2-2) named `readPlain`;
  `writePlain` got the same `path.resolve`, and
  its test and plant (C62) were added on the advisor tool's read, which found the write side untested (the local
  checks were stopped and run again on it). Round 2's fix pass was read by no reviewer (the budget's two rounds);
  the advisor agent reads it on the final head.
- Slips of this chat's rules: listed in the card's "Slips" (times typed ahead of the clock, git and gh after another
  program in reads, a merge commit without attribution, the CI limit told as 15 minutes when it is 25, two wrong
  counts of the fix pass's plants told in the chat, one notes commit whose note on the board said more than that the
  add-on is active).

## Risks
1. The world is built and held but queried by nothing yet: its first real test is 4c's movement and aim.
2. The players' file grew 4.7 times (mostly the standard library's number reading and writing); recorded, not gated.
3. CI's toolchain run varies with the runner (8:48 to 13:02 on this seat's heads); about 12 minutes under its limit
   at the slowest.
4. Two risks security-auditor left for acceptance, both on the developer's own computer: a browser that sends no
   `Sec-Fetch-Site` header is served by the dev server as a tool would be; a folder swapped for a link between the
   tools' check and their open is followed (a race on the developer's own disk). [after: the advisor agent's word.]

## Follow-ups (in `docs/STATUS.md`'s 4b-2 line and its archive block, by docs-keeper [after: their ids])
- heapless 0.9.4 or later when rstar or parry3d next move (security-auditor's dependency read).
- The licence texts for the hosted build (security-auditor).
- The schema's `unit` column ships in the players' file and nothing reads it at run time (the advisor agent, step 11).
- The first field with a default goes to the advisor agent first, with `read.rs`'s "nothing missing" rule (step 11).
- 4c carries the re-read's condition 4: a recording that covers the first collision query.
- The generated content types have no reader until 4c (EA-3); the local match's world moves to the room when 4c's
  step queries it, and a rule keeps `content` out of the room's and the client endpoint's manifests (EA-7); 4c's
  editor panel imports the schema limit function from product code (EA-8).
- CI's Chromium run near its 5-minute limit (IE-1, the CI Builder's); `content.json`'s fixed name when hosting is
  built (IE-2).
- The committed rays' per-class minimum counts (TE-4, 4c), with one ray of each of RR-1's three new paths (beyond
  the reach, a limit cut to it, a ramp that overflows), which run natively only today (netcode-architect, round 2);
  the static-world spec's wall-clock bound (TE-6, kept).
- From round 2: `serve.test.mjs`'s two link tests still skip under CI where a link cannot be made (security-auditor;
  outside this diff, from 3a); the types-to-schemas test compares keys, corner lengths and words, not a member's
  kind (engine-architect; 4c's reader); `boot.ts`'s two calls of the tested handling are read, not tested
  (engine-architect).

## Proposed next step
Change-set 4c, starting with the page's scene and the map's reading (moved from 4b-2), then the soldier's walk on the
static world.

## Decisions needed
None. [after: unless the advisor agent hands one up, for example on security-auditor's two left risks.]

## For Mustafa
1. Your go in the builder chat for the merge of this pull request at its final head [after: the full id], which
   the advisor agent approved [after]. Your Advisor opens the draft from the title and text in
   `.checkpoints/cs4b-content/pr-text-4b2.md` at commit [after] and merges by merge commit with that head as the
   expected one. There is no Brave check: nothing a player sees changes in a match that runs.
