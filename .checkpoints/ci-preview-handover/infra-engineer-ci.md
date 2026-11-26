# infra-engineer: CI review of the preview hand-over texts (worktree .gate/ci4, head 13ce313, main 2483e9a), 2026-11-26

## Plan
1. Rebuild both files from the drafts in the scratchpad (patch), compare sha256 with the stated values.
2. Read the rebuilt toolchain.yml whole; check each item of the brief: custom shell, $GITHUB_WORKSPACE and the relative
   path, the two `if:`, artifact inputs, include-hidden-files, upload-artifact v7 / node24 on the runner, lane 2's
   preview.yml (name, path, artifact, success), the comment fixes and TOKEN-WORDS, the 15-minute limit, the firefox
   project's two patterns.
3. Sources: the action's text at cf430e0 (scratchpad clone ua/src), the runner's source (WhichUtil.cs, ScriptHandler.cs),
   GitHub docs (workflow syntax, workflow commands), Playwright 1.65.0's createFileMatcher, gh run data of main's run.
4. Findings with severity, file:line and replacement text; how the first run's proof is read.

## Notes (as found)
- Rebuild: toolchain.yml 01898e4b... -> 067521c960c768ef29c9ed310151c53d649f2b5190fd6df120784c7837c03464 (matches);
  playwright.config.ts ef2136a1... -> 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8 (matches).
- Custom shell: docs (workflow syntax, read 2026-11-26): "`command [options] {0} [more_options]`", "GitHub interprets
  the first whitespace-delimited word of the string as the command"; own flags replace the default `set -e`/pipefail.
  Runner source (actions/runner main, read 2026-11-26): ScriptHandler.cs resolves a custom shell's command with
  `WhichUtil.Which(parsed.shellCommand, ...)`, `{0}` required; WhichUtil.cs returns a fully qualified path unchanged if
  the file exists (`Path.IsPathFullyQualified(command) && File.Exists(command)`), before any PATH search. The default
  `bash` of the other run steps is resolved through Which with the prepended path (GITHUB_PATH). /usr/bin/bash exists on
  ubuntu-24.04 (merged /usr; trial runs used it; on the runner inferred, high confidence). OK.
- $GITHUB_WORKSPACE: set for every step; the cut step writes `$PWD/.gate/dist-production` from the workspace. Upload's
  relative path: dist/upload/index.js:129971 `ensureAbsoluteRoot(Pattern.globEscape(process.cwd()), pattern)`; the runner
  starts actions in the workspace. Root of the artifact = the search path (search.ts:154-157), so index.html sits at the
  artifact's top, as lane 2's preview.mjs:195 requires ("no index.html at the top" otherwise). OK.
- `if:` on both steps: push runs -> event_name 'push' -> skipped; fork PRs -> head.repo.full_name differs -> skipped;
  implied success() so a red cut step skips both. The concurrency group differs between the push run (refs/heads/...) and
  the pull_request run (refs/pull/N/merge), so neither cancels the other. OK.
- Inputs: action.yml at cf430e0: name, path, if-no-files-found (default warn; `error` set), retention-days (1..90, 7 set),
  compression-level, overwrite (default false), include-hidden-files (default false), archive (default true). The draft
  drops include-hidden-files (advisor N1); the root item is never judged hidden (dist:130173, basename of each visited
  item; root basename `dist-production`). The build has 5 files, none dot-named (trials.md:2-3). Not needed today.
