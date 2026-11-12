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
