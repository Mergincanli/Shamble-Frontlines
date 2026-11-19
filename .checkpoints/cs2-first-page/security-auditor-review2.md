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
  in-folder link (noext -> app.js, x.txt -> index.html: 200 by the real type) or a Windows 8.3 short name is served by the
  real file's type; no file outside the folder or of another type becomes reachable (the real path decides both).
- S3 CLOSED worker-link.ts:78 (returns before #takeBlock once failure is set) + boot.ts:53-62 (terminate on first problem, error listener once). Error-event path leaves link.failure empty: frames keep posting to a terminated Worker (no-op), no blocks arrive, no growth.
- S4 CLOSED serve.mjs:107 realpathSync.native(root); fs/promises realpath is the native binding too, so both sides resolve alike.
- S5 CLOSED build.mjs:130 checkOutDir before buildWasm; build-checks.mjs:157-161; buildPage's only caller is main(). Residual (not a finding): any folder named dist-<x> anywhere is emptied, and a dist-<x> link empties its target (planted link: out of model).
- S6 CLOSED for what round 1 asked of the scan (build-checks.mjs:121-126) and the header (serve.mjs:43-51); exact string
  equality with round 1's policy checked in the probe (true). Not taken from my fix text: the spec failing on any
  securitypolicyviolation (no listener anywhere in packages/e2e) and Cross-Origin-Resource-Policy: same-origin.
- S7 CLOSED error-hook.ts:16 (C0 + DEL -> space, cut at 300). Nit: U+0085, U+2028, U+2029 pass (no input carries them in cs2).

## 08:19:35 UTC more checks
- S3 shown: round-1 harness rerun on the fixed worker-link.ts (scratchpad sec-review2/link-after-failure2.mts, GC given
  time to sweep): after an error message, 300 blocks of 1 MiB with frames drawn -> every answer "", 0 input posts, blocks 1,
  0.0 MiB held (round 1: 297.6 MiB). (A run without the sweep wait showed 16.6 MiB: sweeper lag, gone after 6 gc+50 ms.)
- Windows device names after the reorder (realpath/stat now run for names outside the type table): not raised. Node 22.x
  node_file.cc RealPath and Stat call ToNamespacedPath before uv_fs_* (read on raw.githubusercontent.com, v22.x branch), so
  libuv gets \\?\C:\...\COM3, where Win32 device-name and trailing-dot parsing are off; round 1's residual looks moot on Node 22
  (trailing-dot variants are not served either: "index.html." is its own missing name under \\?\).
- Policy on every answer (probe: GET/HEAD 200, 400, 404, 405, 421 all carry the identical string): safe. CSP binds documents
  and workers only; on js/css/wasm it is ignored except the Worker's script, where it is the point; on text/plain refusals it
  makes the page inert and unframeable; no report-uri/report-to, so no outbound request; Node's own connection-level
  400/408/431 carry no headers and no body. The page boots under it in the browser tests, and the Worker could not compile
  the Wasm without 'wasm-unsafe-eval' in its own response's policy, so that path is proven. Production flavour ships no
  server: the hosted policy is R6/infra's job later.
- three console route (three-picture.ts): three 0.187.0 calls the function with 'log'/'warn'/'error' only (utils.js:213,
  271, 309); reportError -> window error -> note() bounds it. Fallback adapter = UNMASKED_RENDERER_WEBGL: logged to the
  console in every build and in the dev report; nothing leaves the machine (note for telemetry later: a fingerprint datum).
- define: JSON.stringify literals; no import.meta.env read left in packages (grep), 0 VITE_ in the built JS. mode u32 and
  the dropped viewTime: no input from outside the page's own Worker; fine.
- NEW N1 (Low; rust-reviewer F6's ground) build.mjs:40 + :59-62: the refused environment misses RUSTC_WORKSPACE_WRAPPER
  (Cargo Book, environment variables: "for workspace members Cargo will execute this specified wrapper"; sim-wasm is a
  member) and RUSTUP_TOOLCHAIN (rustup book, overrides: ranks above rust-toolchain.toml, so it swaps the pinned 1.99.1);
  cargo config files outside the repo ($CARGO_HOME/config.toml, a parent's .cargo/config.toml: [build] rustflags,
  [profile.release]) are outside any env check. So "the manifests' settings alone" (build.mjs:59) does not hold yet. A
  seatbelt against accidental settings (PATH can swap cargo itself). Fix: add RUSTC_WORKSPACE_WRAPPER; refuse
  RUSTUP_TOOLCHAIN unless it names rust-toolchain.toml's channel (rustup sets it for its own child processes); say in the
  comment that config files are not covered.
- NEW N2 (Low, rest of S6's fix) packages/e2e/tests/skeleton/skeleton.spec.ts:12-31: nothing fails on a
  securitypolicyviolation, so a violation that does not throw (a blocked style or image) passes unseen; fix: an init script
  that records document securitypolicyviolation events, asserted empty beside errors (Worker violations throw and are counted).
- NEW N3 (Low) serve.mjs:47-51: Cross-Origin-Resource-Policy: same-origin not sent (round 1 asked it with S6): another
  site's page can still embed the files no-cors (onload/onerror tells it the server runs); nothing secret is served, and
  Brave blocks sites' localhost sub-requests by default. Fix: add the header to COMMON_HEADERS.
- Carry-over Low (round 1 comment slips): wasm-bindgen-tool.mjs:4 fixed; :6 still cites docs/DECISIONS.md TOOL-SOURCE,
  absent at d257bb6 (git grep 0 for TOOL-SOURCE and for the tool's byte counts in docs/): for the docs pass.
- For memory (not written: the brief allows notes only): Node 22 fs bindings namespace absolute Windows paths in C++
  (\\?\), so DOS device names and trailing dots do not resolve through fs.realpath/stat; pipeline() closes the file on an
  aborted response (old pipe leaked 20 of 20, shown again); env-refusal lists for cargo need RUSTC_WORKSPACE_WRAPPER and a
  RUSTUP_TOOLCHAIN rule.

## 08:19:35 UTC verdict round 2: S1-S7 closed; new N1-N3 Low (follow-ups under the review budget; change-set at the size stop line); carry-over comment slip Low (docs pass). Anti-cheat table: no player action reaches a server in cs2 (local Worker only).
