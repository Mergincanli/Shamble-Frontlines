# Brief for the verifier: change-set 4b-1 "Content", on its final code

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/lane1-takeover`, branch `cs/4b-content`. The
final code is commit `2039e3c` (`git diff 2039e3c HEAD -- crates packages` is empty: the commits on top hold notes
only). The change-set is `git diff origin/main...HEAD -- crates packages` plus `Cargo.lock`. What it is, and the
rules that bind every agent in this tree: `.checkpoints/cs4b-content/review-brief-4b1.md` (read "Rules for you"
first: no source or doc edits; notes only, yours is `.checkpoints/cs4b-content/verifier-4b1.md`; git read-only,
each git command alone in its shell call with nothing in front of it; the repository is public).

**What to verify: the claims of the report's draft**, `.checkpoints/cs4b-content/report-draft-4b1.md` (the marks
`[[...]]` are still to be filled by the main session: CI on the final head, your own verdict, the advisor
agent's, the pull request's number). Its "Evidence", "What changed" and "Reviews" sections are the claims. PASS
means: every number and every statement of fact there that you checked holds, and what you could not check is
named. FAIL names the claim, what you found, and the proof.

**Runs you may make (no other agent runs cargo or the browsers now; docs-keeper edits only `docs/`):**
1. The full local checks, which are the toolchain job's own commands by hand:
   `bash .checkpoints/cs4b-content/local-checks.sh > .gate/verifier-local-checks.log 2>&1` (about 9 minutes; it
   uses the tree's own `target/`, builds both pages and runs the 18 Chromium tests; it calls no git). It must end
   with `ALL LOCAL CHECKS PASSED`. Read the numbers out of YOUR log: the Rust test counts in debug and release,
   the packages' test counts, the boundary tests, the browser tests and their time, the players' file's size and
   sha256 (the draft says it is main's: 54,514 bytes, a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094),
   the million-input fuzz run's histogram (in the release step: `cargo test --release -p devtools --locked --test
   content_fuzz -- --nocapture` prints it, and the caps' table with the bytes held).
2. The corpus: `node packages/client/scripts/corpus-texts.mjs --check`; count its parts yourself from
   `crates/devtools/vectors/content-corpus.txt` (26,653 numbers; the draft's split), and check by your own small
   script that the corpus holds real ties at many counts of binary places and both signs of 2^-24.
3. `node packages/client/scripts/legacy-extract.mjs --check`; the time report against the draft's sentence
   (68 of 69; 600 and 60).
4. The written lines: `git diff --numstat origin/main -- crates packages` (alone in its call, to a file under
   `.gate/`), added lines, the seven data files under `crates/devtools/vectors/` left out: 7,164.
5. His words: `node .checkpoints/cs4b-content/decisions-4b1.mjs check` (every line of his five messages of
   2026-11-26 in `docs/DECISIONS.md` against the saved messages, byte for byte); and every quote of his in the
   draft against `docs/DECISIONS.md`.
6. The planted defects: `.checkpoints/cs4b-content/plants-4b1.txt` is the main session's whole run on 21e1550
   (35 caught, 1 control held; its log `.gate/plants-4b1.log`). **Do not run `plants-4b1.py`** (it edits source
   files and writes that result file). Check the result file against the log for five plants of your choice
   (the command, its exit, the words found in the output), and plant TWO defects of your own choice by hand in a
   COPY of the workspace under `.gate/verifier-4b1-scratch/` (with its own `CARGO_TARGET_DIR` there), each of
   which the draft's evidence says would be caught: say which check caught it and with what words.
7. The reviews: each reviewer's verdict and findings as the draft tells them, against their notes
   (`rust-reviewer-review-4b1.md` and `-round2.md`, `engine-architect-review-4b1.md` and `-round2.md`,
   `security-auditor-review-4b1.md` and `-round2.md`, `test-engineer-review-4b1.md`): is any finding left out,
   softened, or called fixed when the tree does not show the fix? Spot-check at least six "fixed" rows in the code.

**Not yours to verify, say so:** CI's runs (the main session reads them with the GitHub tools: gate and toolchain
green on 495204a, runs 45860625806 and 45860625893; on 21e1550 the gate is green, run 45868324364, and the
toolchain run 45868324356 was still going when this was written); Firefox (CI only); the legacy gate (its
packages are not installed in this worktree).

**Your final answer** (at most 25 lines): PASS or FAIL; then each claim you checked with what you ran and the
number you got; then every claim that does not hold, with severity (Critical, High, Medium, Low) and the exact
correction; then what you did not check.
