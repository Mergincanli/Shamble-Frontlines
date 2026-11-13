# engine-architect: review round 2 of foundation step 2.2 (PR #48)

Scope: only what changed since round 1's head 4d78ad6, up to 4bccf55 (`git diff 4d78ad6..4bccf55 -- . ':(exclude).checkpoints'`).
Worktree HEAD at start 2b28c3d, at the end b958f37 (saver commits); both differ from 4bccf55 only under .checkpoints/ (git diff --name-only: 0 files; working tree clean, `git status --short` empty).
Memory: none on disk (memory: local; no agent-memory folder anywhere); round-1 notes read (engine-architect-review-r1.md).

## Plan
1. Read the full diff (11 files), commit by commit: 1de98a9 (doc comments + report), 4bd9dd5 (Cargo.toml), 297d48a/8d622ac (M7 + exact revert), 8a4f9b0, 4bccf55 (docs).
2. Read ADR 0002 part 3, part 4 (:247-278), PM-0002-53 (:1319), OQ-0002-7/-8, migration plan 2.2-2.5; ADR 0004 migration phase 2; the plan Part 3 (plan.md:265-276).
3. Read the advisor ruling D1 (conditions 1-7, esp. 7e); the README's scope lines and where Q-A1 and Q2.2-a sit.
4. Focus 1: the row OVERFLOW-ALL (form, setting and reach, PM-0002-53's test, the owner's rewording; placement).
5. Focus 2: the decision entry vs the row and Cargo.toml:21-29; my own flag check on a scratch copy.
6. Focus 3: the four crate doc comments vs ADR 0002 part 3, the migration plan and plan Part 3.
7. Focus 4: EMPTY-SLOTS, OQ-0002-8-TEST, CLIENT-NET-RULE vs ADR 0002 and the D14-D16 skeptics and verdicts.
8. Focus 5: anything else that crosses a module boundary or misstates the ADRs.
9. Security's round-2 notes only where they bear on the row; the other round-2 notes for overlap.
10. Advisor consultations; verdict.

