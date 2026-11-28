# security-auditor: U7 Block 0 dependency short read (DEP-HOLD)

Reader: security-auditor (Opus 5.5), 2026-11-23. Branch claude/netcode-proof-test-plan-hcky5g.
Inputs: u7-brief.md rev 1.1 section 5 (Block 0) and section 0.1; advisor-u7-brief.md (sha256 65e109aa...044df3, checked)
C6 (:167-173), C7 (:175-181), C10 (:197-220, items (1)-(10)). Earlier reads reused, not redone: security-r1-deps.md,
security-u6c-deps.md.
Rules for this read: no npm install / ci / audit, no cargo fetch / update / build; read-only `cargo tree --locked
--offline` and registry/OSV reads only; no npmrc contents read (`npm config get` only).
Scratch (read-only scripts, none writes a file) in .checkpoints/netcode-proof-test/security-u7/: npm-read.mjs (sha256
90c3a68f...1bdd; a version's registry facts and its tarball hashed in memory), web-lock-check.mjs (sha256
99bbae398d5de67ef15e8cf259aab40f523a6b06c67ee07592b6639d99786d21; Block 0a's binding check of the two npm files, with a
model mode and six plants), lockmodel-u7.mjs (sha256 dadfdf1f...2f3e; the U6b, U6c and U7 lock models).

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
  <spike>`; `node check.mjs --only redact` runs that step alone (check.mjs:3, :2666-2673). F26's fix is in this tree
  (redact-check.mjs:137-144 NPM_TARBALL, :237-241 the skip; integrity blanked :158-166). Baseline today:
  `redact-check: 340 item(s) checked, 0 finding(s)`. The walk skips .git, node_modules, target and .gate (:277), so
  web/node_modules is never scanned; web/dist is, and a .wasm there is "binary file: cannot be checked", a finding
  (:273-275, :298-301): N-U7D-1 below.
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
  dependents through npt-server, with wasm-bindgen's features still exactly `default` and `std` (SEC-U7D-2).
- Step 5, wasm32 builds never compile the dev edges: clippy-wasm32 is `cargo clippy --locked --target
  wasm32-unknown-unknown -p <LEAF..., npt-tcpinfo>` without --all-targets (check.mjs:1970-1971, :2014-2019, :2619);
  the wasm step builds npt-hash-wasm only (:2213, :2221); build.mjs builds `-p npt-client-wasm` cdylib. Today's
  `cargo tree -p npt-client-wasm -e normal --target wasm32-unknown-unknown`: npt-client, npt-link, npt-proto, npt-sim,
  libm, rand_chacha, ppv-lite86, zerocopy, rand_core; no serde, js-sys, web-sys. check.mjs counts dev edges as not
  shipped (:78 `isShipped`: kind null or build), so C6's "the dev edge ships nothing" holds for the deps step.
- Step 5, the lock model (lockmodel-u7.mjs): the U6b model reproduces 210bf2df...a219 (235 packages) and the U6c model
  ea6efb003a59f06946c5ba48f3ba73ec1c92d79757e07f6a7ab5b79610a79111 (252 packages, 2,477 lines), both equal to the
  earlier reads. U7 over U6c: npt-client-wasm's array becomes npt-client, npt-room, npt-server, npt-sim, wasm-bindgen
  (each name one version, so no "name version" form); 4 lines added, 0 removed; 252 packages; 2,481 lines; sha256
  8a2ebb34b1c10bc8daaaa604c18f3be6e707aecd5d21c275e7daf86196f2d9b6; wasm-bindgen's own block unchanged.
- Step 5, OSV querybatch 2026-11-23T03:18:54Z (security-u6c/osv.mjs) over wasm-bindgen, -macro, -macro-support,
  -shared 0.2.130, js-sys 0.3.107, bumpalo 3.20.3, cfg-if 1.0.5, rustversion 1.0.23, unicode-ident 1.0.27, quote
  1.0.47, proc-macro2 1.0.107, syn 3.0.9, wasm-bindgen-cli 0.2.130: 0 hits. Sparse index: 0.2.130 is the newest
  stable of wasm-bindgen, -macro, -shared and -cli, none yanked; checksums equal Cargo.lock:1882, :1894, :1917 and,
  for the CLI, security-r1-deps.md:277 (c37f6904...fc7a). Rust-version 1.81 (crates), 1.86 (CLI); workspace 1.99.1.
- Step 5, tools now: `cargo deny --version` cargo-deny 0.22.0; `cargo audit --version` cargo-audit-audit 0.23.0. No
  wasm-bindgen binary in ~/.cargo/bin: the CLI is Block 0b's install (brief section 5: `--root` under .gate/).

## Answers to C10's items
**(1) web/package.json exactly: CLEAR.** Keys exactly `name`, `"private": true`, `"type": "module"`,
`"devDependencies": {"typescript": "6.0.3"}`; no `version`, `scripts`, `overrides`, `dependencies`,
`optionalDependencies`, `peerDependencies`, `bundleDependencies`, `workspaces` or any other key. The name is the
builder's (a plain lower-case npm name, not `typescript`; `npt-web` as U8's `npt-browser`). `private` blocks a publish;
no `scripts` means `ignore-scripts` has nothing of ours to skip either. Bound by web-lock-check.mjs 1a-1d.

**(2) web/package-lock.json as made: CLEAR.** lockfileVersion 3, `requires: true`, name as package.json; `packages`
exactly `""` (name and devDependencies typescript 6.0.3 only) and `node_modules/typescript` with version 6.0.3,
resolved https://registry.npmjs.org/typescript/-/typescript-6.0.3.tgz, integrity
sha512-y2TvuxSZPDyQakkFRPZHKFm+KKVqIisdg9/CZwm9ftvKXLP8NRWj38/ODjNbr43SsoXqNuAisEf1GdCxqWcdBw==, `dev: true`, and
otherwise only license, bin and engines (no hasInstallScript, no dependencies). The integrity is the registry's own
dist.integrity, which I re-derived from the tarball's bytes (step 3). Bound by web-lock-check.mjs 2a-2f, the integrity
pinned to this value: a lock whose integrity differs (a re-publish, a mirror, an interception) fails 2f.

**(3) How it is made: CLEAR WITH CONDITIONS.** C10's two commands, run with npm's `--prefix` on web/ (absolute path, so
no `cd`) and with `--no-audit --no-fund` added to both (SEC-U7D-3): `npm install` and `npm ci` otherwise send the
package list to the registry's audit endpoint after installing (npm's `audit` defaults to true), which is the refused
`npm audit` by another door; the repository's own unattended form is `npm ci --prefix tools --ignore-scripts
--no-audit --no-fund` (routine-guard.test.mjs:214). `--before=2026-11-16` for a run on 2026-11-23 (the run day in UTC
less 7 days); typescript 6.0.3 (2026-04-16) passes any such date, so `--before` changes nothing here and stays as
SEC-R1D-6's habit. `npm config get registry` must read https://registry.npmjs.org/ (it does today; no user, global or
builtin npmrc exists in this container); npm 10.9.9 and node v22.23.4 recorded. If TLS fails through the container's
proxy: NODE_EXTRA_CA_CERTS=/root/.ccr/ca-bundle.crt in that command's environment (/root/.ccr/README.md:27-28); never
`strict-ssl` off, never an npmrc or `cafile` written anywhere.

**(4) The `.npmrc` substitute for web/: CONFIRMED, now.** advisor-u8-brief.md C2 applies to web/ as written: no
`.npmrc` anywhere under spikes/ (none tracked, none on disk; an `.npmrc` is a gate file, advisor.md:43, and
guard-protected, routine-guard.mjs:38), and every npm command that touches web/ carries `--ignore-scripts` with
`npm_config_ignore_scripts=true` in its environment: Block 0a's two, C7's `web` step, rs.yaml's (wip/u8-build.patch:889).
Added for web/: no `web/npm-shrinkwrap.json` (npm reads it before the lock). Why it is enough: an `.npmrc` would also
cover a command typed without the flag, but web/'s only package has no install script, no binding.gyp and no
dependency (step 3) and package.json has no `scripts`, so a bare command has nothing to run today; a dependency with an
install script needs a new read anyway. U8's test 4 (wip/u8-build.patch:3398-3407) walks all of spikes/netcode-proof,
so it covers web/ once U8 merges; until then web-lock-check.mjs 4a-4b check it at Block 0a. Stop B confirms U8's and
RS's as C10 says.

**(5) `npm audit`'s output: substitute ruled.** `npm audit` is not run in this session (a permission was refused; not
retried, and not reached through `npm install`'s own audit: item 3). Block 0a records instead: the UTC time and the
answer of one OSV query for every package the lock holds, i.e. `typescript` 6.0.3 on ecosystem npm, expected `{}` (today
`{}`, and `{}` for every version of the package); the note "npm audit not run (permission refused); npm audit's bulk
endpoint serves the GitHub Advisory Database, which OSV imports; typescript 6.0.3's tarball read in
security-u7-deps.md (no install script, no dependency, registry-signed)". Any OSV entry returns here before `npm ci`. A
later session or RS that can run `npm audit` in web/ records it then; anything but "found 0 vulnerabilities" returns
here (SEC-U7D-1).

**(6) The redact step on the new lock: CLEAR.** F26's fix is in this tree; the modelled package.json and lock scan
clean through redact-check's own `scanText` (web-lock-check.mjs check 6), and the spike scans clean today (340 items, 0
findings). Block 0a runs `node check.mjs --only redact` after the lock exists: PASS, 0 findings, items = the count
before plus 2 (web/node_modules is skipped by name, redact-check.mjs:277). N-U7D-1 is the later trap (web/dist).

**(7) Block 0b's manifest lines: CLEAR WITH CONDITIONS (exactly these).** (a) Cargo.toml [workspace.dependencies]: one
entry `wasm-bindgen = "=0.2.130"` (string form; comments free), no `features`, `default-features`, `optional` or
`package` key, so the default feature `std` and nothing else. (b) crates/client-wasm/Cargo.toml: line 28's comment
replaced by `wasm-bindgen.workspace = true` (no `features` key added at the member, no target-specific table), and a
new `[dev-dependencies]` with exactly `npt-room.workspace = true`, `npt-server.workspace = true` and
`npt-sim.workspace = true` (C6: three; section 5's two-and-three-line wording is superseded, as section 0.1 says
conditions rule). Nothing else in either file: [lints], [lib] and crate-type unchanged; no [patch], [profile],
cargo-features or rustflags key; no other manifest touched.

**(8) The lock diff: CLEAR WITH CONDITIONS.** Against U6c's merged lock: `git diff --numstat -- Cargo.lock` prints `4	0`;
the four lines are ` "npt-room",`, ` "npt-server",`, ` "npt-sim",`, ` "wasm-bindgen",` after ` "npt-client",` in
npt-client-wasm's array and nowhere else; no [[package]], version, source or checksum line; the package count equals
U6c's merged lock (252 by the model). If U6c's merged lock is the model (ea6efb00...9111), the after-lock is sha256
8a2ebb34b1c10bc8daaaa604c18f3be6e707aecd5d21c275e7daf86196f2d9b6; if U6c's real lock differs, only that after-hash is
void and the four-line rule binds. No network is needed: every package is already locked and cached, so the refresh runs
`--offline` (stronger than U6c's online refresh: no newer patch can enter).

**(9) The feature tree: CLEAR WITH CONDITIONS (one command added).** C10's command, `cargo tree --locked --offline -p
npt-client-wasm -e features --target wasm32-unknown-unknown -i wasm-bindgen`, includes dev edges (the cargo book), so it
will show wasm-bindgen's features exactly `default` and `std` but also getrandom 0.2.17, getrandom 0.4.3, js-sys
0.3.107 and web-time 1.1.0 as dependents through npt-server's dev edge (today's npt-server graph on wasm32 sets `std`
only). Added as the binding view of the shipped crate (SEC-U7D-2): the same with `-e normal,features`, whose only
package dependent is npt-client-wasm, features `default` and `std`. C10's `-e normal` check stands: no line names
js-sys, web-sys, wasm-bindgen-futures, serde or serde_json (nor getrandom or tokio). serde-serialize,
enable-interning, spans, strict-macro, gg-alloc, msrv and rustversion stay off workspace-wide.

**(10) deny, audit, check, counts: YES, as U6c's binding check step 6 words them.** cargo-deny 0.22.0 and cargo-audit
0.23.0 as installed (printed today); `cargo deny --locked check advisories bans sources` exit 0 with the warnings U6c's
merged run recorded and no new one (0b adds no package, so no new multiple-versions name); `cargo audit --file
Cargo.lock` the same crate count as U6c's run, 0 vulnerabilities, the same allowed warnings (RUSTSEC-2026-0441
pin-utils) and no new one; both database dates recorded; `cargo check --workspace --all-targets --locked` 0 warnings
(native: npt-client-wasm now compiles wasm-bindgen 0.2.130, its macro and build script on the build host: cleared by
security-r1-deps.md:232-233, build.rs read above); check.mjs's test and deps counts equal before and after 0b. The CLI:
`cargo install wasm-bindgen-cli --version 0.2.130 --locked` with default features, CARGO_TARGET_DIR and `--root` under
.gate/ (security-r1-deps.md:234-237); recommended `--bin wasm-bindgen` (only the binary used is installed);
`<root>/bin/wasm-bindgen --version` prints 0.2.130; its .crate in ~/.cargo/registry/cache has sha256 c37f6904...fc7a.

## Findings
- **SEC-U7D-1 (Low; closed by item (5)'s ruling, residual accepted):** `npm audit` cannot run in this session, so
  Block 0a has no npm-audit output. Exploit sketch: an npm-only malware advisory on the one package that OSV lacks goes
  unseen. Evidence: the refused permission (task); npm 10 docs (bulk endpoint); GitHub 2021-10-07 (GHSA powers npm
  audit). Fix: OSV query recorded (`{}`), plus the tarball read (no install script, no dependency, integrity re-derived,
  publisher typescript-bot, one registry signature); `npm audit` recorded wherever it can run later.
- **SEC-U7D-2 (Low; closed by item (9)'s added command):** C10's `-e features` view includes dev edges, so js-sys,
  getrandom and web-time appear beside the shipped crate. Exploit sketch: a real shipped edge to js-sys (with its
  default `unsafe-eval` feature) added later hides among the expected dev-edge lines. Evidence: the cargo book
  (cargo-tree, `-e features`); today's `cargo tree -p npt-server -e features --target wasm32-unknown-unknown -i
  wasm-bindgen`. Fix: `-e normal,features` as the binding view; `-e normal` names no js-sys.
- **SEC-U7D-3 (Low; closed by item (3)'s condition):** C10's commands without `--no-audit` make npm send the
  dependency list to the audit endpoint during install, the refused `npm audit` by another door. Exploit sketch: none
  against the game; a permission refusal routed around. Evidence: npm's `audit` config defaults to true; the repo's
  NIGHT_NPM form (routine-guard.test.mjs:214). Fix: `--no-audit --no-fund` on both commands.
- **N-U7D-1 (Low; a build note for the T-builder and R4, not a security finding: the scanner fails closed):** once
  build.mjs writes web/dist/, the redact step reads npt_client_wasm_bg.wasm as "binary file: cannot be checked" and
  check.mjs fails (redact-check.mjs:273-277, :298-301: dist is not a skipped folder). Fix: check.mjs runs only where
  web/dist is absent, or build.mjs takes an output folder and the T5 and check runs build under .gate/ (as C13 does for
  dist-bites); never add `dist` to SKIP_DIRS.
No Critical, High or Medium.

## Verdicts
- **Block 0a: CLEAR WITH CONDITIONS.** Conditions: (a) the commands below, in order, exactly (the `--before` date moves
  with the run day); (b) web-lock-check.mjs all PASS and the OSV answer `{}` before `npm ci`; any FAIL, any OSV entry,
  any extra package or key, a registry other than https://registry.npmjs.org/, or `npm ci` adding other than 1 package
  stops the save and returns here; (c) the `.npmrc` substitute of item (4); (d) never `strict-ssl` off or a written npm
  config; (e) dependency-check.md as below.
- **Block 0b: CLEAR WITH CONDITIONS.** Conditions: the manifest lines of item (7) exactly; the binding check below
  passes as written; only a difference from (7)-(10) comes back here (C10).

## Block 0a: the commands (the main session, repository root as working folder; 12 commands)
Before them, the main session writes spikes/netcode-proof/web/package.json with the Write tool, exactly:
`{"name": "npt-web", "private": true, "type": "module", "devDependencies": {"typescript": "6.0.3"}}` (two-space JSON,
one key per line, final newline; the name may differ, item (1)).
1. `node --version` (expect v22.23.4)
2. `npm --version` (expect 10.9.9)
3. `npm config get registry ignore-scripts omit before --prefix /home/user/Shamble-Frontlines/spikes/netcode-proof/web`
   (expect registry=https://registry.npmjs.org/, omit= with no `dev`)
4. `npm_config_ignore_scripts=true npm install --package-lock-only --ignore-scripts --no-audit --no-fund
   --before=2026-11-16 --prefix /home/user/Shamble-Frontlines/spikes/netcode-proof/web`
5. `node /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/security-u7/web-lock-check.mjs
   /home/user/Shamble-Frontlines/spikes/netcode-proof` (expect 13 PASS, "all PASS", exit 0)
6. `date -u +%Y-%m-%dT%H:%M:%SZ`
7. `curl -sS -H "Content-Type: application/json" --data
   '{"package":{"ecosystem":"npm","name":"typescript"},"version":"6.0.3"}' https://api.osv.dev/v1/query` (expect `{}`)
