# UM-4 security read (seat S, launch-1750-msgs.md "S"), security-auditor, 2026-11-27
Commit read: 8fb32bc8 (the `ops` step in spikes/netcode-proof/check.mjs), on branch
claude/netcode-proof-test-plan-hcky5g at b81061cf. Read only: no install, no npm audit, no network, no cargo; this is
the only repository file written (one probe ran in the scratchpad, on stand-in folders there only).

## Plan
- Block 1, the two commands: `run()` (check.mjs:1951-1964: spawnSync, no shell, env = process.env), opsStep's two calls
  (check.mjs:2683-2706). Questions: (a) Node's glob `ops/**/*.test.mjs`: what does it select today, does it reach
  ops/browser/node_modules (third-party code run as a test), does it follow symlinks, can a planted file under ops/
  run as a test; (b) `bash` found by PATH, BASH_ENV/ENV, NODE_OPTIONS and other inherited environment; (c) what
  wrappers.test.sh and the ops Node tests themselves run (sudo, systemctl, network, browser/driver downloads,
  SEC-R1D-3); (d) can output forge the pass (the last-line parse, "# tests N").
- Block 2, the install record: dependency-check.md's UM-4 section against security-r1-deps.md (webdriverio 10.2.0
  standalone only; SEC-R1D-2, -3, -6) and SEC-U7D-3; the lockfile's integrity and resolved hosts (read only). Added by
  the main session at 18:0x UTC: confirm or refute UM4H-1 (advisor-um4-nb2b-head.md:56-60), as its own line.
- Block 3, OPS_MISSING (check.mjs:2692-2693): working directory of `--prefix ops/browser`, what the line leaves out.
- Findings with severity, file:line and evidence; the verdict last.

## Block 1: the two commands (DONE 19:0x UTC; claims narrowed after the advisor tool's check, 20:0x UTC)

### SEC-UM4-1 (Medium): an inherited absolute GIT_DIR makes the ops step commit into that repository, deleting its tree
- What: opsStep runs `node --test 'ops/**/*.test.mjs'` with check.mjs's whole environment (check.mjs:2696 through
  `run`, whose `env` defaults to process.env, check.mjs:1951-1952). One of the files it now selects,
  ops/tests/operator.test.mjs:151-188, builds a "temporary" repository by `git init -q`, `git add .` and
  `git commit -q -m results` (then `a leak`, with leak.txt holding a public resolver address) with cwd = a mkdtemp
  folder and the inherited environment (operator.test.mjs:157; :162 `env: { ...process.env, HOME: home }`). With an
  absolute GIT_DIR exported, git takes GIT_DIR as the repository and the cwd as its work tree, so the commands act on
  the repository GIT_DIR names, not on the temporary folder.
- Exploit sketch (an accident more than an attack): `node check.mjs` (or `--only ops`) run with an absolute GIT_DIR
  in the caller's environment -> two commits on that repository's current branch, the first with a tree of
  verdict.txt alone (every tracked file deleted), the index rewritten; a later save or the background saver can push
  them.
- Not verified: which callers export GIT_DIR. From memory only (no network today): githooks(5) says hooks get
  GIT_DIR and GIT_WORK_TREE exported and must clear them before running git elsewhere; `git rebase -x` and
  `git bisect run` not checked. A relative GIT_DIR would resolve against the test's own cwd (the temp folder) and is
  harmless (git's rule, reasoned, not probed). Today's callers (the lane runner, by hand) export none that I know of.
- Why Medium when block 1's other environment routes are no finding: BASH_ENV, NODE_OPTIONS or a PATH'd `bash` can
  only forge a result on a machine the operator made hostile; GIT_DIR is an ordinary variable git itself exports, and
  the effect is damage to repository data (a tree-deleting commit), not a wrong PASS. Reach is small (no known caller
  today), so Medium, not High.
- Evidence, the probe (git 2.43.0; re-runnable: two empty folders, `real` and `tmp-repo`; real gets a.txt and b.txt
  and one commit "two files"; tmp-repo gets verdict.txt): operator.test.mjs's three calls word for word, through
  execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@example.invalid", ...a], { cwd: tmp-repo, env:
  { ...process.env, GIT_DIR: <absolute real/.git> } }) with a = ["init", "-q"], ["add", "."], ["commit", "-q", "-m",
  "results"]. Output: "real before: 1 commit(s); tree: a.txt,b.txt", "real after: 803357a results / b40973f two
  files", "real HEAD tree: verdict.txt", and tmp-repo has no repository of its own ("fatal: not a git repository").
  Script kept at /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/gitdir-probe/probe.mjs
  while the scratchpad lasts. Nothing in check.mjs or ops/ clears git's variables (Grep for GIT_DIR, GIT_WORK_TREE,
  GIT_INDEX_FILE under spikes/netcode-proof/ops and in check.mjs: none). The test predates UM-4; UM-4 is what puts it
  inside every full check run. Confidence: high on the effect, low-to-medium on how often a caller sets it.
