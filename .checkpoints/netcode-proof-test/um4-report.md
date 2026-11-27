# UM-4, the check script's `ops` step: the report (DRAFT until B5's head read of the fix commit)
Built on 2026-11-27 by the main session. Plan um4-plan.md rev 0.3, approved by the advisor agent (A2, UM4-1 to UM4-8),
built after his U8-FILLS (a) (history.md:8137). Two commits:
- 8fb32bc8, pushed in 787eb984: the step, its tests and the install record.
- f1b46682, pushed in 1d4e81a: the round's fix pass.

Reviews: the test-engineer and the security auditor read 8fb32bc8 (review-um4-nb2b-test.md block 1;
review-um4-security.md); the advisor agent read 787eb984's head (advisor-um4-nb2b-head.md block 1), ruled on the round
(advisor-um4-r1.md) and reads f1b46682's head (advisor-um4-fix-head.md: to fill).

## What it is
check.mjs's 13th step, `ops`:
- **Runs:** U8's operator tests (`node --test 'ops/**/*.test.mjs'`, Node's own glob), then `bash ops/tests/wrappers.test.sh`,
  both in the spike folder.
- **Fails, never skips,** when:
  - either part fails;
  - no Node test ran;
  - the wrappers' last stdout line is not "wrappers.test.sh: N ok, 0 failed" with N above 0;
  - ops/browser's install is missing. It then runs nothing, with one line naming the install, at its absolute path.
- **The environment:** neither call gets a variable whose name begins GIT_ (SEC-UM4-1, below).
- **CI:** CI never runs check.mjs (no workflow names the spike), so the step is proven by local counted runs. A CI net
  for later heads is T-5, carried.

## Size, against his U8-FILLS (a)
- **198 lines added in all,** against the "about 20-30" his answer was given on:
  - 8fb32bc8: check.mjs 29/2, check.test.mjs 58/1;
  - f1b46682: check.mjs 13/5, check.test.mjs 98/10.
  These are outside U8's stop, as he ruled.
- **U8's own files:** 3 lines rewritten in place (operator.test.mjs 1/1, redact-check.test.mjs 2/2), 0 net. U8 stays at
  6,282 of his 6,284. That is the advisor agent's reading of a 0-net change; it is his to reverse.

## The install (dependency-check.md, its UM-4 record)
- **The form:** ops/browser's real install, in the web step's cleared form (`npm ci --ignore-scripts --no-audit
  --no-fund`, npm_config_ignore_scripts=true).
- **The files:** package.json and package-lock.json hashes unchanged.
- **The count:** "added 167 packages", in the security read's corrected words: the lock's non-root entries (160
  distinct name@version and 7 duplicate placements). The 162 in security-r1-deps.md is a resolver's distinct count that
  adds two packages npm never places.
- **The tools:** npm 10.9.9 and node v22.23.4, read after the install. The registry was not recorded at the install;
  the lock's sha512 integrity binds every tarball.

## Evidence
- **8fb32bc8:** the cold counted run passed all 13 steps (test 931 and 1,028; ops 104 of 104 and "315 ok, 0 failed";
  self 85; redact 464 items, 0 findings). CI 4/4 green on 787eb984.
- **f1b46682, red first by name on the shipped code** (red-first.log: 3 of 3 failing, each for its own assert):
  - the env pin: each call got no env where the GIT_-free set was expected;
  - the victim test: a scratch repository that GIT_DIR names changed, its HEAD, last message and objects;
  - the missing-folder pin: the relative prefix.
- **The three restores:** each of U8's three old lines, restored alone, turns the victim test red (V1, V2, V3). So the
  advisor's widening, UM4R-1, is shown on a throwaway repository, not only reasoned.
- **Mutants, each killed by a named test:**
  - M1 and M3 (the either-part test);
  - W1 (the web step's test);
  - M4 and M10 (the T-2 test);
  - M8 (the first ops-step test's path pin);
  - M-env and M-wild (the env pin).
  After them, the four fixed files' sha256 all matched, so no mutant reached the commit.
- **The counted run** through main-run.sh (`--only self,ops,redact,refs`): self 88 of 88, ops 104 of 104 and "315 ok, 0
  failed", redact 464 items and 0 findings, refs 2181, all equal to H0's base.
  - Its diff hash 446f383d… equals the hash of `git diff` from H0 to f1b46682, so the tested tree is the committed tree.
- **Gate and CI:** GATE PASS (--no-smoke), repo map OK. CI 4/4 green on 1d4e81a: gate 46066802730 and 46066808752,
  toolchain 46066803492 and 46066808757.
- **This repository:** untouched by the deliberately vulnerable runs. No `side` branch exists, and HEAD is still the
  lane's branch.

## The review round and its ruling
- **The test-engineer (T-1, Medium):** the either-part test could not fail. Its stand-ins tripped later guards. Fixed;
  M1 and M3 prove it now.
- **The security auditor (SEC-UM4-1, Medium):** with an absolute GIT_DIR inherited (a git hook exports one), U8's
  temporary-repository tests would commit into the repository it names. The first such commit deletes every tracked
  file.
  - No caller does that today: this shell has no GIT_DIR, and the lane runner sets none.
  - The advisor widened it (UM4R-1) to the two redact tests, which also run in the redact step.
  - Fixed both halves: the step's environment and the three helper lines.
- **The Lows:**
  - taken: T-2, T-4, SEC-UM4-2, SEC-UM4-3, UM4H-1;
  - carried: T-3, UM4R-2, T-5.
- **The ruling:** every item APPROVE; no second round (NPT-TRIM: no Critical or High).
- **A correction:** 8fb32bc8's message said "Five new tests, each red by its own name first". That held for three of
  the five. f1b46682's message carries the correction, and history is not rewritten.

## Carried (the card's Open findings, for PR #40's closing docs pass)
- **T-3:** a skipped or todo test passes the ops and web steps.
- **UM4R-2:** the redact step's test run passes the whole environment. The refs step's `git ls-files` is for B5 to rule
  beside it.
- **T-5:** a CI job for check.test.mjs, infra-engineer's, after the close. It is a .github/ change: tier 2, his ask
  first.
- **The security auditor's memory line:** met twice, u3c C2b F2 and SEC-UM4-1.

## Not verified
- The hazard under a real git hook. That is by design: no git variable is ever exported here; the victim test sets one
  only inside its child's environment, naming its own scratch repository.
- Whether `git rebase -x` or `git bisect run` export GIT_DIR.
- B5's head read: to fill.

## Decisions needed
None. OPS-GIT-ENV-B was not raised, since all three of U8's lines stayed 0 net.

## For Mustafa
1. Only if you read your 6,284 stop as counting lines rewritten in place: say so. The three lines then come out by a new
   commit, and the tests' half of SEC-UM4-1 reopens on PR #40's closing list.
