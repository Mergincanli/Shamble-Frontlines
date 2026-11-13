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
- Advisor tool consulted once, after orientation and before the edits: keep the plan; CUT-ESCAPE says "shown locally,
  not on the runner" with no local git version; G0 keeps "M2: exit 2" and does not import "(exit 1 in 2.2's job)";
  expand only from skeptic-confirmed text (not EA-1's cargoConfigFiles clause, not SA-2's "relies on" or container
  example, not "all three jobs restore and save", no `--no-self-update`); nothing open worded as decided; STATUS cuts
  from 2.2-owned text; check the name sets by script; report the two discrepancies first.
- Report diff 4d78ad6..HEAD: only G0, OVERFLOW-WASM -> OVERFLOW-XCRATE-CI, REQUIRED-CHECK, SETUP-NODE-CACHE, the eight
  new 2.3 rows, OVERFLOW-PER-CRATE, EMPTY-SLOTS and OQ-0002-8-TEST changed in the two lists (WASM-ARTIFACT's row did not;
  its archive text named OVERFLOW-WASM).
- DONE docs/STATUS_ARCHIVE.md, "From foundation 2.2": intro rewritten (sources, codes, provenance); G0, REQUIRED-CHECK,
  SETUP-NODE-CACHE reworded in place; OVERFLOW-WASM replaced in place by OVERFLOW-XCRATE-CI; WASM-ARTIFACT joins
  OVERFLOW-XCRATE-CI; a "From review round 1" group with the eight new 2.3 items after CRATE-LICENCE; OVERFLOW-PER-CRATE
  rewritten, EMPTY-SLOTS reworded, OQ-0002-8-TEST added after it. Each repository line re-checked today is tagged
  [verified 2026-11-13]; reviewer and skeptic runs stay attributed. Three cross-item sentences then attributed (CUT-CARRY's
  link to CARGO-NET-OFFLINE and TWO-PLAYWRIGHTS; CANCEL-ON-MAIN's queue reading; FEATURE-UNIFY's G5 widening).
- DONE docs/STATUS.md: Last updated 2026-11-13; #48's row (round 1 done: 5 reviewers, 17 findings, each checked by a
  skeptic, all hold; fix pass 1de98a9 to 8a4f9b0; route A on the ruling D1, M7; waits on round 2's four reviewers, the
  verifier, the Fable advisor's whole read before ready, the report, the advisor agent's final-head review, his go with
  K1, Q2.2-a and OVERFLOW-ALL); Waiting on Mustafa: K1, Q2.2-a and OVERFLOW-ALL, "all still waiting" (2 lines); the 2.2
  follow-ups list (29 + 5 names; its trailing archive pointer dropped, the section header gives it; 4 -> 5 lines); Next
  item 2 rewrapped to 5 lines ("then" and the pointer's repeated date 2026-11-10 dropped) to stay at 120 lines.
- Checks: STATUS.md 120 lines; names by script: report 29 + 5 = archive = STATUS (same order for 2.3), no duplicates;
  no OVERFLOW-WASM left except the archive's "replaces OVERFLOW-WASM"; repo-map-check "repo map OK: 120 entries cover
  107 required paths", exit 0; his quoted STATUS lines untouched; new lines all <= 120 characters (680, 681 and 800 are
  older, unchanged lines); git status: only the two docs and this file.
- Stale lines seen and left (renumbered): STATUS.md:21 and :46 (was :47; his Windows run); STATUS.md:56 (Routines read
  2026-11-10); STATUS.md:92-94 (2.1b's follow-ups have no archive block); ROADMAP.md:3; README.md:16-26.
- Advisor tool consulted a second time, before handback: the intro overclaimed ("the crates", "2.2's plan" among the
  2026-11-13 lines; neither is tagged in an item) and SETUP-NODE-CACHE's run ids had lost their [assumed] tag. Fixed:
  the intro now says every repository line the new and reworded items cite (toolchain.yml, gate.yml, Cargo.toml, the
  root package.json, ADR 0002, CLAUDE.md, advisor.md, the guard; each re-read today), and [assumed] lists
  "SETUP-NODE-CACHE's runs". G2 checked: ADR 0002 :571, the Rust lint stage. Line citations into the current STATUS
  shifted by the Next-item rewrap (lines 41-95 up by one): only the report's `docs/STATUS.md:47` (report :267-268); the
  rest are dated citations of the pre-diet STATUS. Re-checked: no new line over 120; names 4 of 4 same; STATUS 120;
  repo-map-check exit 0. Ready to hand back.
