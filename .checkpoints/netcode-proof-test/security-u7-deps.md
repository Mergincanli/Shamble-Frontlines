# security-auditor: U7 Block 0 dependency short read (DEP-HOLD)

Reader: security-auditor (Opus 5.5), 2026-11-23. Branch claude/netcode-proof-test-plan-hcky5g.
Inputs: u7-brief.md rev 1.1 section 5 (Block 0) and section 0.1; advisor-u7-brief.md (sha256 65e109aa...044df3, checked)
C6 (:167-173), C7 (:175-181), C10 (:197-220, items (1)-(10)). Earlier reads reused, not redone: security-r1-deps.md,
security-u6c-deps.md.
Rules for this read: no npm install / ci / audit, no cargo fetch / update / build; read-only `cargo tree --locked
--offline` and registry/OSV reads only; no npmrc contents read (`npm config get` only).
Scratch (read-only scripts, none writes a file) in .checkpoints/netcode-proof-test/security-u7/: npm-read.mjs (sha256
90c3a68f...1bdd; a version's registry facts and its tarball hashed in memory), web-lock-check.mjs (sha256
99bbae39...6d21; Block 0a's binding check of the two npm files, with a model mode and six plants).

## Plan
1. Confirm the hash and the five "covered already" cites C10 names.
2. Read the npm rule's sources: advisor-u8-brief.md C2, U8's form in wip/u8-build.patch; .gitignore for node_modules.
3. Registry facts (read only): typescript 6.0.3's publish time, dist.integrity (and an independent sha512 of the
   tarball in memory), install scripts, dependencies; newest 6.0.x; OSV for npm typescript 6.0.3 (npm audit's stand-in).
4. check.mjs's redact step (F26) against a lock of the expected shape.
5. Block 0b: today's manifests and lock; wasm-bindgen 0.2.130's manifest features; who brings wasm-bindgen into the lock
   today and with what features; the four array lines over U6c's modelled lock (count, hash); the feature tree
   expectations of item (9), including the dev edges `-e features` pulls in.
6. Answer items (1)-(10) by number; findings; per-half verdict; Block 0a's commands; dependency-check.md lines; 0b's
   binding check. DONE.

## Notes (one line per finding, saved as I go)
- Step 1: advisor-u7-brief.md sha256 65e109aa7732307310bbdee71053d72a96613ac65e3cc277581b4a6506044df3 = the brief's
  section 0.1. Cites hold as C10 states them: security-r1-deps.md:238 "typescript 6.0.3 exact (U7): CLEAR"; :245-247
  the npm rule (`npm ci --ignore-scripts`, web/.npmrc, committed lock, registry + sha512, `npm audit` recorded);
  :218-220 SEC-R1D-6 (`--before` 7 days back); :232-233 wasm-bindgen =0.2.130 CLEAR WITH CONDITIONS (std only, no
  serde-serialize, no web-sys / wasm-bindgen-futures, js-sys =0.3.107 allowed); :234-237 wasm-bindgen-cli 0.2.130 "(U7
  here, U8 on RS)" by `cargo install --locked`, default features, binary `wasm-bindgen` only; :276-277 its crate
  sha256 c37f6904...fc7a; security-u6c-deps.md:186-190 cargo-deny 0.22.0 and cargo-audit 0.23.0 "YES again".
- Step 2, the npm rule's sources: advisor-u8-brief.md:185-189 (C2): "No `.npmrc` anywhere under spikes/: any `.npmrc` is
  a gate file (advisor.md:43) and guard-protected (routine-guard.mjs:38)", `--ignore-scripts` on every npm command with
  `npm_config_ignore_scripts=true` in its environment, "U7's brief takes the same rule for `web/`". U8's form
  (wip/u8-build.patch:889-890, :1151-1152, :3398-3424): `npm_config_ignore_scripts=true npm ci --ignore-scripts`; its
  test asserts no tracked `.npmrc` (git ls-files) and none on disk under SPIKE = spikes/netcode-proof (:3689-3690),
  skipping node_modules, target and .gate. The night guard also refuses `npm ci` when an `.npmrc` sits in the project,
  tools/ or $HOME (routine-guard.mjs:556-558); npm reads npm-shrinkwrap.json before package-lock.json (:82, :563).
