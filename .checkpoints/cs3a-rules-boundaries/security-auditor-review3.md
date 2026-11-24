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
