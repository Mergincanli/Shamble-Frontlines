# security-auditor: round 3 review of change-set 3a (fix commit a1b2e802 over 79666ad9)

Started 2026-11-24 16:14 UTC (date -u). Worktree .gate/ci-wasm-wt, branch cs/3a-rules-boundaries at a1b2e802.
Scope: only `git diff 79666ad9 a1b2e802 -- . ':!.checkpoints'` (11 files). Round 2 notes: security-auditor-review2.md
(REVISE: High = dev server served pkg-rules through npm's link node_modules/@shamble/sim-worker; Lows N1, N2;
relative CARGO_HOME note). Shared worktree read-only; builds, servers, plants only in my copy .gate/security-auditor-cs3a/r3/.
16:18 coordinator: the head to judge is now 8619d539 (one file since a1b2e802: isolation.spec.ts; CI red on a1b2e802).

## Plan
1. Read the diff (11 files); hold check (no *.toml, *.lock, package*.json, kit/CI/gate path in it).
2. rulesDir: build.mjs now writes the test-only module to <root>/.gate/rules-module/{release,dev}; every reader
   (rules.test.mjs, wasm-rules.spec.ts, build checks) follows; nothing in packages/ or a served folder holds it.
3. vite.config.mjs fs.allow = client, ../sim-worker/src, ../sim-worker/pkg, ../types, ../../node_modules/three.
   Check every entry against links (find -type l) and alternative paths; three's own files and ../types: any link
   or path back into packages/sim-worker or .gate/.
