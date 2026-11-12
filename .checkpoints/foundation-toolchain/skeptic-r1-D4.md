# skeptic, review round 1, D4 (= SA-1, Low), foundation 2.2 (PR #48), head 4d78ad61e652f2e35ddf51c49557e725420248c8

Plan: re-read toolchain.yml at 4d78ad6 (what runs inside the cut, which namespaces, which uid, what runs after it), the
report's Risks lines (:140-150), the CI logs of the head's toolchain runs (checkout's post step: which git commands, git
version, where the action code and node live), then reproduce the crossing in a scratch repo under
/home/user/Shamble-Frontlines/.gate/skeptic-D4/ (core.fsmonitor + the post step's exact git commands, as far as the
guard allows), weigh the impact against gate.yml's existing exposure, rate it on CLAUDE.md's scale, and judge the
proposed fix (right, complete, what it would break). Default: does not hold unless the evidence is there.

## Checks (one line each, as done)
- [checked] worktree HEAD 32386e1; `git diff --stat 4d78ad6 HEAD` touches only `.checkpoints/`, so the reviewed files are
  read from 4d78ad6 with `git show`.
- [checked] report at 4d78ad6, :145-146: "it is not a sandbox. The steps after it run with the network, and inside it the
  tests share the runner's process and mount namespaces (step 7's security review checks the rest)". Quoted correctly.
- [checked] plan.md :386-389 (section 4.4, notes deleted before ready): "the cleared groups and `no_new_privs` close the
  two easy ways out (docker, sudo)". Quoted correctly; :371 (N5) already notes a resolver "through a socket file".
- [checked] toolchain.yml at 4d78ad6, :68-73: `sudo -n unshare --net` (a network namespace only: no mount, PID, IPC or
  user namespace), `setpriv --reuid="$(id -u)" --regid=nogroup --clear-groups --no-new-privs`, `env -i HOME="$HOME"
  ...`. The cut is the job's last step (:60-88); inside it run `cargo test`, `npm ci --offline --ignore-scripts`,
  `npm test --workspaces` (vitest) and `npx tsc`. Before it, `npm ci --ignore-scripts` and `cargo fetch` run no
  package code, so in this job third-party JS runs only inside the cut. `permissions: contents: read`; no secret named.
