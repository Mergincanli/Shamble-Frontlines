# Review brief, change-set 4b-2 "The world", round 2 (on what changed), common to every reviewer

Written by Builder V6 (the main session), 2026-11-27. Your own focus is in your section at the end. This is the
review budget's second and last round (`CLAUDE.md`, "Review budget"): it reads only what the fix pass changed, and
your own round-1 findings with what was done about each. A third round comes only for an open Critical or High.

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2`, local branch `cs/4b-content` (pushed as
`claude/ecstatic-wozniak-5cbdzs`). Round 1 read code head **c442a806d83dceac6a272b22ad67b7f01de476fa**. The fix
pass is code head **70f11e130ee9dc0aabbb61a01bb9d3a6c013418e** (commits after it touch only `.checkpoints/`). What you read:
`git diff c442a806d83dceac6a272b22ad67b7f01de476fa 70f11e130ee9dc0aabbb61a01bb9d3a6c013418e -- . ":(exclude).checkpoints"`, and your own notes of round
1, `.checkpoints/cs4b-content/<your-name>-review-4b2.md`. Between the two heads, `docs/DECISIONS.md` changed too
(1c4906e: his six messages of the change-set, copied by script and checked back by
`.checkpoints/cs4b-content/decisions-4b2.mjs check`): not yours to review unless your section names it.

**What the fix pass did, finding by finding** (round 1's six notes files hold each finding in full; your notes may
name them otherwise: security-auditor's L-a, L-b and L-c are SA-1, SA-2 and SA-3; engine-architect's M1 is EA-1 and
its L1 to L7 are EA-2, EA-4, EA-5, EA-6, EA-7, EA-8 and EA-3 in that order; netcode-architect's L1 is NA-2 and its L2
is NA-1; rust-reviewer's ramp finding, re-rated Medium, is RR-1, the spawns' `minItems` RR-2 and the stale doc RR-3;
test-engineer's findings are TE-1 to TE-8 as its final answer numbered them, kept in the card's entry "Round 1,
test-engineer" in `.checkpoints/cs4b-content.md`):
- **TE-1 (High)** a swap between two rows the second route does not read passed every check. Fixed: a test on the
  real table (`packages/client/scripts/testlines.test.mjs`, "every row of the table sends its Export value to the
  field of the same name, but for one named rename"): every row's field is the last step of its Export name, but for
  `stamina.refill_time_from_empty_s` to `stamina_refill_time_s`; 15 filled rows, each field once. The second route's
  doc (`crates/devtools/tests/content_files.rs`) says the names hold the other ten rows. Plant C38 (two rows whose
  Export values are equal, 0.164 and 0.164, trade fields: the content needs no change) is caught by that test; the
  controls C39 and C40 show both value routes pass it.
- **RR-1 (Medium)** a ray with numbers near `f32::MAX` could overflow inside the cast. Fixed in
  `crates/sim/src/collision.rs`: `RAY_REACH` (2^20, 1,048,576 m); a ray whose origin or direction holds a number
  beyond it gives no hit, a limit beyond it is cut to it, and a ramp whose slope arithmetic gives a NaN gives no
  hit. Test `a_ray_beyond_the_reach_and_a_ramp_beyond_f32_give_no_hit` (rust-reviewer's probe at scales 1 and 1e5, a
  hit on the slope, the reach's edges and the next float above, a direction of 1e-45 with an endless limit, a ramp
  at plus and minus 3e38). Plants W14 (the reach), W15 (the cap), W16 (the NaN guard): each caught by that test.
  (The next float above the reach is made from its bits: the simulation crate's source scan refused `next_up`, a
  float method outside ADR 0004 part 7's list, in the test's first form; found by the local checks before the
  commit.)
- **RR-2 (Low)** the schema said nothing of the fewest spawns. Fixed: `map::MIN_SPAWNS` (1), the loader's refusal
  names it, the schema writes `minItems` 1 for spawns (regions: none), `map.schema.json` written again (+1 line);
  two schema tests; plant C48.
- **RR-3 (Low)** words: the order test's doc in `crates/sim-wasm/src/lib.rs` names the six checks in their order.
- **EA-1 (Medium)** the page's wiring of the Worker's answers to its copy of the bundle had no test. Fixed in
  `packages/client/src/app/content.ts`: `takeWorkerMessage` and `takeWorkerError`, which `boot.ts` calls; however the
  match ends before `ready` the copy is let go, on `ready` it is read once; `PageContent.holding`; two wiring tests
  in `content.test.ts`. Plants C41 and C42.
- **EA-2 (Low)** the generated types and the schemas could drift apart. Fixed: a test in
  `packages/types/src/content-schema.test.mjs` ties them (the same keys, the same corner lengths, the same words).
  Plant C47 (a key dropped from a generated type).
- **EA-3 (report only)** two sentences for the report: in its draft, `.checkpoints/cs4b-content/report-draft-4b2.md`.
- **EA-4 (Low)** the page's bundle format and version were its own copies. Fixed: `BUNDLE_FORMAT` and
  `BUNDLE_VERSION` exported from `content.ts`, tied to `crates/content/src/bundle.rs` by
  `packages/client/src/app/content-ties.test.mjs` (new). Plant C46.
- **EA-6 (Low)** test helpers were public in the docs. Fixed: `#[doc(hidden)]` on `test_bundle()` and the three
  `TEST_TEXT`s (`bundle.rs`, `movement.rs`, `map.rs`), with a doc line telling a test that rests on a map's shapes to
  name its map.
