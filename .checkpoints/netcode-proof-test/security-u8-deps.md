Task: security-auditor DEP-HOLD binding check of U8's browser package (PR #40, lane
claude/netcode-proof-test-plan-hcky5g), 2026-11-22. Earlier read: security-r1-deps.md B5 (:143-180), verdict
(:241-247), U8 binding check (:256-257). Read only: the held lock, registry metadata, OSV. No installs, no builds,
no npm audit. (No Edit tool in this agent: the notes are rewritten whole with Write at each block.)

Files held: .gate/u8-deps/package.json sha256 8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90,
.gate/u8-deps/package-lock.json sha256 a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524
(both re-hashed by me: match).

## Plan (blocks)
- C0: the lock's facts (lockfileVersion, count, resolved/integrity, install scripts, os/cpu/optional entries).
- C1: binding check: resolve webdriverio 10.2.0 from registry packuments with the cut-off 2026-11-15 (npm --before
  semantics), compare with the lock package by package; explain 167 vs B5's 162.
- C2: OSV querybatch on every (name, version) in the lock; identify the 4 moderates; rule on each.
- C3: C2's substitute for the npm rule (ignore-scripts): confirm or amend.
- C4: verdict and dependency-check.md lines; DONE.

## Notes

### C0 the lock's facts DONE
- Re-hashed both held files: equal to the brief's sha256s. package.json: one dependency, `"webdriverio": "10.2.0"`
  (exact), private, no scripts, no overrides.
- Lock (node script over package-lock.json, read only): lockfileVersion 3; 167 entries under `packages` (root
  excluded) = 160 distinct name@version + 7 duplicate placements (string-width 7.2.0 x2, decamelize 6.0.1 x2,
  undici 8.12.4 x3, readable-stream 2.3.8 x2, safe-buffer 5.1.2 x2, string_decoder 1.1.1 x2: 1+1+2+1+1+1 = 7).
  0 optional, 0 peer, 0 dev, 0 link entries. Every `resolved` on https://registry.npmjs.org/, every integrity sha512.
  hasInstallScript only on node_modules/edgedriver and node_modules/geckodriver.
- Registry cross-check, all 167 entries (scratchpad scripts/lock_vs_registry.mjs: packument per name): integrity
  equal to the registry's dist.integrity, resolved equal to dist.tarball, dependencies / optional / peer maps equal
  to the published manifest, deprecated flag equal, every version published before 2026-11-15T00:00Z, every
  dependency edge resolves (node_modules lookup) to a version satisfying its range: 0 problems.
- Install scripts per registry (preinstall/install/postinstall or gypfile): only edgedriver 8.0.0 and geckodriver
  8.0.0 (postinstall `node ./dist/install.js`, the driver download). None elsewhere: CONFIRMED.
- Deprecated: whatwg-encoding 3.1.1 ("Use @exodus/bytes instead"), informational. Bins: @puppeteer/browsers,
  crc-32, edgedriver, geckodriver, jiti, safe-regex2, wait-port.
- Youngest versions (all <= cut-off): electron-to-chromium 1.5.486 (2026-11-14 23:04Z, @wdio/utils dep, a data
  table), bare-buffer 3.7.2 and bare-url 2.5.7 (11-13), ws 8.23.0, @zip.js/zip.js 2.25.0, glob 14.0.6, bare-path
  3.1.3 (11-12), brace-expansion 5.0.13, minimatch 11.0.8 (11-11), @puppeteer/browsers 3.2.7 (11-10).
- glob/minimatch in the lock: glob 14.0.6 (@wdio/config ^14.0.2) -> minimatch 11.0.8; readdir-glob 3.0.0 ->
  minimatch 10.2.6 (nested). So B5's minimatch 10.2.6 is still there, via archiver 8.0.0 -> readdir-glob 3.0.0.

