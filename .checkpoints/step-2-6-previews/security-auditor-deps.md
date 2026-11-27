# security-auditor: dependency check (M3, DEP-HOLD) on hosting/package.json + hosting/package-lock.json
Started 2026-11-25. Scope: only these two files (local commit 1f4fd0d, not pushed).

## Plan (blocks)
D1 sha256 of both files + read both whole.
D2 criteria by script over the lockfile (python3 -I).
D3 fresh folder .gate/step26-deps/ with only the two files: npm ci --ignore-scripts --no-audit --no-fund, npm audit,
   npm audit signatures.
D4 publish dates of every package version (npm view <name> time --json) vs the 7-day rule (cutoff 2026-11-18).
D5 the three install-script packages: what their scripts do; does skipping them break `pages deploy` of a static folder.
D6 other risks for a token-holding job (run-time fetches, typosquats, maintainers).
D7 verdict.

## D1
- sha256sum (repo root, 2026-11-25): package.json b47551b125f78a667a031dd0718a025981c1395c036be3c330b5239aef7b9726,
  package-lock.json df24ffc392062d041d4be8f9ac5b51be7a6696e0c5a8cbfc3bac0dc07a4924fc: both match the brief.
- package.json (10 lines): private true (:4), license UNLICENSED, one dependency wrangler "4.164.0" exact (:8); no
  scripts, overrides, workspaces, bin, engines. Root package.json workspaces ["packages/*"] (:9-11): hosting/ outside.
- package-lock.json read whole (1463 lines): v3 (:4), root (:7-14) = manifest; 91 packages, all flat under
  node_modules/ (no nesting, no link/bundled/dev entries). Notable: miniflare 5.20261117.0-beta (:1227-1251) and youch
  4.1.0-beta.10 (:1439-1451) are pre-release versions, pinned exactly by wrangler (:1393) and miniflare (:1238): they
  are what Wrangler 4.164.0 itself ships with, not a choice of ours. Optional peers (kerberos, workers-types,
  bufferutil, utf-8-validate) not in the tree.

## D2 (script .gate/step26-deps/scripts/check.py, python3 -I, 2026-11-25)
- Checks: manifest private, deps == {wrangler}, exact x.y.z, none of scripts/overrides/workspaces/dev/optional/peer/
  bundled/resolutions/bin/publishConfig/packageManager/config; lock v3, no v1 section, name/version match, root deps ==
  manifest, root keys only name/version/license/dependencies; wrangler at pin; every entry: flat path, no
  link/inBundle/dev/extraneous/deprecated, only known keys, `resolved` EXACTLY
  https://registry.npmjs.org/<name>/-/<basename>-<version>.tgz (catches a lookalike tarball path or host), integrity
  sha512 decoding to 64 bytes, single hash; every dependency edge present.
- Result: 91 entries, FAILS none. hasInstallScript: esbuild@0.28.2, fsevents@2.3.3, workerd@1.20261117.1.

## D3 (fresh folder .gate/step26-deps/install/ holding only the two files; node 22.23.4, npm 10.9.9; 2026-11-25 ~20:23 UTC)
- sha256 of the copies = the brief's values (before and after npm ci: npm ci left both files unchanged). No .npmrc in
  the repo; `npm config get registry` = https://registry.npmjs.org/.
- `npm ci --ignore-scripts --no-audit --no-fund`: "added 40 packages in 3s", exit 0. `wrangler --version`: 4.164.0.
- `npm audit`: "found 0 vulnerabilities", exit 0.
- `npm audit signatures`: "audited 39 packages", "39 packages have verified registry signatures", "24 packages have
  verified attestations"; --json: invalid [], missing [].
- 40 installed vs 39 audited: the one not audited is @img/sharp-wasm32 0.35.5 (arborist replay,
  scripts/edges.cjs): installed with no incoming edge (its two parents, sharp-freebsd-wasm32 and
  sharp-webcontainers-wasm32, are os/cpu-filtered out), and npm's verify-signatures walks edges, so it skips it. Its
  tarball was still checked against the lockfile's sha512 by npm ci. Registry signature checked by hand in D4.

## D4 (script .gate/step26-deps/scripts/registry.mjs; registry.npmjs.org packuments, 2026-11-25 ~20:25 UTC; output .gate/step26-deps/registry.jsonl)
- For all 91 entries: lock integrity == registry dist.integrity (91/91); registry ECDSA signature over
  `name@version:integrity` verified against https://registry.npmjs.org/-/npm/v1/keys (91/91; negative test: the same
  signature over "wrangler@4.164.1:..." -> false, so the check bites). This covers @img/sharp-wasm32 too.
