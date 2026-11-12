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
