# Advisor B3: the pushed-head reads of UM-4 (8fb32bc8, pushed in 787eb984) and NB2-ONEWAY (merge cb600ede, pushed in b81061cf), 2026-11-27

Plan: launch-1750-msgs.md section B; main's advisor.md (changed since c9bd268?). Block 1, UM-4: the commit's parents,
files and numstat against um4-plan.md; its done lines; .gate/um4/'s logs; dependency-check.md's UM-4 section; CI on
787eb984. Block 2, NB2-ONEWAY: nb2b-merge-plan.md's plan, NB2B-1 to -11 and its Done lines; the merge's parents and
tree; what it brings against T; .gate/nb2b-merge/'s logs; his "yes" in history.md; CI on b81061cf; PR #40's state; the
two facts to weigh. Then Decisions needed, For Mustafa, the status line. Read only; printing forms, quoted.

- **advisor.md:** main's last change is still c9bd268 (2026-11-12).
- **Heads, printed:** origin/main b8f02b0149cc…; HEAD = origin's branch = 86cee194…; PR #40's head cfbd6a7e….
  `git diff --name-only b81061cf cfbd6a7e` lists 6 paths, none outside .checkpoints/.

## 1. UM-4 (8fb32bc8): APPROVE WITH FINDINGS (one Low, notes only)
- **The commit:**
  - 8fb32bc873452… has parent 65b2f2965aac… (H0, the plan's step 0); 787eb9845869… has parent 8fb32bc8….
  - Its files (`--raw`), all M at 100644: spikes/netcode-proof/check.mjs 29/2, check.test.mjs 58/1, and two notes
    files, dependency-check.md 11/0 and um4-plan.md 56/0.
  - Outside spikes/ and .checkpoints/: nothing (`git diff --name-only 65b2f29 8fb32bc8`, filtered, printed nothing).
- **The tested tree is the committed tree:**
  - `git diff 65b2f29 8fb32bc8 -- spikes/netcode-proof | sha256sum` printed 09c26ef2ecc4…b858.
  - That equals the diff line of counted.log, ops-pass.log and absent.log (each with HEAD 65b2f296 and untracked
    e3b0c442…, none).
- **The step, read against UM4-1 and UM-4 (a)-(e):**
  - `opsStep(runner, present)` fails and runs nothing when ops/browser/node_modules is missing, with one line naming the
    install (OPS_MISSING).
  - It runs `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, both in HERE.
  - It fails on either part's failure, on `# tests 0`, and on a wrappers last line that is not "wrappers.test.sh: N ok,
    0 failed" with N above 0. It reads that last line from stdout.
  - STEPS gains `ops` after `web`, and the header's step list names it.
- **The tests:** five new ones pin both commands (program, arguments, HERE), the failing part with nothing after it,
  the zero count (two forms), four bad last lines, and the missing folder (one line, no call). The list pin gains
  "ops". The done lines record red then green by name (A to D: 79 of 80, 81 of 81, 82 of 85, 85 of 85).
- **The logs, each printed:**
  - light-base.log (15:59:38, exit 0): refs 2166, marks 81 (rows unchanged), self 80 of 80, redact 11 of 11 with 464
    items and 0 findings.
  - absent.log (16:02:31, exit 1): "FAIL ops … ops/browser/node_modules is missing (UM-4 (d)): install it first with
    `npm ci --ignore-scripts --no-audit --no-fund --prefix ops/browser` …".
  - ops-pass.log (16:03:17, exit 0): "PASS ops 8.9 s ops: 104 of 104 tests passed; wrappers.test.sh: 315 ok, 0 failed".
  - counted.log (16:26:19, exit 0): "check: all 13 steps pass"; test 931 and 1,028 passed, 0 failed, 0 skips; wasm
    27,000 ticks, 0 mismatching; refs 2166; marks 81; self 85 of 85 (80 + 5 new); web 70 of 70; ops 104 of 104 and
    "315 ok, 0 failed"; redact 464 items, 0 findings.
  - Every bar of step 5 by B's C1 rule is met.
- **The install:**
  - `ls -d` lists /home/user/Shamble-Frontlines/spikes/netcode-proof/ops/browser/node_modules.
  - package.json and package-lock.json hash 8370495dee2ed77e… and a0efce8e511a8865…, the cleared values.
  - dependency-check.md's section records the form as run (the web step's cleared flags), the hashes before and after,
    and "added 167 packages".
- **CI on 787eb984** (`gh run view`, each printed): gate push 46034034986, gate pull_request 46034043528, toolchain push
  46034034877 and toolchain pull_request 46034043538. Each reads conclusion "success", attempt 1, headSha 787eb984….
  - No workflow names spikes/ or netcode-proof: `git grep -n -i 'spikes\|netcode-proof' 787eb984 -- .github/` printed
    nothing with "git grep exit: 1" beside it, and a control grep in the same file found its line.
  - So UM-4's proof is the counted run above, tied to the commit by its diff hash, not CI.