- Publish dates: oldest blake3-wasm 2.1.5 (2020-11-29); newest wrangler 4.164.0 2026-11-17T11:15:02Z (8.4 days at
  check time). Then miniflare 5.20261117.0-beta 2026-11-17T11:03Z, workerd + 5 platform packages 2026-11-17T00:58-01:02Z,
  @jridgewell/resolve-uri 3.1.3 2026-11-13T04:00Z. 7-day cutoff 2026-11-18T20:25Z: all 91 pass. No deprecated version.
- wrangler@4.164.0's registry manifest: dependencies equal the lockfile's (:1387-1408) exactly; no install/prepare
  script (only dev scripts). npm `latest` is now 4.168.0 (not needed: audit clean; a bump is a new DEP-HOLD).
- Publishers: Cloudflare packages by GitHub Actions with attestations (maintainer wrangler-publisher et al.); esbuild
  by GitHub Actions (maintainer esbuild); sharp/@img by GitHub Actions (lovell); youch/poppinss virk; no unexpected
  maintainer, no lookalike name (all names are the well-known packages Wrangler itself pins).
- Recent non-provenance release checked: @jridgewell/resolve-uri 3.1.3 (2026-11-13, first release since 3.1.2 of
  2024-02-14, publisher jridgewell, no attestation). Diffed against 3.1.2 (npm pack into .gate/step26-deps/cmp/old):
  only the URL regex (adds bracketed IPv6 hosts), a comment, a README typo and the version; no new require, no I/O.
  Benign.

## D5 (the three install scripts; read from the installed copies)
- esbuild 0.28.2 `postinstall: node install.js` (11,773 bytes): find @esbuild/<platform>; if missing, run
  `npm install <pkg>@0.28.2` outside the lockfile, else download https://registry.npmjs.org/<pkg>/-/...tgz directly
  (no lockfile integrity check); then hard-link the native binary over bin/esbuild (speed only); then check the
  binary's version. workerd 1.20261117.1 `postinstall: node install.js` (9,321 bytes): the same pattern for
  @cloudflare/workerd-<platform>. fsevents 2.3.3 `install: node-gyp rebuild`: macOS only (os darwin), not installed
  on Linux at all (absent from node_modules).
- Skipping them: the platform packages arrive through optionalDependencies under the lockfile's sha512, so the
  scripts' only remaining effect is the bin hard-link. Proven locally (scripts off): `esbuild --version` 0.28.2,
  `require('esbuild').transformSync` works, `workerd --version` "workerd 2026-11-17". Turning scripts on would only
  add the out-of-lockfile fallback download paths: keeping them off is strictly safer.
