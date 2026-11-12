# Skeptic, review round 1, D6 (= SA-3, Low): head 4d78ad6

Plan: (1) read the recorded finding SETUP-NODE-CACHE at the head (docs/STATUS_ARCHIVE.md ~600, the report ~170) and
the three setup-node steps (gate.yml, toolchain.yml) at 4d78ad6; (2) fetch setup-node at its pinned SHA into my own
folder (.gate/skeptic-D6) and re-read action.yml and the cited src lines (main.ts, cache-utils.ts, cache-restore.ts,
cache-save.ts) and the built dist, so the claim rests on the shipped code, not the TS source alone; (3) re-read the CI
logs of the head's three jobs for `package-manager-cache` and any cache restore/save line; (4) test the condition in
scratch (a copy of the dist logic run on a package.json with devEngines.packageManager = npm, and without), and check
whether ~/.npm is what gets cached, under which key, and whether the save is in the post step; (5) weigh the exploit
sketch (who can write ~/.npm, which branch scope restores it, npm ci's integrity check); (6) judge holds, severity,
and the fix. Nothing in the worktree changes but this file.

## Checks
- C1 the recorded text at 4d78ad6: docs/STATUS_ARCHIVE.md:600-602 ("what it caches if a `packageManager` field is
  added to the unprotected root `package.json` is unverified (step 7's security-auditor)") and the report
  docs/reports/2026-11-12-foundation-2-2.md:170 ("setup-node v7.0.0's `package-manager-cache: true` if a
  `packageManager` field is added"); STATUS.md:97 names it only. Both name the top-level field alone. R5-CI-F16
  (docs/research/r5-toolchain-quality.md:672) already flags the trigger as "inferred (WebFetch summary only)".
- C2 my own fetch of actions/setup-node (git fetch --depth 1 into .gate/skeptic-D6/dl/setup-node, read by git show):
  the pinned SHA 820762786026740c76f36085b0efc47a31fe5020 is refs/tags/v7.0.0 on the remote (v7 and v7.1.0 now point
  at 949feb2). action.yml:23-25: package-manager-cache, default true, "caching is enabled when either
  devEngines.packageManager or the top-level packageManager field in package.json specifies npm"; :46-47 post =
  dist/cache-save/index.js, post-if: success().
- C3 src (and the shipped dist, same code: dist/setup/index.js:99696-99800, :98290-98340; dist/cache-save/index.js
  :92929-92942): main.ts:70-89 auto-cache only when the `cache` input is empty and package-manager-cache is true;
  main.ts:144-173 reads `${GITHUB_WORKSPACE}/package.json`, checks devEngines.packageManager FIRST (object or array,
  `.name` against /^(\^)?npm(@.*)?$/), then the top-level string; cache-utils.ts:21-29 path = `npm config get cache`
  (the whole ~/.npm), :186-222 one root folder; cache-restore.ts:43-44 key node-cache-<RUNNER_OS>-<arch>-npm-<hash>,
  :77-91 lockfile = first of package-lock.json, npm-shrinkwrap.json, yarn.lock in the workspace ROOT only, :61 exact
  key (no restore-keys for npm); cache-save.ts:57-73 save unless the primary key was hit. Every citation in SA-3
  holds (line ranges off by one or two at the edges only). The log line says "packageManager field" even when
  devEngines triggered it (main.ts:81-83).
- C4 behaviour, not reading (.gate/skeptic-D6/scripts/detect.mjs: the shipped dist's getNameFromPackageManagerField,
  23 lines extracted by brace matching, run with GITHUB_WORKSPACE on eight folders built from the head's root
  package.json): head as-is -> undefined (no auto-cache); + packageManager "npm@10.9.9" -> "npm"; + devEngines.
  packageManager {name: npm} -> "npm"; + the array form [pnpm, npm] -> "npm"; + {name: npm, onFail: warn} -> "npm";
  + devEngines.runtime only -> undefined; + packageManager "pnpm@9.0.0" -> undefined; no package.json (main today, no
  root package.json at 2aa5e9c) -> undefined. So the devEngines trigger is real and the recorded text misses it.
- C5 CI logs of the head's pull_request runs (toolchain job 131510257155, gate 131510254606, smoke 131510255220; all
  head_sha 4d78ad6, success): each shows `package-manager-cache: true` in setup-node's inputs, then "Found in cache"
  and "Environment details" only: no "Detected npm" line, no npm cache restore, and the post step prints nothing (no
  saved state). The only cache in any job is smoke's Playwright cache (actions/cache, hit, not saved). Today nothing
  is cached by setup-node, on main or here: SA-3's "no restore line" holds.
- C6 the steps and the path: gate.yml is identical on main (2aa5e9c) and the head; its setup-node steps (:29-31,
  :60-62) and toolchain.yml:37-39 pass node-version only (no cache, no package-manager-cache, no
  cache-dependency-path); no `.npmrc` anywhere in the head's tree, so the cached path would be npm's default ~/.npm
  (`npm config get cache`), the whole folder (_cacache, _logs, _npx, ...). With no cache-dependency-path all three jobs
  hash the ROOT package-lock.json (findLockFile), though gate and smoke install from tools/package-lock.json: one key,
  different contents.
- C7 the cut can write ~/.npm: toolchain.yml:68-73 runs it as `setpriv --reuid="$(id -u)"` with `env -i HOME="$HOME"`;
  the log (job 131510257155:254) prints uid=1001(runner) inside, and the cut's own `npm ci --offline` reads ~/.npm
  (log :381 "added 46 packages in 1s"). setup-node's post step (action.yml:46-47) runs after the cut step (log :447,
  21:37:37.17, after tsc's "Version 6.0.3" at :37.14), only on success. SA-3's "save after the cut" holds.
- C8 which job's folder would be saved (job times, API): push runs: gate 9 s, toolchain 26 s, smoke 36 s; PR runs:
  gate 9 s, toolchain 30 s, smoke 48 s. On a key miss all three save under the same key and the first wins (the
  others hit a reserve collision, cache-save.ts:64-70); with today's times that is the gate job's ~/.npm (tools'
  packages only), and later runs hit and never save. The toolchain job's folder (the one exposed to code inside the
  cut) is saved only when gate does not save first (gate red or cancelled: post-if success()), which a PR's own code
  can arrange. So the sketch is reachable but not the default path; SA-3 says "restore and save" for all three, which
  is loose (one save lands per key), not wrong in substance.
- C9 the reach (npm 10.9.9, the runner's version per the log; .gate/skeptic-D6/scripts/poison.mjs, scratch cache):
  both lockfiles at the head carry sha512 for every registry entry (root 64 of 64, tools 169 of 169; the root's 5
  others are workspace links). A: the genuine tarball's content file overwritten with junk: `npm ci --offline` exit 1
  ENOTCACHED, nothing installed; `npm ci` online exit 0, genuine refetched. B: the registry URL's index key repointed at
  a self-consistent malicious is-number 7.0.0 (its own correct sha512), genuine content deleted: offline exit 1
  ENOTCACHED, nothing installed; online exit 0, genuine installed, never the malicious one. SA-3's "a failed or slow
  install, not swapped code" holds. (npx: both jobs' npx calls use `--no` after `npm ci`, so the local bin wins and
  ~/.npm/_npx is not consulted; not a code route today.)
- C9b the npx aside, verified rather than recalled: npm 10.9.9's libnpmexec/lib/index.js:128-160 runs a command from
  the nearest node_modules/.bin (localFileExists) before it ever reaches the npx cache, unless the project's own
  manifest declares a bin of that name; no manifest at the head (root, tools, the five packages) has a `bin`. Both
  `npx --no` calls (smoke in tools/, the cut's tsc) run after `npm ci`, so ~/.npm/_npx is never consulted today.
- C10 scope (GitHub's dependency-caching reference, fetched 2026-11-12 via docs.github.com's article API, lines
  211-215 and 233-235): a pull_request run's cache is created for refs/pull/N/merge and "can only be restored by
  re-runs of the pull request"; push runs write their branch's scope; only push, workflow_dispatch, schedule and a few
  others may write main's scope, and "The `pull_request` event is not affected". So a cache a pull_request run writes after
  the cut reaches only that PR's re-runs and cannot reach main's scope; a push run's cache sits in its branch's scope,
  which PRs based on that branch may also restore (line 211). SA-3's "same branch scope" holds. (Corrected after the
  advisor tool: an earlier line here said "never other PRs" and "no untrusted-to-trusted path", more than the docs say.)
- C11 the fix. (a) `package-manager-cache: false` on every setup-node step: right (main.ts:25-27 and :78 skip the auto
  branch; `cache` is unset, so nothing is cached) and complete for the trigger; it keeps today's behaviour exactly
  (C5: nothing is cached now), so it costs nothing and breaks nothing. At 2.3 toolchain.yml is replaced, so it belongs
  on every setup-node step of the workflows 2.3 leaves, not on "three" as such. (b) "or design caching deliberately
  with `cache-dependency-path` per job": per-job lockfiles give the toolchain job (root lockfile) and gate/smoke
  (tools/package-lock.json) different keys, so it fixes the shared key and the mixed contents (C6, C8) and closes the
  cross-job restore SA-3 sketches; what stays is setup-node saving the whole ~/.npm in its post step after the cut,
  which later toolchain runs in the same scope restore. A deliberate design would save before the cut
  (actions/cache/restore and /save around the online `npm ci`). (a) does not block deliberate caching later: an
  explicit `cache` input is checked first (main.ts:74). (c) the reword (devEngines.packageManager, the whole ~/.npm, the one root-lockfile key, the save
  after the cut): right; it can also drop "unverified" and state the reach (C9, C10). R5-CI-F16 (r5:672) carries the
  same one-field reading but already marks it "inferred": optional pointer. The reword should cite its source
  (setup-node@8207627 action.yml:24, main.ts:144-173) and move SETUP-NODE-CACHE out of the archive's [assumed]
  provenance list (docs/STATUS_ARCHIVE.md:556-560 lists "SETUP-NODE-CACHE's job log" there; C5 verifies it). It is a
  Low: it rides the push D1 needs and would not justify a push on its own (CLAUDE.md, review budget).
- C12 isCacheFeatureAvailable (cache-utils.ts:305-320), called at main.ts:72 before the field check, warns when the
  cache service is missing; none of the three logs carries that warning (0 matches each), and smoke restored its
  Playwright cache in the same run: the service is there, so the field alone decides.

## Verdict
Holds, Low. Every factual claim re-checked against setup-node v7.0.0's shipped code and its behaviour (C2-C4), the
head's workflows and CI logs (C5-C7), and npm's own behaviour on a poisoned cache (C9): the recorded SETUP-NODE-CACHE
names one of the two triggers and leaves "what it caches" open. The trigger is off today on main and here, and the
worst reach once on is a failed or slow install in the writing PR's re-runs, or in a branch's scope and the PRs based
on that branch, never main's scope from a pull_request run (C9, C10). Low (wording and
completeness of a follow-up), not Medium. Nuance: "all three jobs restore and save" is loose: one save lands per key,
today usually the gate job's (C8). Fix (a) and (c) are right; (b) closes the cross-job restore
but leaves the post-cut save for later toolchain runs. Ran: C4, C9, the lockfile counts. Read: C2, C3, C5-C7, C9b,
C10, C12. Inferred, not run: which job's save wins (C8: job times plus cache-save.ts's reserve-collision comment).