- node24: action.yml `using: 'node24'`; main's last toolchain run 45818982052 (2026-11-26 12:22 UTC) ran on runner
  2.338.2, image ubuntu-24.04 20261122.23.1. Node 24 support in the runner predates that version (inferred from runner
  releases, medium-high confidence; lane 2's preview.yml uses the same pin, not yet run on workflow_run).
- Lane 2's preview.yml read at dc00c79 (fetched 2026-11-26; file last changed 9ded5a8): job `check` (lines 52-60) needs
  workflow_run.name 'toolchain', path '.github/workflows/toolchain.yml', event 'pull_request', conclusion 'success',
  head_repository == repository; download-artifact v8.0.2 by `name: preview-site`, `run-id`, token with actions: read
  (lines 87-96). preview.mjs:241 detects the hand-over on main with
  `/^[ \t]*name:[ \t]*(["']?)preview-site\1[ \t]*(?:#.*)?$/m`: the draft line `          name: preview-site` matches.
  workflow_run starts only from main's copy of preview.yml: no preview until lane 2's file is on main. OK.
- Comments: :4 and :130 corrected, :94 TOKEN-WORDS ("The only run step with the token") as STATUS_ARCHIVE.md:1344-1346;
  the guard step holds no token in its env (GITHUB_TOKEN is not exported to run steps; ACTIONS_RUNTIME_TOKEN goes to
  actions only, dist:82477 reads it in the upload's own process). True.
- Time: main's run: job 12:22:06-12:28:06 (6.0 min), the cut step 5 min 34 s. Plus the replay test in Firefox (1.1 s in
  the trial), the guard (<1 s), the action's download at "Set up job" and an 892,209-byte upload (seconds). Far inside 15.
- Firefox patterns: Playwright 1.65.0 lib/util.js:97-108 prefixes a string pattern without `**/` with `**/` and matches
  the absolute path (minimatch). So the project takes `<testDir>/**/rules/**/*.spec.ts` and `.../replay/**/*.spec.ts`;
  today: tests/rules/wasm-rules.spec.ts and tests/replay/determinism.spec.ts (3 tests listed in the trial). The
  skeleton folder (allocation spec) is not matched. OK.
- Line lengths of the rebuilt workflow: line 4 is 156 characters; the header block wraps at <=120.
- CUT-ESCAPE (STATUS_ARCHIVE.md:803-818) already records that the cut's code can reach later steps through GITHUB_ENV /
  GITHUB_PATH and work-folder files; GitHub docs (workflow commands, 2026-11-26): only NODE_OPTIONS is refused in
  GITHUB_ENV. So BASH_ENV / LD_PRELOAD planted from inside the cut reach the guard step and the upload; the absolute
  custom shell stops PATH and GITHUB_PATH, not those. Security-auditor rates that; from CI only the comment's wording.
- Hidden-file check tried here (scratchpad guard2.sh, the draft guard plus two lines): real build exit 0 ("5 files,
  892209 bytes"); a planted assets/.planted exit 1 with its line.
- Re-run with `overwrite: false`: whether artifact names are scoped to the run or to the attempt was not found in
  GitHub's docs (re-run page read 2026-11-26: silent on artifacts); unverified.

## Findings
- IE-1 (Low) toolchain.yml:4 (after the draft): 156 characters where the header wraps at 120. Replace lines 4-6 with:
    # The cut: the step "Build and test with the network cut off" runs in its own network namespace (sudo unshare --net),
    # with loopback up, back as the runner's user with no docker group and no_new_privs; a TCP probe to an address
    # resolved before the cut must connect outside and fail inside (only curl exit 7 or 28 counts as cut).
  Optional, same block (the header lists what the job does and does not yet name the hand-over), after line 13:
    # After the cut, on pull requests from this repository only, the tested player build goes to the preview workflow
    # (preview.yml, lane 2) as the artifact preview-site.
- IE-2 (Low) toolchain.yml:218: "with absolute paths, since the cut's user can write in PATH" reads as protection it is
  not (CUT-ESCAPE: BASH_ENV or LD_PRELOAD planted through the cut step's GITHUB_ENV file reach this step; only
  NODE_OPTIONS is refused). Add one line after :218:
          # Not a boundary: the cut's code can still reach this step through the runner's files (CUT-ESCAPE).
  (Severity of the exposure itself: security-auditor's.)
- IE-3 (Low) toolchain.yml:225-226 / :231: without include-hidden-files, a future dot-named file in the build is left
  out of the artifact silently (no red), the guard still counts it, so the guard's count and the upload's "there will be
  N files" differ, and the preview is not the tested build. preview.mjs:160 refuses dot names anyway. Add after :226
  (tried: real build exit 0, a planted dot file exit 1):
          hidden=$(/usr/bin/find "$d" -mindepth 1 -name '.*' -print)
          [ -z "$hidden" ] || { echo "a name starting with a dot (the upload would leave it out):"; echo "$hidden"; exit 1; }
  and the :231 comment becomes: "# An empty upload fails; the check above fails on a name starting with a dot, which the
  upload would leave out." Alternative: keep the draft and accept the silent drop (none today).
- IE-4 (Low, likely benign, follow-up) the upload's `overwrite` stays false: if artifact names are scoped to the run, not
  the attempt, re-running a pull request run whose first attempt uploaded preview-site fails the step (409), and lane
  2's runbook (docs/runbooks/previews.md:45-46 on its branch) prescribes that re-run. The action's own client
  expects several artifacts of one name in one run: dist/upload/index.js:129006-129009 ("More than one artifact found for
  a single name, returning newest"), consistent with names scoped per attempt; not shown. Read it on the first re-run,
  or later add `overwrite: true` (action.yml: deletes the old one first). Not for this round.
- IE-5 (Low, information, corrected after the advisor tool's read) the guard's printed bytes: `du -sb` with GNU
  coreutils 9.4 (the container's; ubuntu-24.04 ships 9.4, inferred) printed 892209, equal to the sum of the five files'
  sizes, though both folders are 4096 bytes by stat: it counts file bytes only here. The upload's "Final size" is the
  zip's: compare file counts, not bytes.

## Overlap with security-auditor-ci.md (same round)
- IE-1 = SA-5 (:4-6 rewrap), IE-2 = SA-3 (:215-218 wording), IE-3 = SA-4 (+ SA-6 on :231). One text each: for :4-6 and
  :215-218 take security-auditor's replacement if it differs (theirs owns the exposure's wording); mine above is a
  candidate. For IE-3 either guard line works (both tried; `find -name '.*'` also catches dot-named folders); SA-6's
  point holds for my :231 text too: "a name starting with a dot" covers folders and what is inside them.

## How the first run's proof is read
- Push run (first, it starts at the push): `gh run view <id> --json jobs --jq '.jobs[].steps[] | [.number,.name,.conclusion] | @tsv'`
  shows both new steps `skipped` (skipped steps print nothing in `gh run view --log`). Confirms the log route works.
- Pull request run: guard log "5 files, <n> bytes"; upload log (texts from the action at cf430e0):
  "With the provided path, there will be 5 files uploaded" (src/upload/upload-artifact.ts:55-57),
  "SHA256 digest of uploaded artifact is <hex>" (dist/upload/index.js:124011),
  "Artifact preview-site successfully finalized. Artifact ID <n>" (dist:124337),
  "Artifact preview-site has been successfully uploaded! Final size is <n> bytes. Artifact ID is <n>" and
  "Artifact download URL: ..." (src/shared/upload-artifact.ts). Cross-check:
  `gh api repos/<owner>/<repo>/actions/runs/<id>/artifacts` (name, size, expiry 7 days out, digest).
- The push run is the first resolution of the new action: "Set up job" downloads every `uses:` action, skipped or not.
  Its log must show "Download action repository 'actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9'".
  If the repository's Actions settings allow only selected actions, every run (push too) goes red there. I could not
  read it: `gh api repos/<repo>/actions/permissions` was refused by the proxy (HTTP 403, "Access to this GitHub Actions
  path is not permitted"). Residual unknown: Mustafa reads Settings > Actions > General > "Actions permissions" (allow
  all, or an allow-list that includes actions/upload-artifact), changing nothing; or the push run shows it.
- node24 shown, not inferred: checkout at its pin 3d3c42e (raw action.yml, read 2026-11-26) is `using: node24` and runs
  main and post in this job on runner 2.338.2 (run 45818982052).
- Other residual unknown: /usr/bin paths on the image (inferred, high confidence).

## Verdict
APPROVE (IE-1..IE-3 as optional text fixes for the asked text; IE-4, IE-5 as notes). No Critical, High or Medium.
Line numbers are the rebuilt file's. Adopting any of IE-1..IE-3 changes the after-sha256 067521c9...; IE-3 also needs a
dot-file case in plant-test.sh. The allow-list setting is the one unknown that could turn the first run red.
