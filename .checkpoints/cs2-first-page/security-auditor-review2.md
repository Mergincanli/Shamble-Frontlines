# security-auditor review 2, cs2 first page (30a5a13..d257bb6, fix commits 29378dd + d257bb6; main folder, read only)

Plan (2026-11-19T08:05Z): read the scoped diff; check S1-S7 one by one on the code at d257bb6 (S1 serve.mjs pipeline + fd test, S2 extname(real), S3 worker-link onMessage + boot terminate, S4 realpathSync.native, S5 checkOutDir order, S6 cspProblems + header, S7 note()); then what the fixes opened (CSP header on every answer incl. errors, #notice textContent, three console route, bundleProblems, define, env refusal list, mode u32); run the client unit tests and probe serve.mjs on my own port from a scratch copy of the reviewed file; one note line per finding with date -u.

## 08:14:22 UTC evidence run (main folder tree = d257bb6 for packages/crates: git diff d257bb6 HEAD empty)
- Client unit tests: 6 files, 48 passed (vitest 5.1.0, 08:10:47).
- Probe (scratchpad sec-review2/probe.mjs; serve.mjs copies from git show 30a5a13 / d257bb6 into scratch, scratch root, port 0):
  NEW: 20 aborts after first chunk -> fd delta 0, 0 on big.js; 20 raw sockets reset 5 ms after the request -> 0, 0;
  5 paused readers hold 5 descriptors while their sockets stay open (by design; no idle limit, as round 1) -> 0 after close.
  OLD: the same aborts -> fd delta 20, 20 on big.js (resets: 20 more): the new unit test (limit < 5) fails on the old code.
- Built dist (07:50): dist/serve.mjs == packages/client/serve.mjs at d257bb6; playwright webServer = node serve.mjs in
  SHAMBLE_DIST, so the browser tests do run under the header. cspProblems [] and bundleProblems(dev) [] on dist (scan-dist.mjs).
  Built JS: 0 eval(, new Function(, createObjectURL, blob:, importScripts, innerHTML, import.meta.env, VITE_; the 29
  "Function(" hits are identifiers (xxxFunction), no bare Function( call; data: only as object keys + one /^data:/ test;
  glue (__wbindgen/__wbg_) and WebAssembly. only in worker-*.js.

## 08:14:22 UTC verdicts S1-S7
- S1 CLOSED serve.mjs:138 pipeline(createReadStream, response, () => {}); test serve.test.mjs:164-179 fails on the old code (shown).
- S2 CLOSED serve.mjs:94-96 type from extname(real); alias.js -> 404 (old: 200 with the source). Side effect, harmless: an
  in-folder link or a Windows variant name (8.3 short name, trailing dot) is served by the real file's type; no file outside
  the folder or of another type becomes reachable (the real path decides both).
- S3 CLOSED worker-link.ts:78 (returns before #takeBlock once failure is set) + boot.ts:53-62 (terminate on first problem, error listener once). Error-event path leaves link.failure empty: frames keep posting to a terminated Worker (no-op), no blocks arrive, no growth.
- S4 CLOSED serve.mjs:107 realpathSync.native(root); fs/promises realpath is the native binding too, so both sides resolve alike.
- S5 CLOSED build.mjs:130 checkOutDir before buildWasm; build-checks.mjs:157-161; buildPage's only caller is main(). Residual (not a finding): any folder named dist-<x> anywhere is emptied, and a dist-<x> link empties its target (planted link: out of model).
- S6 CLOSED for what round 1 asked of the scan (build-checks.mjs:121-126) and the header (serve.mjs:43-51); exact string
  equality with round 1's policy checked in the probe (true). Not taken from my fix text: the spec failing on any
  securitypolicyviolation (no listener anywhere in packages/e2e) and Cross-Origin-Resource-Policy: same-origin.
- S7 CLOSED error-hook.ts:16 (C0 + DEL -> space, cut at 300). Nit: U+0085, U+2028, U+2029 pass (no input carries them in cs2).
