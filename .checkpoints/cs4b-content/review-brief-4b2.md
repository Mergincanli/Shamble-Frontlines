# Review brief, change-set 4b-2 "The world" (round 1), common to every reviewer

Written by Builder V6 (the main session), 2026-11-27. Your own focus is in your section at the end.

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2`, local branch `cs/4b-content` (pushed as
`claude/ecstatic-wozniak-5cbdzs`), code head **c442a806d83dceac6a272b22ad67b7f01de476fa** (commits after it touch
only `.checkpoints/`, and `docs/DECISIONS.md` at 1c4906e: his six messages of the change-set, copied by script and
checked back by `.checkpoints/cs4b-content/decisions-4b2.mjs check`; added after this brief was first written). Main b8f02b0 is merged in (35e96ad), and main has not moved since. The change-set is
`git diff b8f02b0149cc928964e54e194cd9c4d68f52bd94 c442a806d83dceac6a272b22ad67b7f01de476fa -- . ":(exclude).checkpoints"`:
100 files, about 9,850 lines added, of which data are `content/` (about 1,200), `Cargo.lock` (300),
`crates/devtools/vectors/` and the recordings; the written-line count by the official rule is 7,978
(`.checkpoints/cs4b-content/advisor-count-rule.md`). 4b-2's count base is main b9aad01; b9aad01 to b8f02b0 touches no
file under `crates/` or `packages/`. The plan: `.checkpoints/cs4b-content/reread-sheet.md` (revision 3, section G,
rows 6 to 12, their revisions marked in bold), `test-list.md` (steps 6 to 11), `step9-order.md` (step 9's three
cuts). The card, with every step's numbers, evidence and departures: `.checkpoints/cs4b-content.md`, from "## 4b-2
(the second pull request)" to "## Slips". **Step 12 (the page's scene) is not in this change-set:** Mustafa moved it,
with the map's reading field by field, to the start of 4c (his STEP12-GO (a), 2026-11-27,
`.checkpoints/cs4-walk/mustafa-2026-11-27-step12-go.md`). So 4b-2 changes nothing a player sees in a match that
runs: the page still draws the empty scene.

**What it is, by step** (the commits: `git log --oneline b9aad01..c442a80 -- crates packages content Cargo.toml Cargo.lock`):
- **6** the collision library (parry3d 0.31.1; 33 lockfile packages) in a manifest batch security-auditor approved by
  its digests (`deps-4b2-brief.md`, `security-auditor-deps-4b2.md`), its overflow overrides, the feature compare by
  name and version with his FEATURE-COMPARE (a)'s two named exceptions (`question-2026-11-26-feature-compare.md`),
  the dependency rules (`crates/devtools/tests/dependency_rules.rs`).
- **7** the built-file check (`packages/client/scripts/build-checks.mjs`: the names, the instructions, the target
  features, the imports list, fused multiply-add, the banned maths, the maths allow-list, the crates the players'
  file is built without, the stamp), run by `build.mjs` on both twins before the bindings tool and on the production
  file after it; `crates/sim/tests/rules_scan.rs`; the native target tests.
- **8** the tools and the data: `content_bundle` (writes `content.json`; `--check`), `testlines.mjs` (his Export to
  the movement table; `--check`), `canonical.mjs`; `content/source/` (his Export at 30 steps, its manifest, the
  hand-over table), `content/movement/soldier.json` (from his Export, by script), `content/maps/test-scene.json`;
  the tests on the real files.
- **9** content to the simulation: the generated content types (`crates/content/src/ts_export.rs`); the page
  folder's plain files and the markup-sink scan; cut A (every page carries `content.json`); cut B (the first
  recording names its frozen content `crates/devtools/recordings/first-loop.content.json`, and every replay checks
  it); cut C (`Sim::new(seed, mode, pad_bytes, bundle)`; the Worker's `start` with the bundle; the page's
  `packages/client/src/app/content.ts`; the development server's `/content.json`; the message caps).
- **10** the static collision world (`crates/sim/src/collision.rs`, `vec.rs`; its tree's bytes and its ray cast;
  the fuzz run and the hostile maps through the builder) and its wiring (the local match holds the world of the map
  `test-scene`; `CreateError::MapMissing` and `::World`; a replay of another map's recording refused).
- **11** the schema writer (`crates/content/src/schema.rs`, test-only; `packages/types/src/generated/content/schema/`;
  the export test's schema folder; a schema-driven limit test, `packages/types/src/content-schema.test.mjs`).

**Rulings the build follows, each for your judgement (say "fine" or a finding):**
(1) rust-reviewer's two maths allow-list entries `C::round` and `C::__multi3` (`rust-reviewer-cutC.md`;
`packages/client/scripts/maths-allowed.mjs`). (2) The message caps (`advisor-cutC.md`, Q1): a thrown text cut to
512 UTF-16 code units, a panic's text whole (its buffer's 1,016-byte room), the page's check of an `error` at 1,040;
a departure from "about 512" in `test-list.md`'s rules for the whole part, for security-auditor to confirm. (3) The
page's parse (Q2): `content.ts` reads the bundle's own fields after the Worker's `ready` and keeps nothing; the map's
reading field by field moved to step 12, now 4c's: a departure from engine-architect's design 4.5, for
engine-architect to confirm. (4) The development server's `/content.json` (Q3): only block 1.9's serving part,
through build.mjs's own `writeContentBundle`. (5) Step 11's rulings (`advisor-step11.md`): no `default` column, the
descriptions in a test-only table, the schema's form (its changes C1 to C7), the schema files counted in full.
(6) The count rule (`advisor-count-rule.md`). (7) The order of building: step 10's core and the pieces of steps 9
and 10 that need no content file came before step 8's second part (while his DIFF-LIST was open); step 9 in three
cuts; cut C and step 10 were committed before their full local checks ended (this environment's Stop hook asks
every change committed and pushed before a turn ends), each check then passing on the same files. (8) The
departures the card records at each step, among them: at step 7, `fma4` dropped from the native assertion, the
vector library's exponential as the plant for its maths, two more exported functions than the sheet named; at step
8, the manifest beside his Export written by script, and the second route's five values (a swap within one of the
three equal pairs passes every check: said as not verified). (9) The players' file: 54,514 bytes on main, 257,421
now (the content loader at cut C, the collision library's tree at step 10); recorded, not gated (ADR 0002's G7).
(10) The re-read's condition 4: no step of the match queries the world in 4b-2, so a recording that covers the
collision queries is 4c's.

**Evidence.** For every code head of this seat: the full local checks (`.checkpoints/cs4b-content/local-checks.sh`:
format, clippy, `cargo test` twice with the generated files written again unchanged, the release tests, the wasm32
builds, the vector generators' checks, each package's tests, the type checks, both page builds, the boundary tests,
the Chromium tests, the recordings what their scripts make), the gate with smoke, and CI (gate, smoke, toolchain)
green: the card's entries give the times and counts. The plants: `.checkpoints/cs4b-content/plants-4b2.py` (84
plants; one counts as caught only when its command fails and its output holds the check's own words); its whole run
on c442a80 writes `.checkpoints/cs4b-content/plants-4b2.txt` and its log `.gate/plants-4b2.log`. The recordings:
cut B against main's `git show` copies (`recordings-proof.mjs` and `.txt`); unchanged since (`git diff e1e2dd4 --
crates/devtools/recordings` is empty).

**Rules for you.**
- You edit NO source file, no doc and no note but your own. You write only your notes,
  `.checkpoints/cs4b-content/<your-name>-review-4b2.md` (your plan first, then each finding as you make it, so a
  restart loses nothing), and scratch files (text, small scripts) beside them; builds go only under your target
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
  built). No network. A defect you plant to prove a point goes in a COPY under `.gate/<your-name>-4b2-scratch/`,
  never in the tree. Never `llvm-objdump -d` on the twins (it filled the disk once); the notes' node scripts
  (`rust-reviewer-cutC-*.mjs`) read a twin's names and bodies.
- The repository is public: put nothing in your notes that is not already in the repository.
- Severity, one scale: Critical (breaks security, data, the guard or main), High (a wrong result or rule a player,
  Mustafa or a session would hit; evidence that does not hold; his words misquoted or loosened), Medium (a real
  defect with a workaround or a small reach), Low (polish, wording). Cite file:line and what you ran for every
  finding; say what you did NOT check.
- Your final answer to the main session: at most 25 lines: your verdict (PASS, or findings to fix), then each
  finding as `severity | file:line | what is wrong | the evidence | the fix you suggest`, most severe first, then
  what you did not check. Opus, the effort set for you; no other agents.

## rust-reviewer
The Rust of steps 6, 9, 10 and 11: `crates/sim/src/collision.rs` against the four conditions of your step-10 read
(`rust-reviewer-step10.md`), the constructor's checks and their order (`crates/sim-wasm/src/lib.rs`), the local
match's world (`local_match.rs`), `crates/devtools/src/replay.rs`'s checks, `files.rs`, `scripts.rs`,
`crates/content/src/schema.rs` and `ts_export.rs` (the folder rules), the manifests and the overflow overrides; the
same bits natively and in WebAssembly; the players' file's size (what grew, and whether anything of a test-only
module or a description reached it); your two allow-list entries as built.

## security-auditor
The development server's `/content.json` (`packages/client/vite.config.mjs`: a request starts a process on the
developer's computer; the server answers on 127.0.0.1 only; the fixed folder `.gate/dev-content`; the middleware
before Vite's own; GET of that one path; the 500 answer's fixed sentence); the caps' departure (ruling (2): confirm
or refuse; your P10); the page's fetch and its sentences (no response body, status text, content key or value, or a
caught error's text reaches the page); the Worker's check of `start` before the glue copies the bundle; the imports
list and the production file's rules on the players' file with content and the library linked; `content/source/` and
the tools' walks (links, pipes, unknown folders); the export test's folder rules (your S-should 18); the dependency
batch as you approved it (any drift since); the built-file check's no-skip rules.

## netcode-architect
Steps 9 and 10: your route (i) as built (the frozen content, the header's digest and map id, the check in every
runner: native, the test module, Node, Chromium); "loading content changed no state" (the recordings byte for byte,
and the plants C17 to C19: a content byte folded into the shared state fails at step 1 natively, in Node and in
Chromium); one content set per match, through the constructor only (CD-7); the replay of another map's recording
refused; the world held by the local match and queried by no step; condition 4 carried to 4c.

## engine-architect
The module boundaries: content, sim, sim-wasm, devtools, the Worker and the page; the boundary's shape (one type,
plain numbers per tick, the bundle crossing once, at the constructor); confirm or refuse the departure from your
design 4.5 (ruling (3)); the development route against your block 1.9; the schema writer against your block 1.10 and
ADR 0005's CD-3; the generated content types, which have no reader in 4b-2 now that the map's reading moved.

## test-engineer
Is it proven: the test list's items for steps 6 to 11 (`test-list.md`, your T-must and T-should), each against the
test that holds it; the plants' whole run (`plants-4b2.txt`: each caught on the check's own words, the controls
held, every file put back); what is said as not verified (the swap within an equal pair at step 8; what runs in
Firefox and what does not); whether any evidence claimed in the card does not hold.

## infra-engineer
CI: the toolchain job's times on this seat's heads (9 min 43 s to 13 min 2 s, the slow one a slower runner: every
phase about a third slower) against its 25-minute limit (#78) and the browser tests' 5-minute one; the development
server's `cargo run --locked` under the job's network cut (`.gate/ci-fbbd84d-toolchain.log` is not committed: read CI's
own log of run 46033756121 with `gh api`, read-only, if you need it); anything in 4b-2 that would make a preview link
or a later hosted build behave differently from CI's page (`content.json` beside `index.html`; the 25 MiB per-file
limit; the players' file's size).
