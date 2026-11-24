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
