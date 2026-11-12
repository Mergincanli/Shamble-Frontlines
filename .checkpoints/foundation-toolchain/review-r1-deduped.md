# Review round 1 (head 4d78ad6): the 20 findings, deduped by the session (2026-11-12)

Source: `review-r1-findings.json` (the five reviewers' returns, wf_b82244e7-a22). Each line: the deduped id, the
findings it merges, the severity (the highest of them), and the session's first reading. The skeptics (run 2) test
each; their verdicts are input, never a filter.

1. D1 = RR-1 [High]: the release overflow setting does not reach sim's inline and generic code compiled in other crates
   (room, sim-wasm, later server): a debug client and a release server can disagree on overflow. Reproduced by
   rust-reviewer (`rust-reviewer-overflow-xcrate.sh`). Fix proposed: `[profile.release] overflow-checks = true` plus
   `[profile.release.package."*"] overflow-checks = false`; it widens ADR 0002 part 4's mechanism: a ruling is needed.
2. D2 = IE-1 [Medium] + SA-4 [Low]: `toolchain` is not a required check (ruleset 25069670 requires gate and smoke
   only); the report must answer REQUIRED-CHECK, carry the plan's Part 10 risk 2 into Risks, tell him in K1-GO that a
   red `toolchain` blocks no merge and triggers no rollback until 2.3, and make "For Mustafa" item 3 name `toolchain`
   too; REQUIRED-CHECK for 2.3 lists the kit places that change with the aggregate check.
3. D3 = IE-2 + TE-1 + TE-2 [Low]: "green on every push/head" is the wrong quantifier (four pushes had no completed run);
   "about 20 seconds" is the fastest run (19 to 45 s, median 24 s).
4. D4 = SA-1 [Low]: the cut's concrete limit: code inside it keeps the runner's uid and shares the filesystem, so it can
   plant `core.fsmonitor` in .git/config and get code run by checkout's post step outside the cut (with the network,
   the docker group and sudo); the report's Risks and a 2.3 finding.
5. D5 = SA-2 [Low]: setpriv without `--inh-caps=-all` and `--bounding-set=-all`; the cut prints `id` and `ip -o link`
   but asserts none of its properties: a CI edit (asks him): a 2.3 finding.
6. D6 = SA-3 [Low]: SETUP-NODE-CACHE also turns on with `devEngines.packageManager`: archive and report text.
7. D7 = IE-3 [Low]: nothing carries the network cut into 2.3: a 2.3 finding (keep PM-0002-37's proof when
   toolchain.yml goes).
8. D8 = IE-4 [Low]: G0's finding should record that the image's own stable equals the pin today (a version-only check
   would pass on the wrong toolchain) and the active-toolchain check.
9. D9 = IE-5 [Low]: cancel-in-progress also cancels main's runs when two merges land close together: a 2.3 finding.
10. D10 = RR-2 [Low]: ADR 0002 part 7's "every crate opts in with `[lints] workspace = true`" and "pedantic at warn on
    sim, protocol and content only" cannot both be expressed in Cargo: a 2.3 finding (and the ADR's next revision).
11. D11 = RR-3 [Low]: default-features in edition 2024 on 1.99.1, and one command unifying features across members: a
    2.3 finding (part 3's one-feature-set rule).
12. D12 = EA-1 [Low]: the root `[workspace]` now captures every Cargo package below the root that has no workspace of
    its own: record it (REPO_MAP or the report) and for 2.3.
13. D13 = EA-2 [Low]: the crate docs give the wrong time for when code arrives ("Empty until its first slice" where the
    skeleton step brings code) and omit client-endpoint's constraints from part 3: lib.rs docs (rides the D1 push).
14. D14 = EA-3 [Low]: OQ-0002-7's decider (the by-day reading of wasm-bindgen's maintenance issue #4533) leaves the
    lasting record when the plan is deleted: EMPTY-SLOTS' text.
15. D15 = EA-4 [Low]: the report does not say whether 2.2 counts as the test of the crates proposed without research
    (OQ-0002-8 (a)): a report sentence.
16. D16 = EA-5 [Low]: a static rule that `client` never imports `net` or the Wasm glue: a 2.3 finding.
17. D17 = TE-3 [Low]: two checks never seen red in CI (the probe's "still reachable" branch, and one more): the report's
    "not proven" list, or a 2.3 finding.
