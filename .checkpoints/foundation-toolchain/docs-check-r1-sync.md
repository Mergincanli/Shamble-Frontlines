# docs-check-r1-sync: adversarial fact check of docs-keeper's review round 1 sync (foundation 2.2, PR #48)

Agent: adversarial fact checker (workflow step). Worktree: .gate/foundation-toolchain-wt, branch
claude/laughing-pasteur-fwya24, HEAD 8632249. Edits: none outside this notes file.

## Scope
docs-keeper's uncommitted edits (`git diff`) to docs/STATUS_ARCHIVE.md ("From foundation 2.2", :551-802) and
docs/STATUS.md (:5, :29, :35-39, :42, :95-99). Sources: the report docs/reports/2026-11-12-foundation-2-2.md (findings
table :218-248, follow-ups :250-269, Reviews :137-166, Decisions needed :271-305, Appendix A :315-486), the ruling
advisor-ruling-d1.md, review-r1-findings.json, review-r1-verdicts.json, skeptic-r1-D*.md, and the repo files cited.

## Plan (units, each saved as done)
1. Mechanical checks: STATUS line count, repo-map-check exit, git status --short, OVERFLOW-WASM grep, name sets
   report vs archive vs STATUS (by script), quotes of Mustafa vs Appendix A, line lengths.
2. Every [verified 2026-11-13] citation re-read: toolchain.yml, gate.yml, Cargo.toml, package.json, ADR 0002 lines,
   CLAUDE.md :119/:130, advisor.md lines, routine-guard.mjs:1486, plan.md Part 3.
3. Each reworded/new archive item vs the report row + the reviewer finding + the skeptic verdict: G0, OVERFLOW-XCRATE-CI,
   REQUIRED-CHECK, SETUP-NODE-CACHE, WASM-ARTIFACT, CUT-CARRY, CUT-ESCAPE, SETPRIV-HARDEN, CANCEL-ON-MAIN,
   LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE, OVERFLOW-PER-CRATE, EMPTY-SLOTS, OQ-0002-8-TEST.
4. The archive intro (codes, provenance tags) and STATUS's #48 row, Waiting on Mustafa, Next item 2, follow-ups.
5. Decisions of Mustafa's: nothing recorded as given (K1, Q2.2-a, OVERFLOW-ALL all still asked).

