# security-auditor: round 2 review of change-set 3a (fix pass 79666ad9 over code head 5c923119)

Started 2026-11-24 15:28 UTC (date -u). Worktree .gate/ci-wasm-wt, branch cs/3a-rules-boundaries at 79666ad9.
Scope: only `git diff 5c92311 79666ad9 -- . ':!.checkpoints'` (23 files, +667/-204). Round 1 notes:
security-auditor-review1.md (M-TREE; L-DEVSERVER, L-ROOM, L-STATS, L-PYI, L-CASE, L-REMAPHOME).
Shared worktree read-only for me; plants, builds and servers only in my copy under .gate/security-auditor-cs3a/.

## Plan
1. Manifest/lock hold: diff --stat on *.toml *.lock *package*.json (empty = hold untouched); kit/CI paths in the diff.
2. M-TREE: runtimeCrates(treeText, root, rootCrate) + missingOverflowOverrides; tests; plants foreign-source,
   tree-any-root in plants.py; my tree-parse probe re-run on the new code (git source, other registry, path crate
   outside crates/, crates/../ spelling, symlink, root line not sim-wasm, empty tree, Windows separators).
3. L-DEVSERVER / cond. 2 (d): vite.config.mjs fs.allow; isolation.spec.ts dev-server case; in my own copy, probe
   the dev server and the preview server for pkg-rules by /@fs/ (other spellings, case, //, %2e, encoded slash),
   ?raw, ?url, ?import, ?inline, /node_modules/@shamble/sim-worker/..., /@id/, symlink into pkg-rules, and the
   preview server's dist only.
4. L-REMAPHOME: registryRemapFlag(cargoHome, homeDir); CARGO_ENCODED_RUSTFLAGS in cargoEnv for every cargo child;
   refusedBuildSettings upper-case (L-CASE); does the env var shut out config-file rustflags (Cargo book), what stays
   outside (config-file [profile], [build] other keys, target.<triple>.linker/runner, [env], [unstable]).
5. L-ROOM: panic-message.ts PANIC_TEXT_ROOM = 1016 check; trapMessage; tests. L-PYI: run lines `python3 -I -B`.
6. New surface: rest of the diff (panic.rs, rng.rs, rules_scan.rs, f32 vectors/generators, host.ts, messages.ts,
   worker.ts, sim-wasm.test.mjs) for anything that widens what the page, Worker, build or CI trusts.
7. Verdict.

## Findings and notes (one line each, as found)
- 15:29 Hold: `git diff 5c92311 79666ad9 --stat -- '*.toml' '*.lock' '*package*.json'` empty. 23 files changed.
- 15:34 Worktree clean, HEAD 79666ad9 (commit 15:27). `vitest run --root packages/client scripts/build-checks.test.mjs`:
  29 passed. Main session's plant outputs (.gate/cs3a/plant-*.out, 15:24-15:26): foreign-source, tree-any-root,
  devserver-open, remap-empty-home, env-lower-case, reader-other-room, trap-sentence, room-drift all CAUGHT (FAIL line
  of the named test; devserver-open at isolation.spec.ts:47 '>' frame).
- 15:35 M-TREE code read (build-checks.mjs:238-249): ours = `<root>/crates/` prefix after \ -> /; first line must start
  `sim-wasm v`; root Cargo.toml members = ["crates/*"], no exclude (Cargo.toml:3). Residual by design: membership is
  cargo's (members/exclude), the check approximates it by folder; an `exclude` needs a root-manifest change (held).
- 15:39 M-TREE real-cargo probe (.gate/security-auditor-cs3a/r2/tree-probe.mjs on scratch r2/tree-ws: members
  ["crates/*"], root profile as ours; cargo 1.99.1 tree + `build -v --release --target wasm32`): real lines fed to the
  real runtimeCrates -> ["dotdot","outside"]. cargo's own flag: outside (path crate outside the workspace) none = OFF ->
  named: right. linked (symlink crates/linked -> outside folder; cargo prints crates/linked, member) ON -> ours: right.
  dotdot (`../sim-wasm/../../vendor/dotdot`, printed normalized, auto-member) ON -> named anyway: stricter, fails
  closed. CRLF and a leading blank line read the same; a root line with leading spaces throws. M-TREE FIXED.
- 15:43 env-flags probe (.gate/security-auditor-cs3a/r2/remap-probe2.mjs: scratch crate r2/remap-crate with its own
  .cargo/config.toml, manifest overflow-checks=true, `cargo build -v --offline --release --target wasm32`, env as
  build.mjs:62 cargoEnv builds it): control [build] rustflags w/o env -> `--cfg evil` reaches rustc (exit 101). With
  CARGO_ENCODED_RUSTFLAGS: [build] rustflags, [target.wasm32-unknown-unknown] rustflags, [target.'cfg(..)'] rustflags all
  shut out (exit 0, only the remap arg). CLAIM HOLDS for rustflags. Still outside: config [profile.release]
  overflow-checks=false -> no `-C overflow-checks=on` (OFF; documented at build-checks.mjs:213-214); config
  [build] rustc-wrapper -> wrapper on the line, `--cfg evil` reaches rustc (exit 101) even with the env: build.mjs:58-59
  "a flag in a cargo config file outside the repository does not reach the compiler" is too broad.
- 15:43 NEW (fix pass) CARGO_HOME holding U+001F: registryRemapFlag keeps it, CARGO_ENCODED_RUSTFLAGS splits at U+001F ->
  rustc got 4 args incl. `--cfg evil` (exit 101). Round 1's --config route kept any value one argument. Reach: whoever
  sets CARGO_HOME (or HOME when unset) already picks $CARGO_HOME/config.toml (rustc-wrapper above) -> no new reach. Low.
- (clock) date -u read 15:40 at the copy and the first server run: the 15:39/15:43 stamps above are estimates that ran
  ahead of the clock; from here stamps are date -u readings.
- 15:41 Copy for servers: r2/setup-copy.mjs -> r2/copy (packages/ w/o node_modules, pkg + pkg-rules as built in the
  shared tree, sha256 equal; root node_modules = links to the shared entries; @shamble/* = relative links into
  copy/packages as npm makes them; dist-development copied). Probe r2/devserver-probe2.mjs (raw http.request paths).
- 15:41 DEV SERVER, one server for all requests: fs.allow = client, sim-worker/src, sim-worker/pkg, types,
  <root>/node_modules (+ vite/dist/client); deny = Vite's default list. Spec's 3 cases: 200 / 403 / 403 as the spec says.
  Through the workspace link: GET /@fs<root>/node_modules/@shamble/sim-worker/pkg-rules/release/devtools_bg.wasm -> 200,
  55,453 bytes = the test-only Wasm (dev 275,366 too); .../devtools.js -> 200 (transformed), ?raw ?import ?url ?init
  -> 200; boundary/rules.test.mjs -> 200. CLAUSE (d) NOT MET. After the link route ran, even the direct spellings gave
  200 (pkg/../pkg-rules, //, ?raw...): order effect suspected (module graph / safe paths) -> re-run one fresh server per
  case. Preview server: every path -> index.html (570 bytes), never a test-only file: preview fine.
- 15:42 FRESH dev server per request (r2/devserver-fresh.mjs, 25 routes): every spelling of the direct path
  packages/sim-worker/pkg-rules/... -> 403 (.., %2e%2e, /@fs//, //, src/../, sim%2Dworker, ?raw, ?raw??, ?import&raw??,
  ?import, ?url, ?inline, ?init, ?worker_file); boundary/ direct -> 403. The workspace-link spelling
  /@fs<root>/node_modules/@shamble/sim-worker/pkg-rules/{release/devtools_bg.wasm, dev/devtools.js, release/devtools.js?raw,
  release/devtools_bg.wasm?url} -> 200, the test-only bytes. Why: vite.config.mjs:35 allows folder("../../node_modules")
  and npm's link node_modules/@shamble/sim-worker -> ../../packages/sim-worker puts the whole package lexically under
  it; Vite checks the requested path, sirv follows the link. Order effect of run 1: once a link-route `?import`
  transform has put a test-only file into the module graph (Vite's safe module paths), the direct spellings give 200
  too. => L-DEVSERVER NOT FIXED; condition 2 clause (d) NOT MET; isolation.spec.ts:46-48 tests only the direct
  spelling, so it passes while the module is served (plant devserver-open only restores the old allow entry).
- 15:43 Real Chromium (r2/page-probe.mjs, playwright-core + /opt/pw-browsers/chromium, my copy, page.request.get as the
  spec does): page's own config -> page runs (30 blocks, errors 0), players' Wasm 200, direct 403/403, WORKSPACE LINK
  200/200. Fix tried in my copy only (r2/copy/packages/client/vite.narrow.config.mjs: folder("../../node_modules/three")
  in place of folder("../../node_modules")): page runs the same (30 blocks, errors 0; the one 404 console line is in both
  runs), link route 403/403. So the fix is one allow entry + the link spelling in isolation.spec.ts:46-48 (+ a plant).
- 15:46 Source read (Vite docs, vite.dev/config/server-options, fetched today): fs.allow "Restrict files that could be
  served via /@fs/ ... accessing files outside this directory list that aren't imported from an allowed file will
  result in a 403" (=> the order effect: an imported file is servable); fs.deny default ['.env', '.env.*',
  '*.{crt,pem,key,p12,pfx,cer,der}', '.npmrc', '.yarnrc.yml', '**/.git/**'], "Setting this option replaces the default
  list entirely"; NOTE "Vite cannot guarantee that a denied file is inaccessible through an alternative path. If you
  have an alternative path, include it in the deny list as well." The workspace link is such an alternative path.
- 15:46 L-ROOM FIXED: panic-message.ts:20 refuses room != PANIC_TEXT_ROOM (1016) before any read (length <= 1016 and
  inside memory); trapMessage :31-35 = "the simulation stopped: <panic>" or the thrown text; worker.ts:57-59 reads an
  empty memory before load. Plants reader-other-room, trap-sentence, room-drift caught. panic.rs: start is `private`
  (no glue export; sim-wasm.test.mjs pins the export list), the hook's `saturating_sub` removes a panic inside the hook.
- 15:46 L-PYI FIXED: both docstrings say `python3 -I -B`; both `--check` pass under -I -B (chacha8 160,384 bytes; f32
  38 ops, 10,048 cases); stdlib imports only, no subprocess/eval/exec/open/socket; write only their two files.
- 15:46 L-CASE FIXED: refusedBuildSettings upper-cases names (build-checks.mjs:227); test rustflags, Rustc_Bootstrap
  refused, cargo_target_dir and path let through; plant env-lower-case caught.
- 15:46 L-REMAPHOME FIXED with a residual: "" and unset -> <home>/.cargo, trailing slashes cut; a relative CARGO_HOME
  is resolved against node's cwd (path.resolve, build-checks.mjs:322) while cargo resolves it against its own cwd = root
  (build.mjs:88,121,145 cwd: root; checked: CARGO_HOME=rel-home made remap-crate/rel-home/.global-cache). Same when run
  from the root (build.mjs:1). Fix: path.resolve(root, ...). Follow-up, not security.
- 15:46 L-STATS not in this pass (a STATUS follow-up by round 1's verdict); messages.ts:144 isFinite change equivalent.
- 15:46 Every cargo child of build.mjs gets cargoEnv (tree :88, rules :121, players :145); no other cargo call in
  packages/. No kit, gate, CI, manifest or lock file in the diff; code unchanged since 79666ad9 (checkpoint commits only).
- 15:46 Source read (Cargo book, doc.rust-lang.org/cargo/reference/config.html, fetched today): build.rustflags "four
  mutually exclusive sources ... checked in order, with the first one being used: 1. CARGO_ENCODED_RUSTFLAGS ...
  3. all matching target.<tuple>.rustflags and target.<cfg>.rustflags ... 4. build.rustflags" (=> the env shuts out
  config rustflags: confirmed by my probe); with --target, host builds (build scripts, proc macros) get none.
  build.rustc-wrapper "Sets a wrapper to execute instead of rustc" (and rustc-workspace-wrapper, nested).
  target.<tuple>.linker "passed to rustc (via -C linker)". [profile] in config "override settings specified in
  Cargo.toml". => build.mjs:58-59 "a flag in a cargo config file outside the repository does not reach the compiler"
  is too broad: linker (a -C flag), rustc-wrapper/rustc-workspace-wrapper/build.rustc and [profile] still do. Low
  (wording of a security claim; the accepted risk "config files outside the repo" stands, now narrower).
- 15:48 Tests at this head (shared tree, read-only runs): sim-worker src panic-message/host/messages 20 passed; boundary
  (sim-wasm + rules, on pkg/pkg-rules as built 15:26) 17 passed.
- 15:48 Links in <root>/node_modules (find -type l, not .bin): only @shamble/{types,net,client,e2e,sim-worker} ->
  ../../packages/*; no link under the other allowed folders. So through folder("../../node_modules") the dev server
  serves every workspace package folder whole; vite.config.mjs:29-31 ("serves only the folders the page loads from ...
  never the test-only module") and commit 79666ad9's "The dev server reads only the Worker's src and pkg folders, so it
  refuses the test-only module (403)" do not hold.
- 15:48 SEVERITY of the dev-server item: HIGH under CLAUDE.md's scale ("evidence that does not hold": the commit's claim,
  the spec's 403s and plant devserver-open are offered as closing my condition 2 clause (d); the same server answers
  200 with the test-only bytes). Direct harm small (127.0.0.1 dev server, public test code, no secret, players never
  use it). Fix (tested in my copy): folder("../../node_modules/three") in place of folder("../../node_modules"); add the
  link spelling to isolation.spec.ts:46-48 (403 for both files) and a plant that widens node_modules back. Sturdier
  alternative: build pkg-rules outside packages/ (e.g. under .gate/), where no workspace link reaches it.
- 15:48 NEW Low N1 (U+001F split, above): build-checks.mjs:321-323 + build.mjs:62. Fix: refuse a cargo home holding a
  control character (U+0000-U+001F, U+007F) in registryRemapFlag, one test line.
- 15:48 NEW Low N2 (wording, above): build.mjs:58-60 and build-checks.mjs:213-215. Fix: "rustflags of a cargo config
  file are shut out; its linker, compiler wrappers and profile settings are not (accepted risk; the flag check)".

## Verdict (round 2, code head 79666ad9)
REVISE. Open: 0 Critical, 1 High, 0 Medium, 3 Low (N1, N2 new; L-STATS carried as its STATUS follow-up).
- M-TREE fixed. L-DEVSERVER NOT fixed, raised to High (workspace-link route 200; claims of 403 do not hold).
  L-ROOM, L-PYI, L-CASE fixed; L-REMAPHOME fixed (relative-path note); L-STATS not in this pass (follow-up).
- Condition 2 clause (d): NOT MET for Vite's dev server; met for serve.mjs, the preview server, the spec's routing.
- New: N1 Low (U+001F split, build-checks.mjs:321-323, build.mjs:62); N2 Low (wording, build.mjs:58-60,
  build-checks.mjs:213-215).
- Hold untouched (no manifest/lock/package.json change). A third round is warranted for the High only (fix tested in
  my copy: allow node_modules/three; spec + plant for the link spelling).
- Not verified: no build, no plants.py run, no e2e suite; Windows only by reading; git-source tree line not from real
  cargo; the narrowed allow-list on one page load only. Memory updated (.claude/agent-memory-local/security-auditor).
- Clock: stamps 15:39-15:46 were up to 2-3 minutes ahead of date -u; the review ran 15:28 to about 15:50 UTC.
