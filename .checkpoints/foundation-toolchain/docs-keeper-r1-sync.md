# docs-keeper, foundation 2.2 review round 1 sync (PR #48): plan and progress

Role: main's `.claude/agents/docs-keeper.md` (read with `git show origin/main:...`). Worktree only:
`/home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt` (branch claude/laughing-pasteur-fwya24; brief's head
8a4f9b0; HEAD now 990cce4, a checkpoint commit touching only `.checkpoints/`). Edits: docs/STATUS_ARCHIVE.md and
docs/STATUS.md only. Never the report, DECISIONS.md, docs/adr/, REPO_MAP, ROADMAP, CLAUDE.md, .claude/, .github/.
No commit, no push. His words: not quoted (pointers to the report and DECISIONS.md).

## Sources (the report is the source of truth)
- The report `docs/reports/2026-11-12-foundation-2-2.md`: "Findings for 2.3" (:217-248), "Follow-ups for later steps"
  (:250-269), "Reviews" (:137-166), Risks (:184-212).
- The advisor agent's ruling D1 (`advisor-ruling-d1.md`): conditions 5 (OVERFLOW-XCRATE-CI) and 6 (stale lines).
- Round 1's notes: review-r1-findings.json (20 findings), review-r1-verdicts.json (17 skeptic verdicts, all hold),
  skeptic-r1-D*.md, security-auditor-review-r1.md.

## Facts re-checked against the repository now (2026-11-13) [verified]
- toolchain.yml :3 (2.3 replaces the file), :16-18 (cancel-in-progress, group by ref), :21-22 (RUSTUP_AUTO_INSTALL=0),
  :37 (setup-node 8207627, no cache input), :46-52 (pin check; its branches exit 1), :68-73 (setpriv keeps the uid; no
  --inh-caps / --bounding-set), :78 ("still reachable" branch), :82-87 (the build and test commands).
- gate.yml :15-17 (the same cancel block), :29 and :60 (setup-node 8207627).
- Root package.json: no packageManager, no devEngines. Cargo.toml: no `exclude`; the route-A tables (:24-28).
- crates/sim-wasm/Cargo.toml: no crate type (a plain library); its lib.rs doc line (OQ-0002-3, OQ-0002-7).
- ADR 0002: :207-209 (part 3's TypeScript rules), :212-217 (features set once, cross-build comparison), :345-346
  (TS_RS_EXPORT_DIR in .cargo/config.toml, no folder), :424 (no-restricted-imports for part 3 only), :433-436 (lint
  table and pedantic), :569 (G0, exit 2), :578 (G9, no cut), :581 (G12's outbound rule), :642, :1082-1084 (OQ-0002-7's
  decider), :1095-1102 (OQ-0002-8 (a)), :1286 (PM-0002-22), :1303 (PM-0002-37, "1 run"), :1310 (PM-0002-44),
  :1319 (PM-0002-53, "a committed test in sim"), :1330 (PM-0002-64, phase 2.5), :1421 (checklist item 2), :1486
  (navbake, slice 2).
- CLAUDE.md :119 and :130; advisor.md :157, :163, :224, :273, :294, :323; routine-guard.mjs:1486 (gate, smoke only).
- plan.md Part 3 row (:269): "the by-day reading is done before the step that adds wasm-bindgen".

## Discrepancies seen in the report (not mine to edit; for the caller)
- Report :201-202 (Risks) and :241 (CUT-ESCAPE): "shown locally with git 2.56.0"; security-auditor-review-r1.md:41
  and skeptic-r1-D4.md:30 ran it on git 2.43.0; 2.56.0 is the runner's version (CI log), whose source the skeptic read.
- Report :227 (G0's evidence): "M2 meanwhile (exit 1 in 2.2's job)"; M2's row (:112) and skeptics D8 (C1) and D17
  read exit 2 at the file read; exit 1 is what the pin check's other branches give (toolchain.yml:47-52).

## Plan, one file at a time
1. docs/STATUS_ARCHIVE.md, "From foundation 2.2" (:551-641):
   a. the intro: the report's tables after review round 1's fix pass (1de98a9, 8a4f9b0) and the ruling D1 are the
      source; codes defined (S1-S5, K1, K1b, M2, M7, E12, D1-D17, reviewer ids, PM/OQ, G0/G3/G5); provenance:
      [verified] by docs-keeper (2026-11-12 list, and 2026-11-13 for the new and reworded items' repository facts),
      SETUP-NODE-CACHE [verified] from setup-node's source (SA-3 read it; skeptic D6 ran its shipped detection code);
      the round-1 items rest on the reviewers' and skeptics' notes (readable at the branch's last checkpoint commit);
      [assumed] as before minus SETUP-NODE-CACHE's job log; M5's runs out (historical), M7's runs in.
   b. reworded in place: G0, REQUIRED-CHECK, SETUP-NODE-CACHE, WASM-ARTIFACT (joins OVERFLOW-XCRATE-CI).
   c. OVERFLOW-WASM replaced in place by OVERFLOW-XCRATE-CI (Medium, the advisor agent's label; its anchor).
   d. a third group "From review round 1": CUT-CARRY, CUT-ESCAPE, SETPRIV-HARDEN, CANCEL-ON-MAIN,
      LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE (the report's order).
   e. later steps: OVERFLOW-PER-CRATE rewritten; EMPTY-SLOTS reworded; OQ-0002-8-TEST added after it.
2. docs/STATUS.md (at most 120 lines; 120 now): Last updated; #48's row (round 1 and its fix pass done, route A; waits
   on round 2, the verifier, the Fable advisor's whole read before ready, the report, the advisor agent's review of
   the final head, his go); Waiting on Mustafa (his go on K1, Q2.2-a and OVERFLOW-ALL, all still waiting); the 2.2
   follow-ups name list. Room: rewrap only; never touch his quoted lines (:11-12, :18-19).
3. Checks: wc -l STATUS.md; repo-map-check; grep for any OVERFLOW-WASM left; git status (only the two docs and this
   file).

## Progress
- plan written (this file).