- Step 2, git: root .gitignore:1 `node_modules/` and :3 `dist/` ignore web/node_modules and web/dist; a
  web/package-lock.json or web/.npmrc is not ignored (`git check-ignore -v`), so either would show in `git status`.
- Step 2, this container's npm (npm's own reads, no file contents): node v22.23.4 (/opt/node22/bin/node), npm 10.9.9;
  `npm config get registry` = https://registry.npmjs.org/; ignore-scripts=false (default, so the flag and the env var
  are both needed), before=null, omit= (empty: devDependencies install), userconfig /root/.npmrc and globalconfig
  /opt/node22/etc/npmrc both absent, no builtin /opt/node22/lib/node_modules/npm/npmrc (existence checked by ls only).
- Step 3, registry (npm-read.mjs, packument read 2026-11-23T03:11:26Z, through the container's proxy): typescript 6.0.3
  published 2026-04-16T23:38:27.905Z, before 2026-11-16T00:00:00Z: yes (7 months older than the cutoff). Last of
  6.0.x (6.0.2 2026-03-23, then 6.0.3; no 6.0.4). Newer stable: 7.0.2 (2026-07-08, dist-tag latest). Not deprecated.
  Manifest: license Apache-2.0, bin {tsc: bin/tsc, tsserver: bin/tsserver}, engines node >=14.17, no dependencies,
  optional or peer dependencies, scripts are dev-only (test, build, lint ...; no preinstall / install / postinstall /
  prepare), no hasInstallScript, no gypfile. Publisher typescript-bot. dist.tarball
  https://registry.npmjs.org/typescript/-/typescript-6.0.3.tgz; dist.integrity
  sha512-y2TvuxSZPDyQakkFRPZHKFm+KKVqIisdg9/CZwm9ftvKXLP8NRWj38/ODjNbr43SsoXqNuAisEf1GdCxqWcdBw==; shasum
  90251dc007916e972786cb94d74d15b185577d21; fileCount 140; unpackedSize 24,346,827; one registry signature, keyid
  SHA256:DhQ8wR5APBvFHLF/+Tc+AYvPOdTpcIDqOhxsBHRwC7U; no provenance attestation.
- Step 3, the tarball read in memory (4,515,854 B): its sha512 and sha1 EQUAL dist.integrity and dist.shasum; 140 tar
  files (= fileCount); no binding.gyp (so npm adds no implicit `node-gyp rebuild`), no .npmrc; its package.json 6.0.3
  with no install-time lifecycle script and no dependencies; top level bin/, lib/, package.json, README.md,
  SECURITY.md, LICENSE.txt, ThirdPartyNoticeText.txt. Confidence: high (registry metadata plus an independent hash).
- Step 3, OSV (2026-11-23T03:12Z): POST api.osv.dev/v1/query {npm, typescript, 6.0.3} -> `{}`; the same without a
  version (every version of the package) -> `{}`. No advisory has ever been recorded for npm typescript.
- Step 3, npm audit's source: npm 10's docs (docs.npmjs.com/cli/v10/commands/npm-audit): npm will "POST it to the default
  configured registry at the path `/-/npm/v1/security/advisories/bulk`" (Quick Audit as fallback); the page names no
  database. GitHub, 2021-10-07, "GitHub Advisory Database now powers npm audit" (github.blog): a proxy over GHSA that
  speaks the npm audit protocol. OSV imports GHSA (and OpenSSF Malicious Packages) per my earlier reads; osv.dev's
  data page lists both sources but does not tie them to npm by name. Confidence: medium-high that OSV's npm answer
  covers what `npm audit` would report for one package; GHSA's npm malware advisories may reach OSV only through
  OpenSSF's MAL- set (medium). For typescript 6.0.3 the tarball read above (no install script, no dependency) closes
  that gap.
- Step 4, the redact step: check.mjs:2643-2648 runs redact-check.test.mjs, then `node ops/operator/redact-check.mjs
  <spike>`; F26's fix is in this tree (redact-check.mjs:137-144 NPM_TARBALL, :237-241 the skip; integrity blanked
  :158-166). Baseline today: `redact-check: 340 item(s) checked, 0 finding(s)`. The walk skips .git, node_modules,
  target and .gate (:277), so web/node_modules is never scanned; web/dist is, and a .wasm there is "binary file:
  cannot be checked", a finding (:273-275, :298-301): N-U7D-1 below.
