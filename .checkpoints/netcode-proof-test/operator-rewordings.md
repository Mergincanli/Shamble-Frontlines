# The operator rewordings to printed output (choice 31), the main session's plan (written 11:3x UTC on 11-27)
Status: PLAN, for the advisor tool before any edit. Its plan check is B's block 3 (c) (advisor-u8-f1-head.md:104-137,
"three net-zero rewordings in one small change by the main session ... no security round"). Net zero, so U8-FILLS
does not touch it; any part that cannot be made net zero waits for his answer and is named in the day's message.

## The sweep (11:2x UTC; printing forms)
- Grep over ops/operator for `--quiet`, `$?`, `LASTEXITCODE`, "exit code", "exit 0", "exits", "any exit", `-eq 0`,
  `||`, `&&`: PROMPT-R1.md:22, PROMPT-R1.md:127, rules.md:15 (B's three). Out of scope, as B's block 3 frames it (steps
  where the operator session reads a result): pre-push:19, :24, :29, :37 (`exit 1`, a hook git itself reads) and
  redact-check.mjs's own `--quiet` option and its code.
- Grep over ops/operator/*.md and runs/R1.md for `grep -q`, `test -`, `[ -`, "prints nothing", "is empty", `-Quiet`,
  `Test-Path`, "silent": PROMPT-R1.md:34 (`git config --get core.hooksPath` prints nothing: already a printing form) and
  R1.md:187 ("never silent", about verdicts). Nothing new.
- Tests that read the two files: operator.test.mjs (both; :98 pins PROMPT-R1.md:22's old form word for word; :69-70
  takes every backticked `git ...` span and wants start check 4's two `git -C` reads; 3.6 checks them against every
  deny), r1plan.test.mjs (:153, :207-211: the pushed list, the `git -C <results> rev-parse --git-path hooks` span and the
  S14-VMS phrase, whitespace-tolerant), browser.test.mjs:133 (rules.md's install forms, untouched here).
- Widths: no test pins them; today PROMPT-R1.md has lines of 140 (:97), 122 and 121, rules.md two of 121.

## The four edits (by Edit, in this order)
E1 (red first). operator.test.mjs:98:
  `    "git -C <run> diff --quiet origin/main -- CLAUDE.md .claude",` becomes
  `    "git -C <run> diff --name-only origin/main -- CLAUDE.md .claude",`
  Then operator.test.mjs alone: the prompt test fails, naming the new string (red, recorded).
E2 (green). PROMPT-R1.md:22-23, B's words:
  `4. \`<run>\` at X: \`git -C <run> rev-parse HEAD\` is X, and \`git -C <run> diff --name-only origin/main -- CLAUDE.md .claude\``
  `   prints nothing; \`.gate\npt-results\` holds no \`CLAUDE.md\` and no \`.claude\`. On a mismatch, stop.`
  (121 and 98 characters). Then operator.test.mjs passes (green, recorded).
E3. PROMPT-R1.md:127-129, B's words rewrapped within the same three lines (127, 130 and 123 characters):
  `   summary. Unless its saved last line reads \`… 0 finding(s)\` (an empty file included), no push: stop and tell him. Just before`
  `   the push, in PowerShell, \`git -C <results> rev-parse --git-path hooks\` must print the clone's own hooks folder, the one holding`
  `   pre-push; anything else: stop and tell him. Then push to \`claude/npt-results\` after the pre-push hook passes (a prompt).`
E4. rules.md:15, B's words, one line (135 characters; the bullet cannot take it at 120 or less in its four lines, as
  each backticked span stays on one line):
  `  (\`git diff --name-only origin/main -- CLAUDE.md .claude\` prints nothing), the results worktree holds neither; on a mismatch it stops.`

## Two wording questions for the advisor tool (B's words are kept unless it rules otherwise)
- Q-A: "(an empty file included)" sits after the quoted line, so it can read as part of the pass. Moved after "no push"
  ("no push (an empty file included)") it can only mean the stop. Same words, same length.
- Q-B: "`… 0 finding(s)`": a line ending "10 finding(s)" also ends in "0 finding(s)"; the space before the 0 is what
  tells them apart. redact-check's summary is "redact-check: <n> item(s) checked, <m> finding(s)", so "`… checked, 0
  finding(s)`" would leave no doubt (9 characters more, still within E3's three lines at 136 or less).

## The proof (B's list), each its own call
P0 base, before E1: `node --test` of operator.test.mjs and r1plan.test.mjs by absolute path (they find their files
  from their own folder, helpers.mjs:10-13), counts recorded; `node spikes/netcode-proof/ops/operator/redact-check.mjs
  spikes/netcode-proof`, items and findings recorded.
P1 red after E1; P2 green after E2 (operator.test.mjs).
P3 after E3 and E4: both test files pass with P0's counts; redact-check 0 findings (items as P0); `git diff --numstat --
  spikes/netcode-proof/ops`: operator.test.mjs 1 1, PROMPT-R1.md 5 5, rules.md 1 1; the changed lines printed by
  `awk` with their numbers, and quoted.
P4: one commit of the three files (its message naming choice 31 and B's block 3 (c)), pushed by a checkpoint save;
  CI on the pushed head.
No cargo, no main-run.sh (it touches every file under crates/), no lane lock: node only.
