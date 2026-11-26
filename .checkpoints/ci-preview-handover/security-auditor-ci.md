# security-auditor: tier-2 review of the preview hand-over texts (worktree .gate/ci4, head 13ce313, main 2483e9a)

## Plan
1. Rebuild toolchain.yml and playwright.config.ts with `patch` in the scratchpad; compare sha256 with the stated values.
2. A: `git ls-remote` for upload-artifact v7 tags myself; read action.yml, src/shared/search.ts,
   src/upload/input-helper.ts at the clone (HEAD check), plus how the token is used (dist).
3. B: (i) absolute paths, (ii) no script step, (iii) action code / post steps in the same-uid work folder.
   Check the runner layout claims (_actions folder, ownership) against docs where possible; mark inferred.
4. C: GUARD-STEP; D: HIDDEN-FILES; E: exposure, `if:`, comments' truth; lane 2's preview.yml if readable.
5. Findings, one line each as found, then a summary.

## Notes (as found)
- Rebuild (patch into scratchpad sa-rebuild, flat file names since the guard refuses a `.github` path even there):
  toolchain.yml 01898e4b... -> 067521c960c768ef29c9ed310151c53d649f2b5190fd6df120784c7837c03464 (matches);
  playwright.config.ts ef2136a1... -> 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8 (matches).
- A: my own `git ls-remote https://github.com/actions/upload-artifact 'refs/tags/v7*'` (2026-11-26):
  `cf430e030ddbb5b0abf93d22962f4752f3646cd9	refs/tags/v7.0.2` (and v7 the same; v7.0.1 043fb46d..., v7.0.0 bbbca2dd...);
  no `^{}` line for any v7 tag, so lightweight tags pointing straight at the commit. Clone HEAD = cf430e0..., clean.
  action.yml: inputs name, path, if-no-files-found, retention-days, compression-level, overwrite, include-hidden-files,
  archive; no token input; runs node24 dist/upload/index.js. search.ts:16 followSymbolicLinks true, :18
  omitBrokenSymbolicLinks, :19 excludeHiddenFiles = !includeHiddenFiles. input-helper.ts:8-58 no token.
  dist/upload/index.js:82477 reads ACTIONS_RUNTIME_TOKEN from env; :130173 hidden test is basename(item.path) for every
  item the walk visits, the root item being the search path itself (basename `dist-production`), so `.gate` is never judged.
- Lane 2's preview.yml read at 9ded5a8 (origin/claude/optimistic-albattani-mdoonz, newer than the advisor's 266f42e):
  job `check` keeps the boundary (name, path, event pull_request, success, head_repository == repository); build is
  data (downloaded to runner.temp, checked, re-uploaded; deploy runs nothing from it). Boundary holds on lane 2's side.
- Runner source (raw.githubusercontent.com actions/runner main, src/Runner.Worker/FileCommandManager.cs, fetched
  2026-11-26): file commands live in `<Temp>/_runner_file_commands`, files `set_env_<guid>`, `add_path_<guid>` per step;
  the set-env block list is `{ "NODE_OPTIONS" }` only. GitHub docs (workflow-commands): GITHUB_ENV values reach "all
  subsequent steps"; GITHUB_PATH prepends for "all subsequent actions"; NODE_OPTIONS refused. So code inside the cut
  (same uid, fixed folder under RUNNER_TEMP) can append BASH_ENV / LD_PRELOAD / a PATH folder for every later step.
- Local check (bash 5.2.21): `BASH_ENV=x /usr/bin/bash --noprofile --norc -euo pipefail step.sh` sources x first, so
  the guard's absolute shell does not stop a BASH_ENV planted through GITHUB_ENV.
- Already recorded: CUT-ESCAPE (STATUS_ARCHIVE.md:803-818, D4, SA-1): the cut is not a sandbox; names GITHUB_PATH /
  GITHUB_ENV, work-folder files, checkout's post step; job has contents: read, no secrets. CUT-PATH :1340-1342.
- Line lengths of the rebuilt file: line 4 is 156 characters (the header block wraps at 120 elsewhere).
- Runner source (main, fetched 2026-11-26): NodeScriptActionHandler.cs sets ACTIONS_RUNTIME_TOKEN unconditionally for
  every node action (main and post steps alike: checkout's and setup-node's post steps hold it today); ScriptHandler.cs
  (run steps) does not set it (it sets ACTIONS_ID_TOKEN_REQUEST_* only with an id-token URL, absent without id-token: write;
  the condition is inferred from the summary). Run steps are written to <Temp>/<guid>.<ext> (same-uid writable, inferred).
