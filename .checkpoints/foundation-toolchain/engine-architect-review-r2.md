# engine-architect: review round 2 of foundation step 2.2 (PR #48)

Scope: only what changed since round 1's head 4d78ad6, up to 4bccf55 (`git diff 4d78ad6..4bccf55 -- . ':(exclude).checkpoints'`).
Worktree HEAD at start: 2b28c3d (saver); 0 files outside .checkpoints differ from 4bccf55 (checked: git diff --name-only, HEAD and working tree).
Memory: none on disk (memory: local; no agent-memory folder anywhere); round-1 notes read (engine-architect-review-r1.md).

## Plan
1. Read the full diff (11 files), commit by commit: 1de98a9 (doc comments + report), 4bd9dd5 (Cargo.toml), 297d48a/8d622ac (M7 + exact revert), 8a4f9b0, 4bccf55 (docs).
2. Read ADR 0002 part 3 (crates and packages), part 4 (:247-278), PM-0002-53 (:1319), OQ-0002-7/-8, migration plan 2.2-2.5; ADR 0004 migration step 2; the plan Part 3 (plan.md:265-276).
3. Read the advisor ruling D1 (advisor-ruling-d1.md), conditions 1-7, esp. 7e; the docs/adr/README.md scope lines and its tables (where Q-A1 and Q2.2-a sit).
4. Focus 1: the register row OVERFLOW-ALL: form (agent ruling, not Mustafa's decision; his go with PR #48's merge), what Cargo.toml sets and how far it reaches, where PM-0002-53's counter test must run, the ADR owner rewords part 4 and PM-0002-53 at the next revision; placement vs README scope lines.
5. Focus 2: the report's decision entry OVERFLOW-ALL (options, the "no" branch, recommendation) vs the row and Cargo.toml:21-29. Mechanical check of the reach in a scratch copy (cargo metadata / profile resolution) if useful.
6. Focus 3: the four crate doc comments vs ADR 0002 part 3, the migration plan and plan Part 3.
7. Focus 4: EMPTY-SLOTS, OQ-0002-8-TEST, CLIENT-NET-RULE in the report and STATUS_ARCHIVE vs ADR 0002 and the D14, D15, D16 skeptics + review-r1-verdicts.json.
8. Focus 5: anything else in the diff that crosses a module boundary or misstates the ADRs (REPO_MAP, ROADMAP, STATUS lines).
9. Read security-auditor-review-r2.md only where it bears on the row (no security re-review).
10. One line per finding as found; advisor consultation before finalizing; verdict.

## Done checks

## Findings
- C1 diff read (11 files): Cargo.toml (route A, :21-29), four lib.rs doc comments, adr/README.md:234 (row), REPO_MAP:16, ROADMAP:116-126, STATUS (rows #48, Waiting, follow-up names), STATUS_ARCHIVE (From foundation 2.2), report (Outcome, Plan and approval, As built, E1, M5, M7, E12, Reviews, Risks, findings for 2.3, follow-ups, OVERFLOW-ALL entry, For Mustafa).
- C2 ADR 0002 read: part 3 (:182-238), part 4 (:247-278), part 5 (:296-345), part 7 (:418-432), migration plan (:1431-1500), OQ-0002-3/-7/-8 (:1022-1103), PM-0002-15..19, -40..-49, -53 (:1319), -63, -64; Appendix A R1-RR-OQ4 (:1562), R5-RUSTLINT-OQ4 (:1619). ADR 0004 migration plan phase 2 (:1251). Line refs the docs cite re-checked: :1082-1084, :1095-1102, :207-209, :1319 all match.
- C3 advisor ruling D1 read (conditions 1-7, C1-C10, the C5 correction: dependency GENERIC code instantiated in a workspace crate untested; wording "dependencies keep the release default setting").
- C4 skeptics D14, D15, D16 read; report and archive texts of EMPTY-SLOTS, OQ-0002-8-TEST and CLIENT-NET-RULE follow their suggested texts.
- C5 README scope: "Decisions the drafts ask for" (:41-45) excludes questions decided "by an agent"; Q2.2-a there only on Mustafa's named words; the loose-ends register (:209-210) "each loose end, with where it is resolved"; Q-A1 (:233) is the precedent for an agent reading in the table "The critic's remaining items and the answered escalations". Row OVERFLOW-ALL (:234) placed beside it, per the ruling's condition 3.
- C6 RR-1's own words (review-r1-findings.json [2].findings[0]): "sim's small, #[inline] and generic functions are generated in the crate that calls them"; no "most". Its fix already said PM-0002-53's test "lives in a package that depends on sim and runs in release, native and wasm32" (origin of the row's phrase).
- C7 ADR 0002 header :5: "Author: engine-architect (the "Pass marks" table: test-engineer, plan step 4)". Appendix A :1586 "R5-RUSTLINT-REC4 | adopted | part 4"; REC4's reason (r5:1356-1358): "a per-package override is allowed for this setting (F13, inferred), so the rest of the workspace keeps the release default" - route A reverses both its scope and that reason.
- C8 security-auditor r2 read: SA2-1 (Medium, "likely ... inferred" vs ADR 0001 part 11's three outcomes), SA2-2 (Medium, no record hands route A's panic side, part 4's explicit-operations rule's reach, to the owners; "dropping *" reach), SA2-3 (Low). Not re-reviewed; SA2-2 bears on the row's hand-off list.
- Draft EA2-1 (Medium): report :295 "misses most of the simulation's code": unsourced quantity; RR-1, the row (:234), Cargo.toml:22 and As built say "inline, generic and small functions". Same paragraph: "for the simulation by naming two crates" (protocol is not the simulation).
- Draft EA2-2 (Low): row :234 hand-off list omits Appendix A's R5-RUSTLINT-REC4 disposition (:1586 adopted -> changed); "in release, native and wasm32" reads as replacing PM-0002-53's four builds (:1319); the archive's OVERFLOW-XCRATE-CI keeps the four builds explicit.
- Noted, not a finding: the row's dependency anchor names sim's tree only; part 4 names sim and protocol. Part 3 (:188) gives protocol "sim and content types only" (no third-party crate of its own, unlike sim's "(and its maths dependencies)"), so sim's tree covers protocol's today; for my revision of part 4.
