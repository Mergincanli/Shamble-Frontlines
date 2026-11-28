# Advisor: UM-4's plan, and the form's printing wording (10:50 UTC window, seat A2, 2026-11-27)
Advisor agent, day session, effort xhigh, read only (no cargo, no git write). Acting by main's advisor.md, re-read by
`git show origin/main:.claude/agents/advisor.md` (unchanged on main since c9bd268). Lane HEAD 5814256 (the sender's).

## Plan
1. Read um4-plan.md rev 0.2 against my UM-4 (advisor-u7-merge.md:39-54); re-read its Facts in the code at HEAD
   (check.test.mjs's pin, check.mjs's STEPS, nodeCounts, the web step's npm form, redact-check's SKIP_DIRS, the refs
   step's `git ls-files`, toolchain.yml, u8-brief.md:524, security-u8-deps.md C3, security-u7-deps.md (3)-(4), B's C1
   rule) by output-carrying forms. Rule block 1 and answer Q1-Q4.
2. Read u8-f1-merge-plan.md:83-87 and :108-110 and exit-silent.md; write the printing wording for M3a and M6's undo,
   each check with a positive control where "nothing printed" is the pass. Block 2.
3. Decisions needed, For Mustafa, Status.

## Notes
- HEAD's anchors, by `git grep -n` (11:1x): check.test.mjs:2847 the step-list pin; check.mjs:1984 nodeCounts, :1989
  summarized, :2663 `export function webStep(runner = run, base = process.env, installed = installedTypescript)`,
  :2666-2667 the npm form (`npm_config_ignore_scripts: "true"`, `["ci", "--ignore-scripts", "--no-audit", "--no-fund",
  "--prefix", WEB]`), :2677-2678 the zero-test rule ("A glob that selects no test exits 0 with "# tests 0" (U7R-TE-4):
  that fails, never passes."), :2696 refs' `git ls-files -z`, :2713 `web:`, :2714 `redact:` (which runs
  ops/operator/redact-check.test.mjs itself), :2722 lastLines. check.test.mjs:1980-1981 pin npm's args and environment.
- redact-check.mjs:277 `const SKIP_DIRS = new Set([".git", "node_modules", "target", ".gate"]);`.
- `git check-ignore -v --no-index spikes/netcode-proof/ops/browser/node_modules/x`: ".gitignore:1:node_modules/".
- CI: `git grep -n -E "check\.mjs|netcode-proof" HEAD -- .github/` printed nothing; control `git grep -c gate\.mjs`
  on .github/ printed "gate.yml:3". So CI does not run the spike's check.mjs.
- `git diff --name-only ddb06ff HEAD -- spikes/` printed nothing; control `6968215 ddb06ff` lists U8-F1's six.
- Main folder: package.json 8370495d…4c90 and package-lock.json a0efce8e…9524 (sha256, = Facts); `ls -la` of
  ops/browser/ shows no node_modules (the real absent case exists today).
- wrappers.test.sh ends `printf '\nwrappers.test.sh: %d ok, %d failed\n'` then `[ "$fails" -eq 0 ]`: exit 1 on a
  failure, but nothing fails it when no check ran.
- security-u7-deps.md:223-226 (SEC-U7D-3): without `--no-audit`, npm "send[s] the dependency list to the audit endpoint
  during install, the refused `npm audit` by another door". security-u8-deps.md C3 (c): the 4 moderate lines are
  expected only "if U8 runs `npm audit` on the laptop".
- The saver stages `git add -A -- DIR` (checkpoint.mjs:194), so during a build it commits notes on its own.
- main's advisor.md last changed in c9bd268 (`git log -1 --format=%h origin/main -- .claude/agents/advisor.md`), re-read
  whole by `git show` at 11:1x. Its plan checklist (symbols, evidence for the change type, risks, reviewers, Brave
  list, risk tier, checkpoint, agents' model and effort) is carried by UM4-2 and UM4-8.
- `git grep -n '"redact"'` over check.mjs and check.test.mjs: only check.test.mjs:2847 lists the step names. The
  only other list is check.mjs's header comment (UM4-2).

## UM-4's plan, ruled
APPROVE WITH CONDITIONS UM4-1 to UM4-8 (one Medium, seven Low). Applied word for word, they need no new check before
the build. The build still waits on his U8-FILLS answer (not ruled here).

The plan carries my UM-4 (a)-(e) in order, with today's line numbers. I checked these of its Facts against HEAD, and
they hold: the pin, STEPS, nodeCounts, the npm form, SKIP_DIRS, the refs step's `git ls-files`, the ignore rule, CI not
running check.mjs, both package hashes, and spikes/ unchanged since ddb06ff. The order line does not hold (UM4-6).
Step 5's bars match B's C1 rule (advisor-u8-f1-merge.md section 2's table, which I read) and U8-F1's M5 done line
(u8-f1-merge-plan.md:217: "ops 104 of 104, 0 fail; wrappers "315 ok, 0 failed"; … "all 12 steps pass""). One correction is mine: my UM-4 (b) gave `npm ci` without `--no-audit` and asked
for "its audit line". By SEC-U7D-3, that command would be the refused `npm audit` by another door. Rev 0.2's flags fix
my words, and I withdraw them.

### Conditions (apply word for word)
- **UM4-1 (Medium): the zero-count rule.** In step 1, after "failing if either fails", add: "and failing when node's
  `# tests` count is 0, or when the wrappers' last line is not `wrappers.test.sh: N ok, 0 failed` with N above 0
  (the web step's U7R-TE-4 rule, check.mjs:2677-2678); each case has its own test, red first."
  Why: a glob that selects nothing exits 0, and wrappers.test.sh exits 0 when no check ran. Without this rule a whole
  run could pass with no ops test run, which is the one thing the step exists to stop.
- **UM4-2 (Low): the form and its pins.** In step 1, add:
  - "The step is `export function opsStep(runner = run, present = <the real folder check>)`, with `ops: (runner = run)
    => opsStep(runner)` in STEPS after `web`, as webStep (check.mjs:2663, :2713)."
  - "Its tests pass a recording runner and a stand-in `present`, and pin both commands' program, arguments and folder
    (HERE), as check.test.mjs:1980-1981 pin npm's."
  - "check.mjs's header step list gains `ops`."
- **UM4-3 (Low): the real missing folder.** Add between steps 2 and 3: "2b. One main-run.sh call, `node check.mjs
  --only ops` (no cargo), into .gate/um4/absent.log: FAIL with step 2's one line naming the install. The main folder has
  no ops/browser/node_modules before step 3."
- **UM4-4 (Low): the preconditions and the saver.** Step 0 opens with U8-F1's M0 preconditions
  (u8-f1-merge-plan.md:61-71), each in a printing form (exit-silent.md):
  - no agent or workflow on the lane, the lane lock free, no cargo, rustc or npt process;
  - HEAD = origin, and the tree clean;
  - `git fetch origin main`, then `git rev-parse origin/main` = b9aad01… (a code, CI or rules move stops: MAIN-52);
  - free disk at least 8,192 MiB;
  - the saver stopped by TaskStop with its proof, and the no-save window running from H0 to step 7's push.
  Add: "Step 0 runs at the build, after his U8-FILLS answer, the same day as step 5 (choice 28)."
  Why: the saver commits notes on its own (checkpoint.mjs:194), and one new tracked file between step 0 and step 5
  moves the refs bar.
- **UM4-5 (Low): step 5's spikes/ proviso.** Replace "provided `git diff --name-only ddb06ff HEAD -- spikes/` lists only
  UM-4's two files (else named first)" with "provided `git diff --name-only ddb06ff HEAD -- spikes/` prints nothing
  (control: `git diff --name-only 6968215 ddb06ff -- spikes/` lists U8-F1's six) and `git status --short
  --untracked-files=all -- spikes/` lists only check.mjs and check.test.mjs, as modified (else named first)".
  Why: before step 7's commit the two files are not in HEAD, so a form that compares two commits cannot list them.
- **UM4-6 (Low): the order line.** In Facts, replace "A UM-4 commit to them before that fast-forward makes the staged
  copies differ from the target, and git refuses it. So UM-4's commit waits until that worktree is fast-forwarded, or
  until A rules it stays (the form's fallback (b))." with "C's target is the fixed commit 0efbbef9…, so no later lane
  commit changes what the builder fast-forwards to. UM-4's commit still waits until C's result is read (C was sent
  11:0x on 11-27), so one thing moves at a time."
- **UM4-7 (Low): step 3's folder.** In step 3, replace `--prefix spikes/netcode-proof/ops/browser` with `--prefix
  /home/user/Shamble-Frontlines/spikes/netcode-proof/ops/browser`. Like the web step's absolute WEB, the command then
  does not depend on the folder it runs in.
- **UM4-8 (Low): the plan's missing protocol lines** (CLAUDE.md protocol item 2). Add after Why:
  - "Change type: logic in the spike's check script. Evidence: red-then-green Node tests on the real code, the real
    missing-folder run (2b), the counted run."
  - "Risk tier 1."
  - "Mustafa's Brave check: none (nothing a player sees, hears or feels)."
  - "Reviewers, one round on the final diff, in the window of the head read (LEAN-MODE's new-agent rule), each at its
    agent file's model and effort:
    - test-engineer: the step, its tests and 2b's log;
    - security-auditor, a short read: the step's two commands, and step 3's install record against its own clearances
      (security-u7-deps.md items (3)-(4) and SEC-U7D-3; security-u8-deps.md C2 and C3)."
  Why security too: CLAUDE.md step 5 gives it deps, and nothing in main's advisor.md lets me leave it out of a plan
  that runs `npm ci`. A clean install of its own cleared lock in its own cleared form should make that read short.

### The plan's questions
- **Q1: is a stand-in for the folder check enough?** For the self tests, yes, with UM4-2's pins. It does not reach the
  real folder check, so a wrong path there would go unseen. UM4-3's one real run of the missing case covers that.
- **Q2: the order of the install?** Right. The red and green tests need no install, and the counted run does. UM4-3's
  real missing-folder run must come before the install, between steps 2 and 3.
- **Q3: what does the plan leave out or add, against my UM-4?**
  - Left out: the zero-count rule (UM4-1) and the protocol lines (UM4-8).
  - Added, and accepted:
    - step 0, B's C1 rule;
    - the web step's flags on U8's command (Q4);
    - no audit line (this follows from Q4).
  - Two side effects, accepted:
    - the ops glob also runs ops/operator/redact-check.test.mjs, which the redact step runs again. U8-F1's M5 count of
      104 includes it, so the bar stands.
    - Whether Node's glob enters node_modules is left to the counted run's 104, as the plan says.
- **Q4: which npm form, and is npm ci's audit "npm audit, never retried in any wording"?** The web step's form:
  `env npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund --prefix <absolute ops/browser>`.
  - npm ci's own audit falls under that rule. SEC-U7D-3 (security-u7-deps.md:223-226) names it "the refused `npm audit`
    by another door". U8's form without `--no-audit` would send that request from this session.
  - U8's "4 moderate" stays security's reading of the same lock. C3 (c) expects those lines only "if U8 runs `npm
    audit` on the laptop".
  - Step 3 records no audit line. Step 4's note says: "npm audit not run (`--no-audit`, SEC-U7D-3); security's U8
    reading of this lock stands (security-u8-deps.md C2, C3 (c))".
  - No new security read is needed before the install. Both parts are cleared, and the two flags only remove network
    calls. Security's read of the final diff is UM4-8's.

## The form's printing wording, given
These replace my section 3 words (advisor-u7-head.md) where the merge form quotes them (u8-f1-merge-plan.md:83-87 and
:108-110), from the form's next use. The meaning stays the same: each `git diff --quiet` becomes the same comparison
with `--name-only`, and "exit 0" becomes "prints nothing". `git update-index --refresh`'s bar is read from what it
prints. One control line proves once, in the same step, that the form prints when trees differ. It is fixed history,
so it never changes (exit-silent.md:66-67 used another pair).
- **M3a (its own call, right before M3):** "First the control, its own call: `git diff --name-only 6968215 ddb06ff --
  spikes/` lists six paths (U8-F1's, under ops/), so this form prints when trees differ. Then `git update-index
  --refresh`. main-run.sh:48-52 touches every file under crates/, so a run through it leaves the index's cached times
  stale, and `git apply --index` refuses such a path without reading its content. The bar: it prints nothing, or only
  `<path>: needs update` lines whose paths are all under .checkpoints/. Then each alone: `git diff --name-only --
  spikes/` (tree = index) and `git diff --name-only HEAD -- spikes/` (tree = H0), each printing nothing. A refusal at M3
  after this is a failing bar (M6)."
- **M6's undo:** "The undo starts with `git update-index --refresh` (the same bar: nothing printed, or only `needs
  update` lines under .checkpoints/) and `git diff --name-only -- spikes/` (printing nothing), then `git apply -R
  --index` of the patch, then `git diff --name-only H0 -- spikes/` (printing nothing)."
- Anything printed where "nothing" is the bar, or a git message instead of the expected lines, is that step's failure,
  quoted word for word (at M3a, nothing is applied; in M6, the undo stops there for the advisor).
- Not mine, and not given here:
  - The form's M2 (:82) and M7. exit-silent.md:102 gives both to the main session's next plan of the form.
  - M0's `git diff --quiet 6968215 H0 -- spikes/` (:72) is on no one's owed list in exit-silent.md. It is the main
    session's plan wording too, so it is the main session's to reword (in the same form: `git diff --name-only`, nothing
    printed). Found here and named, nothing more.

## What I did not check
- I ran nothing: no check.mjs, no main-run.sh, no npm.
- Not read:
  - u8-f1-build.md:252 (the 1 failing test without the install);
  - u8-report.md's Node 22.23 line (`node --test ops/` fails);
  - the kept install's `find … -name "*.test.mjs"` (08:5x).
  The plan's step 2b and the counted run re-prove the first and the last.
- Not read: whether Node's `--test` glob enters node_modules (the plan says so too). The counted run's 104 is the bar.
- His U8-FILLS: not ruled (as asked).

## Decisions needed
None new. His U8-FILLS (msg-2026-11-28.md) stays open, and UM-4's build waits on it. Only UM-4's plan is ruled here.

## For Mustafa
Nothing new. His U8-FILLS answer is already on the day message's list.

Status: COMPLETE 2026-11-27T11:15Z