- Step 4, the modelled npm files: npm 10's root-entry shape read from U8's lock (ops/browser/package-lock.json:1-13:
  name, version only when package.json has one, the dependency key; no `private`) and an entry's keys from
  tools/package-lock.json:1478-1490 (version, resolved, integrity, license, flags, bin, engines). web-lock-check.mjs
  --model: 13 PASS, scanText 0 findings on both texts. Plants 1-6 (hasInstallScript; integrity changed; a scripts key;
  a second package; another registry host; `^6.0.3`) each FAIL exactly their check (2d, 2f, 1a, 2b, 2e, 1d), exit 1.
- Step 5, today's files (sha256): Cargo.lock c55bab3c...6b4d, Cargo.toml 5ebfbedf...d36a, deny.toml 90404ee6...9ae1
  (= U6a's Block 0 after-state, as security-u6c-deps.md:24-25 found; U6b's and U6c's Block 0 not yet run here);
  crates/client-wasm/Cargo.toml c68877f3...ab20: [dependencies] `npt-client.workspace = true` (:27) and the comment line
  `# wasm-bindgen joins at U7, at the version the wasm-bindgen CLI is pinned to.` (:28); no [dev-dependencies]; lints
  allow unsafe_code (:15-16). npt-room, npt-server and npt-sim are already [workspace.dependencies] path entries
  (Cargo.toml:36-39), so C6's `.workspace = true` lines need no workspace edit.
- Step 5, the lock today: wasm-bindgen 0.2.130 (Cargo.lock:1879-1888: cfg-if, rustversion, wasm-bindgen-macro,
  wasm-bindgen-shared), -macro, -macro-support, -shared 0.2.130, js-sys 0.3.107 (:663-671) are locked, one version
  each; no web-sys, wasm-bindgen-futures. npt-client-wasm (:797-801) has the one array line ` "npt-client",`.
- Step 5, wasm-bindgen 0.2.130's published manifest (~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/
  wasm-bindgen-0.2.130/Cargo.toml:42-56): `default = ["std"]`, `std = []`; serde and serde_json optional, only through
  `serde-serialize` (:48-52, :66-72); build-dependency rustversion (as rustversion-compat, :82-84). So default features
  turn on no optional dependency and add no lock line. Its build.rs (49 lines) only prints rerun-if-changed, check-cfg
  and cfg lines from CARGO_CFG_TARGET_* and the compiler version: no network, no file write.
- Step 5, who reaches wasm-bindgen today (`cargo tree --workspace --all-features --target all -e features -i
  wasm-bindgen`, read-only): only wasm32-only edges of the QUIC/TLS stack: getrandom 0.2.17 (ring's
  wasm32_unknown_unknown_js), getrandom 0.4.3 (wasm_js, quinn-proto), js-sys 0.3.107, web-time 1.1.0; feature `std`
  alone (from js-sys's std). js-sys's default features include `unsafe-eval` there (not ours: no edge from
  npt-client-wasm's shipped graph; see item (9)). npt-server's graph on wasm32 sets wasm-bindgen's `std` only.
- Step 5, the edges: the cargo book's cargo-tree page: "`features` ... If this is the only kind given, then it will
  automatically include the other dependency kinds" and "The default is `normal,build,dev`". So C10's item (9) first
  command (`-e features` alone) includes C6's dev edges: on wasm32 it will list getrandom, js-sys and web-time as
  dependents through npt-server, with wasm-bindgen's features still exactly `default` and `std`.
- Step 5, wasm32 builds never compile the dev edges: clippy-wasm32 is `cargo clippy --locked --target
  wasm32-unknown-unknown -p <LEAF..., npt-tcpinfo>` without --all-targets (check.mjs:1970-1971, :2014-2019, :2619);
  the wasm step builds npt-hash-wasm only (:2213, :2221); build.mjs builds `-p npt-client-wasm` cdylib. Today's
  `cargo tree -p npt-client-wasm -e normal --target wasm32-unknown-unknown`: npt-client, npt-link, npt-proto, npt-sim,
  libm, rand_chacha, ppv-lite86, zerocopy, rand_core; no serde, js-sys, web-sys.