- Wrangler's bundle requires esbuild, miniflare, workerd and blake3-wasm at load (wrangler-dist/cli.js:18,19,29,31).
- Own probe (scripts off; HOME/XDG_CONFIG_HOME in .gate/step26-deps/probe/home; dummy 40-char token "PPP..."; account
  0...0; neutral project; static site = index.html + _headers; cwd an empty folder; strace -f execve,connect):
  reached /accounts/0.../pages/projects/neutral-probe-project -> "Authentication error [code: 10000]", then /accounts
  "Invalid access token [code: 9109]", exit 1. exec'd: node, then @esbuild/linux-x64/bin/esbuild --service=0.28.2
  --ping (esbuild's service starts and answers with scripts off); workerd never exec'd. connects: only the local
  proxy. The empty folder stayed empty. Wrangler's log (110 lines) holds no token, "authorization" or "bearer".
  (A first probe with a hyphenated dummy stopped earlier at format check 6111: same load path.)
- Runner platform ubuntu-24.04 x64 (texts/preview.yml:36 etc.) = linux-x64 glibc, the same platform package as here.
- Proven locally: install, load, platform binaries, argument parsing, the API call path, no token in the log. Only M5
  proves: the real upload (upload-token exchange, blake3 asset hashing, file upload, deployment record, _headers
  applied) with a real token; none of those touch the skipped scripts (JS + Wasm, no esbuild or workerd for a folder
  without functions/ or _worker.js), so the risk that M5 fails *because* scripts are off is low (inference).

## D6 (other risks for the token-holding job)
- Run-time fetches: Wrangler's update check reached the registry ("update available 4.168.0"): data only, no code
  fetched or run. Telemetry to sparrow.cloudflare.com (cli.js:164858) is off by texts/preview.yml:29
  (WRANGLER_SEND_METRICS "false"): outside these two files; re-checked in round 1.
- Pre-release versions (miniflare 5 beta, youch 4.1.0-beta.10, unenv 2.0.0-rc.24) are Wrangler's own exact pins.
- npm installs some extras on glibc (linuxmusl, sharp-wasm32, @emnapi/runtime, tslib): integrity-checked, unused.
- No finding against the two files.

## Corrections and additions (after the advisor agent's check of my record)
- D1 tree = commit: `git diff --stat 1f4fd0d -- hosting/...` empty (working tree = 1f4fd0d630e8...); `git diff --stat
  HEAD 1f4fd0d -- hosting/...` empty at HEAD 098ed460 (later commits are checkpoints/docs only). Only untracked path
  at the time: docs/runbooks/ (not mine, not in scope).
- D3 exit codes: the first "exit 0" was tail's status. Re-run from a cleared node_modules with captured statuses:
  `npm ci --ignore-scripts --no-audit --no-fund` exit 0, "added 40 packages in 3s"; `npm audit` exit 0, "found 0
  vulnerabilities"; `npm audit signatures` exit 0, "audited 39 packages", "39 packages have verified registry
  signatures", "24 packages have verified attestations" (outputs: .gate/step26-deps/ci2.txt, audit2.txt, sig2.txt).
  sha256 of the copies unchanged after.
- Attestations: verified by npm for the installed (audited) packages only; my registry.mjs checked signatures and
  integrity for all 91, not attestations.
- Install-script list cross-checked two ways: (a) the registry manifests (registry.jsonl) carry
  preinstall/install/postinstall only on esbuild 0.28.2 (postinstall), fsevents 2.3.3 (install), workerd
  1.20261117.1 (postinstall); `prepare`/`benchmark:install` entries (path-to-regexp, cookie, @sindresorhus/is, undici,
  @jridgewell/*) do not run on a registry install; (b) `find node_modules -name binding.gyp`: only
  sharp/src/binding.gyp, not at the package root and sharp has no gypfile flag or install script, so no implicit
  node-gyp build. The list = the lockfile's hasInstallScript flags.
- Builder's error 9106 not reproduced as such: my dummies gave 6003/6111 (hyphenated) and 10000 + 9109 (40-char), and
  a fourth run hit 10429 (rate limited; probing stopped). Same conclusion: the API call path is reached.
- Hosts: probe 3 (strace write/sendto -s 300, grep CONNECT): only `CONNECT api.cloudflare.com:443` (x2); probe 4 with
  a fresh HOME: api.cloudflare.com x1. Wrangler still printed "update available 4.168.0": the update lookup's host was
  not observed in probes 3-4 (likely a cache outside HOME, e.g. the OS temp folder: inferred, not proven); probe 2's
  second connect is the likely registry lookup. A version lookup only; no code fetched (no execve beyond node and
  the esbuild binary).
- Stray write: the D3 `wrangler --version` ran with the real HOME and may have written under ~/.config/.wrangler
  (Wrangler's log folder). Left in place, as told (nothing outside .gate/step26-deps removed).

## D7 verdict
APPROVE hosting/package.json sha256 b47551b125f78a667a031dd0718a025981c1395c036be3c330b5239aef7b9726 and
hosting/package-lock.json sha256 df24ffc392062d041d4be8f9ac5b51be7a6696e0c5a8cbfc3bac0dc07a4924fc (re-hashed just
before this verdict). No finding against the two files.
hasInstallScript: esbuild 0.28.2, fsevents 2.3.3 (darwin only, not installed on Linux), workerd 1.20261117.1.
Observations for round 1 (preview.yml / the script), not findings and given no severity:
- O1 Wrangler's update check runs inside the token step (a registry version lookup, no code); WRANGLER_SEND_METRICS
  "false" is already at texts/preview.yml:29.
- O2 `npm audit signatures` skips the orphan @img/sharp-wasm32 (no incoming edge); covered here by my own signature
  check; a future bump's DEP-HOLD should re-check it the same way.
- O3 Any bump of wrangler = a new DEP-HOLD and a tier-2 PR (plan :127); 4.168.0 is current latest.
Not verified: the real upload with a real token (M5); attestations for non-installed platform packages; the update
lookup's host; behaviour on runners other than ubuntu-24.04 x64.
