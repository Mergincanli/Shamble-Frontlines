Task: advisor-cs1-security (advisor ruling during the build of cs1 "Types": the security answer under condition 3, and the CI process miss P1). Started 2026-11-18 12:28 UTC. Seat: day-session advisor agent (SHAMBLE_ROLE unset; not the PR Advisor routine). I edit nothing but this file.

## Plan (blocks, in order)
B1. Read the security answer (security-auditor-deps.md B0-B14, B1b, B1c), the checkpoint (cs1-types.md, "A process miss"), plan.md (condition 3), my plan notes (advisor-plan.md, M3). DONE at start.
B2. Read the rules: CLAUDE.md (Never list, severity scale, review budget), main's .claude/agents/advisor.md (git show origin/main:), .claude/scripts/checkpoint.mjs ~90-190, .github/workflows/toolchain.yml + gate.yml, docs/DECISIONS.md 2026-11-14 13:58:05 "How to work".
B3. Verify the record myself: git log/show of 566d1d1, da22f96 (messages, files), the lock diff (nine entries), checksums vs ~/.cargo cache (spot), CI run 168 facts via gh (jobs, steps, times), artifacts count; checkpoint.mjs skip-ci logic at the cited lines.
B4. Spot-check the security evidence: two or more per-crate claims (build.rs of proc-macro2/thiserror lines; ts-rs Config::new reads no env; export_to_string signature; no remove_* in ts-rs), cargo tree reach (offline, builds nothing).
B5. Read the build so far: client-endpoint {lib,layout,layout_ts}.rs, protocol {lib,messages,ts_export}.rs, packages/types files; L3 scratch-folder code; deviations 1-3; size count.
B6. Rule on points 1-5; draft the decision entry; write the answer.

## Notes