- GitHub docs (expressions): "A default status check of success() is applied unless you include one of these
  functions": the `if:` lines imply success(), so no upload after a red cut step. Verified.
- Lane 2's hosting/preview.mjs at 9ded5a8: checkTree walks with lstat (no link followed), refuses names starting with a
  dot (:160), names outside a safe set (:163), host-acted files (:165), non-plain items (:188), >= 25 MiB, too many
  files, missing index.html; copies plain files to a fresh folder, adds main's _headers, rechecks in deploy. Build never run.
- Proposed guard line (dot names too) tried in scratchpad sa-guard: clean tree prints nothing; a `.env` file, a
  `.well-known` folder and a symbolic link are each printed (so the step would fail).
- No agent memory file found (.claude/**memory** in worktree and main folder): durable decision handed to the session.

## Findings
- SA-1 (Medium, pre-existing CUT-ESCAPE, does not stop the change) B(iii): code inside the cut keeps the runner's uid,
  so it can change what later steps run: the action code under the work folder's _actions (inferred ownership), the
  step's GITHUB_ENV/GITHUB_PATH files under <Temp>/_runner_file_commands (folder and block list verified in runner main;
  writability inferred), later run-step scripts in <Temp>, and a process left running (TOCTOU on the guard). The upload
  gets ACTIONS_RUNTIME_TOKEN and the network, but so do checkout's and setup-node's post steps today (verified handler):
  no new token class. Fix (follow-up, not this text): the cut as its own throwaway uid on its own copy of the workspace
  (output copied out by root and checked), plus `unshare --pid --fork` so no cut process outlives the step.
  A High would need: a secret, write scope or id-token: write in this job; anything that executes or trusts the
  artifact's content (preview.yml running a file from it, or taking a host-acted file such as _headers/_redirects/
  _worker.js from it); or a text claiming the steps after the cut are protected.
- SA-2 (Low) the accepted-risk record CUT-ESCAPE (STATUS_ARCHIVE.md:816-818) rests on "contents: read and no secrets"
  and names "a separate job whose output nothing trusted runs" as a boundary; after this change a workflow holding the
  Cloudflare token publishes what the cut wrote. The acceptance now also rests on preview.yml treating it as data
  (lstat walk, refuse list, never run). Fix: add that sentence to CUT-ESCAPE's follow-up in the docs pass.
- B(i): absolute shell and programs: keep (they close the PATH door, CUT-PATH). They do not close BASH_ENV (verified
  locally: sourced under --noprofile --norc) or LD_PRELOAD (loads into /usr/bin/bash itself) set through GITHUB_ENV;
  no per-vector patches in the text: the uid split is the fix. No finding on the text beyond SA-3's wording.
- B(ii): no extra step at all is safe too; the guard step is risk-neutral (a run step: no ACTIONS_RUNTIME_TOKEN, no job
  token) and adds a check and a count. Recommend keeping it (C).
- SA-3 (Low) toolchain.yml (rebuilt) :215-218: the guard's comment reads as protection; state its limit. Replacement in
  the summary.
- SA-4 (Low) :225-226 + D: with include-hidden-files off a dot-named file is dropped silently and the guard's count
  and the upload's count differ with nothing red; lane 2 refuses dot names anyway. Fix: the guard fails on dot names too.
- SA-5 (Low, nit) :4 is 156 characters; the header wraps at 120 elsewhere. Rewrap :4-6.
- SA-6 (Low) :231 "no file whose own name begins with a dot leaves": also dot-named folders and all inside them.
- SA-7 (Low) citations ask 2 carries: input-helper.ts getInputs runs :8-58 (plan says 8-45); omitBrokenSymbolicLinks
  is search.ts:18 (advisor-plan.md says :17).
- E: artifact public to any signed-in GitHub user for 7 days (action.yml:65: anonymous downloads prompted to log in):
  the production page only, which the preview publishes to everyone anyway: no finding. It is also a new way out of
  the cut for whatever the cut can read; nothing secret is in reach there today (the job token lives only in the gh
  step's env; persist-credentials false), and SA-1's routes reach further: covered by SA-1. The `if:` is tidiness;
  the boundary is preview.yml's `check` job (name, path, event, success, head_repository): holds at 9ded5a8. Line 216
  names preview.yml, which is on lane 2's branch, not main: true once lane 2 merges; harmless before (no consumer).
  TOKEN-WORDS :94 true (upload holds the runtime token, not the job token, and is not a run step). :130 true.
- Any replacement text changes 067521c9...: the session rebuilds, re-hashes and re-runs plant-test.sh (plus a dot case).

## Corrections after the second read (these win over the lines above)
- Evidence labels: the three runner files (FileCommandManager.cs, NodeScriptActionHandler.cs, ScriptHandler.cs) were
  read on actions/runner's main branch through a fetched summary, not the raw text and not the hosted runner's version.
  "Main and post steps alike" is my inference (one handler runs a node action; the summary does not say it). The
  ScriptHandler id-token condition stays inferred. Ownership of _actions and writability of _runner_file_commands and
  of <Temp> from inside the cut: inferred (same uid, runner layout), not measured. Verified: tag lines, the action's
  source lines, the docs' success() rule, BASH_ENV locally, the guard lines locally. No claim on token lifetime or cache scope.
- B(i): severity word: no finding on the text (keep the absolute paths); its residual limits belong to SA-1 (Medium,
  pre-existing). B(ii): no finding: no step is also safe; recommend keeping the guard (C).
- Line numbers above refer to the rebuilt file (067521c9...), not the diff.
- preview.yml / preview.mjs claims are as read at 9ded5a8 on lane 2's branch, not main.
- Memory: the agent file says `memory: local`; no folder found at .claude/agent-memory-local/security-auditor/ (main
  folder and worktree) nor /root/.claude/agent-memory/: the threat model was not read from memory.

## D for both branches of C
- GUARD-STEP yes (with SA-4's line): dot names fail the guard, so the setting never decides anything; leave
  include-hidden-files off (default false). Recommended.
- GUARD-STEP no: off drops a dot-named file or folder silently and the preview deploys without it; `true` puts it in the
  7-day artifact (public to signed-in users) and lane 2's refuse list (preview.mjs:160) turns the preview red. Choose
  off: a stray dot file (.env, .git, .DS_Store) never leaves the runner, and a missing file shows in the page itself.
  Lane 2's reason for `true` (".gate may count as hidden") does not hold: dist/upload/index.js:130173 tests the basename
  of each item visited from the search path down; the first item is `dist-production`.
- The comment texts below are for GUARD-STEP yes with SA-4. If GUARD-STEP is no, the guard comment (:215-218) loses its
  last two sentences about the check and keeps only the first, and :231 reads as in SA-6's plain variant.

## Exact replacement texts (rebuilt-file lines), applied to a scratch copy and checked
- SA-5, :4-6 become:
# The cut: the step "Build and test with the network cut off" runs in its own network namespace (sudo unshare --net),
# with loopback up, back as the runner's user with no docker group and no_new_privs; a TCP probe to an address
# resolved before the cut must connect outside and fail inside (only curl exit 7 or 28 counts as cut).
- SA-3, :217-218 become (four lines after :216 unchanged):
      # treats these files as data). The cut's code wrote the build, so first a check that it holds only plain files
      # and folders, none named with a leading dot (the upload follows symbolic links and drops dot names unasked),
      # with absolute paths, since the cut's user can write in PATH. It catches a link or dot name left in the build and
      # prints the count; it is no boundary: code that ran inside the cut is the runner's user and can still change
      # later steps (CUT-ESCAPE).
- SA-4, :225-226 become:
          bad=$(/usr/bin/find "$d" -mindepth 1 \( ! -type f ! -type d -o -name '.*' \) -print)
          [ -z "$bad" ] || { echo "not a plain file or folder, or a name that begins with a dot:"; echo "$bad"; exit 1; }
- SA-6, :231 becomes (with SA-4):
      # An empty upload fails; nothing whose own name begins with a dot leaves, file or folder (the check above refuses
      # such names, so its count equals the upload's).
  plain variant (without SA-4 / without the guard):
      # An empty upload fails; nothing whose own name begins with a dot leaves, file or folder (the build holds none today).
- Checks on the scratch copy with all four (sa-tools/apply.py): sha256 d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8
  (067521c9... no longer applies to that variant); no changed comment line over 120 characters; YAML parses (11 steps);
  the guard script passes `bash -n`; on the real build here "5 files, 892209 bytes", exit 0; a planted `.DS_Store`,
  exit 1 with its path. plant-test.sh needs a dot-file and a dot-folder case added. The session rebuilds the diff,
  re-hashes and re-runs the trials before ask 2.
- Correction: the plain variant of :231 above is 124 characters; use two lines instead:
      # An empty upload fails; nothing whose own name begins with a dot leaves, file or folder (the build holds
      # none today).