- **NA-1 (Low)** the boundary's replay test ran the content check and the constructor before the recording's own
  check. Fixed: `checkReplayable` first (`packages/sim-worker/boundary/replay.test.mjs`).
- **NA-2 (= EA-5, Low)** `NO_CONTENT`'s doc in `crates/sim-wasm/src/local_match.rs` now says zeros until change-set
  8 brings the bundle's sha256 into the join (ADR 0005, CD-8).
- **SA-1 (Low)** an address `new URL` cannot read threw inside the plugin. Fixed in
  `packages/client/vite.config.mjs`: `pathOf` gives no path for it, and the request goes on to Vite's own handlers.
  The isolation spec asks `//` and checks no 500. Plant C44.
- **SA-2 (Low)** another site's page could make the developer's browser start the content tool. Fixed: a request
  whose `Sec-Fetch-Site` is there and is neither `same-origin` nor `none` gets 403 and a fixed sentence, and starts
  nothing. The isolation spec asks with `cross-site`. Plant C43.
- **SA-3 (Low)** the tools checked the file's own folder for a link, not the folders above it. Fixed in
  `packages/client/scripts/testlines.mjs` (`plainFolders` from the base down, the base itself first; a read opens
  with `O_NOFOLLOW` and checks the handle with `fstat`), and `canonical.mjs` passes its base; a test of a link
  higher up and of a read outside the base. Plant C45.
- **TE-2 (report only)** the two native `cfg!` assertions stand behind sim's `compile_error!`: the report says so.
- **TE-3 (Low)** a link test that could not make its link skipped. Fixed: under CI it fails
  (`packages/client/scripts/build-checks.test.mjs`, `linksOrSkip`).
- **TE-4 (Low)** no ray that meets a shape had a -0.0 direction part. Fixed: -0.0 rays in the shared-plane test.
  The per-class minimum counts of the committed rays: deferred to 4c, with the casts' first caller.
- **TE-5 (Low)** the hostile bundles passed on any refusal. Fixed: each is pinned to the loader's own sentence
  (`packages/e2e/tests/skeleton/content.spec.ts`), read from the players' file under Node by
  `.checkpoints/cs4b-content/hostile-sentences.mjs`, never typed. Plant C49 (the loader's duplicate-key sentence
  changed).
- **TE-7 (Low)** the schema limit test's title overclaimed. Fixed: retitled.
- **TE-8 (Low)** C20 to C29 ran on the players' file C19 leaves folded. Fixed in the plants script: C21 and C26
  build it clean first; the clean controls C50 (every boundary test) and C51 (the replay, isolation and content specs
  in Chromium). C4 now runs the hand-over's own check alone (`-t`), because the new table-name test in the same file
  catches C4's pair.
- **Deferred, each to STATUS as a follow-up:** EA-7 and EA-8 (4c's plan), IE-1 (the CI Builder's file), IE-2 (a
  hosting change-set), TE-4's per-class counts (4c), TE-6 (kept as a backstop).

**Evidence on 70f11e130ee9dc0aabbb61a01bb9d3a6c013418e.** The full local checks
(`.checkpoints/cs4b-content/local-checks.sh`: ALL LOCAL CHECKS PASSED, 21:17 UTC; 236 Rust tests twice, 235 in
release, client 152, sim-worker 53, types 13, net 1, boundary 39, 29 Chromium tests in 217 s), the gate with smoke
(PASS), CI (gate, smoke, toolchain: the main session reads it), and the plants: their run by name on the fix pass's
changed or new entries, and the whole run (`.checkpoints/cs4b-content/plants-4b2.txt`, its log
`.gate/plants-4b2.log`): WHOLERUN. The players' file: 257,421 bytes and 95,441 instructions walked, as at c442a80;
its sha256 changed (2e886646... from 3660132b...): the file carries the path `crates/sim/src/collision.rs` once (the
world's build has panic places there) and the fix pass moved lines in that file (read so, not proved byte by
byte; rust-reviewer may judge it). The card
(`.checkpoints/cs4b-content.md`, "The fix pass, round 1's findings") gives the times and the counts. The official
count (`.checkpoints/cs4b-content/advisor-count-rule.md`): 8,347 of 8,450.