4. Fresh copy r3/copy at the head (packages w/o node_modules, three as a real copy, other root node_modules as links,
   @shamble/* relative links, the stale pkg-rules copy, a .gate/rules-module copy). One fresh dev server per route +
   one warmed server (page loaded in Chromium first): link route to pkg-rules, /@fs/ to .gate/rules-module, .., %2e,
   //, ?raw ?url ?import ?inline ?init ?worker_file, /@id/, '#', HMR vite:invoke. Page runs on a cold Vite cache.
5. isolation.spec.ts new asserts (a1b2e802 and 8619d539); plants devserver-open / devserver-link in plants.py;
   builder's logs (.gate/cs3a/plants-fix2.log, fix2.log).
6. N1 (control characters refused, relative home resolved against base), N2 (comment wording): read + probe.
7. Anything in the diff that widens what the build, the page or the Worker trusts.
8. Verdict.

## Findings and notes (one line each, as found)
- 16:17 Hold: `git diff 79666ad9 8619d539 --stat -- '*.toml' '*.lock' '*package*.json' .github .claude tools CLAUDE.md`
  empty. a1b2e802: 11 files (+120/-59); 8619d539: isolation.spec.ts only (+7/-3).
- 16:17 References: `git grep pkg-rules|rules-module` at a1b2e802: build.mjs:12,71,119; vite.config.mjs:33; the two
  specs; rules.test.mjs:5,11,12,24,61,68; .gitignore:16 keeps packages/sim-worker/pkg-rules/. CI (toolchain.yml) names
  neither; its dev build (:173) is non-production, so it makes .gate/rules-module (`.gate/` ignored, .gitignore:2).
- 16:17 Vite 8.3.7 source (node_modules/vite/dist/node/chunks/node.js): resolvedAllowDir = normalizePath(path.resolve)
  (:25098, no realpath); isFileLoadingAllowed (:18005) = deny glob, safeModulePaths, then allow by prefix (lexical);
  sirv shouldServe -> checkLoadingAccess (:17882) on the lexical joined path; loadAndTransform (:20047) checks ?raw ?url
  ?inline .svg .wasm ?vite-wasm-instance ids, else plugin load, else fs read only if isFileLoadingAllowed. So a link
  matters only when its lexical path lies under an allowed folder.
- 16:17 Links under the allowed folders (find -type l): packages/client 0, sim-worker/src 0, sim-worker/pkg 0,
  packages/types 0, node_modules/three 0 (24 MB, a real folder), vite/dist/client 0. Nested node_modules in
  client/types/sim-worker/net hold only .vite caches (+ client .vite-temp). @shamble/sim-worker exports only "." and
  "./worker" (package.json), so a bare /@id/@shamble/sim-worker/pkg-rules/... should not resolve: to probe.
- 16:18 The stale copy in the shared tree: packages/sim-worker/pkg-rules/{release,dev} (15:26), release wasm sha256
  6f368718... = .gate/rules-module/release/devtools_bg.wasm (live target).
- 16:21 Copy r3/copy (.gate/security-auditor-cs3a/r3/setup-copy.mjs): packages w/o node_modules (stale pkg-rules
  inside), .gate/rules-module, three as a REAL copy (allow list is lexical), other root entries linked, @shamble/* as
  npm links; sha256 of both wasm copies, vite.config.mjs and isolation.spec.ts equal to the shared tree (8619d539).
- 16:23 FRESH dev server per route (r3/devserver-fresh.mjs + r3/routes.mjs; Vite 8.3.7, the page config, as the spec
  starts it): 82 HTTP routes, LEAKS 0 (a leak = body holds the test-only bytes or a marker export name). Controls 200:
  players wasm (application/wasm), three/package.json, types/package.json, /@id/@shamble/sim-worker/worker.
  403: .gate/rules-module by /@fs/ with ?raw ?url ?import ?inline ?init ?raw?? ?import&raw?? ?inline&import
  ?worker_file ?url&import ?.svg?.wasm?init ?vite-wasm-instance ?direct ?t= ?v= .d.ts, sec-fetch-dest script; .. and
  %2e%2e/%2E%2E from pkg, three, client, types; /@fs//, //, /./. Stale copy: direct, every query, .. from pkg/src/
  three/types/client, file/../..; THE LINK ROUTE node_modules/@shamble/sim-worker/pkg-rules/... (wasm, js, ?raw ?url
  ?init ?import ?inline), via @shamble/client/.. and @shamble/types/.., from three/.. -> all 403. Link package.json,
  link pkg/sim_wasm_bg.wasm, link src/worker.ts, link boundary test -> 403.
  200 text/html (SPA fallback, the page): ..%2f, %252e, root-relative ../%2e%2e, absolute path w/o /@fs, /@id/<abs>,
  /node_modules/... root-relative, .map, pkg%2f..%2f. 400: any # (rejectInvalidRequestMiddleware). 500: /@id/ bare
  subpath (exports field refuses: error page quotes the path and a stack, no file text).
  200 stub only: ?worker / ?sharedworker of devtools.js (G, stale, link): generated WorkerWrapper naming the path; its
  Worker address (?worker_file) -> 403. No bytes.
- 16:23 HMR WebSocket vite:invoke fetchModule (token from /@vite/client): "fetchModule is disabled in this
  environment" for all 9 (control included): closed route.
- 16:25 ORDER effect on ONE server (r3/order-probe.mjs): ?worker / ?sharedworker / /@id/ error first, then the direct
  spellings of G, stale and link (?worker_file, plain, ?import, ?raw, wasm, ?url, ?init): all 403, leaks 0. Source:
  safeModulePaths only grows from importAnalysis of a transformed (allowed) module and the html entry (node.js:26640,
  :37038); no allowed file imports the module.
- 16:27 COLD cache page run (r3/page-warm.mjs on r3/cold, a second fresh copy: no packages/client/node_modules/.vite at
  start), real Chromium /opt/pw-browsers/chromium: page RUNS (31 blocks, worker 25 ticks, report errors 0); the one
  console 404 is /favicon.ico (r3/page-404.mjs: request logger in front of the Vite stack; browser-internal, accept
  image/*). Spec asserts through page.request: players 200 application/wasm; .gate/rules-module wasm + js 403; link
  package.json + link players wasm 403; stale pkg-rules wasm 403 (no content-type header: the 403 page sets none);
  a missing file 200 text/html (SPA fallback). WARMED server, all 88 routes + 9 invokes again: leaks 0, the same
  answers as fresh (200s only controls, ?worker stubs, SPA fallback).
- 16:28 Spec at 8619d539 read (isolation.spec.ts:47-58): players = [200, application/wasm] (a fallback 200 cannot
  pass); the four 403s unchanged (files that exist after a non-production build, so a missing one fails loudly as a
  200); the old place: content-type must not hold "wasm" (403 sets no type; missing = 200 text/html). Sound. The
  last check bites only where a stale copy exists, which is the only place the old folder exists at all.
- 16:29 CI (gh run list, branch claude/loving-hopper-6p0mg2): 8619d539 gate success, toolchain success; a1b2e802
  toolchain FAILURE (gate success). Builder log .gate/cs3a/ci-8619d539-toolchain.log:1048-1051 both dev-server
  isolation cases pass, 14 passed; ci-a1b2e802 log:1080 failed at isolation.spec.ts:54 (the old not-200 line).
- 16:29 Plants (plants.py:320-325): devserver-open swaps the sim-worker/src+pkg entries for folder("../..") ->
  caught at the .gate/rules-module 403 line; devserver-link widens three back to node_modules -> caught at the link
  package.json 403 line (.gate/cs3a/plants-fix2.log:87-88, "96 of 96 caught" 16:13). Both catching lines are
  unchanged at 8619d539.
- 16:30 N1 FIXED (r3/remap-n1.mjs, importing the head build-checks.mjs read-only): all 33 control characters
  (U+0000-U+001F, U+007F) refused in CARGO_HOME, in the home folder and in the base: 0 let through; every accepted
  value stays ONE argument after a split at U+001F. build-checks.test.mjs: 29 passed (shared tree, vitest 5.1.0).
- 16:30 Relative-home note FIXED: path.resolve(base, ...) with base = root (build.mjs:66), and every cargo call runs
  in root (build.mjs:95, :128, :152). Residual, not security: a relative CARGO_HOME with ".." is normalized by node
  but (I believe) joined un-normalized by cargo; the remap would then miss and rules.test.mjs:68-72 fails (fails
  closed). Not probed.
- 16:30 N2 FIXED as asked: build.mjs:58-64 and build-checks.mjs:213-216 now say only rustflags are shut out and name
  linker, compiler wrapper, [env], profile as still open.
- 16:31 Trust widening, none found: vite.config.mjs narrower; build.mjs writes .gate/rules-module (rmSync of the
  profile folder, then wasm-bindgen; never read back by the build); page and Worker code unchanged (panic.rs,
  wasm.rs, f32_vectors.rs: comments and a message; rules_scan.rs: a stricter test scan). serve.mjs unchanged and
  confines by lexical + realpath checks (serve.mjs:84, :88-94). Routine guard: node runs only tracked scripts equal
  to main (routine-guard.mjs:988-1012), and a .gate/ entry script is refused (:1001); the PR Advisor runs nothing and
  the night shift could write the old folder as well, so the new place gives no routine new reach.
- 16:32 Head now 1218b4ff (checkpoint commits only; `git diff 8619d539 HEAD -- . ':!.checkpoints'` empty). Boundary
  tests in the shared tree (vitest 5.1.0, root packages/sim-worker, boundary/): 2 files, 17 passed, loading the
  module from .gate/rules-module (built 16:18).
- 16:33 Supplement (r3/devserver-extra.mjs, fresh server each): round 2's remaining spellings on the stale copy and
  the link (/@fs//, //, sim%2Dworker, ?worker_file, ?raw??, ?import&raw??, node_modules//, %40shamble) and encoded
  names (%2egate, %2Egate ?raw, devtools%5fbg, rules%2Dmodule, three/%2e%2e ?url, three/build/../../.. ?import):
  17 more, leaks 0 (403, or the page for %40 which decodeURI keeps encoded).
  TOTAL: 99 HTTP spellings on fresh servers, 88 on a warmed one, 31 in order on one server, 9 WebSocket invokes x 2:
  no answer held the test-only bytes or glue text. => THE HIGH IS CLOSED; condition 2 clause (d) MET for the dev server.
- 16:34 NEW Low N3 (wording of the accepted risk, no new reach): build.mjs:60-63 and build-checks.mjs:212-216 list
  what a cargo config file outside the repository can still set (linker, compiler wrapper, [env], profile) and name
  the gate flag check; the list omits source replacement ([source.crates-io] replace-with = a directory source:
  swaps a crate's code with no diff; the Cargo book, source-replacement page: the directory checksum file "is not a
  security mechanism and does not protect against malicious changes") and build.rustc ("Sets the executable to use
  for rustc", config page). Neither is a flag, so the flag check does not see them. Fix: name both, or "among
  others". STATUS follow-up; no push on its own. Sources read today: doc.rust-lang.org/cargo/reference/
  source-replacement.html (replacement set in .cargo/config.toml; "core assumption ... source code is exactly the
  same"; checksum file not a security mechanism) and .../config.html (build.rustc; [source] keys, env not supported;
  [patch] in config files, config patch wins over Cargo.toml; config files read from cwd and every parent, then
  $CARGO_HOME/config.toml). Confidence: high on the docs; not probed with cargo.

## Verdict (round 3, judged head 8619d539; code equal at 1218b4ff)
APPROVE from security. Open: 0 Critical, 0 High, 0 Medium, 1 new Low (N3, wording, follow-up); L-STATS stays the
STATUS follow-up it was.
- The High (dev server served the test-only module through npm's link): CLOSED. Clause (d) of condition 2: MET for
  Vite's dev server (and still for serve.mjs, the preview server, the spec's routing).
- N1 fixed (33 control characters x 3 places refused; 29 unit tests pass); relative-home note fixed (base = root =
  cargo's cwd); N2 fixed as asked.
- Nothing in the diff widens what the build, the page or the Worker trusts.
- Not verified: Windows/macOS (junctions, case-insensitive paths; read only); the e2e suite and plants.py not run by
  me (CI green on 8619d539 by gh; builder log 96/96); cargo not run (".." in a relative CARGO_HOME, N3 by docs only).
