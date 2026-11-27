# Brief for the verifier: change-set 4b-2 "The world", on its final code

DRAFT until round 2 ends: the marks FINALHEAD and [after] are filled by the main session before you start.

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2`, local branch `cs/4b-content` (pushed as
`claude/ecstatic-wozniak-5cbdzs`). The final code is commit FINALHEAD (`git diff FINALHEAD HEAD -- crates packages
content Cargo.toml Cargo.lock` is empty: the commits on top hold notes only). The change-set is
`git diff b8f02b0149cc928964e54e194cd9c4d68f52bd94 FINALHEAD -- . ":(exclude).checkpoints"` (main b8f02b0 is merged in
at 35e96ad; 4b-2's count base is main b9aad01, and b9aad01 to b8f02b0 touches nothing under `crates/` or `packages/`).
What it is, and the rules that bind every agent in this tree: `.checkpoints/cs4b-content/review-brief-4b2.md` and
`review-brief-4b2-round2.md` (read "Rules for you" first: no source or doc edits; notes only, yours is
`.checkpoints/cs4b-content/verifier-4b2.md`; git read-only, each git command alone in its shell call with nothing in
front of it; the repository is public).

**What to verify: the claims of the report's draft**, `.checkpoints/cs4b-content/report-draft-4b2.md` (the marks
[after] are still to be filled by the main session: CI on the final head, your own verdict, the advisor agent's).
Its "Evidence", "What changed", "Reviews" and "Decisions and deviations" sections are the claims. PASS means: every
number and every statement of fact there that you checked holds, and what you could not check is named. FAIL names
the claim, what you found, and the proof.

**Runs you may make (no other agent runs cargo or the browsers now; docs-keeper edits only `docs/`):**
1. The full local checks, the toolchain job's own commands by hand:
   `bash .checkpoints/cs4b-content/local-checks.sh > .gate/verifier-local-checks.log 2>&1` (about 13 minutes; it
   uses the tree's own `target/`, builds both pages and runs the Chromium tests; it calls no git). It must end with
   `ALL LOCAL CHECKS PASSED`. Read the numbers out of YOUR log: the Rust test counts in debug and release, the
   packages' test counts, the boundary tests, the browser tests and their time, the players' file's size and sha256.
2. The written lines: `git diff --numstat b9aad01954be666340a7ec5c6e9fc9353b11abf1 FINALHEAD -- crates packages`
   (alone in its call, to a file under `.gate/`), read by `node .checkpoints/cs4b-content/count-4b2.mjs <that file>`
   (the rule: `.checkpoints/cs4b-content/advisor-count-rule.md`): the draft's official and raw numbers.
3. His words: `node .checkpoints/cs4b-content/decisions-4b2.mjs check` (his six messages of the change-set in
   `docs/DECISIONS.md` against the saved messages, byte for byte); and every quote of his in the draft and in
   `pr-text-4b2.md` against `docs/DECISIONS.md` or the saved message it names.
4. The tools on the real files: `node packages/client/scripts/testlines.mjs --check`; `cargo run -p devtools --locked
   --bin content_bundle -- --check` (with your own `CARGO_TARGET_DIR`); the recordings unchanged since cut B
   (`git diff e1e2dd4 FINALHEAD -- crates/devtools/recordings` empty).
5. The planted defects: `.checkpoints/cs4b-content/plants-4b2.txt` is the main session's whole run on FINALHEAD (its
   log `.gate/plants-4b2.log`). **Do not run `plants-4b2.py`** (it edits source files and writes that result file).
   Check the result file against the log for five plants of your choice, at least two of them from the fix pass
   (W14 to W16, C38 to C54): the command, its exit, the words found in the output, and that the words are the
   failure's own. Plant TWO defects of your own choice by hand in a COPY of the workspace under
   `.gate/verifier-4b2-scratch/` (with its own `CARGO_TARGET_DIR` there), each of which the draft's evidence says
   would be caught: say which check caught it and with what words.
6. The reviews: each reviewer's verdict and findings as the draft tells them, against their notes (round 1:
   `rust-reviewer-review-4b2.md`, `security-auditor-review-4b2.md`, `netcode-architect-review-4b2.md`,
   `engine-architect-review-4b2.md`, `test-engineer-review-4b2.md`, `infra-engineer-review-4b2.md`; round 2: each
   `-round2.md`): is any finding left out, softened, or called fixed when the tree does not show the fix?
   Spot-check at least six "fixed" rows in the code, TE-1 and RR-1 among them.

**Not yours to verify, say so:** CI's runs (the main session reads them with the GitHub tools [after]); Firefox (CI
only); the legacy gate's smoke (the main session runs it).

**Your final answer** (at most 25 lines): PASS or FAIL; then each claim you checked with what you ran and the number
you got; then every claim that does not hold, with severity (Critical, High, Medium, Low) and the exact correction;
then what you did not check.