**Rules for you** (as in round 1).
- You edit NO source file, no doc and no note but your own. You write only your notes,
  `.checkpoints/cs4b-content/<your-name>-review-4b2-round2.md` (your plan first, then each finding as you make it, so
  a restart loses nothing), and scratch files (text, small scripts) beside them; builds go only under your target
  folder below.
- Git: read-only commands only (`git diff`, `git log`, `git show`, `git status`), and every git command starts its own
  shell call with nothing in front of it: no `cd x && git`, no `git -C`, no `git -c`, no pipe or variable before it.
  NEVER git add, commit, push, switch, checkout, restore, stash, merge, reset or worktree.
- Start with `cd /home/user/Shamble-Frontlines/.gate/cs4b-2` alone in its own shell call.
- Builds and tests: only with your own target folder,
  `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/<your-name>-4b2-target` on every cargo call,
  always with `--locked`; never `cargo fmt` without `--check`; never `cargo update`, `cargo add` or anything that
  writes `Cargo.lock`. Do NOT run `packages/client/scripts/build.mjs`, the plants script, the Playwright browser
  tests, `npx`, `npm`, or anything that writes into `packages/`, `content/`, `.gate/rules-module/` or a `.gate/dist-*`
  folder: the main session owns those. The Node tests run with `node node_modules/vitest/vitest.mjs run --root
  packages/sim-worker --dir packages/sim-worker/boundary <name>` (they read the players' file the main session
  built), and a package's own tests from inside it with `node ../../node_modules/vitest/vitest.mjs run <file>`. No
  network. A defect you plant to prove a point goes in a COPY under `.gate/<your-name>-4b2-scratch/`, never in the
  tree. Never `llvm-objdump -d` on the twins.
- The repository is public: put nothing in your notes that is not already in the repository.
- Severity, one scale: Critical (breaks security, data, the guard or main), High (a wrong result or rule a player,
  Mustafa or a session would hit; evidence that does not hold; his words misquoted or loosened), Medium (a real
  defect with a workaround or a small reach), Low (polish, wording). Round 2's findings get this round's one fix
  pass; whatever is still open after it, below Critical or High, becomes a STATUS follow-up (a third round only for
  an open Critical or High). The written-line count stands at 8,347 of the 8,450 stop line, so say for each fix you
  suggest about how many lines it takes. Cite file:line and what you ran for every finding; say what you did NOT
  check.
- Your final answer to the main session: at most 20 lines: your verdict (PASS, or findings), then each finding as
  `severity | file:line | what is wrong | the evidence | the fix you suggest`, most severe first, then for each of
  your round-1 findings one word (closed, open, or deferred as agreed), then what you did not check.

## rust-reviewer
Your RR-1 as built: is 2^20 the right bound for this world (the maps' `bounds` cap, the tree's numbers), do your four
conditions of step 10 still hold with the reach, the cap and the NaN guard, the test's values (its probe, the edges,
the tiny direction, the large ramp) and whether any ray within the reach now answers differently; RR-2 (`MIN_SPAWNS`,
the refusal, the schema's `minItems`, the regenerated file); RR-3's words; EA-6's `#[doc(hidden)]` and the doc line;
TE-4's -0.0 rays; any other Rust in the diff. The same bits natively and in WebAssembly for the new branches.

## security-auditor
Your SA-1, SA-2 and SA-3 as built: the `Sec-Fetch-Site` rule (a request that sends none: a tool, not a browser;
`same-origin`; `none`; `same-site` and `cross-site` refused), the 403's headers and sentence, `pathOf` and what goes
on to Vite's handlers; the tools' folder walk from the base (the base itself, every folder down, `O_NOFOLLOW`, the
handle's `fstat`, a read outside the base refused); TE-5's pinned sentences (what the page shows of a hostile bundle,
whole and as text); TE-3 (no skip under CI).

## engine-architect
Your EA-1, EA-2, EA-4 and EA-6 as built: `takeWorkerMessage` and `takeWorkerError` (every way a match ends before
`ready`, and `ready` itself), what `boot.ts` now does, `PageContent.holding`; the types-to-schemas tie (what it reads,
what it would miss); the format and version tie to `bundle.rs`; EA-3's two sentences in the report's draft; whether
the deferrals EA-7 and EA-8 to 4c's plan stand.

## netcode-architect
Your NA-1 and NA-2 as built; and RR-1 for the simulation: the reach, the cap and the NaN guard give the same answer
on every target (native, the test module, Node, Chromium), change no committed recording or ray line, and leave
condition 4 (a recording that covers the collision queries) with 4c.

## test-engineer
Your TE-1, TE-3, TE-4, TE-5, TE-7 and TE-8 as built: the table-name test against your swap (and its rename's one
row), C38 with its controls C39 and C40, C4's narrowing; the plants' new entries and their words (each a failure's
own); the whole run on 70f11e130ee9dc0aabbb61a01bb9d3a6c013418e; the deferral of TE-4's per-class counts and TE-6; whether any evidence the card now
claims does not hold.
