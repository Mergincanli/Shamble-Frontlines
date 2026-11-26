# Review brief, change-set 4b-1 "Content" (round 1), common to every reviewer

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/lane1-takeover`, branch `cs/4b-content`, head
`ef2edec` (main `3b949c1` merged in). The change-set is everything under `crates/` and `packages/` that differs
from `origin/main`: `git diff origin/main...HEAD -- crates packages` (about 6,600 written lines; the files under
`crates/devtools/vectors/` are data). The plan it was built from: `.checkpoints/cs4b-content/reread-sheet.md`
(revision 3, sections A to E and the step table G, steps 1 to 5) and `test-list.md`; the card with every step, its
numbers and its departures: `.checkpoints/cs4b-content.md` ("Steps done", 9 to 17).

**What it is.** The content crate (`crates/content`: no dependency at all) with one reader for one cut grammar of
JSON (Mustafa, 2026-11-26: "Keep the reader to what the content files need and refuse everything else, duplicate
keys included. Fuzz it before the first value rests on it."), one canonical writer, the bundle, two kinds (movement,
map), the aim's reach in the loader (his note: "until the aim codec widens, the loader refuses a playable map larger
than its reach"), the time unit (3,000 units a second) with its rounding report, the links check on the recordings'
readers (his LINKS-CHECK (c)), and the proofs: a number corpus and a tree corpus made by a browser engine, a seeded
generator of 37 classes of broken files with each case's verdict worked out by the generator, committed digests
compared natively, in WebAssembly under Node and in Chromium and Firefox, a 1,000,000-input fuzz run, 25 planted
defects (`.checkpoints/cs4b-content/plants-4b1.py`; first run in `.gate/plants-4b1.out`). No content file exists
yet; nothing in the players' file changes (the simulation does not depend on the content crate before 4b-2).

**Departures from the sheet, each for your judgement (say "fine" or a finding):** (1) the proofs live in
`crates/devtools/src/content_proofs.rs`, not in `crates/content/tests/` (the WebAssembly side runs the same
generator, and content depends on nothing); (2) `Field` and `Range` live in `read.rs`; (3) the four classes that
make megabytes run a tenth of the per-class count; (4) the browsers run the release profile of the test module only
(the dev profile runs under Node); (5) the tree corpus is new; (6) the reader names a forbidden byte as that wherever
it stands; (7) the writer's rule for a number exactly between two shortest texts (`even_of_a_tie` in `number.rs`),
found by the corpus; (8) every corner of a map but the bounds lies on whole millimetres, the bounds on whole
centimetres, so that "at least 1 cm thick" is exact; (9) the loader asks for at least one spawn, not for the
match's slot count; (10) a map's own rules name the field and no line; (11) the reviewed list of duration fields is
a data file the script reads (`crates/devtools/vectors/legacy-duration-paths.txt`, game-designer's read; `FIXED_DT`
is kind `step` by the builder's call), the report does not list the fields that are no durations, and it has no
shot counts; (12) the links check is 223 lines where his answer said "about 15" (two reads of 40 lines, the rest
planted links and a scan); (13) `#[rustfmt::skip]` on `fn case` and `impl Map` in `content_proofs.rs` (laid out as
tables by the formatter itself at 120 columns).

**Rules for you.**
- You edit NO source file and no doc. You write only your notes: `.checkpoints/cs4b-content/<your-name>-review-4b1.md`
  (your plan first, then each finding as you make it, so a restart loses nothing) and scratch files (text, small
  scripts) beside it; builds go only under the target folder named below.
- Git: read-only commands only (`git diff`, `git log`, `git show`, `git status`), and every git command starts its
  own shell call with nothing in front of it: no `cd x && git`, no `git -C`, no `git -c`, no pipe or variable before
  it. NEVER git add, commit, push, switch, checkout, restore, stash, merge, reset or worktree.
- Start with `cd /home/user/Shamble-Frontlines/.gate/lane1-takeover` alone in its own shell call.
- Builds and tests: allowed only with your own target folder, `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/lane1-takeover/.gate/<your-name>-4b1-target`
  on every cargo call, always with `--locked`; never `cargo fmt` without `--check`; never `cargo update`, `cargo add`
  or anything that writes `Cargo.lock`. Do NOT run `packages/client/scripts/build.mjs`, the Playwright browser tests,
  `npx`, `npm` or anything that writes into `packages/` or `.gate/rules-module/`: the main session owns those (the
  Node tests can be run with `node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir packages/sim-worker/boundary <name>`).
  No network. If you plant a defect to prove a point, do it in a COPY under `.gate/<your-name>-4b1-scratch/`, never
  in the tree.
- The repository is public: put nothing in your notes that is not already in the repository.
- Severity, one scale: Critical (breaks security, data, the guard or main), High (a wrong result or rule a player,
  Mustafa or a session would hit; evidence that does not hold; his words misquoted or loosened), Medium (a real
  defect with a workaround or a small reach), Low (polish, wording). Cite file:line and what you ran for every
  finding; say what you did NOT check.
- Your final answer to the main session: at most 25 lines: your verdict (PASS, or findings to fix), then each
  finding as `severity | file:line | what is wrong | the evidence | the fix you suggest`, most severe first, then
  what you did not check. Opus, the effort set for you; no other agents.