## Done checks
- C1 diff read (11 files): Cargo.toml (route A, :21-29), four lib.rs doc comments, adr/README.md:234 (row), REPO_MAP:16, ROADMAP:116-126, STATUS (row #48, Waiting, follow-up names), STATUS_ARCHIVE (From foundation 2.2), report (Outcome, Plan and approval, As built, E1, M5, M7, E12, Reviews, Risks, findings for 2.3, follow-ups, OVERFLOW-ALL entry, For Mustafa).
- C2 ADR 0002 read: header (:1-7), part 3 (:182-238), part 4 (:247-278), part 5 (:296-345), part 7 (:418-432), migration plan (:1431-1500), OQ-0002-3/-7/-8 (:1022-1103), PM-0002-15..19, -40..-49, -53 (:1319), -63, -64; Appendix A R1-RR-OQ4 (:1562), R5-RUSTLINT-REC4 (:1586), R5-RUSTLINT-OQ4 (:1619). ADR 0004 migration plan phase 2 (:1252). Citations re-checked: :1082-1084, :1095-1102, :207-209, :202-205, :1319 all match.
- C3 advisor ruling D1 read (conditions 1-7, C1-C10, the C5 correction: dependency generic code untested; wording "dependencies keep the release default setting").
- C4 skeptics D13-D16 and their entries in review-r1-verdicts.json (reason, fix_ok) read; the report's and archive's EMPTY-SLOTS, OQ-0002-8-TEST and CLIENT-NET-RULE follow the D14, D15 and D16 suggested texts and fix_ok points.
- C5 README scope: "Decisions the drafts ask for" (:41-45) excludes questions decided "by an agent"; Q2.2-a sits there only on Mustafa's named words (:95); the loose-ends register (:209-210) "Each loose end, with where it is resolved"; Q-A1 (:233) is the precedent for an agent's reading in "The critic's remaining items and the answered escalations"; the row (:234) sits beside it, per the ruling's condition 3.
- C6 RR-1's own words (review-r1-findings.json, RR-1): "sim's small, #[inline] and generic functions are generated in the crate that calls them"; no "most". Its fix: PM-0002-53's test "lives in a package that depends on sim and runs in release, native and wasm32" (origin of the row's phrase).
- C7 ADR 0002 :5 "Author: engine-architect (the "Pass marks" table: test-engineer, plan step 4)". Appendix A :1586 "R5-RUSTLINT-REC4 | adopted | part 4"; REC4 (r5:1353-1358): checks "for sim and protocol", reason "a per-package override is allowed for this setting (F13, inferred), so the rest of the workspace keeps the release default"; route A changes both.
- C8 security-auditor r2: SA2-1, SA2-2 (Medium), SA2-3 (Low); rust-reviewer r2: RR2-1 (Medium), RR2-2..4 (Low); test-engineer r2: TE2-1..7 (Low). Read for overlap; not re-reviewed.
- C9 my flag check (scratch /home/user/Shamble-Frontlines/.gate/engine-architect-r2/head, `git archive 4bccf55` of Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/; cargo 1.99.1, --locked --offline): native release `-C overflow-checks=on` on all six crates; wasm32 release on the five Wasm crates; bench on sim and room; dev: no flag (rustc's default, on).
- C10 Cargo Book, Profiles (doc.rust-lang.org/cargo/reference/profiles.html, read 2026-11-13, official): "To override the settings for all dependencies (but not any workspace member), use the "*" package name"; precedence: named package, then "*" ("For any non-workspace member"), build-override, [profile.*], defaults; dev overflow-checks true, release false; test inherits dev, bench inherits release; "Cargo only looks at the profile settings in the Cargo.toml manifest at the root of the workspace." Sure: high.
- C11 E12's wasm32 leg (xcrate-check.sh:107-140): a scratch cdylib with `#[unsafe(no_mangle)]` exports and no `[lints] workspace = true`. My own lint run was refused by the guard (relative write after cd); test-engineer's run shows the workspace lints refuse it ("usage of the unsafe `no_mangle` attribute", -D unsafe-code).
- C12 REPO_MAP:18 unchanged since 4d78ad6; D13's fix_ok point 3 (add "the aim cast, the block writer and its layout table") not applied; report:156 says "D13 ... fixed: 1de98a9" (RR2-4 (b) has it).
- C13 M7: `git diff 297d48a^ 8d622ac` empty; 297d48a removes 3 lines of Cargo.toml only. ADR 0002's text: no diff 4d78ad6..4bccf55.
- C14 advisor consultation (before done): narrowed EA2-3 to the Appendix A row; moved the cost and protocol-anchor points to my revision notes; EA2-4 narrowed to what TE2-3 and WASM-ARTIFACT do not say; overlap marked "concur"; citations fixed.

## Verdict
APPROVE from my angle (module boundaries, the ADR register). 0 Critical, 0 High, 1 Medium (EA2-1), 3 Low (EA2-2, EA2-3, EA2-4). Nothing calls for a round 3. EA2-1 rides SA2-1's edit of the same paragraph if a push happens; otherwise all four become STATUS follow-ups.

## Findings
- EA2-1 Medium (concurs with test-engineer's TE2-7, which rates it Low), docs/reports/2026-11-12-foundation-2-2.md:295-296 (decision entry OVERFLOW-ALL). Defect: "A review found that this misses most of the simulation's code as soon as another crate calls it" states a share nobody measured and words the review wider than it was. Evidence: RR-1's words, "sim's small, #[inline] and generic functions are generated in the crate that calls them" (C6); the row (docs/adr/README.md:234), Cargo.toml:22 and As built (report:74) all say "inline, generic and small functions"; no sim code exists. Same paragraph, :294: "turns on overflow checks for the simulation by naming two crates" (protocol is the message codec). Not High: the decision and the recommendation stand with the corrected wording. Not Low: it is the text Mustafa decides from, and it disagrees with the row his go covers. Fix: "misses the simulation's small, inline and generic functions whenever another crate calls them, because that code is then compiled inside the caller"; :294 "for the simulation and the message protocol, by naming their two crates".
- EA2-2 Low, report:293-310 vs docs/adr/README.md:234. Defect: the entry asks only about the setting, but his go "for this row" also records two rules the entry never states. Evidence: the row also records (1) dependencies keep the release default until the first third-party crate enters sim's dependency tree, decided then with its cost measured; (2) PM-0002-53's counter test moves from "a committed test in sim" (ADR 0002:1319) to a crate that depends on sim. CLAUDE.md asks each entry to be self-contained. Fix: one sentence: "The row also records that outside crates keep the default until the first one enters the simulation's dependencies (decided then, with its cost measured), and that the planned overflow test moves to a crate that uses the simulation."
- EA2-3 Low, docs/adr/README.md:234 (state column). A gap in the ruling's condition 3, which the row copies faithfully; not a recording defect. Defect: the hand-off list for ADR 0002's next revision names part 4, Appendix A's R1-RR-OQ4 answer and PM-0002-53, not Appendix A's R5-RUSTLINT-REC4 row, which route A changes. Evidence: C7 (0002:1586 "adopted"; r5:1353-1358 scope and reason). SA2-2 cites REC4's reason for the panic side; the ledger row is this finding's addition. Fix: "part 4, its Appendix A rows for R1-RR-OQ4 and R5-RUSTLINT-REC4 (adopted becomes changed) and PM-0002-53".
- EA2-4 Low, docs/adr/README.md:234 and report:231 (OVERFLOW-XCRATE-CI; STATUS_ARCHIVE:617-631). A gap in the ruling's wording, copied by the row; concurs with TE2-3 on the four builds and the missing runner. Defect: "runs from a crate that depends on sim, in release, native and wasm32" leaves open where the wasm32 half can live under part 3. Evidence: room and client-endpoint take no wasm-bindgen (ADR 0002:202-205), and PM-0002-44 plants exactly "wasm-bindgen ... added to client-endpoint" (:1310), so a wasm-bindgen-test dev-dependency there depends on whether 2.3's G5 counts dev-dependencies (undecided); sim-wasm holds "the wasm-bindgen bindings ... and nothing else" (:192) and has none until the boundary spike; a raw export needs part 4's unsafe exception (C11); the anchor can fall at 2.3, before sim-wasm has bindings (TE2-2). The crate must also be a root-workspace member: Cargo reads profiles only from the root manifest (C10). Fix: the row and the follow-up say "a root-workspace crate that depends on sim; where its wasm32 half lives is a part 3 question for ADR 0002's owner, settled in 2.3's plan (the native half first if needed)".

## Concur, not counted
- TE2-7 = EA2-1. TE2-3 (four builds, runner) and TE2-2 (anchor at 2.3) overlap EA2-4, which adds the part 3 host and the membership.
- RR2-1 (the "*" trigger: any workspace crate's dependencies; shared generics at opt-level 0, 1, "s", "z" reach members): it also covers my draft note that the anchor names sim's tree while part 4 names sim and protocol.
- RR2-4 (a) "comes first" in sim-wasm lib.rs:2-3 (I had noted the same ambiguity) and (b) D13's REPO_MAP:18 half (my round-1 EA-2): Low, as it says.
- SA2-2: route A's panic side belongs in the row's hand-off; part 4's explicit-operations rule reaching every workspace crate is mine to reword.

## Notes for my revision of part 4 (not findings)
- PM-0002-53's cost clause measures sim step time; route A also checks room's and the server's own arithmetic, whose cost falls in the server tick (ADR 0001's budget, not read this session).
- PM-0002-53 sits in the pass-marks table credited to test-engineer (0002:5): reword it with test-engineer.
- The dependency anchor: part 4 names sim and protocol; RR2-1's wider trigger replaces it.

## Checked and found right
- Row form: "the advisor agent's ruling (D1, 2026-11-13), never his decision; his go for this row: PR #48's merge" (not worded as given); engine-architect rewords part 4, R1-RR-OQ4's answer and PM-0002-53 at the next revision; ADR 0002's text untouched (C13).
- Row content matches condition 3 item for item: the two tables, every workspace crate in every profile, dependencies at the release default, the anchor and its two options (a named override beats "*", C10), PM-0002-53 from a crate that depends on sim ("a test inside sim's own package cannot see the gap": RR-1, test-engineer's run), M7 and E12.
- Placement follows the README's scope lines (C5); the table's heading fits only by Q-A1's precedent.
- Entry, row and Cargo.toml:21-29 agree on the setting, the reach and the release-default dependencies; (a) "your merge #48 covers the row" = the row's "his go for this row: PR #48's merge"; (b) = condition 4 (part 4's blocks back, comments narrowed, row out, an open High deferred on his word, a new head, the advisor's review, his go again). Exceptions: EA2-1, EA2-2.
- C9: every member gets `-C overflow-checks=on` in native release, wasm32 release and bench; dev is on by default.
- client-endpoint lib.rs:1-4 lists part 3's constraints (table :190: no bindings, transport, async runtime, clock; built for Wasm inside sim-wasm and natively for the headless clients; rules :202-205); "2.4-2.5" fits 2.4's layout table (PM-0002-15/-16 through PM-0002-63) and 2.5's skeleton.
- sim-wasm lib.rs:1-3: bindings wait for the spike's exports (OQ-0002-3; plan Part 3 rows 1-2); the by-day reading first (OQ-0002-7's decider, :1082-1083; plan Part 3 row 1).
- sim and room lib.rs:2 "the skeleton step (foundation 2.5)": 2.5's done-when (ADR 0002:1467-1472) holds ADR 0004's first-recording milestone (0004:1252); room's first code is 2.5 (D15 skeptic c11).
- EMPTY-SLOTS (report:259-263; archive :786-793): the D14 suggested text, with the throwaway spike's use named; OQ-0002-8-TEST (report:264-268; archive :794-802): the D15 text, facts plus an open question for the Advisor, PM-0002-44 and PM-0002-64 named; CLIENT-NET-RULE (report:250; archive :769-777): the D16 limits, PM-0002-64, the part 3 wording needing his go.
- ROOT-WORKSPACE (report:249; archive :758-768), my round-1 EA-1: exclude tested, the --config note for 2.4.
- REPO_MAP:16, ROADMAP 2.2 entry (eight planted failures: M1a, M1b, M2-M7), STATUS: match route A; his go on Q2.2-a and OVERFLOW-ALL "all still waiting".
- M7's revert exact (C13).

## Not verified
- M7's and E12's CI and local runs (rust-reviewer and test-engineer re-ran them; I did not). My C9 checks flags, not behaviour.
- "*" on a non-member, and dependency generic code instantiated in a member (TE2-1, RR2-1, the ruling's C5): not run by me.
- The workspace lints refusing `no_mangle`: my run was refused by the guard; I cite test-engineer's.
- Whether 2.3's G5 counts dev-dependencies (undecided).
- The ADR names no step for sim's first code; "2.5" is a reading (if 2.4's protocol types nest sim types, sim gets code at 2.4).
- ADR 0001's server-tick budget (for the cost note).

## Scratch leftovers (not deleted)
- /home/user/Shamble-Frontlines/.gate/engine-architect-r2/ (head/, target/, target-w/, target-b/, target-d/). The guard refused the whole lint-experiment command, so nothing in head/ was modified (head/crates/sim-wasm/src/lib.rs cmp-identical to 4bccf55's).