- [checked] CI logs re-downloaded (/home/user/Shamble-Frontlines/.gate/skeptic-D4/logs/): PR job 131510257155 :254-:259
  inside the cut `uid=1001(runner) gid=65534(nogroup) groups=65534(nogroup)`, only `lo`, curl exit 7; inside the cut
  cargo compiles into the workspace (:260) and `npm ci --offline` reinstalls `node_modules` in the workspace root
  ("added 46 packages", :381), then vitest runs four times (:384-:434), so the tests can write the checkout's folder
  (needed to rewrite `.git/config`; the checkout step created it as the same uid 1001). :447-:448 two "Post job cleanup." (setup-node's, then
  checkout's), :449-:450 git 2.56.0, :456, :459, :462 `/usr/bin/git submodule foreach --recursive ...` three times,
  after the cut, as the runner outside the namespace. Push job 131510223737 :435-:451 the same (foreach at :444, :447,
  :450). The evidence lines cited by SA-1 (447-463) hold.
- [checked] local reproduction, git 2.43.0, fresh scratch repo /home/user/Shamble-Frontlines/.gate/skeptic-D4/repo
  (`git init`, `git add a.txt`: an index with no fsmonitor extension; no commit, the guard refuses `git -c` and
  `git config`): `core.fsmonitor = <abs path>/hook.sh` appended to `.git/config` as a plain file edit (what code inside
  the cut would do, no git involved); `git version` left no marker; `git submodule status` (no submodules) exit 0 and
  marker.txt got "fsmonitor hook ran: uid=0 args=2 ..." then "args=1 ...". The post step's exact `git submodule
  foreach` was not run: main's guard denies it (routine-guard.mjs:1052 "git submodule foreach runs commands; not
  here"), and I did not route around that.
- [checked] git v2.56.0 source (the runner's version, CI :450), fetched as data into
  /home/user/Shamble-Frontlines/.gate/skeptic-D4/gitsrc/: git-submodule.sh :186 and :571 send `foreach` and `status` to
  `submodule--helper`; builtin/submodule--helper.c :458 module_foreach calls module_list_compute before it runs any
  command, :882 module_status calls the same, and module_list_compute :241 calls repo_read_index; read-cache.c :1962-1967
  post_read_index_from -> tweak_fsmonitor; fsmonitor.c tweak_fsmonitor -> add_fsmonitor (index without the extension)
  -> refresh_fsmonitor -> query_fsmonitor_hook (`cp.use_shell = 1`, so the value can even be a shell line).
  fsmonitor-settings.c: a non-boolean core.fsmonitor = hook mode unless the repo is bare or (Linux has
  FSMONITOR_OS_SETTINGS = unix in config.mak.uname :77-78) the worktree is on NFS/CIFS/SMB; fsm-settings-unix.c checks
  only IPC mode. The runner's workspace is a local disk, so hook mode applies. So `foreach` runs the planted hook at
  its index read, with or without submodules, before its own command. Documentation/config/core.adoc :78-79: "this
  variable contains the pathname of the "fsmonitor" hook command" (SA-1's git-config(1) quote holds at v2.56.0).
  Not tested on the runner itself; the crossing rests on the CI log + v2.56.0 source + the local sibling-path run.
- [checked] man7.org pages (saved as data under /home/user/Shamble-Frontlines/.gate/skeptic-D4/man/):
  network_namespaces(7) lists devices, IP stacks, routes, firewall, ports "In addition, network namespaces isolate the
  UNIX domain abstract socket namespace"; unix(7): pathname sockets are "bound to a filesystem pathname" and
  "connecting to a stream socket object requires write permission on that socket", so with the mount namespace shared,
  a pathname socket stays reachable by file permissions (SA-1's route b holds as reasoning). CI :257 shows ordinary
  lookups fail inside ("getent: no answer"), so that route needs code that talks to such a socket on purpose; whether
  the image's resolver or D-Bus socket answers there is untested (SA-1 says so). kill(2): "the real or effective user
  ID of the sending process must equal the real or saved set-user-ID of the target process": same uid 1001, no PID
  namespace, so code inside can signal the runner's processes (route c holds).
- [checked] exploit sketch's packages, root node_modules (the same as CI's `npm ci`): vite 8.3.7's
  dist/node/chunks/node.js imports `rolldown` statically (it loads with vitest, which the log shows running) but
  `lightningcss` only by dynamic `import("lightningcss"` (lazy, for CSS); so "lightningcss runs inside the cut" is not
  shown for these tests (its files are there for any code inside to load). A small overstatement in the sketch; it
  changes nothing.
- [checked] main's gate.yml: the smoke job runs the toolkit's packages (Playwright, `playwright install --with-deps`)
  with the network and sudo, and saves ~/.cache/ms-playwright (key `playwright-chromium-<os>-<version>`) through
  actions/cache's post step; `run:` step code there can reach a JavaScript post step by the same routes. So SA-1's
  "gate.yml's toolkit dependencies have the same exposure today" holds; the cut adds no exposure and removes none.
- [checked] GitHub docs, "Dependency caching" reference (docs.github.com, fetched 2026-11-12, saved under
  /home/user/Shamble-Frontlines/.gate/skeptic-D4/ghdocs/): a pull-request run's cache "is created for the merge ref
  (refs/pull/.../merge) ... can only be restored by re-runs of the pull request. It cannot be restored by the base
  branch or other pull requests"; "only these workflow triggers can create or overwrite caches in the default branch's
  scope: push, workflow_dispatch, ..."; "Workflow runs cannot restore caches created for child branches or sibling
  branches." toolchain.yml runs on `push` for every branch, main included.
- [checked] not shown by any log line: the runner's docker group outside the cut (`id` is printed only inside, :254;
  research-network.md :53 planned an `id` before the cut, the job does not print one). Not needed for the claim:
  GitHub documents passwordless sudo, so code outside the cut has root anyway.
- [sourced] impact: cache writes from a run land in its own ref's scope (refs/pull/N/merge or the branch), so reach
  into main's scope needs the hostile code to run on a push to main, i.e. already merged, where gate.yml has the same
  reach; `contents: read`, no secrets, an ephemeral VM. No wrong result for a player, Mustafa or a session; the report
  overclaims nothing ("it is not a sandbox"). The defect is the report's IOU "(step 7's security review checks the
  rest)", which points at notes deleted before ready: Low (wording/completeness), as SA-1 rates it.

## Fix, judged
- Right in substance: report text plus a 2.3 finding, no CI edit in 2.2 (A10: any other change to the CI job asks him
  again). It breaks nothing (docs only).
- Not complete as written:
  1. The 2.3 option "no post-job step that runs git in a workspace the tests can write" closes only the fsmonitor
     instance, not the route: setup-node's post step (:447) and checkout's own post step still run after the cut. By
     the runner's layout (reasoning; no log line shows where action code lives or who owns it), their code sits in the
     runner's work folder, writable by uid 1001, and the runner applies the step's GITHUB_PATH and GITHUB_ENV files
     after it (`env -i` hides their names, not the files). A boundary is a different uid that cannot write the runner's
     files (with its own copy of the workspace), a container or VM, or a separate job whose output nothing trusted
     runs; plus the cache design.
  2. Present the routes as examples of one class (later steps, post steps and the runner run or read files the same
     uid can write), not a closed list of three; mark each tested or not: route a's mechanism shown locally (git
     2.43.0, `git submodule status`) plus v2.56.0 source plus the CI log, not run on the runner; route b from
     documentation only; route c from kill(2); the cache reach from GitHub's caching docs (own ref's scope; main's
     scope only from a push to main).
  3. "Hermetic tests" says more than the cut gives (the filesystem is shared): keep the report's own words, "the tests
     run without the network" (:145).
  4. Per the report's convention (:156), the 2.3 finding's full text goes in STATUS_ARCHIVE "From foundation 2.2", with
     a row in the report's table.

## Verdict
D4 holds as stated, severity Low. Not shown: the exact `foreach` (the guard denies it) and anything on the runner
itself; the docker group outside the cut; whether the resolver or D-Bus socket answers there; lightningcss loading in
these tests.