- Fix (either, both best): opsStep passes `runner` an environment without git's location variables (GIT_DIR,
  GIT_WORK_TREE, GIT_INDEX_FILE, GIT_OBJECT_DIRECTORY, GIT_COMMON_DIR, GIT_NAMESPACE), with a check.test.mjs case
  pinning that env; and operator.test.mjs's `git` helper runs with those removed and GIT_CEILING_DIRECTORIES set to
  the temp folder's parent. browser.test.mjs:109's `git ls-files` (read only) would then also read the right
  repository.

### Cleared (no finding)
- No shell anywhere: `run` is spawnSync with an argv (check.mjs:1952), so the glob reaches Node as one argument and
  Node's own glob expands it; `process.execPath` is absolute (no PATH lookup for node).
- Node v22.23.4's test runner builds its file list with `exclude: (name) => name === 'node_modules'`, called per
  directory entry's name (read from the node binary's embedded lib/internal/test_runner/runner.js and
  internal/fs/glob), so no `*.test.mjs` inside ops/browser/node_modules can run as a test at any depth (and today
  the install holds none: `find ops -path '*/node_modules/*' -name '*.test.mjs'` printed nothing).
- Today's selection is 9 tracked files (ops/tests/{browser,cloudinit,firewall,operator,r1plan,r1sh,redact,s14}.test.mjs
  and ops/operator/redact-check.test.mjs); `git status --short --ignored ops` shows only the ignored node_modules.
  Node's glob follows a symlinked directory one level (its isSymbolicLink branch), but ops/ holds no symlink outside
  node_modules; a planted test or symlink is a committed change, which review is the wall against (Low at most, not
  reported). Glob results begin with `ops/`, never `-`, so no file name becomes a node option.
- `bash` by PATH (check.mjs:2699) is the same class as check.mjs's other bare commands (cargo x9, git, npm, rustc x2):
  the operator's machine. BASH_ENV, exported BASH_FUNC_* functions, NODE_OPTIONS or a hostile reporter could forge a
  pass only on a hostile machine, never on a clean run: not a finding. No file named `bash` exists in the spike
  folder for a relative PATH entry to reach.
- wrappers.test.sh sources only the committed ops/sbin/npt-lib.sh (wrappers.test.sh:10; that file is `readonly`
  constants and function definitions only), runs `bash -n`, grep, sed and `visudo -c -q -f` on repo files
  (:183, check mode), never sudo, ssh or a wrapper; its last stdout line is its own printf (:235), and opsStep reads
  stdout apart from stderr (check.mjs:2701-2703); a forged line needs a committed change.
