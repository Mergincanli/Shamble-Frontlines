# Brief for the verifier: change-set 4c-1 "The page's scene", on its final code and docs

Written by Builder V6 (the main session), 2026-11-28, after review round 2, the fix pass and docs-keeper's docs pass.
The marks [after] in the report's draft are the main session's to fill after you: your verdict, the final head's CI
runs, the advisor agent's.

**Where.** The git worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2`, local branch `cs/4c-soldier` (pushed as
`claude/ecstatic-wozniak-5cbdzs`). Base: main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f (main has not moved since;
the main session fetches it again before the final head). **The last code commit is
0645432f648457b0e03e2f4d4280eb3389e1da65** (`git diff 0645432f648457b0e03e2f4d4280eb3389e1da65 HEAD -- crates
packages content Cargo.toml Cargo.lock` is empty); **the docs pass is 9283d302**; the commits on top hold notes only.
The change-set: `git diff dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f 9283d302 -- . ":(exclude).checkpoints"`.
What it is: the plan sheet `.checkpoints/cs4c-soldier/reread-sheet.md` (sections D and G), the advisor agent's
verdict `advisor-reread-4c.md`, the tally of every review finding `review-s-round.md`, the card
`.checkpoints/cs4c-soldier.md`.

**Rules for you.** No source, doc or note edits; your notes file is
`/home/user/Shamble-Frontlines/.gate/cs4b-2/.checkpoints/cs4c-soldier/verifier-4c1.md` (plan first, a line after each
check); scratch copies and logs under `/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/`. Git read-only, each git
command alone in its shell call with nothing in front of it (not even `date` or `cd`), one git per call; every file a
shell command writes spelled out with its full path. Never commit. The repository is public: copy no personal detail
into your notes. Agents running beside you: none (docs-keeper and the reviewers are done).

**What to verify: the claims of the report's draft**, `.checkpoints/cs4c-soldier/report-draft-4c1.md` (sections
"Plan and approval", "What changed", "Evidence", "Reviews", "Decisions and deviations", "For 4c-2"), **the pull
request's text** `pr-text-4c1.md`, and **docs-keeper's rows**: in `docs/STATUS.md` 4c-1's row, "Waiting on Mustafa",
the follow-ups "From change-set 4c-1" and the three 4b-2 follow-ups marked met (4B-TYPES-READER for the keys,
4B-MEMBER-KINDS, 4B-BOOT-CALLS); in `docs/STATUS_ARCHIVE.md` the block "From change-set 4c-1" and 4c-1's two risks;
`docs/ROADMAP.md`'s 4c-1 entry; `docs/REPO_MAP.md`'s new rows. PASS means: every number and every statement of
fact there that you checked holds, and what you could not check is named. FAIL names the claim, what you found, and
the proof.

**Runs you may make:**
1. **The full local checks**, the toolchain job's own commands by hand:
   `bash .checkpoints/cs4c-soldier/local-checks.sh > /home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/verifier-local-checks-4c1.log 2>&1`
   (about 13 minutes; the tree's own `target/`; it builds both pages into `.gate/dist-production` and
   `.gate/dist-development` and runs the Chromium tests; it calls no git). It must end with `ALL LOCAL CHECKS
   PASSED`. Read the numbers out of YOUR log: the packages' test counts, the boundary tests, the browser tests and
   their time, the render counts the first page reports.
2. **The written lines:** `git diff --numstat dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f 0645432f648457b0e03e2f4d4280eb3389e1da65 -- crates packages`
   (alone in its call, redirected to a file under `.gate/`), read by `node .checkpoints/cs4c-soldier/count-4c.mjs <that file>`.
3. **His words:** `node .checkpoints/cs4c-soldier/decisions-4c.mjs check`, `node .checkpoints/cs4c-soldier/section14-4c.mjs check`,
   `node .checkpoints/cs4c-soldier/loop-check-4c.mjs`, `node .checkpoints/cs4c-soldier/quote-check-12bc.mjs`; and every
   quote of his in the draft and the pull request's text against `docs/DECISIONS.md` or the saved message it names
   (`.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md` and the other saved messages there).
4. **The planted defects:** `.checkpoints/cs4c-soldier/plants-4c.txt` is the main session's whole run on 2b2f44c2
   (its log `.gate/plants-4c.log`). **Do not run `plants-4c.py`** (it edits source files and writes that result file).
   Check the result file against the log for five plants of your choice, at least two of the fix pass's (F1 to F17),
   F1 (the hang, caught by the outer timeout) among them: the command, its exit, the words found, and that the words
   are the failure's own. Plant **two defects of your own** by hand in a COPY of the workspace under
   `/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/verifier-4c1-scratch/`, each of which the draft's evidence says
   would be caught (for example a ramp's top on the wrong side, or a patch of ground half its depth): say which
   check caught it and with what words.
5. **The reviews:** each reviewer's verdict and findings as the draft tells them, against their notes (round 1:
   `review-s-graphics-engineer.md`, `review-s-engine-architect.md`, `review-s-perf-auditor.md`,
   `review-s-security-auditor.md`, `review-s-art-director.md`, `review-s-test-engineer.md`; round 2:
   `review-s2-graphics-engineer.md`, `review-s2-test-engineer.md`; the tally `review-s-round.md`): is any finding left
   out, softened, or called fixed when the tree does not show the fix? Spot-check at least six "fixed" rows in the
   code at 0645432f, GE-F1 and TE-1 among them.
6. **The after picture:** retake it from your local checks' development build:
   `node .checkpoints/cs4c-soldier/shot-4c.mjs /home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/dist-development /home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/scene-after-4c1-verifier.png 2`
   (from the worktree's root; it prints the page's render counts and backend). Look at it: does it show what the
   report's "For Mustafa" step 2 and 3 say he will see (the houses, the yard's blocks, the street and the lane a
   little darker than the grass)? Say what does not match.
7. The gate (optional): the main session's run at 9283d302 passed with smoke (`.gate/gate-4c1-docs.log`); you may run
   `PLAYWRIGHT_BROWSERS_PATH=/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/pw-1194 node tools/gate.mjs` yourself.

**Not yours to verify, say so:** CI's runs (the main session reads them); Brave and a real GPU; WebGPU (the
container's Chromium has none); the scene in Firefox.

**Your final answer** (at most 25 lines): PASS or FAIL; then each claim you checked with what you ran and the number
you got; then every claim that does not hold, with severity (Critical, High, Medium, Low) and the exact correction;
then what you did not check.