### C1 binding check DONE
- Method: scratchpad scripts/npm_tree_before.mjs, npm 10.9.9's own npm-pick-manifest with `before:
  2026-11-15T00:00Z` and Node 22's version, per (name, range), walking deps + non-optional peers + optional deps,
  all platforms. It models npm's version pick, not arborist's dedupe.
- Result: 162 distinct name@version (the same count as B5). Set difference against the lock's 160:
  in resolver not lock = @types/node 26.6.6 and undici-types 8.9.0; in lock not resolver = none.
  Why: @types/split2 4.2.3 and @types/ws 8.18.2 depend on `@types/node: "*"`; arborist reuses the already placed
  @types/node 22.20.5 (required `^22.20.4` by webdriverio, webdriver, @wdio/types, @wdio/repl), which satisfies `*`,
  so 26.6.6 and its undici-types ~8.9.0 never enter the tree. My resolver picks the newest per range, so it counts
  them. So 162 = the lock's 160 + those 2; 167 = 160 + the 7 duplicate placements. Package by package the lock is
  webdriverio 10.2.0's tree as of the cut-off: BINDING CHECK HOLDS.
- Same resolver with no cut-off (today, B5's view): also 162, differing from the cut-off tree in six versions the
  7-day rule holds back: bare-buffer 3.7.2 (today 3.7.3), bare-fs 4.8.4 (4.8.5), bare-url 2.5.7 (2.5.8),
  electron-to-chromium 1.5.486 (1.5.492), minimatch 11.0.8 (11.0.9), undici 8.12.4 (8.13.1). OSV lists no advisory
  on any of the six held-back versions (C2), so no "security fix excepted" case under SEC-R1D-6 applies.

### C2 the four moderates DONE
- OSV querybatch over the lock's 160 distinct name@version (scripts/osv_lock.py --npm, 2026-11-22): 1 hit,
  minimatch 10.2.6 GHSA-g3hv-75wg-m249 (CVE-2026-118764). OSV entry: GitHub-reviewed 2026-10-30, severity MODERATE,
  CVSS 4.0 AV:N/AC:L/AT:N/PR:N/UI:N/VC:N/VI:N/VA:L, CWE-405; affected >=7.0.0 <11.0.1; globparts amplification in
  firstPhasePreProcess() on an attacker-controlled pattern (optimizationLevel >= 2). Confidence high.
- npm's own advisory source, read as registry metadata (POST registry.npmjs.org/-/npm/v1/security/advisories/bulk
  with the lock's names and versions; not `npm audit`): exactly one advisory, the same GHSA on minimatch,
  vulnerable_versions ">=7.0.0 <11.0.1", severity moderate.
- So "4 moderate severity vulnerabilities" = 1 advisory + 3 dependents npm audit counts "via" it: readdir-glob 3.0.0
  (needs minimatch ^10.2.2: every 10.x is affected), archiver 8.0.0 (needs readdir-glob ^3.0.0: 3.0.0 is the only
  3.x), webdriverio 10.2.0 (needs archiver ^8.0.0: 8.0.0 is the only 8.x). 1 + 3 = 4. Confidence high on the
  advisory set (two sources agree); medium-high that npm's count is exactly this chain (arborist's via counting
  inferred from the ranges, not seen because npm audit was not run).
- Reach (published tarballs, read as data): webdriverio 10.2.0 imports archiver once (build/node.js:16909,
  `import { ZipArchive } from "archiver"`) for installExtension's zipDirectory (:16941-16965), which walks the
  directory itself with fs.readdirSync and calls `archive.append(...)` per file (:16971-16994). readdir-glob is
  called only by archiver's `directory()` (lib/core.js:632) and `glob()` (lib/core.js:672-697), which webdriverio
  never calls. So minimatch 10.2.6 is loaded at import but never matches a pattern in our use; and the patterns it
  would take are local paths, never network input.
- Fix available but not required: readdir-glob 4.0.0 (2026-11-09, inside the cut-off) differs from 3.0.0 only in
  package.json (minimatch ^11.0.1, engines node >=20; diff -r of the two tarballs), so `"overrides":
  {"readdir-glob": "4.0.0"}` would drop minimatch 10.2.6 and resolve to the tree's minimatch 11.0.8.
- Rulings: GHSA-g3hv-75wg-m249 on minimatch 10.2.6: ACCEPTED (it is SEC-R1D-5, already accepted; unreached, as
  above; Low here). readdir-glob 3.0.0, archiver 8.0.0, webdriverio 10.2.0: not separate advisories, the same one
  counted via the chain: ACCEPTED on the same reason. No override required; refuse none. Re-check if any spec calls
  archiver directly or WebdriverIO's directory/glob upload path appears in a later version.

### C3 C2's substitute for the npm rule DONE
- npm docs (docs.npmjs.com/cli/v10/using-npm/config, read 2026-11-22): ignore-scripts "If true, npm does not run
  scripts specified in package.json files"; "Any environment variables that start with `npm_config_` will be
  interpreted as a configuration parameter"; sources "sorted by priority": command line, then environment, then
  npmrc files. So `--ignore-scripts` on the command line wins over any user or global npmrc on the laptop, and the
  environment variable covers a command that forgets the flag. A project `.npmrc` adds nothing those two do not.
- CONFIRMED, with three amendments: (a) the only install command is `npm ci --ignore-scripts` (never `npm install`,
  `npm update`, `npm install <pkg>`), so the committed lock is binding and a mismatch with package.json fails;
  (b) no `npx` / `npm exec` / `npm create` anywhere in R1.md, PROMPT-R1.md or section 6.2 (they can fetch and run a
  package outside the lock); the specs run as `node <script>.mjs`; (c) the "npm audit output recorded" part of my
  rule is met for this lock by C2 (one advisory, accepted); if U8 runs `npm audit` on the laptop, the expected
  output is exactly those 4 moderate lines, anything else returns here. Skipping edgedriver's and geckodriver's
  postinstall is consistent with SEC-R1D-3 (drivers installed by hand, hashes recorded, given by
  `wdio:*Options.binary`).

### C4 verdict DONE
CLEARED WITH CONDITIONS. The two files are cleared as they are (no change to either). Conditions: C3's (a)-(c);
SEC-R1D-2 and SEC-R1D-3 in the spec config before any laptop run (unchanged from security-r1-deps.md:241-244);
specs as plain Node scripts on standalone webdriverio, no @wdio/cli testrunner. dependency-check.md lines:
8370495dee2ed77e6a937c6f40c72a5485015428d57f90505f4e440504293c90  spikes/netcode-proof/ops/browser/package.json
a0efce8e511a8865d50008172b3f88e9cc072efbe92a85a035340562b7c49524  spikes/netcode-proof/ops/browser/package-lock.json

DONE