- **Tier:** 1. check.mjs is the spike's own checker, not a gate file (advisor.md's list).
- **The review round** (test-engineer, security-auditor's short read) runs in its own seats, and its findings are
  answered before UM-4 closes. This read does not wait for it.
- **Finding UM4H-1 (Low, notes):** npm's "added 167 packages" equals the lock's 167 non-root entries (`node -e` over
  package-lock.json printed "lock entries: 168 non-root: 167"). security-r1-deps.md:158 counts the cleared tree as 162
  packages. The lock's hash is the cleared one, so the set is the cleared set. To spare a reader the doubt, the
  section should say so in one clause: "167 = the lock's non-root entries; security-r1-deps.md's 162 counts the tree
  as resolved that day". It is for security-auditor's short read to confirm.

## 2. NB2-ONEWAY (merge cb600ede): APPROVE WITH FINDINGS (two Lows, records only)
- **The merge:**
  - cb600edee64b… has parents d24340b74ce6… (H0) and 29a901994f5b… (T). Its tree is dd5973f48411…, X from NB2B-3's
    merge-tree.
  - b81061cf3884… has parent cb600ede….
  - The message names his word (history.md:8133-8151), T and c56afd07. It equals merge-msg.txt.
- **It brings exactly T's changes:**
  - `git diff --stat H0 cb600ede` and `git diff --stat 591e2cb T` both read "12 files changed, 665 insertions(+), 17
    deletions(-)". `--name-status` gives the 12 paths of the plan's Facts: six Builder 2 notes, and six spike files
    under crates/judge and marks/, all M.
  - `git diff 591e2cb T | sha256sum` and `git diff H0 cb600ede | sha256sum` both print 4186a3759bbe…15dc, byte for byte.
  - `git diff --name-only T cb600ede -- …/crates/judge …/marks .checkpoints/netcode-judge.md .checkpoints/netcode-judge`
    printed nothing. Its control (591e2cb..T over marks/) printed README.md, SERIES.md and marks.json.
  - marks.json at the merge hashes a41ee583982748…24f2, his.
- **His reply:** history.md:8219-8228 holds "yes" (17:03 UTC, transcript line 116029, 3 characters) as his reply to
  the command named at 16:4x. That is NB2-PROMPT (a) and his 12:25 item 3.
- **The logs, each printed:**
  - light-base.log (17:05, HEAD d24340b7, untracked none): refs 2166, marks 81, self 85, redact 464 items, 0 findings.
  - check.log (lock taken 17:20:52, end 17:45:01, exit 0, HEAD cb600ede, diff and untracked e3b0c442…: the merge's tree
    exactly): "check: all 13 steps pass"; test 935 and 1,032 (UM-4's + 4 each), 0 failed, 0 skips; refs 2171 = 2166 + 5;
    redact 464 items (+ 0, NB2B-1), 0 findings; marks 81, rows unchanged; self 85; web 70; ops 104 and 315 ok; wasm
    27,000 ticks, 0 mismatching.
  - judge-tests.log (HEAD cb600ede): "test result: ok." with 19, 4, 87 and 26 passed (136), 0 failed.
  - rejudge-47-measured.log and -bites.log: "PM-0001-47 MOVED moved to Slice 1 change-set 7 (Lag) …".
  - gate.txt: "GATE PASS (5a smoke skipped: --no-smoke)".
  - Every bar of step 4 (NB2B-1, -5) is met.
- **CI on b81061cf** (`gh run view`, each printed): gate push 46044847948, gate pull_request 46044855530, toolchain push
  46044848099 and toolchain pull_request 46044855642. Each reads success, attempt 1, headSha b81061cf…. NB2B-4 is met.
- **PR #40** (`gh api …/pulls/40`): "draft=true mergeable=true state=blocked", so no conflict.
- **The two facts, weighed:**
  - **H0 is the saver's termination autosave, d24340b7, not the explicit save.**
    - `git log` reads "autosave (terminated) [skip ci]". `git diff --name-only d24340b7^ d24340b7` lists only
      .checkpoints/netcode-proof-test.md and history.md: the card's resume line and his reply's entry, exactly what H0
      must carry.
    - The done line records H0 = `git ls-remote` = `git rev-parse HEAD`, and light-base.log's header reads HEAD
      d24340b7 with a clean tree.
    - So H0 is what the plan's H0 means: the pushed commit that carries the resume line. Who saved it does not change
      that. No effect on the merge.
    - **NB2BH-1 (Low, form):** the next plans of this form define H0 as "the pushed commit that carries the resume line,
      by whichever save, `git ls-remote` = `git rev-parse HEAD`", so a termination autosave is foreseen, not found.
  - **Step 4 began at 17:20:52, after a worker restart, not at 17:07.**
    - check.log's own lock line gives 17:20:52, and its header proves the run's tree was the merge's, clean (HEAD
      cb600ede, diff and untracked empty).
    - Nothing ran on the tree in between: the next commit is b81061cf, notes only.
    - The corrected done line is right.
    - **NB2BH-2 (Low, record):** the first time was written before the run started. Done lines record a time from the
      log, after the event.
- **Owed before NB2's report and this merge's report** (his rule of 2026-11-15, history.md:2125-2127: "every change to
  marks.json or to the judge gets a reviewer's read before its unit closes, and your report to me names it in its own
  line with what covered it"):
  - (i) The reviewer's read of `git diff 591e2cb T -- …/crates/judge …/marks` (NB2B-8; the test-engineer seat now,
    review-um4-nb2b-test.md) is in, and its findings are answered.
  - (ii) Each report names it in its own line, with what covered it.
  - (iii) NB2's own report goes out before this one, or with it in separate blocks (NB2B-9).
  - (iv) This merge's report carries NB2BH-1's fact in one line: H0 d24340b7, the saver's termination autosave
    carrying the resume line and his reply's entry; the explicit save found nothing.
  - (v) The corrected 17:20:52.

## Decisions needed
None. Both heads stand. The open items are reports' lines and the review round's answers, inside the plans.

## For Mustafa
1. Nothing to do now.
2. For the day's message: UM-4 landed (the checker now runs U8's operator tests and the wrappers' test itself).
3. Also for the day's message: Builder 2's one-way delay work is merged at the tip he named. Its judge and marks change
   gets its reviewer's line in the reports.

Status: COMPLETE 2026-11-27T18:03Z
