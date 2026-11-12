# security-auditor, review round 1, foundation 2.2 (PR #48), head 4d78ad61e652f2e35ddf51c49557e725420248c8

Base: origin/main 2aa5e9c. Diff: `git diff origin/main...4d78ad6 -- . ':!.checkpoints'` (42 files).
Memory: no local agent memory found for security-auditor in this container (none under .claude/ or ~/.claude/).

## Plan
1. Read the report draft (done), the diff, `.github/workflows/toolchain.yml` line by line.
2. The cut: sudo -n, unshare --net, loopback, setpriv (--reuid/--regid/--clear-groups/--no-new-privs), env -i, the
   probe and its control. Reason from util-linux setpriv(1), unshare(1), capabilities(7), user_namespaces(7),
   network_namespaces(7), prctl no_new_privs docs; test in a scratch copy under .gate/ if the container allows.
   Question: are all capability sets (effective, permitted, inheritable, ambient, bounding) empty after setpriv?
   Does the report claim more than the cut gives?
3. CI hygiene: permissions, SHA pins (and that each SHA is the tag it names), persist-credentials, secrets,
   pull_request_target, concurrency, triggers, what a fork PR can do, cache poisoning (setup-node cache).
4. SETUP-NODE-CACHE as it stands.
5. npm dependency review: package-lock.json (64 registry packages, sha512, 5 links, install scripts), npm ci flags.
6. Cargo: Cargo.lock, rust-toolchain.toml, workspace lints, no third-party crates.
7. Kit lines K2a, K2b, K3, K4 against Appendix A words (A2) and DECISIONS.md 2026-11-10.
8. DECISIONS.md new entries and the register row Q2.2-a: words byte-exact vs Appendix A; status right.
9. CI logs for the head's runs (curl, no token).

## Findings (one line each, as found)
- [checked] CI logs, head 4d78ad6: toolchain push 43312017234 (job 131510223737), pull_request 43312027194 (job 131510257155), both success; inside the cut `uid=1001(runner) gid=65534(nogroup) groups=65534(nogroup)`, only `lo`, `curl: (7)`; GITHUB_TOKEN "Contents: read, Metadata: read"; "Cache mode: write" on the PR run; setup-node prints `package-manager-cache: true` and no cache restore/save line. Logs saved under .gate/secaud-r1/logs/.
- [checked] setpriv, local test as root, util-linux 2.39.3 (Ubuntu 24.04's version): `setpriv --reuid=1001 --regid=nogroup --clear-groups --no-new-privs` gives CapInh=CapPrm=CapEff=CapAmb=0, CapBnd unchanged (full), NoNewPrivs=1, Uid r/e/s/fs 1001, Gid all 65534, Groups empty. Confirms "capability sets empty" for the four held sets; bounding set full but inert under no_new_privs (commoncap: NNP intersects new permitted with old = 0).
- [checked] his words: Appendix A1-A12 equal the session transcript byte for byte (by message uuid; A9, A11 are queued_command attachments); DECISIONS.md quotes A1-A6 and A8-A13 whole, A7 inside the BRANCH-47 entry (its tail, byte-exact), the Queued entry's two bullets a contiguous slice of A9 (script .gate/secaud-r1/words_check.py, words_check2.py). No misquote found.
- [checked] K2a, K2b, K3, K4, K1b as written equal the diffs shown to him at 2026-11-12T15:46:13Z (transcript message ac0da789...), the message his A10 "exactly as you showed them" answers (.gate/secaud-r1/kshown.py). The same message told him "The second one governs ... the first under a header marked 'Replaced'"; A10 did not object, so A8's "Replaced" status rests on a told-and-unopposed reading, not on inference alone.
- [checked] register row Q2.2-a: matches the advisor's rev1 ruling ("(a) is final ... needs his go"); "His go for this row: PR #48's merge" matches ADR-INDEX option (a); not recorded as his go already given.

## Resumed after the container restart (session 2)
- [checked] npm audit on a copy of the head's package.json + package-lock.json (sha256 97ff5fc4... equal to `git show 4d78ad6:package-lock.json`): 0 vulnerabilities at every level (.gate/secaud-r1/npm-audit.json; total 74 deps counted by audit).
- [checked] action pins: `git ls-remote https://github.com/actions/checkout` gives refs/tags/v7.0.1 = 3d3c42e5aac5ba805825da76410c181273ba90b1 (lightweight, also v7); actions/setup-node refs/tags/v7.0.0 = 820762786026740c76f36085b0efc47a31fe5020 (a v7.1.0 = 949feb24 exists). Both pins are the official repos' tag commits; the run logs download exactly these SHAs. (api.github.com refused by the proxy for other repos; ls-remote used.)
- [checked] SETUP-NODE-CACHE: setup-node action.yml at 8207627 (sha256 5d765941..., fetched from raw.githubusercontent at the SHA): `package-manager-cache` default true, "caching is enabled when either devEngines.packageManager or the top-level packageManager field in package.json specifies npm". Root package.json (4d78ad6) has neither field: auto cache off today; the log shows no restore line and an empty post step. The recorded finding names only `packageManager`; `devEngines.packageManager` is a second trigger (note for the finding's text, Low). Runner header "Cache mode: write" on the PR run = the job's runtime token can write caches in its ref's scope.
- [checked] SETUP-NODE-CACHE, what it would cache (setup-node src at 8207627: main.ts:24-27, :71-88, :150-172; cache-utils.ts:23-26; cache-restore.ts:35-61; cache-save.ts:35-74): auto cache when `devEngines.packageManager` (object or array, name ~ /^(\^)?npm(@.*)?$/) or top-level `packageManager` names npm; path = `npm config get cache` (~/.npm, whole folder incl. _cacache, _npx, _logs); key node-cache-linux-x64-npm-<hashFiles(package-lock.json)>, exact-key restore only (no restore-keys for npm); saved by the post step (post-if success) after the cut, only on a key miss. Code inside the cut can write ~/.npm before that save; npm ci re-verifies sha512 from the lockfile, the key is exact, and branch scoping keeps a feature branch's or a PR's entry away from main: Low. Fix at 2.3 (CI edit, asks him): `package-manager-cache: false`, and the finding's text should name devEngines too.
- [checked] npm lock review (.gate/secaud-r1/scripts/lockcheck.py on `git show 4d78ad6:package-lock.json`): lockfileVersion 3; 64 registry entries, all https://registry.npmjs.org/ with sha512, no entry without integrity, no git/tarball/URL source; 5 workspace links + 5 packages/* entries + root (75 keys; the report's "69 entries" counts links + registry only, fine); every entry dev; one hasInstallScript: fsevents 2.3.3 (optional, darwin); 27 os/cpu-restricted optional natives (15 @rolldown/binding-*, 12 lightningcss-*), the linux-x64-gnu ones run inside the cut as native code; chains coherent (vitest 5.1.0 -> obug 3.0.0, why-is-node-running, vite 8.3.7 -> rolldown 1.2.18, lightningcss 1.33.0 (MPL-2.0), postcss). E11 confirmed. npm audit 0 (above).
