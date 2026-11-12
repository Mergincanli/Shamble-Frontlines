# test-engineer check of the U4d2 builder brief, rev 1

Brief: .checkpoints/netcode-proof-test/u4d2-brief.md (rev 1, drafted by the rust-reviewer; 719 lines at check start). Read only.
Its notes: u4d2-brief-notes.md (220 lines at start).
Head at check start: 3032b88 (2026-11-12 08:15 UTC). spikes/ since adb1bab: one commit, 28a85e7 (scene.rs +7: Scene::effect_rows, U4e's L3).
Memory: no test-engineer memory directory exists (.claude/ has no agent-memory; checked); nothing to carry in.
Form: as u4e-brief-r3-check.md (plan, progress log, check tables, recheck rows, findings, verdict).

## Plan
1. Read the brief whole; section map (line ranges); first stale spots.
2. Read the plan §5 U4d2 (:666-702), Lanes (:840 on), §4.1, §4.2, §4.15 and each section §5 U4d2 cites; plan.md lines U4d2's rows name.
   Check (1) coverage table: Builds, check.mjs steps, Tests, Bites, Reviewers.
3. Carries: u4s-carries.md "## U4d2 or the main session" (29, 41, 42, 52, 65, 66), entry 34 (and u4a-build.md, u4ab-*.md: was 34 met?),
   "## Any later change to npt-sim's synthetic recordings ..." (67, 71, 72): is U4d2's work such a change? Placeholders vs word-for-word.
4. Rulings: advisor-u4.md conditions 2, 7, 8 (C-U7-1), 10 (b); advisor-u4-wave1.md W1-TARGET :558-587 and every line naming U4d2;
   advisor-u4d-close.md B1.2, B1.3, B2; advisor-u4df.md R1, R3, R4.1, R6; advisor-u4j-r2.md B2.3; advisor-u4e-brief.md F2-ROUTE, L4.
5. Rules every builder brief carries: resume card "Rules in force"; history.md "Decisions taken" (USAGE-SAVINGS (a)); runner u4e-run.sh.
6. Check (4): recheck rows at the head: sim (Replayer, Recording, GENERATOR, synthetic recordings), lab subcommands, proto/Cargo.toml,
   l1 FateLog::for_each_canonical, tcpinfo unsafe pattern, marks.json rows, c-u2-pm-0001-3.mjs, check.mjs and its tests, ADR lines.
7. Check (2) testability of each test/bite/check.mjs step; check (5) the questions for the advisor and section 7.
8. Findings TB-n with severity, brief line, fix; verdict.

## Progress log
- [start] plan written. No cargo or check.mjs runs (W1-TARGET); read-only git only.