## Progress
- plan written (this file).
- Unit 1 (mechanical) DONE: STATUS.md 120 lines; repo-map-check "repo map OK: 120 entries cover 107 required paths",
  exit 0; git status --short: the two docs + docs-keeper-r1-sync.md (M) + this file (??), nothing else; OVERFLOW-WASM in
  docs/ only at STATUS_ARCHIVE.md:614 ("it replaces OVERFLOW-WASM"), rest in .checkpoints notes; names by script
  (scratchpad names.mjs): report 29 + 5 = archive = STATUS, no extras, no dups; STATUS order = report; archive order
  differs only by NODE-26's older position (after SAVER-SKIP-CI, pre-existing). Lines: archive 680/681/800 >120 are old
  lines; STATUS :29 (#48 table row) is 1436 chars (a table row; docs-keeper's "every new line <= 120" is not true of it).
- Unit 2 (repo citations) DONE, all hold on the branch: toolchain.yml :3, :16-18, :21-22, :46 (sed, so M2's exit 2 under
  bash -e), :47-52 (exit 1 branches), :48-50, :68-73, :69 (setpriv exact), :74 (id; ip -o link), :78, :82-87;
  gate.yml :15-17; setup-node 820762786026 = 8207627 (v7.0.0) at toolchain.yml:37, gate.yml:29 and :60; Cargo.toml :2-4,
  :25-29; root package.json: no packageManager, no devEngines; ADR 0002 :207-209, :212-217, :345-346, :424, :433-436,
  :569 (G0 exit 2 on any setup failure), :571 (G2 Rust lint), :574 (G5 same-feature check), :578 (G9), :581 (G12),
  :642, :1082-1084 (#4533), :1095-1102 (answered by the Advisor 2026-11-03, (a)), :1303 (PM-0002-37, 1 run), :1305,
  :1308, :1310 (PM-0002-44, no phase), :1319 (test in sim), :1323 (PM-0002-57), :1330 (PM-0002-64, 2.5, 1-minute),
  :1421-1422 (foundation checklist item 2); CLAUDE.md :119, :130; advisor.md :157, :163, :224, :273-276, :294, :323;
  routine-guard.mjs:1486; register docs/adr/README.md :95 (Q2.2-a, "His go for this row: PR #48's merge") and :234
  (OVERFLOW-ALL: never his decision, go = #48's merge, engine-architect rewords part 4/PM-0002-53). Kit grep: other
  gate+smoke hits are comments (guard :6), a deny message (guard :990), the gate skill's --no-smoke, or gate.yml itself.
- Unit 3 (items vs findings + verdicts) DONE for: G0 (IE-4, D8, D17: holds; IE-4's 2nd mutation "install step removed
  under RUSTUP_AUTO_INSTALL=0" kept by D8 fix_ok 3 but listed only as a fact, not a mutation, in report :227 and archive
  :607-613: Low), REQUIRED-CHECK (IE-1, SA-4, D2: holds, incl. :294, guard :6/:990 as "comments"), SETUP-NODE-CACHE
  (SA-3, D6: holds), CUT-CARRY (IE-3, D7, D17: holds), CUT-ESCAPE (SA-1, D4: holds; no git version, right: local runs
  were git 2.43.0, SA notes :41, D4 notes :30; work-folder route is "reasoning only" in D4 fix_ok (1), unmarked: Low),
  SETPRIV-HARDEN (SA-2, D5: holds), CANCEL-ON-MAIN (IE-5, D9: holds), LINT-TABLE-PEDANTIC (RR-2, D10: holds),
  FEATURE-UNIFY (RR-3, D11: holds), ROOT-WORKSPACE (EA-1, D12: holds, §4 per D12), EMPTY-SLOTS (EA-3, D14: the archive
  uses D14's suggested text; "a foundation step" is D14's reading, plan.md:269 says "the step that adds wasm-bindgen":
  Low), OQ-0002-8-TEST (EA-4, D15; ADR :1099-1102 Advisor's answer; :222-229 reasons: holds), CLIENT-NET-RULE (EA-5,
  D16: holds but archive :766 names only "PM-0002-22's counter" for both page rules; the Wasm rule is PM-0002-2's
  counter (ADR :1265; D16 notes :36; PM-0002-64 keeps both): Low). All 17 verdicts hold at reviewer severity.
- Unit 4 (intro, STATUS) DONE: fix-pass range 1de98a9..8a4f9b0 holds (git log; report last changed 8a4f9b0); main still
  2aa5e9c (ls-remote); E12 figures hold (xcrate-before/after txt: 1 of 5 then 5 of 5; wasm32 release 0 then trap);
  plan.md:269-274 supports EMPTY-SLOTS' clauses; skeptic-r1-D3.md check 3 has the four cancelled reverts (2-4 s);
  STATUS #48 row, Waiting on Mustafa, Next item 2 (only "then" and a repeated date dropped), follow-ups: hold.
- Unit 5 (his decisions/words) DONE: no go recorded as given (K1, Q2.2-a, OVERFLOW-ALL: "asked", "still waiting"); the
  diff adds no quote of his (quoted strings: code, section names, ADR :1454-1455's done-when, toolchain.yml:78).

## Candidate findings (before the advisor consult)
1. Medium, report :227: G0 evidence "M2 meanwhile (exit 1 in 2.2's job)"; M2 exited 2 (report :112; D8, D17; job
   130098974545; toolchain.yml:46 under bash -e). Archive :600/:608-609 right. Fix the report.
2. Medium, report :201-202 and :241: "shown locally with git 2.56.0"; local runs were git 2.43.0 (SA notes :41; D4
   notes :30); 2.56.0 = runner's git, source read only (D4 notes :37). Archive :705 right (no version). Fix the report.
3. Low, archive :766: "PM-0002-22's counter" for both page rules; Wasm rule = PM-0002-2 (ADR :1265, :1330; D16).
4. Low, archive :607-613 + report :227: IE-4's 2nd mutation (install step removed under RUSTUP_AUTO_INSTALL=0) kept by
   D8 fix_ok 3, absent from the mutation list (only "never seen working").
5. Low, archive :704-706: CUT-ESCAPE marks only the fsmonitor route; work-folder route is reasoning only (D4 fix_ok 1,
   2 asks each route marked); SA-1's routes b (pathname sockets/resolver) and c (signals) dropped.
6. Low, archive :783-784 + report :258: "a foundation step" attributed to 2.2's plan Part 3; plan.md:269 says "the
   step that adds wasm-bindgen"; "foundation" is D14's reading (fix_ok 3).
7. Low, archive :569-570: EMPTY-SLOTS left [assumed] without a [verified] tag for its plan clauses (plan not in the
   2026-11-13 list); E12's figures likewise untagged (docs-keeper.md:25, :32). Content holds.
8. Low, report :267-268: `docs/STATUS.md:47` is now :46 after the sync's rewrap of Next item 2.
9. Low, STATUS :29: hand-back's "every new line <= 120" untrue for the #48 table row (1436 chars, was 1093); allowed by
   one line per PR (docs-keeper.md:14); hand-back claim only.

## After the advisor consult (1)
- Unchanged rows: the report's table first appeared at 03b5a54 (after the archive's last commit 13feadc), so the 19
  rows unchanged since 4d78ad6 were compared with their archive text by script (scratchpad side.mjs): all 19 match in
  meaning (AUTO-ASK ... BRIEF-STALE). No unsynced row.
- Tracked: review-r1-findings.json, review-r1-verdicts.json and all 17 skeptic-r1-D*.md are in git (19 of 19).
- OVERFLOW-WASM, git grep over tracked files outside .checkpoints/: only STATUS_ARCHIVE.md:614 ("it replaces").
- lightningcss: skeptic-r1-D4.md:58-62 calls "lightningcss runs inside the cut" a small overstatement (lazy import);
  archive :703 and report :200 carry it: folded into the CUT-ESCAPE finding.
- Severity of 1 and 2: Medium; ADR-INDEX rests on M2 failing closed (report :278), not its exit code; no decision entry
  mentions the git version. Candidate 9 moved to "checked" (no file to fix).
- Final re-run: STATUS 120 lines; repo-map-check exit 0; git status: the two docs + this notes file; docs diff 203/43
  unchanged, so cited lines hold. HEAD 6947c63 (saver autosave).

## Final findings
1. Medium, report :227 (confirmed docs-keeper's report): "M2 meanwhile (exit 1 in 2.2's job)"; fixer: main session.
2. Medium, report :202 and :241 (confirmed): "git 2.56.0" for the local run (was 2.43.0); fixer: main session.
3. Low, archive :766: PM-0002-22 named for both page rules; add PM-0002-2.
4. Low, archive :606-613 and report :227: IE-4's install-step mutation (kept by D8) missing from the mutation list.
5. Low, archive :703-706: CUT-ESCAPE: lightningcss overstatement (D4 :58-62); work-folder and file-command routes
   unmarked (D4 fix_ok 1-2); SA-1's routes b and c dropped.
6. Low, archive :783 and report :258: "a foundation step" attributed to the plan (plan.md:269: "the step"); it is D14's
   own suggested text.
7. Low, archive :569-570: EMPTY-SLOTS' plan clauses and E12's figures carry no [verified]/[assumed] tag (both hold).
8. Low, report :268 (confirmed): `docs/STATUS.md:47` is now :46 after the sync's rewrap; fixer: main session.
- After the advisor consult (2): docs-keeper's "no other current citation into the shifted STATUS range" holds: git grep
  outside .checkpoints/ finds only dated pre-diet citations (LEGACY_INVENTORY.md rows, r1 :841 etc., r2 :456 etc.,
  kit-v3-4 report :110) besides the 2.2 report's :267-268. Candidate 9 (table row length) is a refuted hand-back claim,
  reported under "checked", not as a finding.
- DONE: findings 1-8 above returned via StructuredOutput (repoMapExit 0, statusLines 120).