- The Node tests' remote and privileged calls are stubbed where r1.sh's functions are cut: lg's ssh and timeout
  (r1sh.test.mjs:419), host_active's systemctl (:434), room_up's setsid/mkfifo (:652), s14_run (:575), chain_run's
  block runner (:488-490); every address is a documentation range (192.0.2.10, 198.51.100.20, r1sh.test.mjs:26).
  browser.test.mjs:146-152 runs the browser scripts with a refused browser: they import webdriverio (its top-level
  code runs, which is the install's purpose, UM-4 (b)) and exit 2 before any SSH or WebDriver call; no driver or
  browser download is reached (SEC-R1D-3's binaries by path, browser.test.mjs:37-50).
- NPT_R1_TREE (r1sh.test.mjs:739-740) lets the inherited environment choose a folder the test writes; it refuses an
  existing one, so it fails closed; the operator's own setting (informational).

## Block 2: the install record, dependency-check.md:161-170 (DONE 19:1x UTC; narrowed 20:0x UTC)

### Held against its clearances
- The form as run (dependency-check.md:163-165): `env npm_config_ignore_scripts=true npm ci --ignore-scripts
  --no-audit --no-fund --prefix <absolute ops/browser>`. Meets security-u8-deps.md C3 (a) (`npm ci --ignore-scripts`
  only, so the committed lock binds), the environment belt (C3, npm's command line > environment > npmrc), and
  SEC-U7D-3 (security-u7-deps.md:223-226: `--no-audit --no-fund`, so npm sent no audit request). No npx, npm exec or
  npm create. browser.test.mjs:108-134 (in the counted run's 104 of 104) holds no .npmrc under spikes/ and every npm
  form with both belts. This recorded form is the binding evidence that install scripts did not run.
- The files: both tracked (`git ls-files`), last changed 2ad2829f (2026-11-22, the U8 clearance day); sha256 before
  and after equal to the cleared lines (dependency-check.md:44-45 = :166-167 = security-u8-deps.md:104-105).
  security-r1-deps.md:245-246's lock rules (every `resolved` on https://registry.npmjs.org/, every integrity sha512)
  are pinned for all 167 entries by browser.test.mjs:86-92.
- "npm audit output recorded" (security-r1-deps.md:246): met for this lock by security-u8-deps.md C2, as C3 (c)
  (:94-96) rules; the record says so (:169-170). The laptop audit (exactly those 4 moderates) stays a laptop condition.
- On disk, supporting only: edgedriver's and geckodriver's postinstall would write their driver to os.tmpdir() or
  EDGEDRIVER_CACHE_DIR / GECKODRIVER_CACHE_DIR (edgedriver dist/install.js:27, geckodriver dist/install.js:38),
  outside node_modules, so "no driver under node_modules" proves nothing; Glob finds no geckodriver*, msedgedriver*
  or edgedriver* in /tmp today (weak: /tmp may have been cleaned). node_modules holds 7 symlinks (`find -type l`),
  the same count as the 7 bins security-u8-deps.md:34-35 lists (not matched name by name).
- SEC-R1D-2 and -3 are unaffected (this container install starts no driver or browser; block 1).
- Not verified: a per-entry comparison of the committed lock, npm's hidden lockfile and the folders on disk (my
  read-only script was refused by the permission classifier; counts only, below); the npm registry configuration
  (not read); npm's printed output (taken from the record; no install log read).

### SEC-UM4-2 (Low): the record leaves out item (3)'s registry and version lines it cites
- What: :164-165 names "the web step's cleared form, security-u7-deps.md items (3)-(4)"; item (3)
  (security-u7-deps.md:139-148) also asks that `npm config get registry` read https://registry.npmjs.org/ and that the
  npm and node versions be recorded, as the web install did (dependency-check.md:53-54). The UM-4 section records
  neither.
- Exploit sketch: none on the bytes: a user or global npmrc pointing elsewhere would have changed only where npm
  fetched from; npm ci checks every tarball against the lock's sha512, so a different tarball fails the install.
  The gap is the record's.
- Evidence: dependency-check.md:161-170 (no registry, npm or node line); today's tools are npm 10.9.9
  (/opt/node22/lib/node_modules/npm/package.json:2) and node v22.23.4 (`node --version`), as on 2026-11-23, but that
  is read after the install, not recorded with it. Confidence: high.
- Fix: in the one-clause note the main session adds for UM4H-1, add "npm 10.9.9, node v22.23.4; registry not
  recorded at the install (the lock's sha512 integrity binds every tarball)", or the values if a log of the install
  holds them.

### UM4H-1 (advisor-um4-nb2b-head.md:56-60): CONFIRMED, with one correction to the proposed clause
- The conclusion holds: the installed set is the cleared set. The lock's sha256 is the cleared a0efce8e...9524; npm's
  hidden lockfile node_modules/.package-lock.json has 167 `node_modules/` entries, the committed lock 167 (Grep
  counts); npm 10.9.9 counts "added" once per ADD node of the tree diff, that is once per node_modules placement
  (npm/lib/utils/reify-output.js:58-62), so "added 167 packages" = the lock's 167 non-root entries.
- The correction: security-r1-deps.md:158's 162 is not the same tree counted another way "as resolved that day". It
  is a distinct name@version count from a newest-per-range resolver (security-u8-deps.md:43-52): 162 = the lock's 160
  distinct name@version + @types/node 26.6.6 and undici-types 8.9.0, which arborist never places (@types/node
  22.20.5 already meets `*`); the lock's 167 = those 160 + 7 duplicate placements (security-u8-deps.md:23-27). U8's
  binding check found the lock to be webdriverio 10.2.0's tree package by package ("BINDING CHECK HOLDS", :52).
- Suggested clause: "167 = the lock's non-root entries (160 distinct name@version and 7 duplicate placements,
  security-u8-deps.md C0); security-r1-deps.md:158's 162 is a resolver's distinct count that adds two packages npm
  never places (security-u8-deps.md C1)". Severity stays Low (a record that could raise a doubt; nothing is wrong).

## Block 3: OPS_MISSING's line, check.mjs:2692-2693 (DONE 19:2x UTC)
- The line asks for nothing unsafe: it names the cleared form exactly (`npm ci --ignore-scripts --no-audit --no-fund`
  with npm_config_ignore_scripts=true, block 2), never `npm install`, `npm audit`, npx or a TLS switch; it installs
  only the committed, cleared lock. The step runs nothing when the folder is missing (check.mjs:2695;
  .gate/um4/absent.log: "FAIL ops 0.0 s" with that one line).
- A routine cannot follow it: the routine guard allows npm only as `npm ci --prefix tools --ignore-scripts --no-audit
  --no-fund` (.claude/hooks/routine-guard.mjs:30, :600, :613); a day session meets the proxy and its prompt first
  (um4-plan.md:195).

### SEC-UM4-3 (Low): `--prefix ops/browser` is relative to the operator's shell, not to the spike folder
- What: check.mjs runs its commands in HERE (the spike folder), but the line's `--prefix ops/browser` resolves
  against wherever the operator types it. The web step uses the absolute path (WEB, check.mjs:2643, :2668).
- Exploit sketch: none that harms: typed at the repository root (`node spikes/netcode-proof/check.mjs` is a natural
  call), ./ops does not exist there (`ls -d ops`: no such file), so npm ci stops on its missing lockfile and installs
  nothing; typed in some other folder that has its own ops/browser lock, it would install that lock (still with
  scripts off), which the operator chose by standing there. Clarity, not safety.
- Evidence: check.mjs:2692-2693 (OPS_MISSING), :34 (HERE), :2683 (OPS_BROWSER_MODULES absolute); .gate/um4/absent.log
  ("pwd /home/user/Shamble-Frontlines/spikes/netcode-proof": the lane runner stands in the spike folder, so the
  recorded use was right). Confidence: high.
- Fix (optional, with the next touch of the line): print the absolute prefix (`join(HERE, "ops", "browser")`, as WEB
  does) or say "in spikes/netcode-proof"; check.test.mjs pins the string, so the test moves with it.

## Verdict
APPROVE WITH FINDINGS. Critical 0, High 0, Medium 1 (SEC-UM4-1), Low 2 (SEC-UM4-2, SEC-UM4-3); UM4H-1 confirmed with a
corrected clause (the advisor's Low, not counted again here). First to act on: SEC-UM4-1, strip git's location
variables from the ops step's environment (and in operator.test.mjs's git helper) before UM-4 closes, or defer it
explicitly; as a security finding it keeps the PR needs-mustafa while open (advisor.md, PR review step 4).

## For the main session to carry (the brief forbids my editing any other file, memory included)
- security-auditor memory, a pattern: a test that runs git in a temporary folder with the inherited environment acts
  on the repository an absolute GIT_DIR names (probe above); check every temp-repository test for a cleared GIT_*
  environment and GIT_CEILING_DIRECTORIES.

Status: COMPLETE 2026-11-27 20:07 UTC
