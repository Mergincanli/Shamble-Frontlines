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