8. `npm_config_ignore_scripts=true npm ci --ignore-scripts --no-audit --no-fund --prefix
   /home/user/Shamble-Frontlines/spikes/netcode-proof/web` (expect "added 1 package")
9. `node -p "require('/home/user/Shamble-Frontlines/spikes/netcode-proof/web/node_modules/typescript/package.json').version"`
   (expect 6.0.3)
10. `node /home/user/Shamble-Frontlines/spikes/netcode-proof/check.mjs --only redact` (expect PASS, 0 findings)
11. `git status --porcelain --untracked-files=all -- spikes/netcode-proof/web` (expect exactly the two `??` lines for
    web/package.json and web/package-lock.json)
12. `sha256sum spikes/netcode-proof/web/package.json spikes/netcode-proof/web/package-lock.json
    .checkpoints/netcode-proof-test/security-u7-deps.md`

## Block 0a: what the builder records in dependency-check.md (a new section, U5's shape)
- `#` header lines: "DEP-HOLD release, U7's Block 0a (web/'s npm files)"; "security-auditor's answer, read in full
  (ends DONE): .checkpoints/netcode-proof-test/security-u7-deps.md" with its sha256 (command 12); the date and time
  written; the conditions in one line (item (3)'s commands with `--no-audit --no-fund`, item (4)'s substitute, item
  (5)'s OSV in place of npm audit, any difference back to security).
- `#` record lines: node and npm versions (1, 2); command 3's four values; command 4 as run, with its `--before` date;
  web-lock-check.mjs's sha256 99bbae39...6d21 and its last line (5); the OSV time, request and answer (6, 7) with the
  note "npm audit not run (permission refused); OSV, GHSA-backed, substitutes (security-u7-deps.md item (5))"; npm ci's
  summary line (8); typescript's installed version (9); the redact step's PASS line with its item count (10); the two
  status lines (11).
- Then, without `#`: `sha256  spikes/netcode-proof/web/package.json` and `sha256  spikes/netcode-proof/web/package-lock.json`
  (command 12). Then the save.

## Block 0b: the binding check (the main session, the hour U6c merges, in spikes/netcode-proof; every cargo call through
the U7 runner with CARGO_NET_OFFLINE=true)
0. Before: `sha256sum Cargo.lock` equals U6c's merged lock as U6c's dependency-check.md section records it (the model:
   ea6efb00...9111); `grep -c '^\[\[package\]\]' Cargo.lock` equals that section's count (252 by the model); `grep -c
   -E '^name = "(web-sys|wasm-bindgen-futures)"' Cargo.lock` = 0; `grep -c '^name = "wasm-bindgen"$' Cargo.lock` = 1;
   npt-client-wasm's block holds the one line ` "npt-client",`; crates/client-wasm/Cargo.toml sha256 c68877f3...ab20
   unless U6c changed it (then its diff comes here first).
1. The edits of item (7). `git diff --stat` touches only Cargo.toml and crates/client-wasm/Cargo.toml (Cargo.lock
   after step 2); `git diff -- Cargo.toml crates/client-wasm/Cargo.toml` shows only item (7)'s lines (and comments).
2. The lock refresh, offline, the one call without `--locked`: `cargo metadata --format-version 1 --offline` (output
   to the runner's log). If it needs the network, stop and come here; never `cargo update` or `generate-lockfile`.
3. The lock diff: `git diff --numstat -- Cargo.lock` prints `4	0`; `git diff -U0 -- Cargo.lock` adds exactly the four
   lines of item (8), in npt-client-wasm's block; `grep -c '^\[\[package\]\]' Cargo.lock` unchanged from step 0;
   `sha256sum Cargo.lock` = 8a2ebb34b1c10bc8daaaa604c18f3be6e707aecd5d21c275e7daf86196f2d9b6 when step 0 read
   ea6efb00...9111 (else only this line is void).
4. The tree (`--locked --offline` on every call):
   - `cargo tree -p npt-client-wasm -e features --target wasm32-unknown-unknown -i wasm-bindgen`: wasm-bindgen's feature
     lines exactly `default` and `std` (dev-edge dependents getrandom, js-sys, web-time expected: SEC-U7D-2);
   - `cargo tree -p npt-client-wasm -e normal,features --target wasm32-unknown-unknown -i wasm-bindgen`: one package
     dependent, npt-client-wasm; features `default` and `std` only;
   - `cargo tree -p npt-client-wasm -e normal --target wasm32-unknown-unknown --prefix none`: today's nine names plus
     wasm-bindgen, cfg-if, wasm-bindgen-macro, wasm-bindgen-macro-support, wasm-bindgen-shared, bumpalo, proc-macro2,
     quote, syn 3.0.9, unicode-ident; no line names js-sys, web-sys, wasm-bindgen-futures, serde, serde_json, getrandom
     or tokio;
   - `cargo tree -p npt-client-wasm -e dev --depth 1` lists npt-room, npt-server, npt-sim;
   - `cargo tree --workspace --all-features --target all -e features -i wasm-bindgen --depth 1`: wasm-bindgen features
     `default` and `std` only.
5. `cargo check --workspace --all-targets --locked`, 0 warnings; `node check.mjs --only test,deps` counts equal before
   and after 0b (run once before step 1, once now).
6. `cargo deny --locked check advisories bans sources` and `cargo audit --file Cargo.lock` as item (10); database dates
   by `git log -1 --format=%cI` run inside ~/.cargo/advisory-db and inside cargo-deny's folder under
   ~/.cargo/advisory-dbs/ (U6c's step 6 words them with `git -C`; this repo's hook refuses `-C`).
7. The CLI as item (10), its version and .crate sha256 recorded.
8. dependency-check.md: a new section in U5's shape naming this file and item (7)-(10)'s conditions, then `sha256  path`
   for spikes/netcode-proof/Cargo.toml, spikes/netcode-proof/Cargo.lock and
   spikes/netcode-proof/crates/client-wasm/Cargo.toml. Then the save.

## Not verified
- The real npm run (the model and the plants stand in; web-lock-check.mjs decides at command 5).
- That OSV holds every advisory `npm audit` would show (medium-high), and npm's registry signature (not checked: `npm
  audit signatures` is the refused command; the integrity is re-derived from the bytes instead). Both my read and Block
  0a's install pass the container's TLS proxy; RS's `npm ci` fetches without it and fails closed on any integrity
  difference from the lock.
- U6b's and U6c's real locks (the U7 after-hash binds only on the modelled before-lock); `cargo metadata --offline`
  writing exactly the four lines (step 3 decides).
- The `-e normal` list's exact proc-macro names (predicted from the locked arrays; the binding part is what it must not
  name).

## Memory candidates (not written: this task allows only this file and scratch)
- U7 Block 0 read (2026-11-23): typescript 6.0.3 integrity sha512-y2Tv...dBw== re-derived from the tarball; npm audit
  substituted by OSV when refused, installs with `--no-audit --no-fund`; `.npmrc` substitute = U8 C2 plus no
  npm-shrinkwrap.json; `cargo tree -e features` alone includes dev edges (use `-e normal,features` for shipped views).

DONE
